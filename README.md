# ⏱️ Time Tracker

[![Latest Release](https://img.shields.io/github/v/release/JonasPalms/time-tracker)](https://github.com/JonasPalms/time-tracker/releases/latest)
[![Downloads](https://img.shields.io/github/downloads/JonasPalms/time-tracker/total)](https://github.com/JonasPalms/time-tracker/releases/latest)
[![License](https://img.shields.io/github/license/JonasPalms/time-tracker)](LICENSE)

A lightweight, cross-platform **time tracking suite** for developers and productivity enthusiasts.

## 📦 Download

[**Download the latest version**](https://github.com/JonasPalms/time-tracker/releases/latest)

### Available Platforms

- 🍎 **macOS** (Apple Silicon & Intel)
- 🪟 **Windows** (coming soon)
- 🐧 **Linux** (coming soon)

## 🚀 What's Included

This repository contains:

### 🖥️ Desktop App

A native desktop application built with **Svelte** and **Tauri**.

**Features:**

- ⏱️ Simple start/stop timer with real-time display
- 📊 View history of all tracked sessions
- 💾 Local SQLite database for data persistence
- 🎨 Clean, minimal UI with dark/light mode
- 🔄 Automatic updates (macOS)
- ⭐ Favorites for quick time entry
- ⌨️ Keyboard shortcuts

## 🛠️ Development Setup

### Prerequisites

- Node.js + npm
- Rust toolchain (`cargo`, `rustc`) for Tauri

If `npm run tauri dev` fails with a `cargo metadata` error, install Rust with:

```bash
xcode-select --install
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
source "$HOME/.cargo/env"
```

To make Cargo available in every new terminal session, add this to `~/.zshrc`:

```bash
source "$HOME/.cargo/env"
```

### Run locally

```bash
npm install
npm run tauri dev
```

## 📄 License

This project is open source and available under the [MIT License](LICENSE).
