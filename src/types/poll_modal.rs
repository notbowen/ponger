use chrono::{DateTime, Utc};
use poise::{serenity_prelude::MessageId, Modal};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Debug, Modal)]
pub struct PollModal {
    #[placeholder = "Message to send participants (supports markdown)"]
    #[paragraph]
    pub description: String,
    #[placeholder = "List of comma-seperated CTFtime URLs"]
    #[paragraph]
    pub urls: String,
}

#[derive(Serialize, Deserialize, FromRow)]
pub struct PollData {
    pub message_id: String,
    pub end_time: i64,
}
