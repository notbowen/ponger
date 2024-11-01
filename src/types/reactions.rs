use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Serialize, Deserialize, FromRow)]
pub struct RolesMapping {
    pub message_id: String,
    pub role_id: String,
}
