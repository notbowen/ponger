use anyhow::anyhow;
use poise::serenity_prelude::{self as serenity, CreateMessage, RoleId};

use crate::types::{global::Data, reactions::RolesMapping};

pub async fn reaction_add_role(
    ctx: &serenity::Context,
    reaction: &serenity::Reaction,
    data: &Data,
) -> anyhow::Result<()> {
    // Check if reaction adder was a bot
    let user = reaction.user(&ctx.http).await?;
    if user.bot {
        return Ok(());
    }

    let guild_id = reaction.guild_id.ok_or(anyhow!("Not in a group!"))?;
    let user_id = user.id;

    let member = ctx.http.get_member(guild_id, user_id).await?;
    let message_id = reaction.message_id;

    let mapping =
        match sqlx::query_as::<_, RolesMapping>("SELECT * FROM reactions WHERE message_id = $1")
            .bind(message_id.to_string())
            .fetch_one(&data.pool)
            .await
        {
            Ok(m) => m,
            Err(_) => return Ok(()),
        };

    let role_id = mapping.role_id.parse::<u64>()?;
    let role = RoleId::new(role_id);

    match member.roles.iter().find(|r| r.get() == role_id) {
        Some(_) => return Ok(()),
        None => {}
    }

    member.add_role(&ctx.http, role).await?;

    member
        .user
        .direct_message(
            &ctx.http,
            CreateMessage::new().content(":white_check_mark: Successfully assigned role!"),
        )
        .await?;

    Ok(())
}

pub async fn reaction_remove_role(
    ctx: &serenity::Context,
    reaction: &serenity::Reaction,
    data: &Data,
) -> anyhow::Result<()> {
    let user = reaction.user(&ctx.http).await?;
    if user.bot {
        return Ok(());
    }

    let guild_id = reaction.guild_id.ok_or(anyhow!("Not in a group!"))?;
    let user_id = user.id;

    let member = ctx.http.get_member(guild_id, user_id).await?;
    let message_id = reaction.message_id;

    let mapping =
        match sqlx::query_as::<_, RolesMapping>("SELECT * FROM reactions WHERE message_id = $1")
            .bind(message_id.to_string())
            .fetch_one(&data.pool)
            .await
        {
            Ok(m) => m,
            Err(_) => return Ok(()),
        };

    let role_id = mapping.role_id.parse::<u64>()?;
    let role = RoleId::new(role_id);

    match member.roles.iter().find(|r| r.get() == role_id) {
        Some(_) => {}
        None => return Ok(()),
    }

    member.remove_role(&ctx.http, role).await?;

    member
        .user
        .direct_message(
            &ctx.http,
            CreateMessage::new().content(":white_check_mark: Successfully removed role!"),
        )
        .await?;

    Ok(())
}
