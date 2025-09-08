mod auth;
mod calendar;
mod database;
mod activity;

use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    dotenvy::dotenv().ok();

    tauri::Builder::default()
        .setup(|app| {
            if let Err(e) = database::init() {
                eprintln!("❌ Database init failed: {}", e);
                std::process::exit(1);
            }

            let conn = database::connection();
            if let Err(e) = database::seeder::seed_credentials(&conn) {
                eprintln!("⚠️ Seeding credentials failed: {}", e);
            }

            // ✅ spawn background task using Tauri's runtime
            let app_handle = app.handle().clone();
            tauri::async_runtime::spawn(async move {
                use tokio::time::{interval, Duration};
                let mut ticker = interval(Duration::from_secs(60)); // 1 minute (testing)

                loop {
                    ticker.tick().await; // waits for next tick
                    println!("🔄 Running scheduled update...");
                    let res = crate::activity::update_hours().await;
                    if let Err(e) = res {
                        eprintln!("Background update error: {:?}", e);
                    } else {
                        println!("✅ Scheduled update completed successfully");
                    }
                }
            });

            // ✅ return correct type
            Ok::<(), Box<dyn std::error::Error>>(())
        })
        .invoke_handler(tauri::generate_handler![
            calendar::test_calendar,
            calendar::add_test_event,
            auth::login_with_google,
            auth::check_calendar_token,
            activity::update_hours,
            activity::update_hours_range,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
