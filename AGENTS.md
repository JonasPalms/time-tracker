# AGENTS.md

Guidance for coding agents working in this repository.

## Project Shape

- Root: desktop app (Svelte 5 + Tauri v2)
- `src-tauri/core/`: shared SQLite access (WAL, queries, writes)
- `src-tauri/mcp-server/`: `timetracker-mcp` binary shipped inside the app

## High-Signal Rules

- Run Tauri commands from the repository root.
- Keep Tauri frontend dev server on port `1420` (required by Tauri config).
- Follow existing Svelte 5 runes pattern in `src/lib/hooks/*.svelte.ts` (module-level state + exported getters).
- Put Tauri business logic in services (`src/lib/services/`), not page components.
- Use `@lucide/svelte` for icons.

## Data/Storage Constraints

- Tauri SQLite lives at `~/Library/Application Support/com.jonaspalmsorensen.time-tracker/`.
- Use local-time timestamps (`YYYY-MM-DD HH:MM:SS`).

## MCP

- Clients launch the `timetracker-mcp` binary next to the app. Do not write other apps' configs.
- Prod SQLite by default. Local override: set `TIMETRACKER_DB=dev` in the client MCP env (e.g. `.cursor/mcp.json`). The app ignores this — debug uses `timetracker-dev.db`, release uses `timetracker.db`.
- Read tools: `list_tasks`, `summarize_range`, `search_tasks`
- Write tools: `create_task`, `set_task_time`, `add_task_time`, `set_task_name`, `set_task_note`, `set_task_date`, `delete_task`. No start/stop of the in-app timer.
- Settings → MCP copies a snippet that points at the sidecar next to the running app.
- Repo/dev MCP should use `src-tauri/binaries/timetracker-mcp` (what `pnpm mcp:bin` writes), not `target/debug/`.
- Rebuild with `pnpm mcp:bin`. `tauri dev` / `tauri build` do this first.

## Validation Commands

- Desktop app (repo root): `pnpm check` and `pnpm build`
- MCP binary: `pnpm mcp:bin`

## Release Notes

- Version bumps: update `src-tauri/tauri.conf.json`, update `CHANGELOG.md`, then tag as `vX.Y.Z`.
