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
    migrate_app_tables(&conn)?;

    Ok(conn)
}

fn migrate_app_tables(conn: &Connection) -> Result<(), String> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS favourites (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            duration_seconds INTEGER NOT NULL,
            created_at TEXT DEFAULT (datetime('now', 'localtime'))
        )",
        [],
    )
    .map_err(|error| format!("Failed to create favourites table: {error}"))?;

    Ok(())
}

pub fn database_path(app_handle: &AppHandle) -> Result<PathBuf, String> {
    let app_data_dir = app_handle
        .path()
        .app_config_dir()
        .map_err(|e| format!("Failed to get app config dir: {e}"))?;

    Ok(app_data_dir.join(time_tracker_core::db_file_name(cfg!(debug_assertions))))
}
