use std::str::FromStr;

use crate::{
    types::{
        global::{Data, Error},
        preference::Preferences,
    },
    utils::{color::random_color, ctf_fetcher::fetch},
};
use poise::serenity_prelude::{self as serenity, ChannelId, EditRole, ReactionType};

use super::preference;

pub async fn send_ctf(
    ctx: poise::ApplicationContext<'_, Data, Error>,
    url: String,
) -> Result<(), Error> {
    let server_id = match ctx.guild_id() {
        Some(id) => id,
        None => {
            ctx.say(":x: This command cannot be run in a DM!").await?;
            return Ok(());
        }
    };

    let preferences = preference::get(&ctx.data.pool, server_id.to_string()).await?;

    let channel_id = ChannelId::from_str(&preferences.channel_id)?;
    let category_id = ChannelId::from_str(&preferences.category_id)?;

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
        .title(&response.title)
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

    let mut message = channel_id
        .send_message(
            &ctx.http(),
            serenity::CreateMessage::new().embed(embed.clone()),
        )
        .await?;

    ctx.say(":white_check_mark: Sent!").await?;

    let role_name = response.title.replace(" ", "-").to_lowercase();
    let role = server_id
        .create_role(
            &ctx.http(),
            EditRole::new()
                .name(&role_name)
                .mentionable(true)
                .colour(random_color()),
        )
        .await?;

    match sqlx::query("INSERT INTO reactions VALUES ($1, $2)")
        .bind(message.id.to_string())
        .bind(role.id.to_string())
        .fetch_optional(&ctx.data().pool)
        .await
    {
        Ok(_) => {
            ctx.say(format!(
                ":white_check_mark: Created role <@&{}>!",
                role.id.to_string()
            ))
            .await?;
        }
        Err(e) => {
            ctx.say(format!(":x: **Something went wrong!**\nLog: {}", e))
                .await?;
        }
    };

    message
        .edit(
            ctx.http(),
            serenity::EditMessage::new()
                .content(format!(
                    ":triangular_flag_on_post: New CTF Alert! :triangular_flag_on_post:\nReact to this message to obtain the <@&{}> role!\n",
                    role.id.to_string()
                ))
                .embed(embed),
        )
        .await?;

    message
        .react(&ctx.http(), ReactionType::Unicode("🔥".to_string()))
        .await?;

    server_id
        .create_channel(
            &ctx.http(),
            serenity::CreateChannel::new(&role_name)
                .kind(serenity::ChannelType::Text)
                .category(category_id)
                .permissions(vec![
                    serenity::PermissionOverwrite {
                        allow: serenity::Permissions::empty(),
                        deny: serenity::Permissions::VIEW_CHANNEL,
                        kind: serenity::PermissionOverwriteType::Role(server_id.everyone_role()),
                    },
                    serenity::PermissionOverwrite {
                        allow: serenity::Permissions::VIEW_CHANNEL,
                        deny: serenity::Permissions::empty(),
                        kind: serenity::PermissionOverwriteType::Role(role.id),
                    },
                ]),
        )
        .await?;

    Ok(())
}
