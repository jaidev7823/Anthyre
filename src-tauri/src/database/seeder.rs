use chrono::Utc;
use rusqlite::{params, Connection, Result};

/// Check if a table is empty
pub fn is_table_empty(conn: &Connection, table: &str) -> Result<bool> {
    let mut stmt = conn.prepare(&format!("SELECT COUNT(*) FROM {}", table))?;
    let count: i64 = stmt.query_row([], |row| row.get(0))?;
    Ok(count == 0)
}

/// Seed Google OAuth credentials from `.env`
pub fn seed_credentials(conn: &Connection) -> Result<()> {
    if is_table_empty(conn, "credentials")? {
        let now = Utc::now().to_rfc3339();

        // Load from env (dotenvy::dotenv() must be called once in main/lib.rs)
        let client_id =
            std::env::var("GOOGLE_CLIENT_ID").unwrap_or_else(|_| "CHANGE_ME_CLIENT_ID".into());
        let project_id =
            std::env::var("GOOGLE_PROJECT_ID").unwrap_or_else(|_| "CHANGE_ME_PROJECT_ID".into());
        let client_secret = std::env::var("GOOGLE_CLIENT_SECRET")
            .unwrap_or_else(|_| "CHANGE_ME_CLIENT_SECRET".into());
        let redirect_uris = std::env::var("GOOGLE_REDIRECT_URI")
            .unwrap_or_else(|_| r#"["http://localhost:1421/oauth2callback"]"#.into());
        let scopes = std::env::var("GOOGLE_SCOPES").unwrap_or_else(|_| {
            "https://www.googleapis.com/auth/calendar \
             https://www.googleapis.com/auth/userinfo.email \
             https://www.googleapis.com/auth/userinfo.profile"
                .into()
        });

        // Constants from Google docs
        let auth_uri = "https://accounts.google.com/o/oauth2/auth";
        let token_uri = "https://oauth2.googleapis.com/token";
        let auth_provider_x509_cert_url = "https://www.googleapis.com/oauth2/v1/certs";

        conn.execute(
            "INSERT INTO credentials (
                client_id, project_id, auth_uri, token_uri, 
                auth_provider_x509_cert_url, client_secret, 
                redirect_uris, scopes, created_at, updated_at
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?9)",
            params![
                client_id,
                project_id,
                auth_uri,
                token_uri,
                auth_provider_x509_cert_url,
                client_secret,
                redirect_uris,
                scopes,
                &now
            ],
        )?;
        println!("✅ Seeded Google credentials from env");
    } else {
        println!("ℹ️ Credentials table already has data, skipping seed");
    }

    Ok(())
}
