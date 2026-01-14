use rusqlite::{Connection, Result};
use std::fs;
use std::path::PathBuf;
use tauri::{AppHandle, Manager};

/// Initialize the database connection and run migrations
pub fn init_db(app_handle: &AppHandle) -> Result<Connection, String> {
    let db_path = get_db_path(app_handle)?;

    // Ensure the directory exists
    if let Some(parent) = db_path.parent() {
        fs::create_dir_all(parent).map_err(|e| format!("Failed to create db directory: {}", e))?;
    }

    let conn = Connection::open(&db_path)
        .map_err(|e| format!("Failed to open database: {}", e))?;

    run_migrations(&conn)?;

    Ok(conn)
}

/// Get the database file path
fn get_db_path(app_handle: &AppHandle) -> Result<PathBuf, String> {
    let app_data_dir = app_handle
        .path()
        .app_config_dir()
        .map_err(|e| format!("Failed to get app config dir: {}", e))?;

    // Use dev database in debug builds, production database in release builds
    let db_name = if cfg!(debug_assertions) {
        "timetracker-dev.db"
    } else {
        "timetracker.db"
    };

    Ok(app_data_dir.join(db_name))
}

/// Run database migrations
fn run_migrations(conn: &Connection) -> Result<(), String> {
    // Create tasks table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS tasks (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            total_seconds INTEGER DEFAULT 0,
            created_at TEXT DEFAULT (datetime('now', 'localtime'))
        )",
        [],
    )
    .map_err(|e| format!("Failed to create tasks table: {}", e))?;

    // Create favourites table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS favourites (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            duration_seconds INTEGER NOT NULL,
            created_at TEXT DEFAULT (datetime('now', 'localtime'))
        )",
        [],
    )
    .map_err(|e| format!("Failed to create favourites table: {}", e))?;

    // Create index on tasks.created_at for faster date lookups
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_tasks_created_at ON tasks(created_at)",
        [],
    )
    .map_err(|e| format!("Failed to create index: {}", e))?;

    // Add note column to tasks (migration for v0.4.0+)
    // This will fail silently if column already exists
    let _ = conn.execute("ALTER TABLE tasks ADD COLUMN note TEXT", []);

    Ok(())
}
