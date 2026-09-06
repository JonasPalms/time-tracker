use rusqlite::{Connection, OptionalExtension};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: i64,
    pub name: String,
    pub total_seconds: i64,
    pub created_at: String,
    pub note: Option<String>,
}

pub fn list_tasks_for_date(conn: &Connection, date: &str) -> Result<Vec<Task>, String> {
    list_tasks_in_range(conn, date, date)
}

pub fn list_tasks_in_range(conn: &Connection, from: &str, to: &str) -> Result<Vec<Task>, String> {
    let mut stmt = conn
        .prepare(
            "SELECT id, name, total_seconds, created_at, note
             FROM tasks
             WHERE date(created_at) >= ? AND date(created_at) <= ?
             ORDER BY created_at DESC",
        )
        .map_err(|error| error.to_string())?;

    let rows = stmt
        .query_map([from, to], map_task)
        .map_err(|error| error.to_string())?;

    rows.collect::<Result<Vec<_>, _>>()
        .map_err(|error| error.to_string())
}

pub fn search_tasks(
    conn: &Connection,
    query: &str,
    from: &str,
    to: &str,
) -> Result<Vec<Task>, String> {
    let like = format!("%{query}%");
    let mut stmt = conn
        .prepare(
            "SELECT id, name, total_seconds, created_at, note
             FROM tasks
             WHERE date(created_at) >= ? AND date(created_at) <= ?
               AND (name LIKE ? OR IFNULL(note, '') LIKE ?)
             ORDER BY created_at DESC",
        )
        .map_err(|error| error.to_string())?;

    let rows = stmt
        .query_map((from, to, like.as_str(), like.as_str()), map_task)
        .map_err(|error| error.to_string())?;

    rows.collect::<Result<Vec<_>, _>>()
        .map_err(|error| error.to_string())
}

pub fn get_task_by_id(conn: &Connection, task_id: i64) -> Result<Option<Task>, String> {
    conn.query_row(
        "SELECT id, name, total_seconds, created_at, note FROM tasks WHERE id = ?",
        [task_id],
        map_task,
    )
    .optional()
    .map_err(|error| error.to_string())
}

pub fn unique_task_names(conn: &Connection) -> Result<Vec<String>, String> {
    let mut stmt = conn
        .prepare("SELECT DISTINCT name FROM tasks ORDER BY created_at DESC LIMIT 50")
        .map_err(|error| error.to_string())?;

    let names = stmt
        .query_map([], |row| row.get(0))
        .map_err(|error| error.to_string())?;

    names
        .collect::<Result<Vec<String>, _>>()
        .map_err(|error| error.to_string())
}

pub fn create_task(
    conn: &Connection,
    name: &str,
    date: &str,
    initial_seconds: Option<i64>,
    note: Option<&str>,
) -> Result<Task, String> {
    let seconds = initial_seconds.unwrap_or(0);
    let created_at = format!("{date} 00:00:00");

    conn.execute(
        "INSERT INTO tasks (name, total_seconds, created_at, note) VALUES (?, ?, ?, ?)",
        (name, seconds, &created_at, note),
    )
    .map_err(|error| error.to_string())?;

    require_task(conn, conn.last_insert_rowid())
}

pub fn set_task_time(conn: &Connection, task_id: i64, total_seconds: i64) -> Result<Task, String> {
    update_one(
        conn,
        "UPDATE tasks SET total_seconds = ? WHERE id = ?",
        (total_seconds, task_id),
        task_id,
    )
}

pub fn add_task_time(conn: &Connection, task_id: i64, seconds: i64) -> Result<Task, String> {
    update_one(
        conn,
        "UPDATE tasks SET total_seconds = total_seconds + ? WHERE id = ?",
        (seconds, task_id),
        task_id,
    )
}

pub fn set_task_name(conn: &Connection, task_id: i64, name: &str) -> Result<Task, String> {
    update_one(
        conn,
        "UPDATE tasks SET name = ? WHERE id = ?",
        (name, task_id),
        task_id,
    )
}

pub fn set_task_note(conn: &Connection, task_id: i64, note: Option<&str>) -> Result<Task, String> {
    update_one(
        conn,
        "UPDATE tasks SET note = ? WHERE id = ?",
        (note, task_id),
        task_id,
    )
}

pub fn set_task_date(conn: &Connection, task_id: i64, date: &str) -> Result<Task, String> {
    let created_at = format!("{date} 00:00:00");
    update_one(
        conn,
        "UPDATE tasks SET created_at = ? WHERE id = ?",
        (created_at, task_id),
        task_id,
    )
}

pub fn delete_task(conn: &Connection, task_id: i64) -> Result<(), String> {
    conn.execute("DELETE FROM active_tracking WHERE task_id = ?", [task_id])
        .map_err(|error| error.to_string())?;

    let changed = conn
        .execute("DELETE FROM tasks WHERE id = ?", [task_id])
        .map_err(|error| error.to_string())?;

    if changed == 0 {
        return Err(format!("Task {task_id} not found"));
    }

    Ok(())
}

fn update_one(
    conn: &Connection,
    sql: &str,
    params: impl rusqlite::Params,
    task_id: i64,
) -> Result<Task, String> {
    let changed = conn.execute(sql, params).map_err(|error| error.to_string())?;
    if changed == 0 {
        return Err(format!("Task {task_id} not found"));
    }
    require_task(conn, task_id)
}

pub(crate) fn require_task(conn: &Connection, task_id: i64) -> Result<Task, String> {
    get_task_by_id(conn, task_id)?.ok_or_else(|| format!("Task {task_id} not found"))
}

fn map_task(row: &rusqlite::Row<'_>) -> rusqlite::Result<Task> {
    Ok(Task {
        id: row.get(0)?,
        name: row.get(1)?,
        total_seconds: row.get(2)?,
        created_at: row.get(3)?,
        note: row.get(4)?,
    })
}
