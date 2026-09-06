use chrono::{Local, NaiveDateTime};
use rusqlite::{Connection, OptionalExtension};
use serde::{Deserialize, Serialize};

use crate::tasks::{add_task_time, get_task_by_id, require_task, Task};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActiveTracking {
    pub task: Task,
    pub started_at: String,
    pub elapsed_seconds: i64,
}

pub fn get_active_tracking(conn: &Connection) -> Result<Option<ActiveTracking>, String> {
    let row = conn
        .query_row(
            "SELECT task_id, started_at FROM active_tracking WHERE id = 1",
            [],
            |row| Ok((row.get::<_, i64>(0)?, row.get::<_, String>(1)?)),
        )
        .optional()
        .map_err(|error| error.to_string())?;

    let Some((task_id, started_at)) = row else {
        return Ok(None);
    };

    let Some(task) = get_task_by_id(conn, task_id)? else {
        conn.execute("DELETE FROM active_tracking WHERE id = 1", [])
            .map_err(|error| error.to_string())?;
        return Ok(None);
    };

    Ok(Some(ActiveTracking {
        elapsed_seconds: elapsed_since(&started_at)?,
        task,
        started_at,
    }))
}

pub fn start_tracking(conn: &Connection, task_id: i64) -> Result<ActiveTracking, String> {
    require_task(conn, task_id)?;

    if let Some(active) = get_active_tracking(conn)? {
        if active.task.id == task_id {
            return Ok(active);
        }
        stop_tracking(conn)?;
    }

    conn.execute(
        "INSERT INTO active_tracking (id, task_id, started_at) VALUES (1, ?, ?)",
        (task_id, now_local()),
    )
    .map_err(|error| error.to_string())?;

    get_active_tracking(conn)?.ok_or_else(|| "Failed to start tracking".into())
}

pub fn stop_tracking(conn: &Connection) -> Result<Option<Task>, String> {
    let Some(active) = get_active_tracking(conn)? else {
        return Ok(None);
    };

    if active.elapsed_seconds > 0 {
        add_task_time(conn, active.task.id, active.elapsed_seconds)?;
    }

    conn.execute("DELETE FROM active_tracking WHERE id = 1", [])
        .map_err(|error| error.to_string())?;

    Ok(Some(require_task(conn, active.task.id)?))
}

fn now_local() -> String {
    Local::now().format("%Y-%m-%d %H:%M:%S").to_string()
}

fn elapsed_since(started_at: &str) -> Result<i64, String> {
    let start = NaiveDateTime::parse_from_str(started_at, "%Y-%m-%d %H:%M:%S")
        .map_err(|_| format!("Invalid started_at: {started_at}"))?;
    Ok((Local::now().naive_local() - start)
        .num_seconds()
        .max(0))
}
