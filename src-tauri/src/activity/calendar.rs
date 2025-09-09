use chrono::{DateTime, Utc};
use reqwest::Client;
use serde::Serialize;

#[derive(Serialize)]
struct EventDateTime {
    dateTime: String,
}

#[derive(Serialize)]
struct CalendarEvent {
    summary: String,
    description: String,
    start: EventDateTime,
    end: EventDateTime,
}

pub async fn add_calendar_event(
    client: &Client,
    token: &str,
    summary: &str,
    description: &str,
    start: DateTime<Utc>,
    end: DateTime<Utc>,
) -> Result<(), String> {
    let event = CalendarEvent {
        summary: summary.into(),
        description: description.into(),
        start: EventDateTime {
            dateTime: start.to_rfc3339(),
        },
        end: EventDateTime {
            dateTime: end.to_rfc3339(),
        },
    };

    let resp = client
        .post("https://www.googleapis.com/calendar/v3/calendars/primary/events")
        .bearer_auth(token)
        .json(&event)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if resp.status().is_success() {
        println!("✅ Event created: {} -> {}", start, end);
        Ok(())
    } else {
        Err(format!("Failed: {}", resp.status()))
    }
}
