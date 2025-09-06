use crate::database;
use chrono::Utc;
use rusqlite::params;
use serde::{Deserialize, Serialize};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

#[derive(Deserialize)]
struct GoogleTokenResponse {
    access_token: String,
    expires_in: i64,
    scope: String,
    token_type: String,
    refresh_token: Option<String>,
}

#[derive(Serialize)]
pub struct AuthResult {
    success: bool,
    message: String,
}

#[tauri::command]
pub async fn login_with_google() -> Result<AuthResult, String> {
    let (credential_id, client_id, client_secret, redirect_uris, scopes) = {
        let conn = database::connection();
        let mut stmt = conn
            .prepare("SELECT id, client_id, client_secret, redirect_uris, scopes FROM credentials LIMIT 1")
            .map_err(|e| e.to_string())?;
        let row = stmt.query_row([], |row| {
            Ok((
                row.get::<_, i64>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
                row.get::<_, String>(3)?,
                row.get::<_, String>(4)?,
            ))
        })
        .map_err(|e| format!("Failed to query credentials: {}", e))?;
        println!("Credential ID: {}", row.0); // Log credential_id
        row
    };

    let listener = TcpListener::bind("localhost:0")
        .await
        .map_err(|e| format!("Bind failed: {}", e))?;
    let local_addr = listener.local_addr().map_err(|e| e.to_string())?;
    let port = local_addr.port();
    let redirect_uri = format!("http://localhost:{}/oauth2callback", port);
    println!("Redirect URI: {}", redirect_uri);

    let auth_url = format!(
        "https://accounts.google.com/o/oauth2/v2/auth?response_type=code&client_id={}&redirect_uri={}&scope={}&access_type=offline&prompt=consent",
        client_id,
        urlencoding::encode(&redirect_uri),
        urlencoding::encode(&scopes)
    );

    if webbrowser::open(&auth_url).is_err() {
        return Err("Failed to open browser".into());
    }

    let (mut socket, addr) = tokio::time::timeout(std::time::Duration::from_secs(120), listener.accept())
        .await
        .map_err(|_| "Timeout waiting for OAuth callback".to_string())?
        .map_err(|e| format!("Accept failed: {}", e))?;
    println!("Connection from: {}", addr);

    let mut buffer = [0u8; 2048];
    let n = socket.read(&mut buffer).await.map_err(|e| e.to_string())?;
    let req_str = String::from_utf8_lossy(&buffer[..n]);
    println!("Raw request: {}", req_str);

    let mut code = None;
    if req_str.contains("/oauth2callback") {
        if let Some(first_line) = req_str.lines().next() {
            if let Some(query_start) = first_line.find('?') {
                let query_part = &first_line[query_start + 1..];
                if let Some(space_pos) = query_part.find(' ') {
                    let query_params = &query_part[..space_pos];
                    for param in query_params.split('&') {
                        if param.starts_with("code=") {
                            code = Some(urlencoding::decode(&param[5..]).unwrap_or_default().to_string());
                            println!("Auth code: {}", code.as_ref().unwrap());
                            break;
                        }
                    }
                }
            }
        }

        let response = if code.is_some() {
            println!("1 true");
            let auth_code = code.unwrap();
            let client = reqwest::Client::new();
            let token_resp = client
                .post("https://oauth2.googleapis.com/token")
                .form(&[
                    ("code", auth_code.as_str()),
                    ("client_id", client_id.as_str()),
                    ("client_secret", client_secret.as_str()),
                    ("redirect_uri", redirect_uri.as_str()),
                    ("grant_type", "authorization_code"),
                ])
                .send()
                .await
                .map_err(|e| {
                    println!("Token exchange error: {}", e);
                    e.to_string()
                })?;
            println!("2 true");
            let token_json: GoogleTokenResponse = token_resp.json().await.map_err(|e| {
                println!("Token parse error: {}", e);
                e.to_string()
            })?;
            println!("3 true");
            let conn = database::connection();
            let now = Utc::now().to_rfc3339();
            println!("4 true");
            conn.execute(
                "INSERT INTO calendar_tokens (
                    user_id, credential_id, access_token, refresh_token, scope, token_type, expiry_date, created_at, updated_at
                ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?8)",
                params![
                    1, // TODO: Replace with dynamic user_id
                    credential_id,
                    token_json.access_token,
                    token_json.refresh_token.unwrap_or_default(),
                    token_json.scope,
                    token_json.token_type,
                    Utc::now()
                        .checked_add_signed(chrono::Duration::seconds(token_json.expires_in))
                        .unwrap()
                        .to_rfc3339(),
                    now
                ],
            )
            .map_err(|e| {
                println!("Database insert error: {}", e);
                format!("Database insert failed: {}", e)
            })?;

            "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\n\r\n<h1>Auth successful!</h1><p>You can close this window now.</p>"
        } else {
            "HTTP/1.1 400 Bad Request\r\nContent-Type: text/html\r\n\r\n<h1>No code received</h1><p>Authorization failed.</p>"
        };
        socket.write_all(response.as_bytes()).await.map_err(|e| e.to_string())?;
    } else {
        socket
            .write_all("HTTP/1.1 404 Not Found\r\nContent-Type: text/html\r\n\r\n<h1>Wrong endpoint</h1><p>This is not the OAuth callback.</p>".as_bytes())
            .await
            .map_err(|e| e.to_string())?;
        return Err("Invalid callback endpoint".to_string());
    }

    Ok(AuthResult {
        success: true,
        message: "Token saved to calendar_tokens".to_string(),
    })
}

#[tauri::command]
pub fn check_tokens() -> Result<String, String> {
    let conn = database::connection();
    let count: i64 = conn
        .query_row("SELECT COUNT(*) FROM calendar_tokens", [], |row| row.get(0))
        .map_err(|e| e.to_string())?;

    if count == 0 {
        return Ok("No tokens found in calendar_tokens table".to_string());
    }

    let mut stmt = conn
        .prepare(
            "SELECT id, user_id, credential_id, access_token, scope, expiry_date, created_at 
             FROM calendar_tokens ORDER BY created_at DESC LIMIT 5",
        )
        .map_err(|e| e.to_string())?;

    let tokens = stmt
        .query_map([], |row| {
            Ok(format!(
                "ID: {}, User: {}, Credential: {}, Access: {}..., Scope: {}, Expires: {}, Created: {}",
                row.get::<_, i64>(0)?,
                row.get::<_, i64>(1)?,
                row.get::<_, i64>(2)?,
                row.get::<_, String>(3)?[..20.min(row.get::<_, String>(3)?.len())].to_string(),
                row.get::<_, String>(4)?,
                row.get::<_, String>(5)?,
                row.get::<_, String>(6)?
            ))
        })
        .map_err(|e| e.to_string())?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| e.to_string())?;

    Ok(format!("Found {} tokens:\n{}", count, tokens.join("\n")))
}