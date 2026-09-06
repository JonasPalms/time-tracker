use std::env;

#[tauri::command]
pub fn get_mcp_binary_path() -> Result<String, String> {
    let path = env::current_exe()
        .map_err(|error| error.to_string())?
        .with_file_name("timetracker-mcp");

    if !path.exists() {
        return Err(format!("MCP binary not found at {}", path.display()));
    }

    Ok(path.to_string_lossy().into_owned())
}
