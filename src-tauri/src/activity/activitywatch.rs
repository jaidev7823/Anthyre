use chrono::{DateTime, Utc};
use reqwest::Client;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AwEvent {
    pub duration: f64,
    pub data: AwEventData,
}

#[derive(Debug, Deserialize)]
pub struct AwEventData {
    pub app: Option<String>,
    pub title: Option<String>,
}

pub async fn get_aw_events(
    client: &Client,
    start: DateTime<Utc>,
    end: DateTime<Utc>,
) -> Result<Vec<AwEvent>, String> {
    let url = "http://localhost:5600/api/0/buckets/aw-watcher-window_DESKTOP-9R9SJ3O/events";
    let resp = client
        .get(url)
        .query(&[("start", start.to_rfc3339()), ("end", end.to_rfc3339())])
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if !resp.status().is_success() {
        return Err(format!("Failed to fetch AW events: {}", resp.status()));
    }

    resp.json::<Vec<AwEvent>>().await.map_err(|e| e.to_string())
}

