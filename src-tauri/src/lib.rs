use rusqlite::Connection;
use std::sync::Mutex;
use tauri::menu::{MenuBuilder, MenuItem, PredefinedMenuItem, SubmenuBuilder};
use tauri::{Emitter, Manager, RunEvent, WindowEvent};

mod commands;
mod db;
mod db_watch;
mod models;

pub struct AppState {
    pub db: Mutex<Connection>,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_process::init())
        .setup(|app| {
            let db_path = db::database_path(app.handle())?;
            let conn = db::init_db(app.handle())?;
            app.manage(AppState { db: Mutex::new(conn) });
            db_watch::start(app.handle().clone(), db_path);

            // Set up macOS menu with Window menu for Minimize support
            #[cfg(target_os = "macos")]
            {
                if let Some(window) = app.get_webview_window("main") {
                    disable_macos_show_animation(&window);
                }

                let check_updates_item = MenuItem::with_id(
                    app,
                    "check_updates",
                    "Check for Updates...",
                    true,
                    None::<&str>,
                )?;

                let app_menu = SubmenuBuilder::new(app, "TimeTracker")
                    .item(&check_updates_item)
                    .separator()
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
                    "check_updates" => {
                        let _ = app.emit("check-for-updates", ());
                    }
                    _ => {}
                }
            }
        })
        .on_window_event(|window, event| {
            if let WindowEvent::CloseRequested { .. } = event {
                commit_active_tracking(window.app_handle());
            }
        })
        .invoke_handler(tauri::generate_handler![
            // Task commands
            commands::tasks::get_tasks_for_date,
            commands::tasks::create_task,
            commands::tasks::update_task_time,
            commands::tasks::add_time_to_task,
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
            commands::mcp::get_mcp_binary_path,
            commands::tracking::get_active_tracking,
            commands::tracking::start_tracking,
            commands::tracking::stop_tracking,
        ])
        .build(tauri::generate_context!())
        .expect("error while running tauri application")
        .run(|app, event| {
            if let RunEvent::ExitRequested { .. } = event {
                commit_active_tracking(app);
            }
        });
}

fn commit_active_tracking(app: &tauri::AppHandle) {
    let Some(state) = app.try_state::<AppState>() else {
        return;
    };
    let Ok(conn) = state.db.lock() else {
        return;
    };
    if let Err(error) = time_tracker_core::stop_tracking(&conn) {
        eprintln!("TimeTracker failed to stop tracking on exit: {error}");
    }
}

#[cfg(target_os = "macos")]
fn disable_macos_show_animation(window: &tauri::WebviewWindow) {
    use objc2_app_kit::{NSWindow, NSWindowAnimationBehavior};

    if let Ok(ptr) = window.ns_window() {
        let ns_window = unsafe { &*(ptr as *const NSWindow) };
        ns_window.setAnimationBehavior(NSWindowAnimationBehavior::None);
    }
}
