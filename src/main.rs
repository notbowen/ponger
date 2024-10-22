use anyhow::Context as _;
use chrono::{self, DateTime};
use poise::serenity_prelude as serenity;
use poise::serenity_prelude::CreateMessage;
use poise::serenity_prelude::{ClientBuilder, GatewayIntents};
use rand::RngCore;
use serde::Deserialize;
use shuttle_runtime::SecretStore;
use shuttle_serenity::ShuttleSerenity;

struct Data {}
type Error = Box<dyn std::error::Error + Send + Sync>;

#[derive(Debug, Deserialize)]
struct CtfTimeResponse {
    ctftime_url: String,
    title: String,
    start: String,
    finish: String,
    url: String,
    logo: String,
    description: String,
}

fn random_color() -> (u8, u8, u8) {
    let mut bytes = [0_u8; 3];
    rand::thread_rng().fill_bytes(&mut bytes);
    (bytes[0], bytes[1], bytes[2])
}

#[poise::command(slash_command)]
async fn send_ctf(
    ctx: poise::ApplicationContext<'_, Data, Error>,
    #[description = "Channel ID"] channel_id: serenity::ChannelId,
    #[description = "CTFTime URL"] url: String,
) -> Result<(), Error> {
    let id = url.split('/').last().unwrap();
    let response = reqwest::get(&format!("https://ctftime.org/api/v1/events/{}/", id))
        .await?
        .json::<CtfTimeResponse>()
        .await?;

    let start_timestamp = DateTime::parse_from_str(&response.start, "%Y-%m-%dT%H:%M:%S%z");
    let end_timestamp = DateTime::parse_from_str(&response.finish, "%Y-%m-%dT%H:%M:%S%z");

    let embed = serenity::CreateEmbed::default()
        .title(response.title)
        .description(format!(
            "**Start**: {}\n**End**: {}\n\n**URL**: {}\n\n**Description**:\n{}",
            format!("<t:{}:R>", start_timestamp.unwrap().timestamp()),
            format!("<t:{}:R>", end_timestamp.unwrap().timestamp()),
            response.url,
            response.description,
        ))
        .thumbnail(if response.logo.is_empty() {
            "https://ctftime.org/static/images/nologo.png".to_string()
        } else {
            response.logo
        })
        .url(response.ctftime_url)
        .color(random_color());

    channel_id
        .send_message(&ctx.http(), CreateMessage::new().embed(embed))
        .await?;

    ctx.say(":white_check_mark: Sent!").await?;
    Ok(())
}

#[shuttle_runtime::main]
async fn main(#[shuttle_runtime::Secrets] secret_store: SecretStore) -> ShuttleSerenity {
    let discord_token = secret_store
        .get("DISCORD_TOKEN")
        .context("'DISCORD_TOKEN' was not found")?;

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![send_ctf()],
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {})
            })
        })
        .build();

    let client = ClientBuilder::new(discord_token, GatewayIntents::non_privileged())
        .framework(framework)
        .await
        .map_err(shuttle_runtime::CustomError::new)?;

    Ok(client.into())
}
