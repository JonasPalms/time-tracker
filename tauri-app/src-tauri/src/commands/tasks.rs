use crate::models::Task;
use crate::AppState;
use chrono::Local;
use rusqlite::OptionalExtension;
use tauri::State;

/// Get tasks for a specific date (YYYY-MM-DD format)
#[tauri::command]
pub fn get_tasks_for_date(state: State<AppState>, date: String) -> Result<Vec<Task>, String> {
    let conn = state.db.lock().map_err(|e| e.to_string())?;

    let mut stmt = conn
        .prepare("SELECT id, name, total_seconds, created_at, note FROM tasks WHERE date(created_at) = ? ORDER BY created_at DESC")
        .map_err(|e| e.to_string())?;

    let tasks = stmt
        .query_map([&date], |row| {
            Ok(Task {
                id: row.get(0)?,
                name: row.get(1)?,
                total_seconds: row.get(2)?,
                created_at: row.get(3)?,
                note: row.get(4)?,
            })
        })
        .map_err(|e| e.to_string())?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| e.to_string())?;

    Ok(tasks)
}

/// Get today's tasks (convenience wrapper)
#[tauri::command]
pub fn get_todays_tasks(state: State<AppState>) -> Result<Vec<Task>, String> {
    let today = Local::now().format("%Y-%m-%d").to_string();
    get_tasks_for_date(state, today)
}

/// Create a new task
#[tauri::command]
pub fn create_task(
    state: State<AppState>,
    name: String,
    date: String,
    initial_seconds: Option<i64>,
) -> Result<Task, String> {
    let conn = state.db.lock().map_err(|e| e.to_string())?;
    let seconds = initial_seconds.unwrap_or(0);

    // Create timestamp at midnight of the specified date
    let created_at = format!("{} 00:00:00", date);

    conn.execute(
        "INSERT INTO tasks (name, total_seconds, created_at) VALUES (?, ?, ?)",
        (&name, seconds, &created_at),
    )
    .map_err(|e| e.to_string())?;

    let id = conn.last_insert_rowid();

    // Fetch the created task
    let task = conn
        .query_row(
            "SELECT id, name, total_seconds, created_at, note FROM tasks WHERE id = ?",
            [id],
            |row| {
                Ok(Task {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    total_seconds: row.get(2)?,
                    created_at: row.get(3)?,
                    note: row.get(4)?,
                })
            },
        )
        .map_err(|e| e.to_string())?;

    Ok(task)
}

/// Update task's total time (set directly)
#[tauri::command]
pub fn update_task_time(
    state: State<AppState>,
    task_id: i64,
    total_seconds: i64,
) -> Result<(), String> {
    let conn = state.db.lock().map_err(|e| e.to_string())?;

    conn.execute(
        "UPDATE tasks SET total_seconds = ? WHERE id = ?",
        (total_seconds, task_id),
    )
    .map_err(|e| e.to_string())?;

    Ok(())
}

/// Add time to a task (increment)
#[tauri::command]
pub fn add_time_to_task(
    state: State<AppState>,
    task_id: i64,
    seconds_to_add: i64,
) -> Result<(), String> {
    let conn = state.db.lock().map_err(|e| e.to_string())?;

    conn.execute(
        "UPDATE tasks SET total_seconds = total_seconds + ? WHERE id = ?",
        (seconds_to_add, task_id),
    )
    .map_err(|e| e.to_string())?;

    Ok(())
}

/// Adjust task time (positive or negative)
#[tauri::command]
pub fn adjust_task_time(
    state: State<AppState>,
    task_id: i64,
    seconds_to_adjust: i64,
) -> Result<(), String> {
    let conn = state.db.lock().map_err(|e| e.to_string())?;

    conn.execute(
        "UPDATE tasks SET total_seconds = total_seconds + ? WHERE id = ?",
        (seconds_to_adjust, task_id),
    )
    .map_err(|e| e.to_string())?;

    Ok(())
}

/// Get tasks in a date range
#[tauri::command]
pub fn get_tasks_in_range(
    state: State<AppState>,
    start_date: String,
    end_date: String,
) -> Result<Vec<Task>, String> {
    let conn = state.db.lock().map_err(|e| e.to_string())?;

    let mut stmt = conn
        .prepare("SELECT id, name, total_seconds, created_at, note FROM tasks WHERE date(created_at) >= ? AND date(created_at) <= ? ORDER BY created_at DESC")
        .map_err(|e| e.to_string())?;

    let tasks = stmt
        .query_map([&start_date, &end_date], |row| {
            Ok(Task {
                id: row.get(0)?,
                name: row.get(1)?,
                total_seconds: row.get(2)?,
                created_at: row.get(3)?,
                note: row.get(4)?,
            })
        })
        .map_err(|e| e.to_string())?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| e.to_string())?;

    Ok(tasks)
}

/// Update task name
#[tauri::command]
pub fn update_task_name(
    state: State<AppState>,
    task_id: i64,
    new_name: String,
) -> Result<(), String> {
    let conn = state.db.lock().map_err(|e| e.to_string())?;

    conn.execute("UPDATE tasks SET name = ? WHERE id = ?", (&new_name, task_id))
        .map_err(|e| e.to_string())?;

    Ok(())
}

/// Delete a task
#[tauri::command]
pub fn delete_task(state: State<AppState>, task_id: i64) -> Result<(), String> {
    let conn = state.db.lock().map_err(|e| e.to_string())?;

    conn.execute("DELETE FROM tasks WHERE id = ?", [task_id])
        .map_err(|e| e.to_string())?;

    Ok(())
}

/// Get unique task names for autocomplete (last 50)
#[tauri::command]
pub fn get_unique_task_names(state: State<AppState>) -> Result<Vec<String>, String> {
    let conn = state.db.lock().map_err(|e| e.to_string())?;

    let mut stmt = conn
        .prepare("SELECT DISTINCT name FROM tasks ORDER BY created_at DESC LIMIT 50")
        .map_err(|e| e.to_string())?;

    let names = stmt
        .query_map([], |row| row.get(0))
        .map_err(|e| e.to_string())?
        .collect::<Result<Vec<String>, _>>()
        .map_err(|e| e.to_string())?;

    Ok(names)
}

/// Get a single task by ID
#[tauri::command]
pub fn get_task_by_id(state: State<AppState>, task_id: i64) -> Result<Option<Task>, String> {
    let conn = state.db.lock().map_err(|e| e.to_string())?;

    let task = conn
        .query_row(
            "SELECT id, name, total_seconds, created_at, note FROM tasks WHERE id = ?",
            [task_id],
            |row| {
                Ok(Task {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    total_seconds: row.get(2)?,
                    created_at: row.get(3)?,
                    note: row.get(4)?,
                })
            },
        )
        .optional()
        .map_err(|e| e.to_string())?;

    Ok(task)
}

/// Update task note
#[tauri::command]
pub fn update_task_note(
    state: State<AppState>,
    task_id: i64,
    note: Option<String>,
) -> Result<(), String> {
    let conn = state.db.lock().map_err(|e| e.to_string())?;

    conn.execute("UPDATE tasks SET note = ? WHERE id = ?", (&note, task_id))
        .map_err(|e| e.to_string())?;

    Ok(())
}
