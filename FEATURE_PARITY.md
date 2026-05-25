# EWT v1.3.5 parity notes for the Tauri redesign

This patch extends the Tauri + React rewrite to cover the public v1.3.5 feature set from the original Rust + egui Eoliann Windows Tools release.

## Covered categories

- Info: whoami, ipconfig, systeminfo, tasklist, running services, installed apps report.
- Tools / Maintenance: context menu, Disk Cleanup C:, Disk Cleanup all partitions, Recycle Bin cleanup, temp cleanup, Prefetch cleanup, Network Reset, SFC + DISM, restore point, storage settings and system restore cleanup.
- Essential Tweaks: ConsumerFeatures, Telemetry, Location Tracking, Wi-Fi Sense, End Task right-click, Recall, Activity History, Storage Sense, hidden files and file extensions.
- Advanced Tweaks: Edge debloat, Adobe network block/unblock, Adobe services/tasks debloat, Copilot policy, display performance, UTC/local hardware clock, OneDrive remove/restore, Explorer tabs best-effort toggle, O&O ShutUp10++ launcher, DNS providers, gpedit on Home, password expiration toggle and Hyper-V/VBS boot disable.
- Network Tools: insecure SMB guest/signing, DNS providers, TCP offload.
- Power Plans / Power Tweaks: Ultimate, High Performance, Balanced, Power Saver, sleep/drive/display timeout tweaks, hibernation defaults.
- Disk Health: physical disk health, operational status, temperature/wear/errors where Windows exposes them.
- Install: winget install/uninstall/upgrade selections, upgrade all, winget repair shortcut, Chrome install and Chrome Web Store launcher.
- Windows App Removal: communication, media/creativity, Microsoft Apps, Bing Apps, games, system/misc and other bundled app packages.
- Customize Preferences: start with Windows, mouse acceleration, NumLock, taskbar search/widgets/task view, Snap, Sticky Keys, verbose logon and BitLocker actions.
- Health: hibernation, cleanup, restore point cleanup/removal, battery report and memory diagnostic.
- Performance: power plans, HAGS, VBS, startup apps, relaunch apps, background apps, visual effects, transparency, Game Mode, windowed optimizations, Game DVR, SysMain, Windows Search, Delivery Optimization and TCP offload.
- Quick Keys: Win+X note, Win+R equivalent, Win+I/settings, regedit, gpedit, Task Manager, Services, System Information, Disk Management, Device Manager, Event Viewer and Computer Management.
- Settings: Windows light/dark theme, taskbar alignment read/center/left and create local user.

## Best-effort items

Some Windows features vary by Windows 11 build, edition or installed optional components. These are implemented as best-effort actions and should be verified on real Windows 11 machines before publishing:

- Explorer tabs toggle.
- Recall feature enable/disable.
- Copilot legacy policy.
- gpedit enablement on Home editions.
- BitLocker actions on Home editions or devices without BitLocker.
- HAGS and VBS registry flags.
- O&O ShutUp10++ launcher depends on the official current O&O download URL.

## Validation

Frontend TypeScript/Vite build passed in this workspace with:

```powershell
npm run build
```

Rust/Tauri build must be validated on Windows with:

```powershell
npm run tauri build
```
