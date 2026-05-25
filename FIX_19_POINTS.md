# EWT v2 - 19 essential fixes

This patch treats the 19 reported points as release blockers.

## Fixed in this patch

1. **Not elevated wording** - UI now says `Standard user`; administrator actions still show admin requirements. Release manifest requests administrator privileges.
2. **Dashboard startup delay** - Dashboard now loads only system overview. Disk Health loads only on its own page or manual refresh.
3. **System Restore cleanup description/action** - UI now states that restore points / shadow copies are deleted; backend uses `vssadmin delete shadows /all /quiet`.
4. **Command freeze/status** - Tauri commands are async wrappers using `spawn_blocking`; UI shows a blocking status overlay while an operation runs.
5. **Battery report path** - Battery report opens a Windows Save File dialog and saves to the user-chosen path.
6. **Classic context menu stuck/freezing** - Registry command uses direct `reg` calls and Explorer restart is detached.
7. **Separate Restart Explorer button** - Removed from UI actions; restart remains internal for tweaks that require it.
8. **Running stuck after command** - Action state resets in `finally`; heavy refresh no longer runs automatically after each command.
9. **Taskbar icon** - Tauri bundle icon config and Windows `.ico` are included. This still requires Windows runtime validation.
10. **UAC** - Release build includes a Windows manifest with `requireAdministrator`. `tauri dev` may behave differently.
11. **v1.3.5 function parity** - Public README categories and v1.3.5 install catalog were used as the parity checklist; Windows validation is still required for each command.
12. **Admin-required hibernation commands** - UAC manifest and admin badges cover these; command still requires elevation by design.
13. **Install shows installed apps** - Install page scans `winget list` and shows `Installed` badges.
14. **Install categories/catalog** - Install catalog restored to categories from v1.3.5: Browsers, Utilities, Communications, Development, Document, Multimedia Tools, Games, Microsoft Tools, Pro Tools.
15. **Install status** - Global overlay and log panel show operation state while winget runs.
16. **Deselect after install** - Selection clears after winget operation completes, then installed scan refreshes.
17. **Scroll top on page change** - Main content scrolls to top whenever a new section is selected.
18. **Invalid REG syntax** - Start with Windows, Mouse Acceleration and NumLock now use PowerShell registry writes instead of broken `reg add` quoting.
19. **Sidebar not visible at default size** - Sidebar is compact and scrollable; nav area has its own vertical scroll and footer stays visible.

## About/legal/support

About now contains project details, Terms & Conditions, Privacy Policy and Donate/Support links.
