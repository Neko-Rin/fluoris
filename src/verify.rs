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
    if response == "You are verified sucessfully".to_string() {
        let max_boba_type = deserial(res).await?;
        //Writes to sqlx data base tells user if the mc user is already in data base
        response = sql_write(max_boba_type, user, ctx).await;
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
    } else if !res.status().is_success() {
        return "Invaild username. Please enter a valid Minecraft username.".to_string();
    } else {
        return "You are verified sucessfully".to_string();
    }
}

pub async fn deserial(res: Response) -> Result<McData, Error> {
    let mc = res.json::<McData>().await?;
    Ok(mc)
}

pub async fn sql_write(max_boba_type: McData, user: String, ctx: Context<'_>) -> String {
    if let Err(_max_tax_type) =
        sqlx::query("INSERT INTO MC (MCID, MCUser, Discord) VALUES ($1,$2,$3)")
            .bind(max_boba_type.id)
            .bind(max_boba_type.name)
            .bind(user)
            .execute(&ctx.data().pool)
            .await
    {
        return "User already registered".to_string();
    };
    return "You are verified sucessfully".to_string();
}
