// === Users ===
pub const CREATE_USERS_TABLE: &str = "
CREATE TABLE IF NOT EXISTS users (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    email TEXT NOT NULL UNIQUE,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);
";


// === Events ===
pub const CREATE_EVENTS_TABLE: &str = "
CREATE TABLE IF NOT EXISTS events (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER NOT NULL,
    title TEXT NOT NULL,
    description TEXT,
    category TEXT,
    planned_start TEXT NOT NULL,
    planned_end TEXT NOT NULL,
    actual_start TEXT,
    actual_end TEXT,
    auto_detected_start TEXT,
    status TEXT DEFAULT 'planned',
    distraction_flag INTEGER DEFAULT 0,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    FOREIGN KEY(user_id) REFERENCES users(id)
);
";

// === Pomodoro Sessions ===
pub const CREATE_POMODORO_TABLE: &str = "
CREATE TABLE IF NOT EXISTS pomodoro_sessions (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    event_id INTEGER NOT NULL,
    start_time TEXT NOT NULL,
    end_time TEXT,
    paused_duration INTEGER DEFAULT 0,
    distraction_count INTEGER DEFAULT 0,
    interval_number INTEGER NOT NULL,
    completed INTEGER DEFAULT 0,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    FOREIGN KEY(event_id) REFERENCES events(id)
);
";

// === Daily Summary ===
pub const CREATE_DAILY_SUMMARY_TABLE: &str = "
CREATE TABLE IF NOT EXISTS daily_summary (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER NOT NULL,
    date TEXT NOT NULL,
    total_planned INTEGER DEFAULT 0,
    total_completed INTEGER DEFAULT 0,
    total_pomodoros INTEGER DEFAULT 0,
    total_distractions INTEGER DEFAULT 0,
    reality_score INTEGER DEFAULT 0,
    summary_text TEXT,
    suggestions_text TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    FOREIGN KEY(user_id) REFERENCES users(id)
);
";

// === Optional: Distractions ===
pub const CREATE_DISTRACTIONS_TABLE: &str = "
CREATE TABLE IF NOT EXISTS distractions (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    event_id INTEGER,
    user_id INTEGER NOT NULL,
    start_time TEXT NOT NULL,
    end_time TEXT,
    type TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    FOREIGN KEY(event_id) REFERENCES events(id),
    FOREIGN KEY(user_id) REFERENCES users(id)
);
";

pub const CREATE_CREDENTIALS_TABLE: &str = "
CREATE TABLE IF NOT EXISTS credentials (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    provider TEXT NOT NULL DEFAULT 'google',
    owner TEXT NOT NULL DEFAULT 'system', -- 'system' (your app) or 'user' (if user provides their own)
    client_id TEXT NOT NULL,
    project_id TEXT NOT NULL,
    auth_uri TEXT NOT NULL,
    token_uri TEXT NOT NULL,
    auth_provider_x509_cert_url TEXT NOT NULL,
    client_secret TEXT NOT NULL,
    redirect_uris TEXT NOT NULL, -- JSON array string
    scopes TEXT NOT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);
";
pub const CREATE_CALENDAR_TOKEN_TABLE: &str = "
CREATE TABLE IF NOT EXISTS calendar_tokens (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER NOT NULL,
    credential_id INTEGER NOT NULL,
    access_token TEXT NOT NULL,
    refresh_token TEXT,
    scope TEXT,
    token_type TEXT,
    expiry_date TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);
";

/// Returns all schema SQL as a single string
pub fn create_all_sql() -> String {
    format!(
        "{}{}{}{}{}{}{}",
        CREATE_USERS_TABLE,
        CREATE_EVENTS_TABLE,
        CREATE_POMODORO_TABLE,
        CREATE_DAILY_SUMMARY_TABLE,
        CREATE_DISTRACTIONS_TABLE,
        CREATE_CREDENTIALS_TABLE,
        CREATE_CALENDAR_TOKEN_TABLE
    )
}
