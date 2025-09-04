mod database;
mod calendar;

use tauri::Manager;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            if let Err(e) = database::init() {
                eprintln!("❌ Database init failed: {}", e);
                std::process::exit(1);
            }
            println!("✅ Anthyre initialized");
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            greet,
            calendar::test_calendar,
            calendar::add_test_event,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
