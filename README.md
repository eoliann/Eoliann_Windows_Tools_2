[![Group](https://img.shields.io/badge/Group-Telegram-blue?style=plastic)](https://t.me/eoliannwindowstool)
![Followers](https://img.shields.io/github/followers/eoliann?style=plastic&color=green)
![Watchers](https://img.shields.io/github/watchers/eoliann/Eoliann_Windows_Tools_2?style=plastic)
![Stars](https://img.shields.io/github/stars/eoliann/Eoliann_Windows_Tools_2?style=plastic)
[![Donate](https://img.shields.io/badge/Donate-PayPal-blue?style=plastic)](https://www.paypal.com/donate/?hosted_button_id=PTH2EXUDS423S)
[![Donate](https://img.shields.io/badge/Donate-Revolut-8A2BE2?style=plastic)](https://revolut.me/adriannm9)
[![Donate](https://img.shields.io/badge/Donate-KoFi-green?style=plastic)](https://ko-fi.com/eoliann)

![Release Date](https://img.shields.io/github/release-date/eoliann/Eoliann_Windows_Tools_2?style=plastic)
![Last Commit](https://img.shields.io/github/last-commit/eoliann/Eoliann_Windows_Tools_2?style=plastic)
[![License: MIT](https://img.shields.io/badge/License-MIT-green.svg?style=plastic)](LICENSE.md)
![OS](https://img.shields.io/badge/OS-Windows-blue?style=plastic)
![Lang](https://img.shields.io/badge/Lang-React_Rust-magenta?style=plastic)

![Total Downloads](https://img.shields.io/github/downloads/eoliann/Eoliann_Windows_Tools_2/total?style=plastic)
![](https://img.shields.io/github/downloads/eoliann/Eoliann_Windows_Tools_2/latest/eoliann-windows-tools.exe?displayAssetName=true&style=plastic&color=green)
![](https://img.shields.io/github/downloads/eoliann/Eoliann_Windows_Tools_2/latest/Eoliann.Windows.Tools_x64-setup.exe?displayAssetName=true&style=plastic&color=red)


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
