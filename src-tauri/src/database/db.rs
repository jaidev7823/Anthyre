use once_cell::sync::Lazy;
use rusqlite::{Connection, Result};
use std::path::PathBuf;
use std::sync::Mutex;
use std::fs;

use super::schema; // bring in schema.rs

static DB_CONNECTION: Lazy<Mutex<Connection>> = Lazy::new(|| {
    let database_path = get_app_data_dir().join("database.db");

    fs::create_dir_all(database_path.parent().unwrap())
        .expect("❌ Could not create Anthyre data directory");

    let conn = Connection::open(&database_path).expect("❌ Failed to open DB");

    println!("✅ Connected to database at: {}", database_path.display());

    Mutex::new(conn)
});

/// Returns the Anthyre data directory path
pub fn get_app_data_dir() -> PathBuf {
    let mut dir = dirs::data_local_dir().unwrap_or_else(|| {
        std::env::current_dir().expect("❌ Could not get current directory")
    });
    dir.push("Anthyre");
    dir
}

/// Get a locked reference to the global connection
pub fn connection() -> std::sync::MutexGuard<'static, Connection> {
    DB_CONNECTION.lock().expect("❌ Failed to lock DB")
}

pub fn init() -> Result<()> {
    let conn = connection();
    conn.execute_batch(&schema::create_all_sql())?;
    println!("✅ Database initialized with full schema");
    Ok(())
}
