# ⏱️ Time Tracker – Svelte + Tauri Desktop App

A lightweight, cross-platform **time registration app** built with **Svelte** (frontend) and **Tauri** (native desktop shell).  
It features a simple timer, history logging, and local data persistence using **SQLite** via Tauri's native plugin system.

## 💡 Key Features (MVP)

- Start/stop timer with real-time display
- Log time entries to a local SQLite database
- View history of tracked sessions
- Cross-platform desktop app (macOS, Windows, Linux)
- Lightweight, fast, and built entirely with JavaScript & Rust backend via Tauri

## 🧱 Stack

- **Frontend:** Svelte + Vite
- **Native Runtime:** Tauri (Rust)
- **Database:** SQLite (via `tauri-plugin-sql`)
- **Bundling:** Tauri for native builds

## 🛠️ Commands

```bash
# Start frontend + Tauri dev shell
npm run tauri dev

# Build frontend + package native app
npm run tauri build
```
