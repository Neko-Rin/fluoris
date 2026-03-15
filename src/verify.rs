use crate::{Context, Error};
use poise::serenity_prelude as serenity;

///Checks if a user is verfied
#[poise::command(slash_command)]
pub async fn verify(
    ctx: Context<'_>,
    #[description = "Selected user"] user: Option<serenity::User>,
) -> Result<(), Error> {
    let u = user.as_ref().unwrap_or_else(|| ctx.author());
    let Some(g) = ctx.guild_id() else {
        let _ = ctx.say("You can't use this in dms").await;
        return Ok(());
    };
    let r = serenity::RoleId::new(1482837780932460788);
    let role = u.has_role(ctx.http(), g, r).await.unwrap_or(false);
    let response = format!("{} is verified is {}", u.name, role);
    ctx.say(response).await?;
    Ok(())
}
