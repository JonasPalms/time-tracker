use crate::models::Favourite;
use crate::AppState;
use tauri::State;

/// Get all favourites
#[tauri::command]
pub fn get_favourites(state: State<AppState>) -> Result<Vec<Favourite>, String> {
    let conn = state.db.lock().map_err(|e| e.to_string())?;

    let mut stmt = conn
        .prepare("SELECT id, name, duration_seconds FROM favourites")
        .map_err(|e| e.to_string())?;

    let favourites = stmt
        .query_map([], |row| {
            Ok(Favourite {
                id: row.get(0)?,
                name: row.get(1)?,
                duration_seconds: row.get(2)?,
            })
        })
        .map_err(|e| e.to_string())?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| e.to_string())?;

    Ok(favourites)
}

/// Create a new favourite
#[tauri::command]
pub fn create_favourite(
    state: State<AppState>,
    name: String,
    duration_seconds: i64,
) -> Result<i64, String> {
    let conn = state.db.lock().map_err(|e| e.to_string())?;

    conn.execute(
        "INSERT INTO favourites (name, duration_seconds) VALUES (?, ?)",
        (&name, duration_seconds),
    )
    .map_err(|e| e.to_string())?;

    Ok(conn.last_insert_rowid())
}

/// Delete a favourite
#[tauri::command]
pub fn delete_favourite(state: State<AppState>, id: i64) -> Result<(), String> {
    let conn = state.db.lock().map_err(|e| e.to_string())?;

    conn.execute("DELETE FROM favourites WHERE id = ?", [id])
        .map_err(|e| e.to_string())?;

    Ok(())
}
