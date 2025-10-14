use chrono::{DateTime, Duration, Timelike, Utc};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;

// Assuming this struct exists in your codebase
use crate::auth::get_latest_token;
use crate::daily_report::get_calendar_events;

#[derive(Serialize, Deserialize, Debug)]
pub struct HourBlock {
    pub hour_24: u8,
    pub label: String,
    pub start: String,
    pub end: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Batch {
    pub start_hour: u8,
    pub end_hour: u8,
    pub label: String,
    pub is_event: bool,
    pub events: Vec<Value>, // Add this field
}

#[tauri::command]
pub async fn fetch_batches() -> Result<Vec<Batch>, String> {
    let client = Client::new();

    // 1) Fetch latest token from DB
    let token_data = get_latest_token()?;
    let token = token_data.access_token;

    // 2) Build today's 24-hour blocks
    let hours = (0..24)
        .map(|h| {
            let hour_12 = if h % 12 == 0 { 12 } else { h % 12 };
            let suffix = if h < 12 { "AM" } else { "PM" };
            let label = format!("{}:00 {} - {}:59 {}", hour_12, suffix, hour_12, suffix);
            HourBlock {
                hour_24: h as u8,
                label,
                start: format!("{:02}:00", h),
                end: format!("{:02}:59", h),
            }
        })
        .collect::<Vec<HourBlock>>();

    // 3) Today range
    let local_now = chrono::Local::now();

    println!("Current UTC time: {}", local_now);
    println!(
        "Current local time: {}",
        local_now.with_timezone(&chrono::Local)
    );
    let start_of_day = local_now
        .date_naive()
        .and_hms_opt(0, 0, 0)
        .unwrap()
        .and_utc();
    println!("Start of day UTC: {}", start_of_day);
    let end_of_day = start_of_day + Duration::hours(24);

    // 4) Fetch events
    let events = get_calendar_events(&client, &token, start_of_day, end_of_day).await?;
    // 5) Make batches
    Ok(make_batches(hours, events))
}

/// Existing batching function
pub fn make_batches(hours: Vec<HourBlock>, events: Vec<Value>) -> Vec<Batch> {
    let mut event_hours: Vec<u8> = Vec::new();
    for ev in &events {
        if let Some(start) = ev["start"]["dateTime"].as_str() {
            if let Ok(dt) = DateTime::parse_from_rfc3339(start) {
                event_hours.push(dt.hour() as u8);
            }
        }
    }

    let mut batches = Vec::new();
    let mut current_start: Option<u8> = None;
    let mut current_end: Option<u8> = None;
    let mut current_is_event = false;

    for h in &hours {
        let has_event = event_hours.contains(&h.hour_24);

        if let (Some(start), Some(end)) = (current_start, current_end) {
            if has_event != current_is_event {
                batches.push(Batch {
                    start_hour: start,
                    end_hour: end,
                    label: if current_is_event {
                        format!("Event: {} - {}", start, end)
                    } else {
                        format!("Free: {} - {}", start, end)
                    },
                    is_event: current_is_event,
                    events: events
                        .iter()
                        .filter(|ev| {
                            if let Some(start_str) = ev["start"]["dateTime"].as_str() {
                                if let Ok(dt) = DateTime::parse_from_rfc3339(start_str) {
                                    // use outer start/end, not start_str
                                    return dt.hour() as u8 >= start && dt.hour() as u8 <= end;
                                }
                            }
                            false
                        })
                        .cloned()
                        .collect(),
                });
                current_start = Some(h.hour_24);
                current_end = Some(h.hour_24);
                current_is_event = has_event;
            } else {
                current_end = Some(h.hour_24);
            }
        } else {
            current_start = Some(h.hour_24);
            current_end = Some(h.hour_24);
            current_is_event = has_event;
        }
    }

    if let (Some(start), Some(end)) = (current_start, current_end) {
        batches.push(Batch {
            start_hour: start,
            end_hour: end,
            label: if current_is_event {
                format!("Event: {} - {}", start, end)
            } else {
                format!("Free: {} - {}", start, end)
            },
            is_event: current_is_event,
            events: events
                .iter()
                .filter(|ev| {
                    if let Some(start_str) = ev["start"]["dateTime"].as_str() {
                        if let Ok(dt) = DateTime::parse_from_rfc3339(start_str) {
                            // use outer start/end, not start_str
                            return dt.hour() as u8 >= start && dt.hour() as u8 <= end;
                        }
                    }
                    false
                })
                .cloned()
                .collect(),
        });
    }

    batches
}
