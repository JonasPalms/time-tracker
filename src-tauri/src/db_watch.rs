use std::path::{Path, PathBuf};
use std::sync::mpsc::{self, RecvTimeoutError};
use std::time::Duration;

use notify::{Event, RecommendedWatcher, RecursiveMode, Watcher};
use tauri::{AppHandle, Emitter};

const DEBOUNCE: Duration = Duration::from_millis(300);

pub fn start(app: AppHandle, db_path: PathBuf) {
    let Some(dir) = db_path.parent().map(Path::to_path_buf) else {
        return;
    };

    std::thread::Builder::new()
        .name("db-watch".into())
        .spawn(move || {
            if let Err(error) = watch_loop(app, dir) {
                eprintln!("TimeTracker db watch failed: {error}");
            }
        })
        .ok();
}

fn watch_loop(app: AppHandle, dir: PathBuf) -> Result<(), String> {
    let (tx, rx) = mpsc::channel();
    let mut watcher = RecommendedWatcher::new(
        move |event| {
            let _ = tx.send(event);
        },
        notify::Config::default(),
    )
    .map_err(|error| error.to_string())?;

    watcher
        .watch(&dir, RecursiveMode::NonRecursive)
        .map_err(|error| error.to_string())?;

    while let Ok(event) = rx.recv() {
        if !is_relevant(&event) {
            continue;
        }

        loop {
            match rx.recv_timeout(DEBOUNCE) {
                Ok(next) => {
                    if is_relevant(&next) {
                        continue;
                    }
                }
                Err(RecvTimeoutError::Timeout) => break,
                Err(RecvTimeoutError::Disconnected) => return Ok(()),
            }
        }

        let _ = app.emit("tasks-changed", ());
    }

    Ok(())
}

fn is_relevant(event: &Result<Event, notify::Error>) -> bool {
    let Ok(event) = event else {
        return false;
    };

    event.paths.iter().any(|path| {
        path.file_name()
            .and_then(|name| name.to_str())
            .is_some_and(is_sqlite_sidecar)
    })
}

fn is_sqlite_sidecar(name: &str) -> bool {
    name.starts_with("timetracker")
        && (name.ends_with(".db") || name.ends_with(".db-wal") || name.ends_with(".db-shm"))
}
