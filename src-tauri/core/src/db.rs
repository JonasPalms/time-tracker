use std::path::Path;
use std::time::Duration;

use rusqlite::Connection;

const BUSY_TIMEOUT: Duration = Duration::from_millis(5_000);

pub fn open(path: &Path) -> Result<Connection, String> {
    let conn = Connection::open(path).map_err(|error| error.to_string())?;
    configure(&conn)?;
    Ok(conn)
}

pub fn open_existing(path: &Path) -> Result<Connection, String> {
    if !path.exists() {
        return Err(format!("TimeTracker database not found at {}", path.display()));
    }

    open(path)
}

fn configure(conn: &Connection) -> Result<(), String> {
    conn.pragma_update(None, "journal_mode", "WAL")
        .map_err(|error| error.to_string())?;
    conn.busy_timeout(BUSY_TIMEOUT)
        .map_err(|error| error.to_string())?;
    Ok(())
}

pub fn migrate(conn: &Connection) -> Result<(), String> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS tasks (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            total_seconds INTEGER DEFAULT 0,
            created_at TEXT DEFAULT (datetime('now', 'localtime'))
        )",
        [],
    )
    .map_err(|error| format!("Failed to create tasks table: {error}"))?;

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

    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_tasks_created_at ON tasks(created_at)",
        [],
    )
    .map_err(|error| format!("Failed to create index: {error}"))?;

    let _ = conn.execute("ALTER TABLE tasks ADD COLUMN note TEXT", []);

    Ok(())
}
