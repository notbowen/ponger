use std::str::FromStr;

use crate::{
    types::{
        global::{Data, Error},
        preference::AnnouncementPrefs,
    },
    utils::{color::random_color, ctf_fetcher::fetch},
};
use poise::serenity_prelude::{self as serenity, ChannelId};

#[poise::command(slash_command)]
pub async fn send_ctf(
    ctx: poise::ApplicationContext<'_, Data, Error>,
    #[description = "CTFTime URL"] url: String,
) -> Result<(), Error> {
    let server_id = match ctx.guild_id() {
        Some(id) => id,
        None => {
            ctx.say(":x: This command cannot be run in a DM!").await?;
            return Ok(());
        }
    };

    let channel_id =
        match sqlx::query_as::<_, AnnouncementPrefs>("SELECT * FROM prefs WHERE server_id = $1")
            .bind(&server_id.to_string())
            .fetch_one(&ctx.data().pool)
            .await
        {
            Ok(p) => ChannelId::from_str(&p.channel_id)?,
            Err(e) => {
                ctx.say(format!(":x: **Something went wrong!**\nLog: {}", e))
                    .await?;
                return Ok(());
            }
        };

    let response = match fetch(url).await {
        Ok(res) => res,
        Err(e) => {
            ctx.say(format!(
                ":x: **Something went wrong!**\nAre you sure the provided URL was valid?\nLog: {}",
                e
            ))
            .await?;
            return Ok(());
        }
    };

    let embed = serenity::CreateEmbed::default()
        .title(response.title)
        .description(format!(
            // TODO: Add a calendar link
            "**Start**: {}\n**End**: {}\n\n**URL**: {}\n\n**Description**:\n{}",
            format!("<t:{}:R>", response.start.timestamp()),
            format!("<t:{}:R>", response.finish.timestamp()),
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
        .send_message(&ctx.http(), serenity::CreateMessage::new().embed(embed))
        .await?;

    ctx.say(":white_check_mark: Sent!").await?;
    Ok(())
}
