# Changelog

All notable changes to TimeTracker will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.5.3] - 2026-01-27

### Added

- Date picker on edit task page to change task date
- Reusable Calendar and DatePicker UI components
- PageHeader component for consistent page header styling

### Changed

- Edit page back button now returns to previous page instead of always going home
- History page header layout matches front page style with navigation on left
- History page shows weekly total time in header row

## [0.5.2] - 2026-01-21

### Added

- Collapsible sidebar navigation replacing top header navigation
- Draggable sidebar resize handle (60-300px width, snaps to collapsed below 100px)
- Sidebar state persists to localStorage

### Changed

- Window size increased to 900x700 (was 600x800) with minimum constraints
- History page: removed redundant title, centered week navigation with accent styling
- Improved visual separation with title bar border

## [0.5.1] - 2026-01-15

### Changed

- Redesigned history page with cleaner flat layout
- Task entries are now clickable buttons to navigate to edit page
- Collapsible day sections with smooth accordion animation
- Improved accessibility with aria-expanded and aria-controls attributes

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
