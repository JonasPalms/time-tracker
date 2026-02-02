use rusqlite::Connection;
use std::sync::Mutex;
use tauri::menu::{MenuBuilder, MenuItem, PredefinedMenuItem, SubmenuBuilder};
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

            // Set up macOS menu with Window menu for Minimize support
            #[cfg(target_os = "macos")]
            {
                let app_menu = SubmenuBuilder::new(app, "TimeTracker")
                    .item(&PredefinedMenuItem::hide(app, Some("Hide TimeTracker"))?)
                    .item(&PredefinedMenuItem::hide_others(app, None)?)
                    .item(&PredefinedMenuItem::show_all(app, None)?)
                    .separator()
                    .item(&PredefinedMenuItem::quit(app, Some("Quit TimeTracker"))?)
                    .build()?;

                let edit_menu = SubmenuBuilder::new(app, "Edit")
                    .item(&PredefinedMenuItem::undo(app, None)?)
                    .item(&PredefinedMenuItem::redo(app, None)?)
                    .separator()
                    .item(&PredefinedMenuItem::cut(app, None)?)
                    .item(&PredefinedMenuItem::copy(app, None)?)
                    .item(&PredefinedMenuItem::paste(app, None)?)
                    .item(&PredefinedMenuItem::select_all(app, None)?)
                    .build()?;

                // Use custom menu items with accelerators for window operations
                // since PredefinedMenuItem doesn't work with frameless windows
                let minimize_item =
                    MenuItem::with_id(app, "minimize", "Minimize", true, Some("CmdOrCtrl+M"))?;
                let zoom_item = MenuItem::with_id(app, "zoom", "Zoom", true, None::<&str>)?;
                let close_item =
                    MenuItem::with_id(app, "close", "Close Window", true, Some("CmdOrCtrl+W"))?;

                let window_menu = SubmenuBuilder::new(app, "Window")
                    .item(&minimize_item)
                    .item(&zoom_item)
                    .separator()
                    .item(&close_item)
                    .build()?;

                // Mark as the Window menu so macOS adds automatic window items
                window_menu.set_as_windows_menu_for_nsapp()?;

                let menu = MenuBuilder::new(app)
                    .item(&app_menu)
                    .item(&edit_menu)
                    .item(&window_menu)
                    .build()?;

                app.set_menu(menu)?;
            }

            Ok(())
        })
        .on_menu_event(|app, event| {
            let id = event.id().as_ref();
            if let Some(window) = app.get_webview_window("main") {
                match id {
                    "minimize" => {
                        let _ = window.minimize();
                    }
                    "zoom" => {
                        if window.is_maximized().unwrap_or(false) {
                            let _ = window.unmaximize();
                        } else {
                            let _ = window.maximize();
                        }
                    }
                    "close" => {
                        let _ = window.close();
                    }
                    _ => {}
                }
            }
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
            commands::tasks::update_task_date,
            // Favourite commands
            commands::favourites::get_favourites,
            commands::favourites::create_favourite,
            commands::favourites::delete_favourite,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
