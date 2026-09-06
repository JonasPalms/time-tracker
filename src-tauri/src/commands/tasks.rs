use crate::models::Task;
use crate::AppState;
use chrono::Local;
use tauri::State;

#[tauri::command]
pub fn get_tasks_for_date(state: State<AppState>, date: String) -> Result<Vec<Task>, String> {
    let conn = state.db.lock().map_err(|e| e.to_string())?;
    time_tracker_core::list_tasks_for_date(&conn, &date)
}

#[tauri::command]
pub fn get_todays_tasks(state: State<AppState>) -> Result<Vec<Task>, String> {
    let today = Local::now().format("%Y-%m-%d").to_string();
    get_tasks_for_date(state, today)
}

#[tauri::command]
pub fn create_task(
    state: State<AppState>,
    name: String,
    date: String,
    initial_seconds: Option<i64>,
) -> Result<Task, String> {
    let conn = state.db.lock().map_err(|e| e.to_string())?;
    time_tracker_core::create_task(&conn, &name, &date, initial_seconds, None)
}

#[tauri::command]
pub fn update_task_time(
    state: State<AppState>,
    task_id: i64,
    total_seconds: i64,
) -> Result<(), String> {
    let conn = state.db.lock().map_err(|e| e.to_string())?;
    time_tracker_core::set_task_time(&conn, task_id, total_seconds)?;
    Ok(())
}

#[tauri::command]
pub fn add_time_to_task(
    state: State<AppState>,
    task_id: i64,
    seconds_to_add: i64,
) -> Result<(), String> {
    let conn = state.db.lock().map_err(|e| e.to_string())?;
    time_tracker_core::add_task_time(&conn, task_id, seconds_to_add)?;
    Ok(())
}

#[tauri::command]
pub fn adjust_task_time(
    state: State<AppState>,
    task_id: i64,
    seconds_to_adjust: i64,
) -> Result<(), String> {
    let conn = state.db.lock().map_err(|e| e.to_string())?;
    time_tracker_core::add_task_time(&conn, task_id, seconds_to_adjust)?;
    Ok(())
}

#[tauri::command]
pub fn get_tasks_in_range(
    state: State<AppState>,
    start_date: String,
    end_date: String,
) -> Result<Vec<Task>, String> {
    let conn = state.db.lock().map_err(|e| e.to_string())?;
    time_tracker_core::list_tasks_in_range(&conn, &start_date, &end_date)
}

#[tauri::command]
pub fn update_task_name(
    state: State<AppState>,
    task_id: i64,
    new_name: String,
) -> Result<(), String> {
    let conn = state.db.lock().map_err(|e| e.to_string())?;
    time_tracker_core::set_task_name(&conn, task_id, &new_name)?;
    Ok(())
}

#[tauri::command]
pub fn delete_task(state: State<AppState>, task_id: i64) -> Result<(), String> {
    let conn = state.db.lock().map_err(|e| e.to_string())?;
    time_tracker_core::delete_task(&conn, task_id)
}

#[tauri::command]
pub fn get_unique_task_names(state: State<AppState>) -> Result<Vec<String>, String> {
    let conn = state.db.lock().map_err(|e| e.to_string())?;
    time_tracker_core::unique_task_names(&conn)
}

#[tauri::command]
pub fn get_task_by_id(state: State<AppState>, task_id: i64) -> Result<Option<Task>, String> {
    let conn = state.db.lock().map_err(|e| e.to_string())?;
    time_tracker_core::get_task_by_id(&conn, task_id)
}

#[tauri::command]
pub fn update_task_note(
    state: State<AppState>,
    task_id: i64,
    note: Option<String>,
) -> Result<(), String> {
    let conn = state.db.lock().map_err(|e| e.to_string())?;
    time_tracker_core::set_task_note(&conn, task_id, note.as_deref())?;
    Ok(())
}

#[tauri::command]
pub fn update_task_date(
    state: State<AppState>,
    task_id: i64,
    new_date: String,
) -> Result<(), String> {
    let conn = state.db.lock().map_err(|e| e.to_string())?;
    time_tracker_core::set_task_date(&conn, task_id, &new_date)?;
    Ok(())
}
