use rusqlite::Connection;
use std::fs;
use std::path::PathBuf;
use tauri::{AppHandle, Manager};

/// Initialize the database connection and run migrations
pub fn init_db(app_handle: &AppHandle) -> Result<Connection, String> {
    let db_path = database_path(app_handle)?;

    if let Some(parent) = db_path.parent() {
        fs::create_dir_all(parent).map_err(|e| format!("Failed to create db directory: {e}"))?;
    }

    let conn = time_tracker_core::open(&db_path)?;
    time_tracker_core::migrate(&conn)?;

    Ok(conn)
}

pub fn database_path(app_handle: &AppHandle) -> Result<PathBuf, String> {
    let app_data_dir = app_handle
        .path()
        .app_config_dir()
        .map_err(|e| format!("Failed to get app config dir: {e}"))?;

    let db_name = if cfg!(debug_assertions) {
        "timetracker-dev.db"
    } else {
        "timetracker.db"
    };

    Ok(app_data_dir.join(db_name))
}
