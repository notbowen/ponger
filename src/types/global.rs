use sqlx::PgPool;

pub struct Data {
    pub pool: PgPool,
}

pub type Error = Box<dyn std::error::Error + Send + Sync>;
