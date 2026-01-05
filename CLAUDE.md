# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

TimeTracker is a cross-platform time tracking suite with two main components:
- **tauri-app**: Native desktop application built with Svelte 5 + Tauri v2
- **raycast-extension**: Read-only Raycast extension for macOS quick access

## Development Commands

### Desktop App (tauri-app/)

```bash
npm run dev              # Start Vite dev server (port 1420)
npm run tauri dev        # Start Tauri with hot reload
npm run build            # Build frontend
npm run tauri build      # Build native app
npm run check            # TypeScript type checking
npm run check:watch      # Watch mode type checking
npm run format           # Format with Prettier
npm run format:check     # Check formatting
```

### Raycast Extension (raycast-extension/)

```bash
npm run dev              # Start Raycast dev mode
npm run build            # Build extension
npm run lint             # ESLint check
npm run fix-lint         # Auto-fix lint issues
```

## Architecture

### Data Flow
```
Svelte Components → Hooks (state) → Services → SQLite Database
```

### Key Directories (tauri-app/src/)

- `routes/` - SvelteKit pages (+page.svelte is main view)
- `lib/hooks/` - Reactive state using Svelte 5 runes ($state, $derived)
- `lib/services/` - Business logic (db.ts, tasks.ts, favourites.ts, settings.ts)
- `lib/components/` - UI components including shadcn/bits-ui in `ui/`
- `lib/utils/` - Utilities (time formatting, class names)

### State Management Pattern

Uses Svelte 5 runes with module-level state in `lib/hooks/*.svelte.ts` files:
- `tracking.svelte.ts` - Global timer state
- `theme.svelte.ts` - Dark/light mode
- `favourites.svelte.ts` - Favorites state
- Export getter functions to maintain reactivity

### Database

SQLite via @tauri-apps/plugin-sql. Tables:
- `tasks` (id, name, total_seconds, created_at)
- `favourites` (id, name, duration_seconds, created_at)

Timestamps use local timezone: `datetime('now', 'localtime')`

## Configuration Notes

- Vite runs on fixed port 1420 (required by Tauri)
- Frameless transparent window with custom traffic light controls
- macOS private APIs enabled in tauri.conf.json
- Database location: `~/.config/time-tracker/`
- Raycast extension uses read-only DB access to prevent locks

## Release Process

- GitHub Actions workflow triggered by `v*` tags
- Builds for Apple Silicon (aarch64) and Intel (x86_64)
- Creates draft GitHub Release
- Changelog follows [Keep a Changelog](https://keepachangelog.com/) format

### Version Bump Workflow

1. Update version in `tauri-app/src-tauri/tauri.conf.json`
2. Add release entry to `CHANGELOG.md` with date and changes
3. Commit with message: `chore: bump version to X.Y.Z`
4. Create and push tag: `git tag vX.Y.Z && git push origin vX.Y.Z`
