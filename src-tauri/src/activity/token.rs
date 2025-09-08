use crate::database;
use chrono::{DateTime, Utc};

#[derive(Debug)]
pub struct CalendarToken {
    pub access_token: String,
    pub refresh_token: String,
    pub expiry_date: DateTime<Utc>,
}

pub fn get_latest_token() -> Result<CalendarToken, String> {
    let conn = database::connection();
    let mut stmt = conn
        .prepare("SELECT access_token, refresh_token, expiry_date FROM calendar_tokens ORDER BY created_at DESC LIMIT 1")
        .map_err(|e| e.to_string())?;

    let row: (String, String, String) = stmt
        .query_row([], |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?)))
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
