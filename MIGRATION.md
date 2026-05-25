# Migration from the egui version

## Recommended workflow

```powershell
git checkout -b redesign-tauri-react
# copy these files over the repository root
npm install
npm run tauri dev
```

## Old → new mapping

| Old egui area | New React page |
| --- | --- |
| Info | Dashboard |
| Tools | Maintenance, Tweaks, Network |
| Disk Health | Disk Health |
| Install | Apps |
| WinApp Removal | Windows Apps |
| Customize Preferences | Tweaks, Settings |
| Health | Maintenance |
| Performance | Performance/Tweaks actions |
| Quick Keys | Settings quick launch cards |
| Settings | Settings/About |

## Backend strategy

The backend intentionally does not expose a generic `execute_command(command: string)` function. All Windows actions are matched by `action_id` in Rust. This makes the frontend simpler and reduces accidental command injection risks.
