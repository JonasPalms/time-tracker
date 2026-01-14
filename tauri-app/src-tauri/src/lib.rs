use rusqlite::Connection;
use std::sync::Mutex;
use tauri::Manager;

mod commands;
mod db;
mod models;

pub struct AppState {
    pub db: Mutex<Connection>,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_store::Builder::default().build())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_process::init())
        .setup(|app| {
            let conn = db::init_db(app.handle())?;
            app.manage(AppState { db: Mutex::new(conn) });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            // Task commands
            commands::tasks::get_tasks_for_date,
            commands::tasks::get_todays_tasks,
            commands::tasks::create_task,
            commands::tasks::update_task_time,
            commands::tasks::add_time_to_task,
            commands::tasks::adjust_task_time,
            commands::tasks::get_tasks_in_range,
            commands::tasks::update_task_name,
            commands::tasks::delete_task,
            commands::tasks::get_unique_task_names,
            commands::tasks::get_task_by_id,
            commands::tasks::update_task_note,
            // Favourite commands
            commands::favourites::get_favourites,
            commands::favourites::create_favourite,
            commands::favourites::delete_favourite,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
