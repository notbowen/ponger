use sqlx::PgPool;

use crate::types::{global::Error, preference::Preferences};

pub async fn get(pool: &PgPool, server_id: String) -> Result<Preferences, Error> {
    Ok(
        sqlx::query_as::<_, Preferences>("SELECT * FROM prefs WHERE server_id = $1")
            .bind(server_id)
            .fetch_one(pool)
            .await?,
    )
}
