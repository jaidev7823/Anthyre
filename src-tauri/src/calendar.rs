use tauri::command;

/// Dummy replacement for Python calendar functions.
/// Later, we’ll replace with real Google Calendar API calls.
#[command]
pub fn test_calendar() -> String {
    "📅 No upcoming events found (stub)".to_string()
}

#[command]
pub fn add_test_event() -> String {
    "✅ Test event created (stub)".to_string()
}
