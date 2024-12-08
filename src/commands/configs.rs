use crate::types::{
    global::{Data, Error},
    preference::Preferences,
};
use poise::serenity_prelude::{Channel, ChannelId};

#[poise::command(slash_command)]
pub async fn configserver(
    ctx: poise::ApplicationContext<'_, Data, Error>,
    #[channel_types("Text")] channel_id: ChannelId,
    #[channel_types("Category")] category_id: Channel,
) -> Result<(), Error> {
    let server_id = match ctx.guild_id() {
        Some(id) => id,
        None => {
            ctx.say(":x: This command cannot be run in a DM!").await?;
            return Ok(());
        }
    };

    let category = match category_id.category() {
        Some(cat) => cat.id,
        None => {
            ctx.say(":x: The \"category\" field was not a valid category!")
                .await?;
            return Ok(());
        }
    };

    match sqlx::query_as::<_, Preferences>(
        "INSERT INTO prefs (server_id, channel_id, category_id) VALUES ($1, $2, $3) ON CONFLICT (server_id) DO UPDATE SET channel_id = EXCLUDED.channel_id;",
    )
    .bind(server_id.to_string())
    .bind(channel_id.to_string())
    .bind(category.to_string())
    .fetch_optional(&ctx.data().pool)
    .await
    {
        Ok(_) => {
            ctx.say(":white_check_mark: Saved/updated configurations!")
                .await?;
        }
        Err(e) => {
            ctx.say(format!(":x: **Something went wrong!**\nLog: {}", e))
                .await?;
        }
    }

    Ok(())
}
