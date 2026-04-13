use dotenv::dotenv;
use fluoris::{Data, age, ping, verify};
use poise::serenity_prelude::{self as serenity};
use sqlx::postgres::PgPoolOptions;

#[tokio::main]
//Connect to discord and start up bot
async fn main() {
    dotenv().ok();
    let token = std::env::var("DISCORD_TOKEN").expect("missing DISCORD_TOKEN");
    let intents = serenity::GatewayIntents::non_privileged();

    let sql_url = std::env::var("SQL_CONNECT").expect("missing SQL Connection Url");

    let pool = PgPoolOptions::new()
        .max_connections(20)
        .connect(&sql_url)
        .await
        .expect("fail to connect to sql");

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![age::age(), ping::ping(), verify::verify()],
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data { pool })
            })
        })
        .build();

    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await;

    client.unwrap().start().await.unwrap();
}
