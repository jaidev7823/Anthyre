use chrono::{Duration, Local, Utc, TimeZone};
use reqwest::Client;
use serde_json::Value;

/// Get today’s start + end in UTC (from local time)
fn today_range_utc() -> (chrono::DateTime<Utc>, chrono::DateTime<Utc>) {
    let local_now = Local::now();

    // Start of day (local)
    let start_local = local_now.date_naive().and_hms_opt(0, 0, 0).unwrap();
    let start_local_dt = Local.from_local_datetime(&start_local).unwrap();

    // End of day (local)
    let end_local = start_local + Duration::days(1);
    let end_local_dt = Local.from_local_datetime(&end_local).unwrap();

    (start_local_dt.with_timezone(&Utc), end_local_dt.with_timezone(&Utc))
}

/// Fetch events from Google Calendar for today
async fn get_calendar_events(
    client: &Client,
    token: &str,
    start: chrono::DateTime<Utc>,
    end: chrono::DateTime<Utc>,
) -> Result<Vec<Value>, String> {
    let url = format!(
        "https://www.googleapis.com/calendar/v3/calendars/primary/events?timeMin={}&timeMax={}&singleEvents=true&orderBy=startTime",
        start.to_rfc3339(),
        end.to_rfc3339()
    );

    let resp = client
        .get(&url)
        .bearer_auth(token)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if !resp.status().is_success() {
        return Err(format!("Google Calendar API failed: {}", resp.status()));
    }

    let data: Value = resp.json().await.map_err(|e| e.to_string())?;
    Ok(data["items"].as_array().cloned().unwrap_or_default())
}

/// Collect event summaries + descriptions into one log string
fn collect_descriptions(events: &[Value]) -> String {
    let mut logs = Vec::new();

    for ev in events {
        let summary = ev
            .get("summary")
            .and_then(|s| s.as_str())
            .unwrap_or("No title");
        let desc = ev.get("description").and_then(|d| d.as_str()).unwrap_or("");
        let start = ev["start"]
            .get("dateTime")
            .or_else(|| ev["start"].get("date"))
            .and_then(|s| s.as_str())
            .unwrap_or("");
        let end = ev["end"]
            .get("dateTime")
            .or_else(|| ev["end"].get("date"))
            .and_then(|s| s.as_str())
            .unwrap_or("");

        if !desc.is_empty() {
            logs.push(format!("• {} ({} → {})\n  {}", summary, start, end, desc));
        } else {
            logs.push(format!("• {} ({} → {})", summary, start, end));
        }
    }

    logs.join("\n")
}

/// Send the log text to Ollama for summarization
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
        model: "mistral", // you can swap model here
        prompt: format!(
            "You are a brutally honest productivity coach.\n\nSummarize today’s logs:\n{}\n\nGive:\n1. Reality Check\n2. Brutal Strategy\n3. Fixes (3 action points).",
            raw_text
        ),
        stream: false,
    };

    let resp = client
        .post("http://localhost:11434/api/generate")
        .json(&payload)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    let data: OllamaResponse = resp.json().await.map_err(|e| e.to_string())?;
    Ok(data.response.replace("\\n", "\n").trim().to_string())
}

/// Get today’s brutal daily summary (callable from frontend)
#[tauri::command]
pub async fn get_daily_summary() -> Result<String, String> {
    let client = Client::new();

    // TODO: replace with your token retrieval logic (from SQLite/db)
    let token = crate::auth::get_latest_token().map_err(|e| format!("Token error: {}", e))?;

    let (start, end) = today_range_utc();
    let events = get_calendar_events(&client, &token.access_token, start, end).await?;

    if events.is_empty() {
        return Ok("❌ No events found for today.".into());
    }

    let raw_logs = collect_descriptions(&events);
    let summary = summarize_day(&client, &raw_logs).await?;

    Ok(summary)
}
