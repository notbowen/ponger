use sqlx::PgPool;

use crate::types::{global::Error, poll_modal::PollData};

pub async fn get(pool: &PgPool, message_id: String) -> Result<PollData, Error> {
    // TODO
    Ok(PollData {
        message_id: "1".to_string(),
        end_time: 0,
    })
}

pub async fn set(pool: &PgPool, data: PollData) -> Result<(), Error> {
    sqlx::query_as::<_, PollData>("INSERT INTO polls (message_id, end_time) VALUES ($1, $2)")
        .bind(data.message_id)
        .bind(data.end_time)
        .fetch_optional(pool)
        .await?;

    Ok(())
}
