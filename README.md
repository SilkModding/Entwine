

# Entwine for SpiderHeck

Entwine is a cross-platform desktop app for managing SpiderHeck mods, configurations, and versions. It uses Svelte for the UI and Tauri (Rust) for native integration and performance.

Repository: https://github.com/SilkModding/Entwine

## Features
- Install, update, and remove SpiderHeck mods
- Edit mod configurations
- Manage game component versions
- Setup wizard for first-time users

## Requirements
- Node.js (latest LTS recommended)
- pnpm (or npm/yarn)
- Rust (stable toolchain)
- Tauri prerequisites:
  - [Rust and Cargo](https://www.rust-lang.org/tools/install)
  - [Tauri CLI](https://tauri.app/v1/guides/getting-started/prerequisites/)
  - On Linux: `libwebkit2gtk-4.0-dev`, `build-essential`, `curl`, `wget`, `libssl-dev`, `appmenu-gtk-module`, `libgtk-3-dev`, `libayatana-appindicator3-dev`

## Installation
1. Clone the repository:
	```bash
	git clone https://github.com/SilkModding/Entwine.git
	cd Entwine
	```
2. Install dependencies:
	```bash
	pnpm install
	```
3. Run the app:
	```bash
    pnpm tauri dev
	```