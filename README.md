# Eoliann Windows Tools 2

Windows 11 administration toolkit built with React + Tauri (Rust).

## Overview

Eoliann Windows Tools centralizes common maintenance and optimization tasks in a single desktop app:

- system information and diagnostics
- cleanup and maintenance tools
- networking and DNS actions
- performance and privacy tweaks
- winget app management
- Windows bundled app removal

The frontend calls a fixed set of backend commands (no arbitrary command execution from UI).

## Tech Stack

- Frontend: React 19 + TypeScript + Vite
- Desktop shell: Tauri 2
- Backend: Rust

## Requirements

- Windows 11
- Node.js 20+
- Rust stable toolchain
- Microsoft WebView2 Runtime
- Administrator rights for elevated actions

## Run Locally

```powershell
npm install
npm run tauri dev
```

## Build

Frontend build only:

```powershell
npm run build
```

Desktop installer/bundles:

```powershell
npm run tauri build
```

Build artifacts are generated under `src-tauri/target/release/bundle`.

## Repository Structure

- `src/` - React UI
- `src-tauri/src/` - Rust commands and app entry
- `src-tauri/icons/` - app icons
- `scripts/` - helper scripts
- `installer/` - installer-related resources

## Safety Notice

Some actions modify system settings, registry, services, packages, or network configuration.
Use on your own responsibility and create a restore point before applying high-risk changes.

## License

This project is licensed under the MIT License. See `LICENSE`.
