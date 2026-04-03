use crate::{Context, Error};
use poise::{CreateReply, serenity_prelude as serenity};

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

    #[derive(serde::Deserialize, Debug)]
    pub struct McData {
        id: String,
        name: String,
    }

    //Put the role you are checking for here, speficially the role id
    let r = serenity::RoleId::new(1482837780932460788);
    let role = u.has_role(ctx.http(), g, r).await.unwrap_or(false);
    let mut response = format!("You are not verified");

    // checks if minecraft username is vaild
    let mc_api_format = format!("https://api.mojang.com/users/profiles/minecraft/{}", name);
    let res = reqwest::get(mc_api_format).await?;

    //checks if user has the role
    if role {
        response = format!("You are verified sucessfully");
        //checks if api returns an error or not
        if res.status().is_success() {
            // deserialize the data to get ready to write to sql
            let max_boba_type = res.json::<McData>().await?;
            println!("{}", max_boba_type.name);
            println!("{}", max_boba_type.id);
        } else {
            response = format!("Invaild username. Please enter a valid Minecraft username.");
            println!("Goodbye")
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
