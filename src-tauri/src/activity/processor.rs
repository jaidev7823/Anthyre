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
/// Modified batching function:
/// - Events: each hour is its own batch (1-hour blocks)
/// Modified batching function:
/// - Events: each hour is its own batch (1-hour blocks)
/// - Multi-hour events appear in EACH hour they span
/// - Free time: batch all consecutive free hours together
pub fn make_batches(hours: Vec<HourBlock>, events: Vec<Value>) -> Vec<Batch> {
    // Map each hour to all events that occur during that hour
    let mut hours_with_events: std::collections::HashMap<u8, Vec<Value>> = std::collections::HashMap::new();
    
    for ev in &events {
        if let Some(start_str) = ev["start"]["dateTime"].as_str() {
            if let Some(end_str) = ev["end"]["dateTime"].as_str() {
                if let (Ok(start_dt), Ok(end_dt)) = (
                    DateTime::parse_from_rfc3339(start_str),
                    DateTime::parse_from_rfc3339(end_str)
                ) {
                    let start_hour = start_dt.hour() as u8;
                    let end_hour = end_dt.hour() as u8;
                    let end_minute = end_dt.minute();
                    
                    // If event ends exactly on the hour (minute=0), don't include that hour
                    let actual_end_hour = if end_minute == 0 && end_hour > start_hour {
                        end_hour - 1
                    } else {
                        end_hour
                    };
                    
                    for hour in start_hour..=actual_end_hour {
                        hours_with_events
                            .entry(hour)
                            .or_insert_with(Vec::new)
                            .push(ev.clone());
                    }
                }
            }
        }
    }

    let mut batches = Vec::new();
    let mut free_batch_start: Option<u8> = None;
    let mut event_batch_start: Option<u8> = None;
    let mut current_events: Vec<Value> = vec![];

    for h in &hours {
        let hour_events = hours_with_events.get(&h.hour_24).cloned().unwrap_or_default();
        let has_event = !hour_events.is_empty();

        if has_event {
            // Close any open free batch
            if let Some(start) = free_batch_start {
                let prev_hour = if h.hour_24 > 0 { h.hour_24 - 1 } else { 0 };
                batches.push(Batch {
                    start_hour: start,
                    end_hour: prev_hour,
                    label: format!("Free: {} - {}", start, prev_hour),
                    is_event: false,
                    events: vec![],
                });
                free_batch_start = None;
            }

            // Check if we can continue batching (same single event)
            let can_batch = hour_events.len() == 1 
                && current_events.len() == 1 
                && events_are_same(&hour_events[0], &current_events[0]);

            if can_batch {
                // Continue batching the same single event
                // (event_batch_start stays the same)
            } else {
                // Close previous event batch if exists
                if let Some(start) = event_batch_start {
                    let prev_hour = if h.hour_24 > 0 { h.hour_24 - 1 } else { 0 };
                    batches.push(Batch {
                        start_hour: start,
                        end_hour: prev_hour,
                        label: format!("Event: {} - {}", start, prev_hour),
                        is_event: true,
                        events: current_events.clone(),
                    });
                }
                
                // Start new event batch
                event_batch_start = Some(h.hour_24);
                current_events = hour_events;
            }
        } else {
            // Close any open event batch
            if let Some(start) = event_batch_start {
                let prev_hour = if h.hour_24 > 0 { h.hour_24 - 1 } else { 0 };
                batches.push(Batch {
                    start_hour: start,
                    end_hour: prev_hour,
                    label: format!("Event: {} - {}", start, prev_hour),
                    is_event: true,
                    events: current_events.clone(),
                });
                event_batch_start = None;
                current_events = vec![];
            }

            // Start or continue free time batch
            if free_batch_start.is_none() {
                free_batch_start = Some(h.hour_24);
            }
        }
    }

    // Close any remaining event batch
    if let Some(start) = event_batch_start {
        batches.push(Batch {
            start_hour: start,
            end_hour: 23,
            label: format!("Event: {} - 23", start),
            is_event: true,
            events: current_events,
        });
    }

    // Close any remaining free batch
    if let Some(start) = free_batch_start {
        batches.push(Batch {
            start_hour: start,
            end_hour: 23,
            label: format!("Free: {} - 23", start),
            is_event: false,
            events: vec![],
        });
    }

    batches
}

/// Helper function to check if two events are the same
fn events_are_same(ev1: &Value, ev2: &Value) -> bool {
    // Compare by ID if available, otherwise by summary and time
    if let (Some(id1), Some(id2)) = (ev1.get("id"), ev2.get("id")) {
        return id1 == id2;
    }
    
    ev1.get("summary") == ev2.get("summary") 
        && ev1.get("start") == ev2.get("start")
        && ev1.get("end") == ev2.get("end")
}