extern crate dotenv;

use chrono::{self, DateTime};
use dotenv::dotenv;
use poise::serenity_prelude as serenity;
use poise::serenity_prelude::CreateMessage;
use poise::Modal;
use serde::Deserialize;

struct Data {}
type Error = Box<dyn std::error::Error + Send + Sync>;
// type Context<'a> = poise::Context<'a, Data, Error>;

#[derive(Debug, Modal)]
#[name = "CTF Details"]
struct CtfDetailModal {
    #[name = "Name of CTF"]
    #[placeholder = "GreyCTF 2024"]
    name: String,
    #[name = "Start of CTF"]
    #[placeholder = "20/04/2024 12:00 +0800"]
    start_date: String,
    #[name = "End of CTF"]
    #[placeholder = "21/04/2024 12:00 +0800"]
    end_date: String,
    #[name = "Link to CTF Platform"]
    #[placeholder = "https://ctf.nusgreyhats.org/"]
    url: String,
    #[name = "CTFTime URL"]
    #[placeholder = "https://ctftime.org/event/2242"]
    ctftime_url: Option<String>,
}

#[derive(Debug, Deserialize)]
struct CtfTimeResponse {
    ctftime_url: String,
    title: String,
    start: String,
    finish: String,
    url: String,
}

#[poise::command(slash_command)]
async fn grab_ctf_details(
    ctx: poise::ApplicationContext<'_, Data, Error>,
    #[description = "Channel ID"] channel_id: serenity::ChannelId,
    #[description = "CTFTime URL"] url: String,
) -> Result<(), Error> {
    let response = reqwest::get(&format!("https://ctftime.org/api/v1/events/{}/", url))
        .await?
        .json::<CtfTimeResponse>()
        .await?;

    let start_timestamp = DateTime::parse_from_str(&response.start, "%Y-%m-%dT%H:%M:%S%z");
    let end_timestamp = DateTime::parse_from_str(&response.finish, "%Y-%m-%dT%H:%M:%S%z");

    let embed = serenity::CreateEmbed::default()
        .title(response.title)
        .description(format!(
            "Start: {}\nEnd: {}\nURL: {}\nCTFTime: {}",
            format!("<t:{}:R>", start_timestamp.unwrap().timestamp()),
            format!("<t:{}:R>", end_timestamp.unwrap().timestamp()),
            response.url,
            response.ctftime_url
        ))
        .color(0x00ff00);

    channel_id
        .send_message(&ctx.http(), CreateMessage::new().embed(embed))
        .await?;

    ctx.say(":white_check_mark: Sent!").await?;
    Ok(())
}

#[poise::command(slash_command)]
async fn send_ctf_details(
    ctx: poise::ApplicationContext<'_, Data, Error>,
    #[description = "Channel ID"] channel_id: serenity::ChannelId,
) -> Result<(), Error> {
    let data = match CtfDetailModal::execute(ctx).await? {
        Some(data) => data,
        None => {
            ctx.say("Could not get CTF details").await?;
            return Ok(());
        }
    };

    let start_timestamp = DateTime::parse_from_str(&data.start_date, "%d/%m/%Y %H:%M %z");
    let end_timestamp = DateTime::parse_from_str(&data.end_date, "%d/%m/%Y %H:%M %z");

    let embed = serenity::CreateEmbed::default()
        .title(data.name)
        .description(format!(
            "Start: {}\nEnd: {}\nURL: {}{}",
            format!("<t:{}:R>", start_timestamp.unwrap().timestamp()),
            format!("<t:{}:R>", end_timestamp.unwrap().timestamp()),
            data.url,
            {
                match data.ctftime_url {
                    Some(url) => format!("\n**CTFTime**: {}", url),
                    None => "".to_string(),
                }
            }
        ))
        .color(0x00ff00);

    channel_id
        .send_message(&ctx.http(), CreateMessage::new().embed(embed))
        .await?;
    Ok(())
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let token = std::env::var("DISCORD_TOKEN").expect("Missing DISCORD_TOKEN");
    let intents = serenity::GatewayIntents::non_privileged();

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![grab_ctf_details(), send_ctf_details()],
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {})
            })
        })
        .build();

    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await;
    client.unwrap().start().await.unwrap();
}
