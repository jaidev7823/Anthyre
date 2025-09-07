use chrono::{DateTime, Duration, Timelike, Utc, Local};
use reqwest::Client;
use rusqlite::params;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::collections::HashMap as Map;
use crate::database;

#[derive(Debug)]
struct CalendarToken {
    access_token: String,
    refresh_token: String,
    expiry_date: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
struct AwEvent {
    duration: f64,
    data: AwEventData,
}

#[derive(Debug, Deserialize)]
struct AwEventData {
    app: Option<String>,
    title: Option<String>,
}

// === 1. Fetch token from DB ===
fn get_latest_token() -> Result<CalendarToken, String> {
    let conn = database::connection();
    let mut stmt = conn
        .prepare("SELECT access_token, refresh_token, expiry_date FROM calendar_tokens ORDER BY created_at DESC LIMIT 1")
        .map_err(|e| e.to_string())?;

    let row: (String, String, String) =
        stmt.query_row([], |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?)))
            .map_err(|_| "No token found".to_string())?;

    let expiry = DateTime::parse_from_rfc3339(&row.2)
        .map_err(|e| e.to_string())?
        .with_timezone(&Utc);

    Ok(CalendarToken {
        access_token: row.0,
        refresh_token: row.1,
        expiry_date: expiry,
    })
}

// === 2. Query ActivityWatch ===
async fn get_aw_events(client: &Client, start: DateTime<Utc>, end: DateTime<Utc>) -> Result<Vec<AwEvent>, String> {
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

// === 3. Summarize events (like Python) ===
fn summarize_events(events: &[AwEvent]) -> (String, String) {
    use std::collections::{HashMap, HashSet};
    let mut total_time = 0.0;
    let mut app_usage: HashMap<String, f64> = HashMap::new();
    let mut app_titles: HashMap<String, Vec<(String, f64)>> = HashMap::new();
    let mut browser_tab_usage: HashMap<String, f64> = HashMap::new();

    let browser_apps: HashSet<&str> = ["chrome.exe", "msedge.exe", "brave.exe", "firefox.exe"].into();

    for ev in events {
        let duration = ev.duration;
        let app = ev.data.app.clone().unwrap_or_else(|| "Unknown".to_string()).to_lowercase();
        let title = ev.data.title.clone().unwrap_or_default();

        total_time += duration;
        *app_usage.entry(app.clone()).or_default() += duration;

        if browser_apps.contains(app.as_str()) {
            if !title.is_empty() {
                *browser_tab_usage.entry(title).or_default() += duration;
            }
        } else {
            app_titles.entry(app).or_default().push((title, duration));
        }
    }

    if total_time == 0.0 {
        return ("PC was off".into(), "No activity recorded".into());
    }

    // --- Aggregate small apps ---
    let mut major_apps = HashMap::new();
    let mut other_time = 0.0;
    for (app, t) in &app_usage {
        if *t / total_time >= 0.05 {
            major_apps.insert(app.clone(), *t);
        } else {
            other_time += *t;
        }
    }
    if other_time > 0.0 {
        major_apps.insert("Other".into(), other_time);
    }

    // --- Build event title ---
    let mut parts = Vec::new();
    let mut major_vec: Vec<_> = major_apps.into_iter().collect();
    major_vec.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

    for (app, t) in major_vec {
        let percent = (t / total_time) * 100.0;
        let app_clean = app.trim_end_matches(".exe").to_string();
        parts.push(format!("{} {:.0}%", app_clean, percent));
    }
    let event_title = parts.join(", ");

    // --- Raw breakdown ---
    let mut raw_lines = Vec::new();
    for (app, titles) in &app_titles {
        let app_total: f64 = titles.iter().map(|(_, d)| d).sum();
        let percent = (app_total / total_time) * 100.0;
        raw_lines.push(format!("{} ({:.1}%):", app, percent));
        let mut sorted_titles = titles.clone();
        sorted_titles.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        for (t, d) in sorted_titles.iter().take(5) {
            raw_lines.push(format!("   • {} (~{:.1}m)", t, d / 60.0));
        }
    }

    if !browser_tab_usage.is_empty() {
        raw_lines.push("\nBrowser activity (tabs):".into());
        let mut sorted_tabs: Vec<_> = browser_tab_usage.into_iter().collect();
        sorted_tabs.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        for (tab, d) in sorted_tabs.into_iter().take(10) {
            raw_lines.push(format!("   • {} (~{:.1}m, {:.1}%)", tab, d / 60.0, (d / total_time) * 100.0));
        }
    }

    let raw_text = raw_lines.join("\n");

    (event_title, raw_text) // raw_text will go to Ollama
}

// === 4. Call Ollama ===
async fn summarize_with_ollama(client: &Client, raw_text: &str) -> Result<String, String> {
    #[derive(Serialize)]
    struct Payload<'a> {
        model: &'a str,
        prompt: String,
        stream: bool,
    }

    #[derive(Deserialize)]
    struct OllamaResponse {
        response: String,
    }

    let payload = Payload {
        model: "mistral",
        prompt: format!(
            "Summarize the activity into 3-5 concise bullet points.\n- Keep each bullet under 80 chars.\n- Focus on main apps and tasks.\n- No preamble or closing text.\n\nRaw log:\n{}",
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

    if !resp.status().is_success() {
        return Err(format!("Ollama failed: {}", resp.status()));
    }

    let data: OllamaResponse = resp.json().await.map_err(|e| e.to_string())?;

    // Unescape any literal \n into real newlines and trim
    let mut summary = data.response.replace("\\n", "\n").trim().to_string();

    // Ensure length is reasonable
    let max_len = 600;
    if summary.len() > max_len {
        summary.truncate(max_len);
        summary.push_str("\n…");
    }

    Ok(summary)
}

// === 5. Push event to Google Calendar ===
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

async fn add_calendar_event(
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
        start: EventDateTime { dateTime: start.to_rfc3339() },
        end: EventDateTime { dateTime: end.to_rfc3339() },
    };

    let resp = client
        .post("https://www.googleapis.com/calendar/v3/calendars/primary/events")
        .bearer_auth(token)
        .json(&event)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if resp.status().is_success() {
        println!("✅ Event created");
        Ok(())
    } else {
        Err(format!("Failed: {}", resp.status()))
    }
}

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
        .with_minute(0).unwrap()
        .with_second(0).unwrap()
        .with_nanosecond(0).unwrap();

    // Start = 1 hour before end
    let start_local = end_local - Duration::hours(1);

    // Convert to UTC for ActivityWatch and Google APIs
    let start = start_local.with_timezone(&Utc);
    let end = end_local.with_timezone(&Utc);

    println!("Processing {start} → {end} ...");

    let events = get_aw_events(&client, start, end).await?;
    let (event_title, raw_text) = summarize_events(&events);
    let description = summarize_with_ollama(&client, &raw_text).await?;

    add_calendar_event(&client, &token.access_token, &event_title, &description, start, end).await?;
    println!("✅ Hour {start} → {end} updated.");

    Ok(())
}
