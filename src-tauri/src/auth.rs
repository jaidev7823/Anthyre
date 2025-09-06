use tauri::Manager;
use serde::Serialize;
use chrono::Utc;
use rusqlite::params;
use std::sync::Mutex;
use std::net::TcpListener;
use std::thread;
use std::io::{Read, Write}; 
use crate::database;

#[derive(Serialize)]
pub struct AuthResult {
    success: bool,
    message: String,
}

#[tauri::command]
pub async fn login_with_google() -> Result<AuthResult, String> {
    let conn = database::connection();

    // Load credentials from DB
    let mut stmt = conn.prepare("SELECT client_id, client_secret, redirect_uris, scopes FROM credentials LIMIT 1")
        .map_err(|e| e.to_string())?;
    let row = stmt.query_row([], |row| {
        Ok((
            row.get::<_, String>(0)?,
            row.get::<_, String>(1)?,
            row.get::<_, String>(2)?,
            row.get::<_, String>(3)?,
        ))
    }).map_err(|e| e.to_string())?;

    let (client_id, client_secret, redirect_uris, scopes) = row;
    let redirect_uri: String = serde_json::from_str(&redirect_uris).unwrap_or(vec!["http://localhost:1420/oauth2callback".to_string()])[0].clone();

    // Step 1: build Google OAuth URL
    let auth_url = format!(
        "https://accounts.google.com/o/oauth2/v2/auth?response_type=code&client_id={}&redirect_uri={}&scope={}&access_type=offline&prompt=consent",
        client_id,
        urlencoding::encode(&redirect_uri),
        urlencoding::encode(&scopes)
    );

    // Step 2: open browser
    if webbrowser::open(&auth_url).is_err() {
        return Err("Failed to open browser".into());
    }

    // Step 3: spin up temporary server to catch the redirect
    let listener = TcpListener::bind("127.0.0.1:1420").map_err(|e| e.to_string())?;
    let mut code: Option<String> = None;

    // Blocking accept until Google redirects back
    for stream in listener.incoming() {
        if let Ok(mut stream) = stream {
            let mut buffer = [0; 1024];
            if stream.read(&mut buffer).is_ok() {
                let req_str = String::from_utf8_lossy(&buffer);
                if req_str.contains("/oauth2callback?") {
                    if let Some(idx) = req_str.find("code=") {
                        let code_str = &req_str[idx + 5..];
                        if let Some(end_idx) = code_str.find('&') {
                            code = Some(code_str[..end_idx].to_string());
                        } else {
                            code = Some(code_str.to_string());
                        }
                    }

                    let response = "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\n\r\nYou can close this window now.";
                    let _ = stream.write_all(response.as_bytes());
                }
            }
        }
        if code.is_some() {
            break;
        }
    }

    let auth_code = code.ok_or("Failed to get auth code".to_string())?;

    // TODO: Exchange code for tokens with reqwest (later phase)

    Ok(AuthResult {
        success: true,
        message: format!("Got auth code: {}", auth_code),
    })
}
