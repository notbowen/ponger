use poise::serenity_prelude as serenity;
use poise::serenity_prelude::{ClientBuilder, GatewayIntents};
use sqlx::postgres::PgPoolOptions;
use types::global::{Data, Error};

mod callbacks;
mod commands;
mod types;
mod utils;

#[tokio::main]
async fn main() {
    // Load .env variables
    if cfg!(debug_assertions) {
        dotenvy::dotenv().expect("Unable to load .env file");
    }

    let discord_token =
        std::env::var("DISCORD_TOKEN").expect("Unable to find environment variable DISCORD_TOKEN");

    let conn_string;
    if cfg!(not(debug_assertions)) {
        conn_string = std::env::var("CONN_STRING").unwrap();
    } else {
        conn_string = std::env::var("PROD_CONN_STRING").unwrap();
    }

    // Create Postgres pool
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&conn_string)
        .await
        .unwrap();

    // Run SQL migrations
    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("Unable to run migrations!");

    // Initialise bot
    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![
                commands::ctfs::send_ctf(),
                commands::configs::set_announcement_channel(),
            ],
            event_handler: |ctx, event, framework, data| {
                Box::pin(event_handler(ctx, event, framework, data))
            },
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data { pool })
            })
        })
        .build();

    let client = ClientBuilder::new(discord_token, GatewayIntents::non_privileged())
        .framework(framework)
        .await;

    client.unwrap().start().await.unwrap();
}

async fn event_handler(
    ctx: &serenity::Context,
    event: &serenity::FullEvent,
    _framework: poise::FrameworkContext<'_, Data, Error>,
    data: &Data,
) -> Result<(), Error> {
    match event {
        serenity::FullEvent::Ready { data_about_bot, .. } => {
            println!("Logged in as {}", data_about_bot.user.name);
        }
        serenity::FullEvent::ReactionAdd { add_reaction } => {
            callbacks::reaction::reaction_add_role(ctx, add_reaction, data).await?;
        }
        _ => {}
    }
    Ok(())
}
