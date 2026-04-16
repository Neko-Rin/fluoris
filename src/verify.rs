use crate::{Context, Error};
use poise::{
    CreateReply,
    serenity_prelude::{self as serenity},
};
use reqwest::Response;

#[derive(serde::Deserialize, Debug)]
pub struct McData {
    pub id: String,
    pub name: String,
}

///Checks if a user is verfied
#[poise::command(slash_command)]
pub async fn verify(
    ctx: Context<'_>,
    #[description = "MC User"] name: String,
) -> Result<(), Error> {
    let u = ctx.author();
    let Some(g) = ctx.guild_id() else {
        let _ = ctx.say("You can't use this in dms").await;
        return Ok(());
    };

    //Put the role you are checking for here, speficially the role id
    let r = serenity::RoleId::new(1482837780932460788);
    let role = u.has_role(ctx.http(), g, r).await.unwrap_or(false);

    // checks if minecraft username is vaild
    let mc_api_format = format!("https://api.mojang.com/users/profiles/minecraft/{}", name);
    let res = reqwest::get(mc_api_format).await?;

    let mut response = run_verify(role, &res);
    let user = ctx.author().id.to_string();
    if response == "You are verified sucessfully" {
        let max_boba_type = deserial(res).await?;
        //Writes to sqlx data base tells user if the mc user is already in data base
        response = sql_write(max_boba_type, user, &ctx.data().pool).await;
    }

    //sends the message so that only the user can see
    ctx.send(CreateReply {
        content: Some(response),
        ephemeral: Some(true),
        ..Default::default()
    })
    .await?;
    Ok(())
}

//checks if the user is verified and puts an vaild MC user name
pub fn run_verify(role: bool, res: &Response) -> String {
    if !role {
        return "You are not verified".to_string();
    }
    if !res.status().is_success() {
        return "Invaild username. Please enter a valid Minecraft username.".to_string();
    }
    "You are verified sucessfully".to_string()
}

pub async fn deserial(res: Response) -> Result<McData, Error> {
    let mc = res.json::<McData>().await?;
    Ok(mc)
}

use sqlx::PgPool;
pub async fn sql_write(max_boba_type: McData, user: String, pool: &PgPool) -> String {
    if let Err(_max_tax_type) =
        sqlx::query("INSERT INTO MC (MCID, MCUser, Discord) VALUES ($1,$2,$3)")
            .bind(max_boba_type.id)
            .bind(max_boba_type.name)
            .bind(user)
            .execute(pool)
            .await
    {
        return "User already registered".to_string();
    };
    "You are verified sucessfully".to_string()
}

#[cfg(test)]

mod tests {
    use super::*;
    use mockito::Server;
    use reqwest::Client;
    use sqlx::PgPool;

    async fn mock_response(server: &mut mockito::Server, status: usize) -> reqwest::Response {
        server
            .mock("GET", "/verify")
            .with_status(status)
            .create_async()
            .await;

        Client::new()
            .get(format!("{}/verify", server.url()))
            .send()
            .await
            .unwrap()
    }

    #[tokio::test]
    async fn test_verify() {
        let mut server = Server::new_async().await;
        let valid = mock_response(&mut server, 200).await;
        let invalid = mock_response(&mut server, 404).await;

        assert_eq!(
            run_verify(false, &valid),
            "You are not verified".to_string()
        );
        assert_eq!(
            run_verify(true, &valid),
            "You are verified sucessfully".to_string()
        );
        assert_eq!(
            run_verify(false, &invalid),
            "You are not verified".to_string()
        );
        assert_eq!(
            run_verify(true, &invalid),
            "Invaild username. Please enter a valid Minecraft username.".to_string()
        );
    }

    #[tokio::test]
    async fn test_deserialze() {
        async fn mock_response(server: &mut mockito::Server, status: usize) -> reqwest::Response {
            server
                .mock("GET", "/verify")
                .with_status(status)
                .with_header("content-type", "application/json")
                .with_body(
                    serde_json::json!({
                        "id": "720087e9f8a54aba9816cc8b91577880",
                        "name": "DepressedMao"
                    })
                    .to_string(),
                )
                .create_async()
                .await;

            Client::new()
                .get(format!("{}/verify", server.url()))
                .send()
                .await
                .unwrap()
        }

        let mut server = Server::new_async().await;
        let res = mock_response(&mut server, 200).await;

        let mc = deserial(res).await.unwrap();

        assert_eq!(mc.id, "720087e9f8a54aba9816cc8b91577880");
        assert_eq!(mc.name, "DepressedMao");
    }

    #[sqlx::test]
    async fn test_sql_write(pool: PgPool) {
        let hi = McData {
            id: "hi".to_string(),
            name: "bye".to_string(),
        };

        let max_tax_type = sql_write(hi, "tax".to_string(), &pool).await;
        assert_eq!(max_tax_type, "You are verified sucessfully".to_string());
    }

    #[sqlx::test]
    async fn test_sql_write_duplicate(pool: PgPool) {
        // Insert once
        let first = McData {
            id: "hi".to_string(),
            name: "bye".to_string(),
        };
        sql_write(first, "tax".to_string(), &pool).await;

        // Insert same data again — should hit duplicate
        let second = McData {
            id: "hi".to_string(),
            name: "bye".to_string(),
        };
        let result = sql_write(second, "tax".to_string(), &pool).await;

        assert_eq!(result, "User already registered");
    }
}
