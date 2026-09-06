use crate::AppState;
use tauri::State;
use time_tracker_core::ActiveTracking;

#[tauri::command]
pub fn get_active_tracking(state: State<AppState>) -> Result<Option<ActiveTracking>, String> {
    let conn = state.db.lock().map_err(|e| e.to_string())?;
    time_tracker_core::get_active_tracking(&conn)
}

#[tauri::command]
pub fn start_tracking(state: State<AppState>, task_id: i64) -> Result<ActiveTracking, String> {
    let conn = state.db.lock().map_err(|e| e.to_string())?;
    time_tracker_core::start_tracking(&conn, task_id)
}

#[tauri::command]
pub fn stop_tracking(state: State<AppState>) -> Result<Option<time_tracker_core::Task>, String> {
    let conn = state.db.lock().map_err(|e| e.to_string())?;
    time_tracker_core::stop_tracking(&conn)
}
