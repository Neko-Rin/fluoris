use crate::{Context, Error};

#[poise::command(slash_command // /ping, 
)]
pub async fn ping(ctx: Context<'_>) -> Result<(), Error> {
    let latency = ctx.ping().await;
    let response = format!(
        "pong, it took jelly fish {:?} to hit the ball back (🪼ྀིྀི⋆˚꩜｡)",
        latency
    );
    ctx.say(response).await?;
    Ok(())
}
