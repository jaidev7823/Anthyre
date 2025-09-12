use chrono::{DateTime, Local, TimeZone, Utc};
use reqwest::Client;
use serde_json::Value;

/// Get today’s start + end in UTC (from local time)
fn today_range_utc() -> (DateTime<Utc>, DateTime<Utc>) {
    let local_today = Local::now().date_naive();
    let start_local = local_today.and_hms_opt(0, 0, 0).unwrap();
    let end_local = local_today.and_hms_opt(23, 59, 59).unwrap();

    let start_local_dt = Local.from_local_datetime(&start_local).single().unwrap();
    let end_local_dt = Local.from_local_datetime(&end_local).single().unwrap();

    (start_local_dt.with_timezone(&Utc), end_local_dt.with_timezone(&Utc))
}


/// Fetch events from Google Calendar for today
pub async fn get_calendar_events(client: &Client, token: &str, start: DateTime<Utc>, end: DateTime<Utc>) -> Result<Vec<Value>, String> {
    let url = "https://www.googleapis.com/calendar/v3/calendars/primary/events";

    let resp = client
        .get(url)
        .bearer_auth(token)
        .query(&[
            ("timeMin", start.to_rfc3339()),
            ("timeMax", end.to_rfc3339()),
            ("singleEvents", "true".to_string()),
            ("orderBy", "startTime".to_string()),
        ])
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if !resp.status().is_success() {
        let status = resp.status();
        let body_text = resp.text().await.unwrap_or_else(|_| "<no body>".to_string());
        return Err(format!("Google Calendar API failed: {} - {}", status, body_text));
    }

    let data: Value = resp.json().await.map_err(|e| e.to_string())?;
    Ok(data["items"].as_array().cloned().unwrap_or_default())
}

/// Collect event summaries + descriptions into one log string
fn collect_descriptions(events: &[Value]) -> String {
    let mut logs = Vec::new();

    for ev in events {
        let summary = ev.get("summary").and_then(|s| s.as_str()).unwrap_or("No title");
        let desc = ev.get("description").and_then(|d| d.as_str()).unwrap_or("");
        let start = ev["start"].get("dateTime").or_else(|| ev["start"].get("date")).and_then(|s| s.as_str()).unwrap_or("");
        let end = ev["end"].get("dateTime").or_else(|| ev["end"].get("date")).and_then(|s| s.as_str()).unwrap_or("");

        if !desc.is_empty() {
            logs.push(format!("• {} ({} → {})\n  {}", summary, start, end, desc));
        } else {
            logs.push(format!("• {} ({} → {})", summary, start, end));
        }
    }

    logs.join("\n")
}
/// Summarize events using Ollama
async fn summarize_day(client: &Client, raw_text: &str) -> Result<String, String> {
    #[derive(serde::Serialize)]
    struct Payload<'a> {
        model: &'a str,
        prompt: String,
        stream: bool,
    }

    #[derive(serde::Deserialize)]
    struct OllamaResponse {
        response: String,
    }

    let payload = Payload {
        model: "mistral",
        prompt: format!(
            "You are a brutally honest productivity coach.\n\nSummarize today’s logs:\n{}\n\nGive:\n1. Reality Check\n2. Brutal Strategy\n3. Fixes (3 action points).",
            raw_text
        ),
        stream: false,
    };

    let resp = client.post("http://localhost:11434/api/generate").json(&payload).send().await.map_err(|e| e.to_string())?;
    let data: OllamaResponse = resp.json().await.map_err(|e| e.to_string())?;

    Ok(data.response.replace("\\n", "\n").trim().to_string())
}

/// Main function callable from frontend
#[tauri::command]
pub async fn get_daily_summary() -> Result<String, String> {
    let client = Client::new();

    // Get token
    let token = crate::auth::get_latest_token().map_err(|e| format!("Token error: {}", e))?;

    // Check expiry
    if Utc::now() > token.expiry_date {
        return Err("Access token expired".into());
    }

    // Get today's events
    let (start, end) = today_range_utc();
    let events = get_calendar_events(&client, &token.access_token, start, end).await?;

    if events.is_empty() {
        return Ok("❌ No events found for today.".into());
    }

    let raw_logs = collect_descriptions(&events);
    let summary = summarize_day(&client, &raw_logs).await?;

    Ok(summary)
}