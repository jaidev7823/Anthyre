mod auth;
mod calendar;
mod database;

use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    dotenvy::dotenv().ok();

    tauri::Builder::default()
        .setup(|_app| {
            if let Err(e) = database::init() {
                eprintln!("❌ Database init failed: {}", e);
                std::process::exit(1);
            }

            let conn = database::connection();
            if let Err(e) = database::seeder::seed_credentials(&conn) {
                eprintln!("⚠️ Seeding credentials failed: {}", e);
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            calendar::test_calendar,
            calendar::add_test_event,
            auth::login_with_google,
            auth::check_calendar_token,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
