use chrono::{DateTime, FixedOffset};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CtfTimeResponse {
    pub ctftime_url: String,
    pub title: String,
    pub start: String,
    pub finish: String,
    pub url: String,
    pub logo: String,
    pub description: String,
}

pub struct CtfEmbedContent {
    pub ctftime_url: String,
    pub title: String,
    pub start: DateTime<FixedOffset>,
    pub finish: DateTime<FixedOffset>,
    pub url: String,
    pub logo: String,
    pub description: String,
}
