use std::str::FromStr;

use chrono::Utc;
use poise::serenity_prelude as serenity;

use poise::{serenity_prelude::ChannelId, Modal};

use crate::types::poll_modal::PollData;
use crate::utils::color::random_color;
use crate::utils::poll;
use crate::{
    types::{
        global::{Data, Error},
        poll_modal::PollModal,
    },
    utils::{ctf_sender::send_ctf, preference},
};

#[poise::command(slash_command)]
pub async fn send(
    ctx: poise::ApplicationContext<'_, Data, Error>,
    #[description = "CTFTime URL"] url: String,
) -> Result<(), Error> {
    send_ctf(ctx, url).await
}

#[poise::command(slash_command, guild_only)]
pub async fn poll(
    ctx: poise::ApplicationContext<'_, Data, Error>,
    duration: i64,
) -> Result<(), Error> {
    let data = PollModal::execute(ctx).await?;

    let data = match data {
        Some(d) => d,
        None => {
            ctx.say(":x: Missing modal response").await?;
            return Ok(());
        }
    };

    let descriptions: Vec<String> = data.description.split('\n').map(str::to_string).collect();
    let ctftime_urls: Vec<String> = data.urls.split(',').map(str::to_string).collect();

    let number_emojis = vec!["1️⃣", "2️⃣", "3️⃣", "4️⃣", "5️⃣", "6️⃣", "7️⃣", "8️⃣", "9️⃣", "🔟"];

    if descriptions.len() != ctftime_urls.len()
        || descriptions.len() > number_emojis.len()
        || descriptions.len() == 0
    {
        ctx.say(":x: Length mismatch / invalid length").await?;
        return Ok(());
    }

    let emoji_desc = descriptions
        .iter()
        .enumerate()
        .map(|(i, desc)| number_emojis[i].to_string() + " " + desc)
        .collect::<Vec<String>>()
        .join("\n");

    let embed = serenity::CreateEmbed::default()
        .title("CTF Poll")
        .description(emoji_desc)
        .color(random_color());

    let server_id = ctx.partial_guild().await.unwrap().id;
    let preferences = preference::get(&ctx.data.pool, server_id.to_string()).await?;

    let channel_id = ChannelId::from_str(&preferences.channel_id)?;
    let message = channel_id
        .send_message(&ctx.http(), serenity::CreateMessage::new().embed(embed))
        .await?;

    let data = PollData {
        message_id: message.id.to_string(),
        end_time: (Utc::now().timestamp()) + (duration * 3600) as i64,
    };

    poll::set(&ctx.data.pool, data).await?;

    // TODO: Helper function to schedule poll closing

    Ok(())
}
