use std::path::PathBuf;

use chrono::{Duration, Local, NaiveDate};
use rmcp::{
    handler::server::{router::tool::ToolRouter, wrapper::Parameters},
    model::{CallToolResult, ContentBlock, Implementation, ServerCapabilities, ServerInfo},
    schemars, tool, tool_handler, tool_router, ServerHandler, ServiceExt,
    transport::stdio,
};
use rusqlite::{Connection, OpenFlags};
use serde::Deserialize;
use serde_json::{json, Value};

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

struct TaskRow {
    id: i64,
    name: String,
    total_seconds: i64,
    created_at: String,
    note: Option<String>,
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
        match (|| {
            let range = resolve_range(args.date, args.from, args.to)?;
            let db = open_tasks_db()?;
            let tasks = list_tasks_in_range(&db, &range.from, &range.to)?;
            Ok(json!({
                "from": range.from,
                "to": range.to,
                "tasks": tasks.iter().map(present_task).collect::<Vec<_>>(),
            }))
        })() {
            Ok(value) => json_result(value),
            Err(error) => error_result(error),
        }
    }

    #[tool(description = "Sum TimeTracker hours by task name for an inclusive local date range. Defaults to the last 7 days.")]
    fn summarize_range(&self, Parameters(args): Parameters<SummarizeArgs>) -> CallToolResult {
        match (|| {
            let today = today_local();
            let range = resolve_range(
                None,
                Some(args.from.unwrap_or_else(|| add_days(&today, -6))),
                Some(args.to.unwrap_or(today)),
            )?;
            let db = open_tasks_db()?;
            let tasks = list_tasks_in_range(&db, &range.from, &range.to)?;
            let mut summary = summarize_tasks(&tasks);
            summary["from"] = json!(range.from);
            summary["to"] = json!(range.to);
            Ok(summary)
        })() {
            Ok(value) => json_result(value),
            Err(error) => error_result(error),
        }
    }

    #[tool(description = "Search TimeTracker task names and notes. Defaults to the last 90 days if no range is given.")]
    fn search_tasks(&self, Parameters(args): Parameters<SearchArgs>) -> CallToolResult {
        match (|| {
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
            let tasks = search_task_rows(&db, &args.query, &range.from, &range.to)?;
            Ok(json!({
                "query": args.query,
                "from": range.from,
                "to": range.to,
                "tasks": tasks.iter().map(present_task).collect::<Vec<_>>(),
            }))
        })() {
            Ok(value) => json_result(value),
            Err(error) => error_result(error),
        }
    }
}

#[tool_handler]
impl ServerHandler for TimeTracker {
    fn get_info(&self) -> ServerInfo {
        ServerInfo::new(ServerCapabilities::builder().enable_tools().build())
            .with_server_info(Implementation::new("time-tracker", "0.1.0"))
            .with_instructions("Read-only access to local TimeTracker task logs.")
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

fn open_tasks_db() -> Result<Connection, String> {
    let path = resolve_db_path();
    if !path.exists() {
        return Err(format!("TimeTracker database not found at {}", path.display()));
    }

    Connection::open_with_flags(&path, OpenFlags::SQLITE_OPEN_READ_ONLY)
        .map_err(|error| error.to_string())
}

fn list_tasks_in_range(db: &Connection, from: &str, to: &str) -> Result<Vec<TaskRow>, String> {
    let mut stmt = db
        .prepare(
            "SELECT id, name, total_seconds, created_at, note
             FROM tasks
             WHERE date(created_at) >= ? AND date(created_at) <= ?
             ORDER BY created_at DESC",
        )
        .map_err(|error| error.to_string())?;

    let rows = stmt
        .query_map([from, to], map_task_row)
        .map_err(|error| error.to_string())?;

    rows.collect::<Result<Vec<_>, _>>()
        .map_err(|error| error.to_string())
}

fn search_task_rows(
    db: &Connection,
    query: &str,
    from: &str,
    to: &str,
) -> Result<Vec<TaskRow>, String> {
    let like = format!("%{query}%");
    let mut stmt = db
        .prepare(
            "SELECT id, name, total_seconds, created_at, note
             FROM tasks
             WHERE date(created_at) >= ? AND date(created_at) <= ?
               AND (name LIKE ? OR IFNULL(note, '') LIKE ?)
             ORDER BY created_at DESC",
        )
        .map_err(|error| error.to_string())?;

    let rows = stmt
        .query_map((from, to, like.as_str(), like.as_str()), map_task_row)
        .map_err(|error| error.to_string())?;

    rows.collect::<Result<Vec<_>, _>>()
        .map_err(|error| error.to_string())
}

fn map_task_row(row: &rusqlite::Row<'_>) -> rusqlite::Result<TaskRow> {
    Ok(TaskRow {
        id: row.get(0)?,
        name: row.get(1)?,
        total_seconds: row.get(2)?,
        created_at: row.get(3)?,
        note: row.get(4)?,
    })
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

fn present_task(task: &TaskRow) -> Value {
    json!({
        "id": task.id,
        "name": task.name,
        "date": task.created_at.get(..10).unwrap_or(""),
        "duration": format_duration(task.total_seconds),
        "total_seconds": task.total_seconds,
        "note": task.note,
    })
}

fn summarize_tasks(tasks: &[TaskRow]) -> Value {
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
    #[cfg(debug_assertions)]
    {
        let _ = dotenvy::dotenv();
    }

    eprintln!("TimeTracker MCP running on stdio");
    eprintln!("Database: {}", resolve_db_path().display());

    TimeTracker::new()
        .serve(stdio())
        .await?
        .waiting()
        .await?;

    Ok(())
}
