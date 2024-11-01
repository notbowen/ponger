use anyhow::anyhow;
use poise::serenity_prelude::{self as serenity, RoleId};

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

    let member = reaction
        .member
        .clone()
        .ok_or(anyhow!("Could not find member"))?;
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

    member.add_role(&ctx.http, role).await?;

    Ok(())
}
