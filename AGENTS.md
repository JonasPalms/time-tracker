# AGENTS.md

Guidance for coding agents working in this repository.

## Project Shape

- Root: desktop app (Svelte 5 + Tauri v2)
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
- Prod SQLite by default. Local override: copy `.env.example` to `.env` and set `TIMETRACKER_DB=dev` or `prod`.
- Read-only tools: `list_tasks`, `summarize_range`, `search_tasks`
- Settings → MCP copies a snippet that points at that binary.
- Rebuild with `pnpm mcp:bin`. `tauri dev` / `tauri build` do this first.

## Validation Commands

- Desktop app (repo root): `pnpm check` and `pnpm build`
- MCP binary: `pnpm mcp:bin`

## Release Notes

- Version bumps: update `src-tauri/tauri.conf.json`, update `CHANGELOG.md`, then tag as `vX.Y.Z`.
