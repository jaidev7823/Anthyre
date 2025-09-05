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

// === Calendar Tokens ===
pub const CREATE_CALENDAR_TOKENS_TABLE: &str = "
CREATE TABLE IF NOT EXISTS calendar_tokens (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER NOT NULL,
    token BLOB NOT NULL,
    created_at TEXT NOT NULL,
    FOREIGN KEY(user_id) REFERENCES users(id)
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

/// Returns all schema SQL as a single string
pub fn create_all_sql() -> String {
    format!(
        "{}{}{}{}{}{}",
        CREATE_USERS_TABLE,
        CREATE_CALENDAR_TOKENS_TABLE,
        CREATE_EVENTS_TABLE,
        CREATE_POMODORO_TABLE,
        CREATE_DAILY_SUMMARY_TABLE,
        CREATE_DISTRACTIONS_TABLE
    )
}
