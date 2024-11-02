use crate::types::{
    global::{Data, Error},
    preference::AnnouncementPrefs,
};
use poise::serenity_prelude as serenity;

#[poise::command(slash_command)]
pub async fn setchannel(
    ctx: poise::ApplicationContext<'_, Data, Error>,
    #[description = "Channel"] channel_id: serenity::ChannelId,
) -> Result<(), Error> {
    let server_id = match ctx.guild_id() {
        Some(id) => id,
        None => {
            ctx.say(":x: This command cannot be run in a DM!").await?;
            return Ok(());
        }
    };

    match sqlx::query_as::<_, AnnouncementPrefs>(
        "INSERT INTO prefs (server_id, channel_id) VALUES ($1, $2) ON CONFLICT (server_id) DO UPDATE SET channel_id = EXCLUDED.channel_id;",
    )
    .bind(server_id.to_string())
    .bind(channel_id.to_string())
    .fetch_optional(&ctx.data().pool)
    .await
    {
        Ok(_) => {
            ctx.say(":white_check_mark: Saved/updated announcement channel!")
                .await?;
        }
        Err(e) => {
            ctx.say(format!(":x: **Something went wrong!**\nLog: {}", e))
                .await?;
        }
    }

    Ok(())
}
