use crate::{
    types::global::{Data, Error},
    utils::{color::random_color, ctf_fetcher::fetch},
};
use poise::serenity_prelude as serenity;

#[poise::command(slash_command)]
pub async fn send_ctf(
    ctx: poise::ApplicationContext<'_, Data, Error>,
    #[description = "Channel ID"] channel_id: serenity::ChannelId,
    #[description = "CTFTime URL"] url: String,
) -> Result<(), Error> {
    let response = match fetch(url).await {
        Ok(res) => res,
        Err(_) => {
            ctx.say(":x: **Something went wrong!**\nAre you sure the provided URL was valid?")
                .await?;
            return Ok(());
        }
    };

    let embed = serenity::CreateEmbed::default()
        .title(response.title)
        .description(format!(
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
