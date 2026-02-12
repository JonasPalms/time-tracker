# AGENTS.md

Guidance for coding agents working in this repository.

## Project Shape

- `tauri-app/`: main desktop app (Svelte 5 + Tauri v2)
- `raycast-extension/`: read-only companion extension

## High-Signal Rules

- Run commands from the correct package directory (`tauri-app/` or `raycast-extension/`).
- Keep Tauri frontend dev server on port `1420` (required by Tauri config).
- Follow existing Svelte 5 runes pattern in `tauri-app/src/lib/hooks/*.svelte.ts` (module-level state + exported getters).
- Put business logic in services (`tauri-app/src/lib/services/`), not page components.
- Use `@lucide/svelte` for icons.

## Data/Storage Constraints

- SQLite is via `@tauri-apps/plugin-sql`.
- DB path is `~/.config/time-tracker/`.
- Use local-time timestamps (`datetime('now', 'localtime')`).
- Raycast extension must keep DB access read-only (avoid lock-causing writes).

## Validation Commands

- Desktop app (`tauri-app/`): `npm run check` and `npm run build`
- Raycast extension (`raycast-extension/`): `npm run lint` and `npm run build`

## Release Notes

- For version bumps: update `tauri-app/src-tauri/tauri.conf.json`, update `CHANGELOG.md`, then tag as `vX.Y.Z`.
