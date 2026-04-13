use crate::{Context, Error};
use poise::{CreateReply, serenity_prelude as serenity};

#[derive(serde::Deserialize, Debug)]
pub struct McData {
    id: String,
    name: String,
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
    let mut response = "You are not verified".to_string();

    // checks if minecraft username is vaild
    let mc_api_format = format!("https://api.mojang.com/users/profiles/minecraft/{}", name);
    let res = reqwest::get(mc_api_format).await?;

    //checks if user has the role
    if role {
        response = "You are verified sucessfully".to_string();
        //checks if api returns an error or not
        if res.status().is_success() {
            // deserialize the data to get ready to write to sql
            let max_boba_type = res.json::<McData>().await?;
            let user = ctx.author().id.to_string();

            //Writes to sqlx data base tells user if the mc user is already in data base
            if let Err(_max_tax_type) =
                sqlx::query("INSERT INTO MC (MCID, MCUser, Discord) VALUES ($1,$2,$3)")
                    .bind(max_boba_type.id)
                    .bind(max_boba_type.name)
                    .bind(user)
                    .execute(&ctx.data().pool)
                    .await
            {
                response = "User already registered".to_string()
            };
        } else {
            response = "Invaild username. Please enter a valid Minecraft username.".to_string();
        }
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
