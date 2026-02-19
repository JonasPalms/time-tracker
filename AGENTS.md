# AGENTS.md

Guidance for coding agents working in this repository.

## Project Shape

- Main desktop app (Svelte 5 + Tauri v2)

## High-Signal Rules

- Run commands from root directory.
- Keep Tauri frontend dev server on port `1420` (required by Tauri config).
- Follow existing Svelte 5 runes pattern in `src/lib/hooks/*.svelte.ts` (module-level state + exported getters).
- Put business logic in services (`src/lib/services/`), not page components.
- Use `@lucide/svelte` for icons.

## Data/Storage Constraints

- SQLite is via `@tauri-apps/plugin-sql`.
- DB path is `~/.config/time-tracker/`.
- Use local-time timestamps (`datetime('now', 'localtime')`).

## Validation Commands

- Desktop app: `npm run check` and `npm run build`

## Release Notes

- For version bumps: update `src-tauri/tauri.conf.json`, update `CHANGELOG.md`, then tag as `vX.Y.Z`.
