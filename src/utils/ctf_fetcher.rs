use crate::types::ctf::{CtfEmbedContent, CtfTimeResponse};
use anyhow::Result;
use chrono::DateTime;

pub async fn fetch(url: String) -> Result<CtfEmbedContent> {
    let id = url
        .split("/")
        .last()
        .ok_or(anyhow::anyhow!("Invalid URL"))?;

    let response = reqwest::get(&format!("https://ctftime.org/api/v1/events/{}/", id))
        .await?
        .json::<CtfTimeResponse>()
        .await?;

    let start_timestamp = DateTime::parse_from_str(&response.start, "%Y-%m-%dT%H:%M:%S%z")?;
    let end_timestamp = DateTime::parse_from_str(&response.finish, "%Y-%m-%dT%H:%M:%S%z")?;

    Ok(CtfEmbedContent {
        ctftime_url: response.ctftime_url,
        title: response.title,
        start: start_timestamp,
        finish: end_timestamp,
        url: response.url,
        logo: response.logo,
        description: response.description,
    })
}
