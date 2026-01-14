# Changelog

All notable changes to TimeTracker will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.5.0] - 2026-01-14

### Changed

- Moved database layer from TypeScript to Rust using rusqlite
- Frontend now communicates with backend via Tauri commands instead of direct SQL
- Removed @tauri-apps/plugin-sql dependency

## [0.4.0] - 2026-01-09

### Added

- Task edit page: dedicated page for editing task name, time, and notes
- Notes field for tasks (plain text)
- Edit button in task dropdown menu and history page

### Fixed

- Fixed Svelte warning about state reference in TaskItemName component

## [0.3.6] - 2026-01-08

### Changed

- Replaced Popover-based dropdown with simpler custom dropdown for task input
- Improved dropdown behavior: blurs input after submitting task

### Fixed

- Fixed dropdown not reopening after submitting task with Enter key

## [0.3.5] - 2026-01-08

### Fixed

- Close dropdown when clicking outside or switching apps
- Fix Enter key selection causing dropdown to reopen
- Escape key now clears input while keeping dropdown open
- Disable native spellcheck/autocorrect on task input

## [0.3.2] - 2026-01-05

### Fixed

- Prevent dropdown auto-highlight when opening task input

## [0.3.1] - 2026-01-05

### Fixed

- Fixed dropdown auto-highlight issue

## [0.3.0] - 2026-01-02

### Changed

- Redesigned settings dialog with cleaner list-based layout
- Improved favourites management with inline add button
- Removed excessive borders from individual settings items
- Added smooth transitions and hover states throughout settings

## [0.2.0] - 2025-XX-XX

### Added

- Initial release of TimeTracker
- Task tracking with start/stop functionality
- Favourite tasks for quick access
- Custom macOS-style window controls (traffic lights)
- Frameless transparent window design
- SQLite database for persistent storage
- Auto-updater support via GitHub Releases

### Technical

- Built with Tauri v2, SvelteKit, and TypeScript
- macOS support (Apple Silicon and Intel)
