use chrono::{Local, NaiveDateTime};
use rusqlite::{Connection, OptionalExtension, Transaction};
use serde::{Deserialize, Serialize};

use crate::tasks::{add_task_time, get_task_by_id, require_task, Task};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActiveTracking {
    pub task: Task,
    pub started_at: String,
    pub elapsed_seconds: i64,
}

pub fn get_active_tracking(conn: &Connection) -> Result<Option<ActiveTracking>, String> {
    read_active(conn)
}

pub fn start_tracking(conn: &Connection, task_id: i64) -> Result<ActiveTracking, String> {
    with_tx(conn, |tx| {
        let task = require_task(tx, task_id)?;

        if let Some(active) = read_active(tx)? {
            if active.task.id == task_id {
                return Ok(active);
            }
            commit_elapsed(tx, &active)?;
        }

        let started_at = now_local();
        tx.execute(
            "INSERT OR REPLACE INTO active_tracking (id, task_id, started_at) VALUES (1, ?, ?)",
            (task_id, &started_at),
        )
        .map_err(|error| error.to_string())?;

        Ok(ActiveTracking {
            task,
            started_at,
            elapsed_seconds: 0,
        })
    })
}

pub fn stop_tracking(conn: &Connection) -> Result<Option<Task>, String> {
    with_tx(conn, |tx| {
        let Some(active) = read_active(tx)? else {
            return Ok(None);
        };

        commit_elapsed(tx, &active)?;
        tx.execute("DELETE FROM active_tracking WHERE id = 1", [])
            .map_err(|error| error.to_string())?;

        Ok(Some(require_task(tx, active.task.id)?))
    })
}

fn read_active(conn: &Connection) -> Result<Option<ActiveTracking>, String> {
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
        return Ok(None);
    };

    Ok(Some(ActiveTracking {
        elapsed_seconds: elapsed_since(&started_at)?,
        task,
        started_at,
    }))
}

fn commit_elapsed(conn: &Connection, active: &ActiveTracking) -> Result<(), String> {
    if active.elapsed_seconds > 0 {
        add_task_time(conn, active.task.id, active.elapsed_seconds)?;
    }
    Ok(())
}

fn with_tx<T>(
    conn: &Connection,
    f: impl FnOnce(&Transaction<'_>) -> Result<T, String>,
) -> Result<T, String> {
    let tx = conn
        .unchecked_transaction()
        .map_err(|error| error.to_string())?;
    let value = f(&tx)?;
    tx.commit().map_err(|error| error.to_string())?;
    Ok(value)
}

fn now_local() -> String {
    Local::now().format("%Y-%m-%d %H:%M:%S").to_string()
}

fn elapsed_since(started_at: &str) -> Result<i64, String> {
    let start = NaiveDateTime::parse_from_str(started_at, "%Y-%m-%d %H:%M:%S")
        .map_err(|_| format!("Invalid started_at: {started_at}"))?;
    Ok((Local::now().naive_local() - start).num_seconds().max(0))
}
