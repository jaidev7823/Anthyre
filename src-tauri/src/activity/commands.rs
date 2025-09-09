use chrono::{DateTime, Duration,Duration as ChronoDuration, Local, Timelike, Utc};
use reqwest::Client;
use serde::Deserialize;
use tokio::time::{sleep_until, Instant};

use crate::activity::{
    token::get_latest_token,
    activitywatch::get_aw_events,
    summarize::{summarize_events, summarize_with_ollama},
    calendar::add_calendar_event,
};

// === 6. Entry point ===
#[tauri::command]
pub async fn update_hours() -> Result<(), String> {
    let client = Client::new();
    let token = get_latest_token()?;

    if Utc::now() > token.expiry_date {
        return Err("Token expired".into());
    }

    // Work in local time to align with user's clock, then convert to UTC for APIs
    let now_local = Local::now();

    // End = top of current hour (e.g., 4:00 when it's 4:25 local)
    let end_local = now_local
        .with_minute(0)
        .unwrap()
        .with_second(0)
        .unwrap()
        .with_nanosecond(0)
        .unwrap();

    // Start = 1 hour before end
    let start_local = end_local - Duration::hours(1);

    // Convert to UTC for ActivityWatch and Google APIs
    let start = start_local.with_timezone(&Utc);
    let end = end_local.with_timezone(&Utc);

    println!("Processing {start} → {end} ...");

    let events = get_aw_events(&client, start, end).await?;
    let (event_title, raw_text) = summarize_events(&events);
    let description = summarize_with_ollama(&client, &raw_text).await?;

    add_calendar_event(
        &client,
        &token.access_token,
        &event_title,
        &description,
        start,
        end,
    )
    .await?;
    println!("✅ Hour {start} → {end} updated.");

    Ok(())
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RangeArgs {
    pub start_iso: String,
    pub end_iso: String,
    #[serde(default)]
    pub date: Option<String>,
    #[serde(default)]
    pub start: Option<String>,
    #[serde(default)]
    pub end: Option<String>,
}

#[tauri::command]
pub async fn update_hours_range(args: RangeArgs) -> Result<(), String> {
    let client = Client::new();
    let token = get_latest_token()?;

    if Utc::now() > token.expiry_date {
        return Err("Token expired".into());
    }

    // Expect RFC3339/ISO strings; parse into UTC
    let start = DateTime::parse_from_rfc3339(&args.start_iso)
        .map_err(|e| format!("Invalid start time: {}", e))?
        .with_timezone(&Utc);
    let end = DateTime::parse_from_rfc3339(&args.end_iso)
        .map_err(|e| format!("Invalid end time: {}", e))?
        .with_timezone(&Utc);

    if end <= start {
        return Err("End time must be after start time".into());
    }

    println!("Processing range {} → {} ...", start, end);

    let mut current_start = start;
    while current_start < end {
        let current_end = (current_start + ChronoDuration::hours(1)).min(end);

        println!("Processing block {} → {} ...", current_start, current_end);

        let events = get_aw_events(&client, current_start, current_end).await?;
        
        let (event_title, raw_text) = if events.is_empty() {
            ("No Activity".to_string(), "".to_string())
        } else {
            summarize_events(&events)
        };

        let description = if raw_text.is_empty() {
            "No activity recorded for this period.".to_string()
        } else {
            summarize_with_ollama(&client, &raw_text).await?
        };

        add_calendar_event(
            &client,
            &token.access_token,
            &event_title,
            &description,
            current_start,
            current_end,
        )
        .await?;
        
        current_start = current_end;
    }

    println!("✅ Range {} → {} updated.", start, end);

    Ok(())
}

// This function is no longer needed since we're using interval-based scheduling in lib.rs
// Keeping it for potential future use
pub async fn run_hourly_updates() -> Result<(), String> {
    let now = Local::now();

    // Round up to the *next* full minute
    let next_minute =
        now.with_second(0).unwrap().with_nanosecond(0).unwrap() + ChronoDuration::minutes(1);

    // Sleep until then
    let wait_duration = (next_minute - now).to_std().unwrap();
    println!("⏳ Waiting until {next_minute}...");

    sleep_until(Instant::now() + wait_duration).await;

    // Range = last minute
    let start_str = (next_minute - ChronoDuration::minutes(1)).to_rfc3339();
    let end_str = next_minute.to_rfc3339();

    let args = crate::activity::RangeArgs {
        start_iso: start_str.clone(),
        end_iso: end_str.clone(),
        date: None,
        start: None,
        end: None,
    };

    println!("▶️ Running update for {start_str} → {end_str}");
    crate::activity::update_hours_range(args).await
}
