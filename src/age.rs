use crate::{Context, Error};
use poise::serenity_prelude as serenity;
use serenity::builder::CreateCommand;

//register the command with discord API
pub fn register() -> CreateCommand {
    CreateCommand::new("age").description("Displays your or another user's account creation date")
}

//Returns the age of discord account
#[poise::command(slash_command // /age,
    )]
pub async fn age(
    ctx: Context<'_>,
    #[description = "Selected user"] user: Option<serenity::User>,
) -> Result<(), Error> {
    let u = user.as_ref().unwrap_or_else(|| ctx.author());
    let response = format!("{}'s account was created at {}", u.name, u.created_at());
    ctx.say(response).await?;
    Ok(())
}
