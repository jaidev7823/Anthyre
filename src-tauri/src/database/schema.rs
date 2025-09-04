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
    start_time TEXT NOT NULL,
    end_time TEXT NOT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    FOREIGN KEY(user_id) REFERENCES users(id)
);
";

// === Settings (generic key-value) ===
pub const CREATE_SETTINGS_TABLE: &str = "
CREATE TABLE IF NOT EXISTS settings (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    key TEXT NOT NULL UNIQUE,
    value TEXT NOT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);
";

/// Returns all schema SQL as a single string
pub fn create_all_sql() -> String {
    format!(
        "{}{}{}{}",
        CREATE_USERS_TABLE,
        CREATE_CALENDAR_TOKENS_TABLE,
        CREATE_EVENTS_TABLE,
        CREATE_SETTINGS_TABLE
    )
}
