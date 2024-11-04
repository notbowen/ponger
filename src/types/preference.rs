use poise::{
    serenity_prelude::{Channel, ChannelId, GuildChannel},
    Modal,
};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Serialize, Deserialize, FromRow)]
pub struct Preferences {
    pub server_id: String,
    pub channel_id: String,
    pub category_id: String,
}
