use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Serialize, Deserialize, FromRow)]
pub struct AnnouncementPrefs {
    pub server_id: String,
    pub channel_id: String,
}
