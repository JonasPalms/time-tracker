use std::path::PathBuf;

use chrono::{Duration, Local, NaiveDate};
use rmcp::{
    handler::server::{router::tool::ToolRouter, wrapper::Parameters},
    model::{CallToolResult, ContentBlock, Implementation, ServerCapabilities, ServerInfo},
    schemars, tool, tool_handler, tool_router, ServerHandler, ServiceExt,
    transport::stdio,
};
use serde::Deserialize;
use serde_json::{json, Value};
use time_tracker_core::Task;

const APP_SUPPORT_DIR: &str = "Library/Application Support/com.jonaspalmsorensen.time-tracker";

#[derive(Clone)]
struct TimeTracker {
    #[allow(dead_code)]
    tool_router: ToolRouter<Self>,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
struct ListTasksArgs {
    /// Single day. Ignored if from/to are set.
    date: Option<String>,
    /// Range start, inclusive.
    from: Option<String>,
    /// Range end, inclusive.
    to: Option<String>,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
struct SummarizeArgs {
    /// Range start, inclusive. Defaults to 6 days before today.
    from: Option<String>,
    /// Range end, inclusive. Defaults to today.
    to: Option<String>,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
struct SearchArgs {
    /// Case-insensitive substring to match in name or note.
    query: String,
    from: Option<String>,
    to: Option<String>,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
struct CreateTaskArgs {
    /// Task name.
    name: String,
    /// Local day YYYY-MM-DD. Defaults to today.
    date: Option<String>,
    /// Starting duration in seconds. Defaults to 0.
    initial_seconds: Option<i64>,
    /// Optional note.
    note: Option<String>,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
struct TaskIdArgs {
    task_id: i64,
}

#[derive(Debug, Deserialize, schemars::JsonSchema, Default)]
struct EmptyArgs {}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
struct SetTaskTimeArgs {
    task_id: i64,
    /// Absolute duration in seconds. Must be >= 0.
    total_seconds: i64,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
struct AddTaskTimeArgs {
    task_id: i64,
    /// Seconds to add. Negative values subtract.
    seconds: i64,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
struct SetTaskNameArgs {
    task_id: i64,
    /// New task name.
    name: String,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
struct SetTaskNoteArgs {
    task_id: i64,
    /// Omit or pass null to clear the note.
    note: Option<String>,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
struct SetTaskDateArgs {
    task_id: i64,
    /// Local day YYYY-MM-DD.
    date: String,
}

#[tool_router]
impl TimeTracker {
    fn new() -> Self {
        Self {
            tool_router: Self::tool_router(),
        }
    }

    #[tool(description = "List TimeTracker tasks for one day or an inclusive date range. Dates are local YYYY-MM-DD. Defaults to today.")]
    fn list_tasks(&self, Parameters(args): Parameters<ListTasksArgs>) -> CallToolResult {
        tool_json(|| {
            let range = resolve_range(args.date, args.from, args.to)?;
            let db = open_tasks_db()?;
            let running = time_tracker_core::get_active_tracking(&db)?;
            let tasks = time_tracker_core::list_tasks_in_range(&db, &range.from, &range.to)?;
            Ok(json!({
                "from": range.from,
                "to": range.to,
                "tasks": tasks.iter().map(|task| present_task(task, running.as_ref())).collect::<Vec<_>>(),
            }))
        })
    }

    #[tool(description = "Sum TimeTracker hours by task name for an inclusive local date range. Defaults to the last 7 days.")]
    fn summarize_range(&self, Parameters(args): Parameters<SummarizeArgs>) -> CallToolResult {
        tool_json(|| {
            let today = today_local();
            let range = resolve_range(
                None,
                Some(args.from.unwrap_or_else(|| add_days(&today, -6))),
                Some(args.to.unwrap_or(today)),
            )?;
            let db = open_tasks_db()?;
            let tasks = time_tracker_core::list_tasks_in_range(&db, &range.from, &range.to)?;
            let mut summary = summarize_tasks(&tasks);
            summary["from"] = json!(range.from);
            summary["to"] = json!(range.to);
            Ok(summary)
        })
    }

    #[tool(description = "Search TimeTracker task names and notes. Defaults to the last 90 days if no range is given.")]
    fn search_tasks(&self, Parameters(args): Parameters<SearchArgs>) -> CallToolResult {
        tool_json(|| {
            if args.query.is_empty() {
                return Err("query must not be empty".into());
            }
            let today = today_local();
            let range = resolve_range(
                None,
                Some(args.from.unwrap_or_else(|| add_days(&today, -89))),
                Some(args.to.unwrap_or(today)),
            )?;
            let db = open_tasks_db()?;
            let running = time_tracker_core::get_active_tracking(&db)?;
            let tasks = time_tracker_core::search_tasks(&db, &args.query, &range.from, &range.to)?;
            Ok(json!({
                "query": args.query,
                "from": range.from,
                "to": range.to,
                "tasks": tasks.iter().map(|task| present_task(task, running.as_ref())).collect::<Vec<_>>(),
            }))
        })
    }

    #[tool(description = "Create a TimeTracker task. Does not start the in-app timer.")]
    fn create_task(&self, Parameters(args): Parameters<CreateTaskArgs>) -> CallToolResult {
        tool_json(|| {
            let name = args.name.trim();
            if name.is_empty() {
                return Err("name must not be empty".into());
            }
            let date = args.date.unwrap_or_else(today_local);
            parse_local_date(&date)?;
            if let Some(seconds) = args.initial_seconds {
                if seconds < 0 {
                    return Err("initial_seconds must be >= 0".into());
                }
            }
            let db = open_tasks_db()?;
            let task = time_tracker_core::create_task(
                &db,
                name,
                &date,
                args.initial_seconds,
                args.note.as_deref(),
            )?;
            Ok(present_task(&task, None))
        })
    }

    #[tool(description = "Set a task's committed duration in seconds. Does not start or stop the timer. A running timer is extra and is added on stop.")]
    fn set_task_time(&self, Parameters(args): Parameters<SetTaskTimeArgs>) -> CallToolResult {
        tool_json(|| {
            if args.total_seconds < 0 {
                return Err("total_seconds must be >= 0".into());
            }
            let db = open_tasks_db()?;
            let task = time_tracker_core::set_task_time(&db, args.task_id, args.total_seconds)?;
            Ok(present_task(&task, None))
        })
    }

    #[tool(description = "Add seconds to a task's committed duration. Negative values subtract. Does not start or stop the timer.")]
    fn add_task_time(&self, Parameters(args): Parameters<AddTaskTimeArgs>) -> CallToolResult {
        tool_json(|| {
            let db = open_tasks_db()?;
            let task = time_tracker_core::add_task_time(&db, args.task_id, args.seconds)?;
            Ok(present_task(&task, None))
        })
    }

    #[tool(description = "Rename a TimeTracker task.")]
    fn set_task_name(&self, Parameters(args): Parameters<SetTaskNameArgs>) -> CallToolResult {
        tool_json(|| {
            let name = args.name.trim();
            if name.is_empty() {
                return Err("name must not be empty".into());
            }
            let db = open_tasks_db()?;
            let task = time_tracker_core::set_task_name(&db, args.task_id, name)?;
            Ok(present_task(&task, None))
        })
    }

    #[tool(description = "Set or clear a task note.")]
    fn set_task_note(&self, Parameters(args): Parameters<SetTaskNoteArgs>) -> CallToolResult {
        tool_json(|| {
            let db = open_tasks_db()?;
            let task = time_tracker_core::set_task_note(&db, args.task_id, args.note.as_deref())?;
            Ok(present_task(&task, None))
        })
    }

    #[tool(description = "Move a task to another local day (YYYY-MM-DD).")]
    fn set_task_date(&self, Parameters(args): Parameters<SetTaskDateArgs>) -> CallToolResult {
        tool_json(|| {
            parse_local_date(&args.date)?;
            let db = open_tasks_db()?;
            let task = time_tracker_core::set_task_date(&db, args.task_id, &args.date)?;
            Ok(present_task(&task, None))
        })
    }

    #[tool(description = "Delete a TimeTracker task. If it is running, the timer is cleared without committing elapsed time.")]
    fn delete_task(&self, Parameters(args): Parameters<TaskIdArgs>) -> CallToolResult {
        tool_json(|| {
            let db = open_tasks_db()?;
            time_tracker_core::delete_task(&db, args.task_id)?;
            Ok(json!({ "deleted": args.task_id }))
        })
    }

    #[tool(description = "Start the TimeTracker timer on a task. Stops any other running task first and commits that elapsed time.")]
    fn start_task(&self, Parameters(args): Parameters<TaskIdArgs>) -> CallToolResult {
        tool_json(|| {
            let db = open_tasks_db()?;
            let running = time_tracker_core::start_tracking(&db, args.task_id)?;
            Ok(present_tracking(&running))
        })
    }

    #[tool(description = "Stop the running TimeTracker timer and add elapsed time to the task. No-op if nothing is running.")]
    fn stop_task(&self, Parameters(_args): Parameters<EmptyArgs>) -> CallToolResult {
        tool_json(|| {
            let db = open_tasks_db()?;
            match time_tracker_core::stop_tracking(&db)? {
                Some(task) => Ok(present_task(&task, None)),
                None => Ok(json!({ "stopped": false })),
            }
        })
    }

    #[tool(description = "Get the currently running TimeTracker timer, if any.")]
    fn get_active_tracking(&self, Parameters(_args): Parameters<EmptyArgs>) -> CallToolResult {
        tool_json(|| {
            let db = open_tasks_db()?;
            match time_tracker_core::get_active_tracking(&db)? {
                Some(running) => Ok(present_tracking(&running)),
                None => Ok(json!({ "running": false })),
            }
        })
    }
}

#[tool_handler]
impl ServerHandler for TimeTracker {
    fn get_info(&self) -> ServerInfo {
        ServerInfo::new(ServerCapabilities::builder().enable_tools().build())
            .with_server_info(Implementation::new("time-tracker", "0.1.0"))
            .with_instructions(
                "Read and write local TimeTracker task logs. total_seconds is committed time only. A running timer is stored in the database and listed as running/running_seconds. Use start_task and stop_task to control it. Do not copy a running display time into set_task_time.",
            )
    }
}

struct DateRange {
    from: String,
    to: String,
}

fn resolve_db_path() -> PathBuf {
    match std::env::var("TIMETRACKER_DB") {
        Ok(value) if value == "dev" => app_support_dir().join("timetracker-dev.db"),
        Ok(value) if value == "prod" || value.is_empty() => app_support_dir().join("timetracker.db"),
        Ok(value) => PathBuf::from(value),
        Err(_) => app_support_dir().join("timetracker.db"),
    }
}

fn app_support_dir() -> PathBuf {
    dirs_home().join(APP_SUPPORT_DIR)
}

fn dirs_home() -> PathBuf {
    std::env::var_os("HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("/"))
}

fn open_tasks_db() -> Result<time_tracker_core::Connection, String> {
    time_tracker_core::open_existing(&resolve_db_path())
}

fn resolve_range(
    date: Option<String>,
    from: Option<String>,
    to: Option<String>,
) -> Result<DateRange, String> {
    if from.is_some() || to.is_some() {
        let resolved_from = from.clone().or_else(|| to.clone()).unwrap_or_else(today_local);
        let resolved_to = to.or(from).unwrap_or_else(today_local);
        parse_local_date(&resolved_from)?;
        parse_local_date(&resolved_to)?;
        if resolved_from > resolved_to {
            return Err(format!("from ({resolved_from}) is after to ({resolved_to})"));
        }
        return Ok(DateRange {
            from: resolved_from,
            to: resolved_to,
        });
    }

    let day = date.unwrap_or_else(today_local);
    parse_local_date(&day)?;
    Ok(DateRange {
        from: day.clone(),
        to: day,
    })
}

fn today_local() -> String {
    Local::now().format("%Y-%m-%d").to_string()
}

fn add_days(date: &str, days: i64) -> String {
    parse_local_date(date)
        .expect("validated date")
        .checked_add_signed(Duration::days(days))
        .expect("date in range")
        .format("%Y-%m-%d")
        .to_string()
}

fn parse_local_date(date: &str) -> Result<NaiveDate, String> {
    NaiveDate::parse_from_str(date, "%Y-%m-%d")
        .map_err(|_| format!("Expected YYYY-MM-DD, got \"{date}\""))
}

fn format_duration(total_seconds: i64) -> String {
    let hours = total_seconds / 3600;
    let minutes = (total_seconds % 3600) / 60;

    if hours > 0 && minutes > 0 {
        format!("{hours}h {minutes}m")
    } else if hours > 0 {
        format!("{hours}h")
    } else if minutes > 0 {
        format!("{minutes}m")
    } else {
        format!("{total_seconds}s")
    }
}

fn present_task(task: &Task, running: Option<&time_tracker_core::ActiveTracking>) -> Value {
    let is_running = running.is_some_and(|active| active.task.id == task.id);
    json!({
        "id": task.id,
        "name": task.name,
        "date": task.created_at.get(..10).unwrap_or(""),
        "duration": format_duration(task.total_seconds),
        "total_seconds": task.total_seconds,
        "note": task.note,
        "running": is_running,
        "running_seconds": is_running.then(|| running.map(|active| active.elapsed_seconds)).flatten(),
    })
}

fn present_tracking(running: &time_tracker_core::ActiveTracking) -> Value {
    json!({
        "running": true,
        "started_at": running.started_at,
        "elapsed_seconds": running.elapsed_seconds,
        "task": present_task(&running.task, Some(running)),
    })
}

fn summarize_tasks(tasks: &[Task]) -> Value {
    let mut by_name: Vec<(String, i64)> = Vec::new();

    for task in tasks {
        if let Some(entry) = by_name.iter_mut().find(|(name, _)| name == &task.name) {
            entry.1 += task.total_seconds;
        } else {
            by_name.push((task.name.clone(), task.total_seconds));
        }
    }

    by_name.sort_by(|a, b| b.1.cmp(&a.1));
    let total_seconds: i64 = by_name.iter().map(|(_, seconds)| *seconds).sum();

    json!({
        "task_count": tasks.len(),
        "total": format_duration(total_seconds),
        "total_seconds": total_seconds,
        "by_name": by_name.iter().map(|(name, seconds)| json!({
            "name": name,
            "duration": format_duration(*seconds),
            "total_seconds": seconds,
        })).collect::<Vec<_>>(),
    })
}

fn tool_json(run: impl FnOnce() -> Result<Value, String>) -> CallToolResult {
    match run() {
        Ok(value) => json_result(value),
        Err(error) => error_result(error),
    }
}

fn json_result(value: Value) -> CallToolResult {
    CallToolResult::success(vec![ContentBlock::text(
        serde_json::to_string_pretty(&value).unwrap_or_else(|_| value.to_string()),
    )])
}

fn error_result(message: String) -> CallToolResult {
    CallToolResult::error(vec![ContentBlock::text(message)])
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    eprintln!("TimeTracker MCP running on stdio");
    eprintln!("Database: {}", resolve_db_path().display());

    TimeTracker::new()
        .serve(stdio())
        .await?
        .waiting()
        .await?;

    Ok(())
}
