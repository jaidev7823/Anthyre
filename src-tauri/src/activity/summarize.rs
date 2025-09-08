use crate::activity::activitywatch::AwEvent;
use reqwest::Client;
use serde::{Deserialize, Serialize};


// === 3. Summarize events (like Python) ===
pub fn summarize_events(events: &[AwEvent]) -> (String, String) {
    use std::collections::{HashMap, HashSet};
    let mut total_time = 0.0;
    let mut app_usage: HashMap<String, f64> = HashMap::new();
    let mut app_titles: HashMap<String, Vec<(String, f64)>> = HashMap::new();
    let mut browser_tab_usage: HashMap<String, f64> = HashMap::new();

    let browser_apps: HashSet<&str> =
        ["chrome.exe", "msedge.exe", "brave.exe", "firefox.exe"].into();

    for ev in events {
        let duration = ev.duration;
        let app = ev
            .data
            .app
            .clone()
            .unwrap_or_else(|| "Unknown".to_string())
            .to_lowercase();
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
            raw_lines.push(format!(
                "   • {} (~{:.1}m, {:.1}%)",
                tab,
                d / 60.0,
                (d / total_time) * 100.0
            ));
        }
    }

    let raw_text = raw_lines.join("\n");

    (event_title, raw_text) // raw_text will go to Ollama
}


pub async fn summarize_with_ollama(client: &Client, raw_text: &str) -> Result<String, String> {
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

    let mut summary = data.response.replace("\\n", "\n").trim().to_string();
    if summary.len() > 600 {
        summary.truncate(600);
        summary.push_str("\n…");
    }

    Ok(summary)
}
