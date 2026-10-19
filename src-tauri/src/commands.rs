use serde::{Deserialize, Serialize};
use tauri::Emitter;
use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio};

const APP_VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Debug, Serialize, Clone)]
pub struct DiskSummary {
    pub name: String,
    pub total_gb: f64,
    pub free_gb: f64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct PsDiskSummary {
    #[serde(rename = "Name")]
    pub name: Option<String>,
    #[serde(rename = "TotalGB")]
    pub total_gb: Option<f64>,
    #[serde(rename = "FreeGB")]
    pub free_gb: Option<f64>,
}

#[derive(Debug, Serialize)]
pub struct SystemOverview {
    pub computer_name: String,
    pub username: String,
    pub windows_caption: String,
    pub version: String,
    pub architecture: String,
    pub cpu: String,
    pub ram_gb: f64,
    pub disks: Vec<DiskSummary>,
    pub uptime: String,
    pub is_admin: bool,
    pub app_version: String,
    pub winget_available: bool,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct PsOverview {
    pub computer_name: Option<String>,
    pub username: Option<String>,
    pub windows_caption: Option<String>,
    pub version: Option<String>,
    pub architecture: Option<String>,
    pub cpu: Option<String>,
    #[serde(rename = "RamGB")]
    pub ram_gb: Option<f64>,
    pub disks: Option<Vec<PsDiskSummary>>,
    pub uptime: Option<String>,
    pub winget_available: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DiskHealth {
    pub friendly_name: String,
    pub media_type: String,
    pub health_status: String,
    pub operational_status: String,
    pub size_gb: f64,
    pub temperature: Option<i64>,
    pub wear: Option<i64>,
    pub read_errors: Option<i64>,
    pub write_errors: Option<i64>,
}

#[derive(Debug, Serialize)]
pub struct RestorePointInfo {
    pub shadow_id: String,
    pub sequence_number: i64,
    pub description: String,
    pub creation_time: String,
}

#[derive(Debug, Serialize)]
pub struct RestorePointsSummary {
    pub total_used_space_gb: f64,
    pub points: Vec<RestorePointInfo>,
}

#[derive(Debug, Serialize, Clone)]
pub struct WingetProgressEvent {
    pub percent: u8,
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SmbGuestStatus {
    pub enable_insecure_guest_logons: Option<bool>,
    pub require_security_signature: Option<bool>,
    pub policy_allow_insecure_guest_auth: Option<i64>,
    pub legacy_allow_insecure_guest_auth: Option<i64>,
    pub smb1_enabled: Option<bool>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct PsRestorePointInfo {
    pub shadow_id: Option<String>,
    pub sequence_number: Option<i64>,
    pub description: Option<String>,
    pub creation_time: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct WindowsAppPackage {
    pub name: String,
    pub family_name: String,
    pub installed: bool,
    pub provisioned: bool,
}

#[derive(Debug, Serialize)]
pub struct WindowsAppsStatus {
    pub packages: Vec<WindowsAppPackage>,
    pub deprovisioned: Vec<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WindowsAppTarget {
    pub package_name: String,
    pub family_name: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct PsWindowsAppPackage {
    pub name: Option<String>,
    pub family_name: Option<String>,
    pub installed: Option<bool>,
    pub provisioned: Option<bool>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct PsWindowsAppsStatus {
    pub packages: Option<Vec<PsWindowsAppPackage>>,
    pub deprovisioned: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct PsDiskHealth {
    pub friendly_name: Option<String>,
    pub media_type: Option<String>,
    pub health_status: Option<String>,
    pub operational_status: Option<String>,
    #[serde(rename = "SizeGB")]
    pub size_gb: Option<f64>,
    pub temperature: Option<i64>,
    pub wear: Option<i64>,
    pub read_errors: Option<i64>,
    pub write_errors: Option<i64>,
}


async fn run_blocking<T, F>(task: F) -> Result<T, String>
where
    T: Send + 'static,
    F: FnOnce() -> Result<T, String> + Send + 'static,
{
    tauri::async_runtime::spawn_blocking(task)
        .await
        .map_err(|error| format!("Background task failed: {error}"))?
}

#[tauri::command]
pub async fn get_system_overview() -> Result<SystemOverview, String> {
    run_blocking(get_system_overview_sync).await
}

#[tauri::command]
pub async fn get_disk_health() -> Result<Vec<DiskHealth>, String> {
    run_blocking(get_disk_health_sync).await
}

#[tauri::command]
pub async fn run_action(action_id: String) -> Result<String, String> {
    run_blocking(move || run_action_sync(action_id)).await
}

#[tauri::command]
pub async fn winget_action(operation: String, package_ids: Vec<String>) -> Result<String, String> {
    run_blocking(move || winget_action_sync(operation, package_ids)).await
}

#[tauri::command]
pub async fn open_system_tool(tool_id: String) -> Result<String, String> {
    run_blocking(move || open_system_tool_sync(tool_id)).await
}

#[tauri::command]
pub async fn get_installed_apps() -> Result<String, String> {
    run_blocking(get_installed_apps_sync).await
}

#[tauri::command]
pub async fn create_local_user(username: String, password: String, add_to_admins: bool) -> Result<String, String> {
    run_blocking(move || create_local_user_sync(username, password, add_to_admins)).await
}

#[tauri::command]
pub async fn winget_upgrade_all_progress(app: tauri::AppHandle) -> Result<String, String> {
    run_blocking(move || winget_upgrade_all_progress_sync(app)).await
}

#[tauri::command]
pub async fn install_chrome_extensions(extension_ids: Vec<String>) -> Result<String, String> {
    run_blocking(move || install_chrome_extensions_sync(extension_ids)).await
}

#[tauri::command]
pub async fn get_restore_points() -> Result<RestorePointsSummary, String> {
    run_blocking(get_restore_points_sync).await
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteRestorePointArgs {
    pub sequence_number: Option<i64>,
}

#[tauri::command]
pub async fn delete_restore_point_by_info(args: DeleteRestorePointArgs) -> Result<String, String> {
    run_blocking(move || delete_restore_point_by_info_sync(args)).await
}

#[tauri::command]
pub async fn get_smb_guest_status() -> Result<SmbGuestStatus, String> {
    run_blocking(get_smb_guest_status_sync).await
}

fn args(values: &[&str]) -> Vec<String> {
    values.iter().map(|value| value.to_string()).collect()
}

fn command_output(program: &str, args: &[String]) -> Result<String, String> {
    let mut command = Command::new(program);
    command.args(args).stdin(Stdio::null()).stdout(Stdio::piped()).stderr(Stdio::piped());

    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        const CREATE_NO_WINDOW: u32 = 0x08000000;
        command.creation_flags(CREATE_NO_WINDOW);
    }

    let output = command.output().map_err(|error| format!("Failed to start {program}: {error}"))?;
    let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();

    if output.status.success() {
        if stdout.is_empty() && !stderr.is_empty() { Ok(stderr) } else { Ok(stdout) }
    } else {
        let code = output.status.code().map_or("unknown".to_string(), |code| code.to_string());
        Err(format!("{program} exited with code {code}.\n{}", if stderr.is_empty() { stdout } else { stderr }))
    }
}

fn command_status(program: &str, args: &[String]) -> bool {
    let mut command = Command::new(program);
    command.args(args).stdin(Stdio::null()).stdout(Stdio::null()).stderr(Stdio::null());

    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        const CREATE_NO_WINDOW: u32 = 0x08000000;
        command.creation_flags(CREATE_NO_WINDOW);
    }

    command.status().map(|status| status.success()).unwrap_or(false)
}

fn parse_percent(line: &str) -> Option<u8> {
    for token in line.split_whitespace() {
        if let Some(value) = token.strip_suffix('%') {
            if let Ok(parsed) = value.trim().parse::<u8>() {
                return Some(parsed.min(100));
            }
        }
    }
    None
}

fn is_spinner_or_noise(line: &str) -> bool {
    let normalized = normalize_output_line(line);
    let trimmed = normalized.trim();
    if trimmed.is_empty() {
        return true;
    }
    matches!(trimmed, "-" | "\\" | "|" | "/")
}

fn normalize_output_line(line: &str) -> String {
    let mut out = String::with_capacity(line.len());
    for ch in line.chars() {
        if ch == '\r' || ch == '\n' {
            continue;
        }
        if ch.is_control() {
            continue;
        }
        out.push(ch);
    }
    out
}

fn winget_upgrade_all_progress_sync(app: tauri::AppHandle) -> Result<String, String> {
    let mut command = Command::new("winget");
    command
        .args([
            "upgrade",
            "--all",
            "--include-unknown",
            "--accept-package-agreements",
            "--accept-source-agreements",
            "--disable-interactivity",
        ])
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());

    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        const CREATE_NO_WINDOW: u32 = 0x08000000;
        command.creation_flags(CREATE_NO_WINDOW);
    }

    let _ = app.emit("winget-progress", WingetProgressEvent { percent: 0, message: "Starting winget upgrade --all...".into() });

    let mut child = command.spawn().map_err(|error| format!("Failed to start winget: {error}"))?;
    let stdout = child.stdout.take().ok_or_else(|| "Failed to capture winget stdout.".to_string())?;
    let reader = BufReader::new(stdout);
    let mut collected = String::new();
    let mut last_percent: u8 = 0;

    for line in reader.lines() {
        let line = normalize_output_line(&line.unwrap_or_default());
        if is_spinner_or_noise(&line) {
            continue;
        }
        collected.push_str(&line);
        collected.push('\n');
        if let Some(percent) = parse_percent(&line) {
            last_percent = percent;
        }
        let _ = app.emit("winget-progress", WingetProgressEvent { percent: last_percent, message: line.clone() });
    }

    let output = child.wait_with_output().map_err(|error| format!("Failed waiting for winget: {error}"))?;
    let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();

    if output.status.success() {
        let normalized = collected.to_lowercase();
        if normalized.contains("no installed package found matching input criteria") {
            let msg = "All detected applications are already up to date or not upgradeable by winget.".to_string();
            let _ = app.emit("winget-progress", WingetProgressEvent { percent: 100, message: msg.clone() });
            return Ok(format!("{}\n\n{}", msg, collected.trim()));
        }
        let _ = app.emit("winget-progress", WingetProgressEvent { percent: 100, message: "winget upgrade --all completed.".into() });
        let final_text = if collected.trim().is_empty() { "winget upgrade --all completed.".to_string() } else { collected.trim().to_string() };
        Ok(final_text)
    } else {
        let _ = app.emit("winget-progress", WingetProgressEvent { percent: last_percent, message: "winget upgrade --all failed.".into() });
        Err(if stderr.is_empty() { "winget upgrade --all failed.".into() } else { stderr })
    }
}

// `New-Item <key> -Force` wipes every value and subkey of an existing registry key,
// so scripts must use Ensure-Key to create a key only when it is missing.
const PS_PRELUDE: &str = "function Ensure-Key([string]$Path) { if (-not (Test-Path -LiteralPath $Path)) { New-Item -Path $Path -Force | Out-Null } }\n";

fn powershell(script: &str) -> Result<String, String> {
    let full = format!("{PS_PRELUDE}{script}");
    command_output("powershell", &args(&["-NoProfile", "-NonInteractive", "-Command", &full]))
}

fn powershell_sta(script: &str) -> Result<String, String> {
    let full = format!("{PS_PRELUDE}{script}");
    command_output("powershell", &args(&["-NoProfile", "-STA", "-Command", &full]))
}

fn cmd_shell(command: &str) -> Result<String, String> {
    command_output("cmd", &args(&["/C", command]))
}

fn is_admin() -> bool {
    command_status("net", &args(&["session"]))
}

fn ps_single_quote(value: &str) -> String {
    format!("'{}'", value.replace('\'', "''"))
}

fn ps_bool(value: bool) -> &'static str {
    if value { "$true" } else { "$false" }
}

fn get_system_overview_sync() -> Result<SystemOverview, String> {
    let script = r#"
$ErrorActionPreference = 'SilentlyContinue'
$os = Get-CimInstance Win32_OperatingSystem
$cpu = (Get-CimInstance Win32_Processor | Select-Object -First 1).Name
$ram = [math]::Round(([double]$os.TotalVisibleMemorySize / 1MB), 2)
$uptimeSpan = (Get-Date) - $os.LastBootUpTime
$uptime = ('{0}d {1}h {2}m' -f [int]$uptimeSpan.TotalDays, $uptimeSpan.Hours, $uptimeSpan.Minutes)
$disks = @(Get-CimInstance Win32_LogicalDisk -Filter "DriveType=3" | ForEach-Object {
  [PSCustomObject]@{
    Name = $_.DeviceID
    TotalGB = if ($_.Size) { [math]::Round(([double]$_.Size / 1GB), 2) } else { 0 }
    FreeGB = if ($_.FreeSpace) { [math]::Round(([double]$_.FreeSpace / 1GB), 2) } else { 0 }
  }
})
[PSCustomObject]@{
  ComputerName = $env:COMPUTERNAME
  Username = [System.Security.Principal.WindowsIdentity]::GetCurrent().Name
  WindowsCaption = $os.Caption
  Version = $os.Version
  Architecture = $os.OSArchitecture
  Cpu = $cpu
  RamGB = $ram
  Disks = $disks
  Uptime = $uptime
  WingetAvailable = [bool](Get-Command winget -ErrorAction SilentlyContinue)
} | ConvertTo-Json -Depth 5 -Compress
"#;

    let json = powershell(script)?;
    let parsed: PsOverview = serde_json::from_str(&json).map_err(|error| format!("Failed to parse system overview JSON: {error}\n{json}"))?;

    Ok(SystemOverview {
        computer_name: parsed.computer_name.unwrap_or_else(|| "Unknown".into()),
        username: parsed.username.unwrap_or_else(|| "Unknown".into()),
        windows_caption: parsed.windows_caption.unwrap_or_else(|| "Windows".into()),
        version: parsed.version.unwrap_or_else(|| "Unknown".into()),
        architecture: parsed.architecture.unwrap_or_else(|| "Unknown".into()),
        cpu: parsed.cpu.unwrap_or_else(|| "Unknown CPU".into()),
        ram_gb: parsed.ram_gb.unwrap_or(0.0),
        disks: parsed
            .disks
            .unwrap_or_default()
            .into_iter()
            .map(|disk| DiskSummary {
                name: disk.name.unwrap_or_else(|| "Unknown".into()),
                total_gb: disk.total_gb.unwrap_or(0.0),
                free_gb: disk.free_gb.unwrap_or(0.0),
            })
            .collect(),
        uptime: parsed.uptime.unwrap_or_else(|| "Unknown".into()),
        is_admin: is_admin(),
        app_version: APP_VERSION.to_string(),
        winget_available: parsed.winget_available.unwrap_or(false),
    })
}

fn get_disk_health_sync() -> Result<Vec<DiskHealth>, String> {
    let script = r#"
$ErrorActionPreference = 'SilentlyContinue'
$items = @(Get-PhysicalDisk | ForEach-Object {
  $disk = $_
  $rel = $disk | Get-StorageReliabilityCounter
  [PSCustomObject]@{
    FriendlyName = $disk.FriendlyName
    MediaType = [string]$disk.MediaType
    HealthStatus = [string]$disk.HealthStatus
    OperationalStatus = (($disk.OperationalStatus | ForEach-Object { [string]$_ }) -join ', ')
    SizeGB = if ($disk.Size) { [math]::Round($disk.Size / 1GB, 2) } else { 0 }
    Temperature = if ($rel) { $rel.Temperature } else { $null }
    Wear = if ($rel) { $rel.Wear } else { $null }
    ReadErrors = if ($rel) { $rel.ReadErrorsTotal } else { $null }
    WriteErrors = if ($rel) { $rel.WriteErrorsTotal } else { $null }
  }
})
$items | ConvertTo-Json -Depth 5 -Compress
"#;
    let json = powershell(script)?;
    if json.trim().is_empty() {
        return Ok(Vec::new());
    }

    let value: serde_json::Value = serde_json::from_str(&json).map_err(|error| format!("Failed to parse disk JSON: {error}\n{json}"))?;
    let raw_items: Vec<PsDiskHealth> = if value.is_array() {
        serde_json::from_value(value).map_err(|error| error.to_string())?
    } else {
        vec![serde_json::from_value(value).map_err(|error| error.to_string())?]
    };

    Ok(raw_items.into_iter().map(|item| DiskHealth {
        friendly_name: item.friendly_name.unwrap_or_else(|| "Unknown disk".into()),
        media_type: item.media_type.unwrap_or_else(|| "Unknown".into()),
        health_status: item.health_status.unwrap_or_else(|| "Unknown".into()),
        operational_status: item.operational_status.unwrap_or_else(|| "Unknown".into()),
        size_gb: item.size_gb.unwrap_or(0.0),
        temperature: item.temperature,
        wear: item.wear,
        read_errors: item.read_errors,
        write_errors: item.write_errors,
    }).collect())
}

fn restart_explorer_detached() -> Result<(), String> {
    let mut command = Command::new("explorer.exe");
    command.stdin(Stdio::null()).stdout(Stdio::null()).stderr(Stdio::null());
    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        const CREATE_NO_WINDOW: u32 = 0x08000000;
        command.creation_flags(CREATE_NO_WINDOW);
    }
    command.spawn().map_err(|error| format!("Failed to restart Explorer: {error}"))?;
    Ok(())
}

fn set_taskbar_search(show: bool) -> Result<String, String> {
    let value = if show { "1" } else { "0" };
    command_output("reg", &args(&[
        "add",
        r"HKCU\Software\Microsoft\Windows\CurrentVersion\Search",
        "/v",
        "SearchboxTaskbarMode",
        "/t",
        "REG_DWORD",
        "/d",
        value,
        "/f",
    ]))?;
    let _ = restart_explorer_sync();
    Ok(if show { "Taskbar search shown.".into() } else { "Taskbar search hidden.".into() })
}

fn restart_explorer_sync() -> Result<String, String> {
    let _ = command_output("taskkill", &args(&["/F", "/IM", "explorer.exe"]));
    restart_explorer_detached()?;
    Ok("Explorer restart requested.".into())
}

fn toggle_context_menu() -> Result<String, String> {
    let key = r"HKCU\Software\Classes\CLSID\{86ca1aa0-34aa-4e8b-a509-50c905bae2a2}";
    let exists = command_status("reg", &args(&["query", key]));
    if exists {
        command_output("reg", &args(&["delete", key, "/f"]))?;
        let _ = command_output("taskkill", &args(&["/F", "/IM", "explorer.exe"]));
        restart_explorer_detached()?;
        Ok("Switched to Windows 11 context menu. Explorer restart requested.".into())
    } else {
        let subkey = format!(r"{}\InprocServer32", key);
        command_output("reg", &args(&["add", &subkey, "/ve", "/t", "REG_SZ", "/d", "", "/f"]))?;
        let _ = command_output("taskkill", &args(&["/F", "/IM", "explorer.exe"]));
        restart_explorer_detached()?;
        Ok("Switched to classic context menu. Explorer restart requested.".into())
    }
}

fn set_dns(primary: &str, secondary: &str) -> Result<String, String> {
    powershell(&format!(r#"
$ErrorActionPreference = 'Stop'
$adapters = Get-NetAdapter -ErrorAction SilentlyContinue | Where-Object {{ $_.Status -eq 'Up' }} | Select-Object -ExpandProperty Name
if (-not $adapters) {{ 'No active adapters found.'; exit 0 }}
foreach ($adapter in $adapters) {{
  try {{ Set-DnsClientServerAddress -InterfaceAlias $adapter -ServerAddresses ('{}','{}') -ErrorAction Stop }}
  catch {{ netsh interface ip set dns name="$adapter" source=static addr={} register=primary; netsh interface ip add dns name="$adapter" addr={} index=2 }}
}}
ipconfig /flushdns | Out-Null
'DNS set to {}, {} on: ' + ($adapters -join ', ')
"#, primary, secondary, primary, secondary, primary, secondary))
}

fn reset_dns() -> Result<String, String> {
    powershell(r#"
$ErrorActionPreference = 'Continue'
$adapters = Get-NetAdapter -ErrorAction SilentlyContinue | Where-Object { $_.Status -eq 'Up' } | Select-Object -ExpandProperty Name
foreach ($adapter in $adapters) {
  try { Set-DnsClientServerAddress -InterfaceAlias $adapter -ResetServerAddresses -ErrorAction Stop }
  catch { netsh interface ip set dns name="$adapter" source=dhcp }
}
ipconfig /flushdns | Out-Null
'DNS reset to automatic on: ' + ($adapters -join ', ')
"#)
}

fn open_ms_settings(uri: &str) -> Result<String, String> {
    powershell(&format!("Start-Process '{}'; 'Opened {}.'", uri, uri))
}

fn set_run_at_startup(enable: bool) -> Result<String, String> {
    let exe = std::env::current_exe().map_err(|e| format!("Could not locate current exe: {e}"))?;
    let exe_str = exe.to_string_lossy().replace('`', "``").replace('"', "`\"");
    if enable {
        powershell(&format!(r#"
$ErrorActionPreference = 'Stop'
$run = 'HKCU:\Software\Microsoft\Windows\CurrentVersion\Run'
Ensure-Key $run
Set-ItemProperty -Path $run -Name 'Eoliann Windows Tools 2' -Value '"{}"'
# Value name used before v1.1.1 (shared with Eoliann Windows Tools v1).
Remove-ItemProperty -Path $run -Name 'Eoliann Windows Tools' -ErrorAction SilentlyContinue
'Start with Windows enabled.'
"#, exe_str))
    } else {
        powershell(r#"
$run = 'HKCU:\Software\Microsoft\Windows\CurrentVersion\Run'
Remove-ItemProperty -Path $run -Name 'Eoliann Windows Tools 2' -ErrorAction SilentlyContinue
Remove-ItemProperty -Path $run -Name 'Eoliann Windows Tools' -ErrorAction SilentlyContinue
'Start with Windows disabled.'
"#)
    }
}

fn set_mouse_acceleration(enable: bool) -> Result<String, String> {
    let (speed, t1, t2) = if enable { ("1", "6", "10") } else { ("0", "0", "0") };
    powershell(&format!(r#"
$ErrorActionPreference = 'Stop'
$path = 'HKCU:\Control Panel\Mouse'
Set-ItemProperty -Path $path -Name MouseSpeed -Value '{}'
Set-ItemProperty -Path $path -Name MouseThreshold1 -Value '{}'
Set-ItemProperty -Path $path -Name MouseThreshold2 -Value '{}'
'Mouse acceleration {}.'
"#, speed, t1, t2, if enable { "enabled" } else { "disabled" }))
}

fn set_numlock(enable: bool) -> Result<String, String> {
    let value = if enable { "2" } else { "0" };
    powershell(&format!(r#"
$ErrorActionPreference = 'Continue'
Set-ItemProperty -Path 'HKCU:\Control Panel\Keyboard' -Name InitialKeyboardIndicators -Value '{}'
Set-ItemProperty -Path 'Registry::HKEY_USERS\.DEFAULT\Control Panel\Keyboard' -Name InitialKeyboardIndicators -Value '{}'
'NumLock on startup {}.'
"#, value, value, if enable { "enabled" } else { "disabled" }))
}

fn battery_report_dialog() -> Result<String, String> {
    powershell_sta(r#"
Add-Type -AssemblyName System.Windows.Forms
$dialog = New-Object System.Windows.Forms.SaveFileDialog
$dialog.Title = 'Save Eoliann Windows Tools battery report'
$dialog.InitialDirectory = [Environment]::GetFolderPath('MyDocuments')
$dialog.FileName = 'ewt-battery-report.html'
$dialog.Filter = 'HTML files (*.html)|*.html|All files (*.*)|*.*'
$result = $dialog.ShowDialog()
if ($result -ne [System.Windows.Forms.DialogResult]::OK) { 'Battery report cancelled by user.'; exit 0 }
$path = $dialog.FileName
powercfg /batteryreport /output $path | Out-String
if (Test-Path $path) { "Battery report saved to: $path" } else { throw "Battery report was not created at: $path" }
"#)
}

fn installed_apps_report_dialog() -> Result<String, String> {
    powershell_sta(r#"
Add-Type -AssemblyName System.Windows.Forms
$dialog = New-Object System.Windows.Forms.SaveFileDialog
$dialog.Title = 'Save installed apps report'
$dialog.InitialDirectory = [Environment]::GetFolderPath('MyDocuments')
$dialog.FileName = 'ewt-installed-apps-report.html'
$dialog.Filter = 'HTML files (*.html)|*.html|All files (*.*)|*.*'
$result = $dialog.ShowDialog()
if ($result -ne [System.Windows.Forms.DialogResult]::OK) { 'Installed apps report cancelled by user.'; exit 0 }
$path = $dialog.FileName
$lines = winget list 2>&1
if (-not $lines -or $lines.Count -eq 0) { throw 'winget did not return any output.' }

$headerIndex = -1
for ($i = 0; $i -lt $lines.Count; $i++) {
  if ($lines[$i] -match '^\s*Name\s{2,}Id\s{2,}Version\s{2,}Available\s{2,}Source\s*$') {
    $headerIndex = $i
    break
  }
}

$rows = New-Object System.Collections.Generic.List[object]
if ($headerIndex -ge 0) {
  for ($i = $headerIndex + 2; $i -lt $lines.Count; $i++) {
    $line = [string]$lines[$i]
    if ([string]::IsNullOrWhiteSpace($line)) { continue }
    if ($line -match '^\s*-{3,}') { continue }
    $parts = $line.TrimEnd() -split '\s{2,}'
    if ($parts.Count -lt 2) { continue }

    $name = if ($parts.Count -ge 1) { $parts[0] } else { '' }
    $id = if ($parts.Count -ge 2) { $parts[1] } else { '' }
    $version = if ($parts.Count -ge 3) { $parts[2] } else { '' }
    $available = if ($parts.Count -ge 4) { $parts[3] } else { '' }
    $source = if ($parts.Count -ge 5) { $parts[4] } else { '' }

    $rows.Add([PSCustomObject]@{
      Name = $name
      Id = $id
      Version = $version
      Available = $available
      Source = $source
    }) | Out-Null
  }
}

$tableRows = if ($rows.Count -gt 0) {
  ($rows | ForEach-Object {
    '<tr><td>' + [System.Net.WebUtility]::HtmlEncode($_.Name) + '</td><td>' + [System.Net.WebUtility]::HtmlEncode($_.Id) + '</td><td>' + [System.Net.WebUtility]::HtmlEncode($_.Version) + '</td><td>' + [System.Net.WebUtility]::HtmlEncode($_.Available) + '</td><td>' + [System.Net.WebUtility]::HtmlEncode($_.Source) + '</td></tr>'
  }) -join "`n"
} else {
  ''
}

$rawText = [System.Net.WebUtility]::HtmlEncode(($lines -join "`r`n"))
$generated = Get-Date -Format 'yyyy-MM-dd HH:mm:ss'
$html = @"
<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8" />
  <meta name="viewport" content="width=device-width,initial-scale=1" />
  <title>EWT Installed Apps Report</title>
  <style>
    body { font-family: Segoe UI, Arial, sans-serif; background: #f7f8fa; color: #111; margin: 0; padding: 24px; }
    .card { max-width: 1100px; margin: 0 auto; background: #fff; border: 1px solid #e4e7ec; border-radius: 10px; padding: 20px; }
    h1 { margin: 0 0 8px; font-size: 22px; }
    p { margin: 0 0 16px; color: #4a5565; }
    table { width: 100%; border-collapse: collapse; margin: 8px 0 20px; }
    th, td { border: 1px solid #d9dee6; padding: 8px 10px; text-align: left; font-size: 13px; }
    th { background: #eef2f7; }
    pre { margin: 0; white-space: pre-wrap; word-wrap: break-word; background: #0f172a; color: #e2e8f0; padding: 14px; border-radius: 8px; overflow: auto; }
  </style>
</head>
<body>
  <div class="card">
    <h1>Installed Apps Report</h1>
    <p>Generated by Eoliann Windows Tools on $generated</p>
    <p>Total detected entries: $($rows.Count)</p>
    <table>
      <thead>
        <tr><th>Name</th><th>Id</th><th>Version</th><th>Available</th><th>Source</th></tr>
      </thead>
      <tbody>
$tableRows
      </tbody>
    </table>
    <h2>Raw winget output</h2>
    <pre>$rawText</pre>
  </div>
</body>
</html>
"@
Set-Content -Path $path -Value $html -Encoding UTF8
if (Test-Path $path) { "Installed apps report saved to: $path" } else { throw "Report was not created at: $path" }
"#)
}

fn open_restore_cleanup_ui() -> Result<String, String> {
    powershell(r#"
Start-Process SystemPropertiesProtection.exe
'Opened System Protection. Use Configure/Delete to remove restore points with the native Windows UI.'
"#)
}

fn create_restore_point_sync() -> Result<String, String> {
    // Windows silently skips Checkpoint-Computer when another restore point was created in the
    // last 24 hours, so the frequency limit is lifted temporarily and the result is verified.
    powershell(r#"
$ErrorActionPreference = 'Stop'
$key = 'HKLM:\SOFTWARE\Microsoft\Windows NT\CurrentVersion\SystemRestore'
Enable-ComputerRestore -Drive "$env:SystemDrive\" -ErrorAction SilentlyContinue
$before = [int64](Get-ComputerRestorePoint -ErrorAction SilentlyContinue | Measure-Object -Property SequenceNumber -Maximum).Maximum
$previous = (Get-ItemProperty -Path $key -Name SystemRestorePointCreationFrequency -ErrorAction SilentlyContinue).SystemRestorePointCreationFrequency
Set-ItemProperty -Path $key -Name SystemRestorePointCreationFrequency -Value 0 -Type DWord
try {
  $job = Start-Job -ScriptBlock {
    Checkpoint-Computer -Description 'Eoliann Windows Tools' -RestorePointType 'MODIFY_SETTINGS' -ErrorAction Stop -WarningAction SilentlyContinue
  }
  if (-not (Wait-Job $job -Timeout 180)) {
    Stop-Job $job -ErrorAction SilentlyContinue
    Remove-Job $job -Force -ErrorAction SilentlyContinue
    throw 'Create restore point timed out after 180 seconds.'
  }
  Receive-Job $job -ErrorAction Stop | Out-Null
  Remove-Job $job -Force -ErrorAction SilentlyContinue
} finally {
  if ($null -eq $previous) { Remove-ItemProperty -Path $key -Name SystemRestorePointCreationFrequency -ErrorAction SilentlyContinue }
  else { Set-ItemProperty -Path $key -Name SystemRestorePointCreationFrequency -Value $previous -Type DWord }
}
$after = [int64](Get-ComputerRestorePoint -ErrorAction SilentlyContinue | Measure-Object -Property SequenceNumber -Maximum).Maximum
if ($after -gt $before) { "Restore point created successfully (#$after)." }
else { throw 'Windows did not create a new restore point. Make sure System Protection is turned on for the system drive.' }
"#)
}

#[cfg(windows)]
mod system_restore {
    use std::ffi::c_void;

    extern "system" {
        fn LoadLibraryW(name: *const u16) -> *mut c_void;
        fn GetProcAddress(module: *mut c_void, name: *const u8) -> *mut c_void;
        fn FreeLibrary(module: *mut c_void) -> i32;
    }

    type SrRemoveRestorePoint = unsafe extern "system" fn(u32) -> u32;

    /// Deletes one restore point through the documented System Restore API (srclient.dll), the
    /// same path used by the Windows System Protection dialog. Unlike `vssadmin delete shadows`,
    /// this is not treated as ransomware-style shadow copy wiping by antivirus products.
    pub fn remove_restore_point(sequence_number: u32) -> Result<(), String> {
        let library: Vec<u16> = "srclient.dll".encode_utf16().chain(Some(0)).collect();
        unsafe {
            let module = LoadLibraryW(library.as_ptr());
            if module.is_null() {
                return Err("System Restore client (srclient.dll) is not available on this system.".into());
            }
            let proc = GetProcAddress(module, b"SRRemoveRestorePoint\0".as_ptr());
            if proc.is_null() {
                FreeLibrary(module);
                return Err("SRRemoveRestorePoint was not found in srclient.dll.".into());
            }
            let remove: SrRemoveRestorePoint = std::mem::transmute(proc);
            let code = remove(sequence_number);
            FreeLibrary(module);
            if code == 0 {
                Ok(())
            } else {
                Err(format!("Windows could not delete restore point #{sequence_number} (error {code})."))
            }
        }
    }
}

#[cfg(not(windows))]
mod system_restore {
    pub fn remove_restore_point(_sequence_number: u32) -> Result<(), String> {
        Err("System Restore is only available on Windows.".into())
    }
}

fn parse_used_shadow_storage_gb(raw: &str) -> Option<f64> {
    for line in raw.lines() {
        let trimmed = line.trim();
        if !trimmed.to_ascii_lowercase().contains("used shadow copy storage space") {
            continue;
        }

        let value_part = trimmed
            .split_once(':')
            .map(|(_, right)| right.trim())
            .and_then(|right| right.split('(').next())
            .map(str::trim)?;

        let mut parts = value_part.split_whitespace();
        let number = parts.next()?.replace(',', ".").parse::<f64>().ok()?;
        let unit = parts.next().unwrap_or("B").to_ascii_uppercase();

        let gb = match unit.as_str() {
            "TB" => number * 1024.0,
            "GB" => number,
            "MB" => number / 1024.0,
            "KB" => number / (1024.0 * 1024.0),
            _ => number / (1024.0 * 1024.0 * 1024.0),
        };
        return Some((gb * 100.0).round() / 100.0);
    }
    None
}

fn list_restore_points() -> Result<Vec<RestorePointInfo>, String> {
    let json = powershell(r#"
$ErrorActionPreference = 'SilentlyContinue'
$points = @(Get-ComputerRestorePoint | Sort-Object SequenceNumber -Descending | ForEach-Object {
  $point = $_
  $created = [string]$point.CreationTime
  try { $created = [Management.ManagementDateTimeConverter]::ToDateTime($point.CreationTime).ToString('yyyy-MM-dd HH:mm:ss') } catch { }
  [PSCustomObject]@{
    ShadowId = ''
    SequenceNumber = [int64]$point.SequenceNumber
    Description = [string]$point.Description
    CreationTime = $created
  }
})
ConvertTo-Json -InputObject $points -Depth 4 -Compress
"#)?;
    if json.trim().is_empty() {
        return Ok(Vec::new());
    }
    let raw: Vec<PsRestorePointInfo> = serde_json::from_str(json.trim())
        .map_err(|error| format!("Failed to parse restore points JSON: {error}\n{json}"))?;
    Ok(raw
        .into_iter()
        .filter_map(|point| {
            let sequence_number = point.sequence_number.filter(|value| *value > 0)?;
            Some(RestorePointInfo {
                shadow_id: point.shadow_id.unwrap_or_default(),
                sequence_number,
                description: point.description.filter(|text| !text.trim().is_empty()).unwrap_or_else(|| "System restore point".into()),
                creation_time: point.creation_time.unwrap_or_else(|| "Unknown".into()),
            })
        })
        .collect())
}

fn get_restore_points_sync() -> Result<RestorePointsSummary, String> {
    if !is_admin() {
        return Err("Administrator privileges are required to read restore points. Please run Eoliann Windows Tools as Administrator.".into());
    }

    let points = list_restore_points()?;
    let used_from_vssadmin = command_output("vssadmin", &args(&["list", "shadowstorage"]))
        .ok()
        .and_then(|raw| parse_used_shadow_storage_gb(&raw));
    let used_from_wmi = powershell("[math]::Round(((Get-CimInstance Win32_ShadowStorage -ErrorAction SilentlyContinue | Measure-Object -Property UsedSpace -Sum).Sum / 1GB), 2)")
        .ok()
        .and_then(|raw| raw.trim().replace(',', ".").parse::<f64>().ok());

    Ok(RestorePointsSummary {
        total_used_space_gb: used_from_vssadmin.or(used_from_wmi).unwrap_or(0.0),
        points,
    })
}

fn delete_restore_point_by_info_sync(args: DeleteRestorePointArgs) -> Result<String, String> {
    if !is_admin() {
        return Err("Administrator privileges are required to delete restore points. Please run Eoliann Windows Tools as Administrator.".into());
    }

    let sequence_number = args
        .sequence_number
        .filter(|value| *value > 0 && *value <= u32::MAX as i64)
        .ok_or_else(|| "Invalid restore point. Press Refresh and select a restore point again.".to_string())?;
    system_restore::remove_restore_point(sequence_number as u32)?;
    Ok(format!("Deleted restore point #{sequence_number}."))
}

fn delete_all_restore_points_sync() -> Result<String, String> {
    if !is_admin() {
        return Err("Administrator privileges are required to delete restore points. Please run Eoliann Windows Tools as Administrator.".into());
    }

    let points = list_restore_points()?;
    if points.is_empty() {
        return Ok("No restore points found.".into());
    }

    let mut lines = Vec::new();
    let mut failed = 0;
    for point in &points {
        match system_restore::remove_restore_point(point.sequence_number as u32) {
            Ok(()) => lines.push(format!("Deleted #{} - {} ({})", point.sequence_number, point.description, point.creation_time)),
            Err(error) => {
                failed += 1;
                lines.push(format!("FAILED #{}: {error}", point.sequence_number));
            }
        }
    }
    let summary = format!("Removed {} of {} restore points.", points.len() - failed, points.len());
    if failed == points.len() {
        Err(format!("{summary}\n{}", lines.join("\n")))
    } else {
        Ok(format!("{summary}\n{}", lines.join("\n")))
    }
}


fn run_action_sync(action_id: String) -> Result<String, String> {
    match action_id.as_str() {
        // INFO
        "whoami" => cmd_shell(r"whoami /all"),
        "ipconfig" => cmd_shell(r"ipconfig /all"),
        "systeminfo_cli" => cmd_shell(r"systeminfo"),
        "tasklist" => cmd_shell(r"tasklist"),
        "services_running" => powershell(r#"Get-Service | Where-Object Status -eq 'Running' | Sort-Object DisplayName | Format-Table -AutoSize Name,DisplayName,Status | Out-String"#),
        "installed_apps_report" => installed_apps_report_dialog(),

        // MAINTENANCE / HEALTH
        "toggle_context_menu" => toggle_context_menu(),
        "restart_explorer" => restart_explorer_sync(),
        "empty_recycle_bin" => powershell("Clear-RecycleBin -Force -ErrorAction SilentlyContinue; 'Recycle Bin emptied.'"),
        "clean_temp_files" => powershell(r#"
Remove-Item "$env:TEMP\*" -Recurse -Force -ErrorAction SilentlyContinue
Remove-Item "$env:WINDIR\Temp\*" -Recurse -Force -ErrorAction SilentlyContinue
'Temporary files cleanup completed.'
"#),
        "clean_prefetch" => powershell(r#"Remove-Item "$env:WINDIR\Prefetch\*" -Recurse -Force -ErrorAction SilentlyContinue; 'Prefetch files cleaned.'"#),
        "disk_cleanup_c" => powershell(r#"Start-Process cleanmgr.exe -ArgumentList '/d C:'; 'Disk Cleanup started for C:.'"#),
        "disk_cleanup_all" => powershell(r#"
$drives = Get-CimInstance Win32_LogicalDisk -Filter "DriveType=3" | Select-Object -ExpandProperty DeviceID
foreach ($drive in $drives) { Start-Process cleanmgr.exe -ArgumentList ('/d ' + $drive.TrimEnd(':')) }
'Disk Cleanup started for fixed drives: ' + ($drives -join ', ')
"#),
        "storage_settings" => open_ms_settings("ms-settings:storagesense"),
        "system_restore_cleanup" => open_restore_cleanup_ui(),
        "sfc_dism" => cmd_shell(r"DISM /Online /Cleanup-Image /CheckHealth & DISM /Online /Cleanup-Image /ScanHealth & DISM /Online /Cleanup-Image /RestoreHealth & sfc /scannow"),
        "network_reset" => cmd_shell(r"netsh winsock reset & netsh int ip reset & ipconfig /flushdns"),
        "create_restore_point" => create_restore_point_sync(),
        "battery_report" => battery_report_dialog(),
        "memory_diagnostic" => powershell(r#"Start-Process mdsched.exe; 'Windows Memory Diagnostic launched.'"#),
        "hibernate_on" => cmd_shell(r"powercfg /hibernate on"),
        "hibernate_off" => cmd_shell(r"powercfg /hibernate off"),
        "hibernation_laptop_defaults" => cmd_shell(r"powercfg /hibernate on & powercfg /change hibernate-timeout-ac 30 & powercfg /change hibernate-timeout-dc 15 & powercfg /setacvalueindex SCHEME_CURRENT SUB_BUTTONS LIDACTION 2 & powercfg /setdcvalueindex SCHEME_CURRENT SUB_BUTTONS LIDACTION 2 & powercfg /setactive SCHEME_CURRENT"),
        "restore_hibernation_defaults" => cmd_shell(r"powercfg /setacvalueindex SCHEME_CURRENT SUB_BUTTONS LIDACTION 1 & powercfg /setdcvalueindex SCHEME_CURRENT SUB_BUTTONS LIDACTION 1 & powercfg /hibernate off & powercfg /setactive SCHEME_CURRENT"),
        "disable_sleep" => cmd_shell(r"powercfg /change standby-timeout-ac 0 & powercfg /change standby-timeout-dc 0 & powercfg /change hibernate-timeout-ac 0 & powercfg /change hibernate-timeout-dc 0"),
        "disable_hdd_timeout" => cmd_shell(r"powercfg /change disk-timeout-ac 0 & powercfg /change disk-timeout-dc 0"),
        "disable_monitor_timeout" => cmd_shell(r"powercfg /change monitor-timeout-ac 0 & powercfg /change monitor-timeout-dc 0"),
        "delete_restore_points" => delete_all_restore_points_sync(),

        // Essential / advanced tweaks
        "disable_consumer_features" => cmd_shell(r#"reg add "HKCU\Software\Microsoft\Windows\CurrentVersion\ContentDeliveryManager" /v SilentInstalledAppsEnabled /t REG_DWORD /d 0 /f & reg add "HKLM\SOFTWARE\Policies\Microsoft\Windows\CloudContent" /v DisableWindowsConsumerFeatures /t REG_DWORD /d 1 /f"#),
        "enable_consumer_features" => cmd_shell(r#"reg add "HKCU\Software\Microsoft\Windows\CurrentVersion\ContentDeliveryManager" /v SilentInstalledAppsEnabled /t REG_DWORD /d 1 /f & reg delete "HKLM\SOFTWARE\Policies\Microsoft\Windows\CloudContent" /v DisableWindowsConsumerFeatures /f"#),
        "disable_telemetry" => powershell(r#"
$tasks = @(
'Microsoft\Windows\Application Experience\Microsoft Compatibility Appraiser',
'Microsoft\Windows\Application Experience\ProgramDataUpdater',
'Microsoft\Windows\Customer Experience Improvement Program\Consolidator',
'Microsoft\Windows\Customer Experience Improvement Program\UsbCeip',
'Microsoft\Windows\Feedback\Siuf\DmClient',
'Microsoft\Windows\Feedback\Siuf\DmClientOnScenarioDownload',
'Microsoft\Windows\Windows Error Reporting\QueueReporting')
foreach ($task in $tasks) { schtasks /Change /TN $task /Disable 2>&1 | Out-Null }
Ensure-Key 'HKLM:\SOFTWARE\Policies\Microsoft\Windows\DataCollection'
Set-ItemProperty 'HKLM:\SOFTWARE\Policies\Microsoft\Windows\DataCollection' AllowTelemetry 0 -Type DWord
Ensure-Key 'HKLM:\SOFTWARE\Policies\Microsoft\Windows\AdvertisingInfo'
Set-ItemProperty 'HKLM:\SOFTWARE\Policies\Microsoft\Windows\AdvertisingInfo' DisabledByGroupPolicy 1 -Type DWord
Set-ItemProperty 'HKLM:\SOFTWARE\Microsoft\Windows\Windows Error Reporting' Disabled 1 -Type DWord -ErrorAction SilentlyContinue
sc.exe stop DiagTrack 2>&1 | Out-Null
sc.exe config DiagTrack start= disabled 2>&1 | Out-Null
'Telemetry reduction applied.'
"#),
        "enable_telemetry" => cmd_shell(r#"reg add "HKLM\SOFTWARE\Policies\Microsoft\Windows\DataCollection" /v AllowTelemetry /t REG_DWORD /d 1 /f & sc config DiagTrack start= auto & sc start DiagTrack"#),
        "disable_location" => powershell(r#"
Ensure-Key 'HKLM:\SOFTWARE\Policies\Microsoft\Windows\LocationAndSensors'
Set-ItemProperty 'HKLM:\SOFTWARE\Policies\Microsoft\Windows\LocationAndSensors' DisableLocation 1 -Type DWord
Set-ItemProperty 'HKLM:\SOFTWARE\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\location' Value 'Deny' -ErrorAction SilentlyContinue
'Location tracking policy applied.'
"#),
        "disable_wifi_sense" => powershell(r#"
Ensure-Key 'HKLM:\Software\Microsoft\PolicyManager\default\WiFi\AllowWiFiHotSpotReporting'
Ensure-Key 'HKLM:\Software\Microsoft\PolicyManager\default\WiFi\AllowAutoConnectToWiFiSenseHotspots'
Set-ItemProperty 'HKLM:\Software\Microsoft\PolicyManager\default\WiFi\AllowWiFiHotSpotReporting' Value 0 -Type DWord
Set-ItemProperty 'HKLM:\Software\Microsoft\PolicyManager\default\WiFi\AllowAutoConnectToWiFiSenseHotspots' Value 0 -Type DWord
'Wi-Fi Sense disabled.'
"#),
        "enable_end_task" => cmd_shell(r#"reg add "HKCU\Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced\TaskbarDeveloperSettings" /v TaskbarEndTask /t REG_DWORD /d 1 /f"#),
        "disable_end_task" => cmd_shell(r#"reg add "HKCU\Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced\TaskbarDeveloperSettings" /v TaskbarEndTask /t REG_DWORD /d 0 /f"#),
        "disable_recall" => cmd_shell(r#"reg add "HKLM\SOFTWARE\Policies\Microsoft\Windows\WindowsAI" /v AllowRecallEnablement /t REG_DWORD /d 0 /f & reg add "HKLM\SOFTWARE\Policies\Microsoft\Windows\WindowsAI" /v DisableAIDataAnalysis /t REG_DWORD /d 1 /f & DISM /Online /Disable-Feature /FeatureName:Recall /Quiet /NoRestart"#),
        "enable_recall" => cmd_shell(r#"reg delete "HKLM\SOFTWARE\Policies\Microsoft\Windows\WindowsAI" /v AllowRecallEnablement /f & reg delete "HKLM\SOFTWARE\Policies\Microsoft\Windows\WindowsAI" /v DisableAIDataAnalysis /f & DISM /Online /Enable-Feature /FeatureName:Recall /Quiet /NoRestart"#),
        "disable_copilot_legacy" => cmd_shell(r#"reg add "HKCU\Software\Policies\Microsoft\Windows\WindowsCopilot" /v TurnOffWindowsCopilot /t REG_DWORD /d 1 /f & reg add "HKLM\SOFTWARE\Policies\Microsoft\Windows\WindowsCopilot" /v TurnOffWindowsCopilot /t REG_DWORD /d 1 /f"#),
        "enable_copilot_legacy" => cmd_shell(r#"reg delete "HKCU\Software\Policies\Microsoft\Windows\WindowsCopilot" /v TurnOffWindowsCopilot /f & reg delete "HKLM\SOFTWARE\Policies\Microsoft\Windows\WindowsCopilot" /v TurnOffWindowsCopilot /f"#),
        "disable_activity_history" => cmd_shell(r#"reg add "HKLM\SOFTWARE\Policies\Microsoft\Windows\System" /v PublishUserActivities /t REG_DWORD /d 0 /f & reg add "HKLM\SOFTWARE\Policies\Microsoft\Windows\System" /v UploadUserActivities /t REG_DWORD /d 0 /f & del /f /q "%APPDATA%\Microsoft\Windows\Recent\*""#),
        "enable_activity_history" => cmd_shell(r#"reg add "HKLM\SOFTWARE\Policies\Microsoft\Windows\System" /v PublishUserActivities /t REG_DWORD /d 1 /f & reg add "HKLM\SOFTWARE\Policies\Microsoft\Windows\System" /v UploadUserActivities /t REG_DWORD /d 1 /f"#),
        "disable_storage_sense" => cmd_shell(r#"reg add "HKCU\Software\Microsoft\Windows\CurrentVersion\StorageSense\Parameters\StoragePolicy" /v 01 /t REG_DWORD /d 0 /f"#),
        "enable_storage_sense" => cmd_shell(r#"reg add "HKCU\Software\Microsoft\Windows\CurrentVersion\StorageSense\Parameters\StoragePolicy" /v 01 /t REG_DWORD /d 1 /f"#),
        "show_hidden_files" => cmd_shell(r#"reg add "HKCU\Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced" /v Hidden /t REG_DWORD /d 1 /f & taskkill /f /im explorer.exe & start explorer.exe"#),
        "hide_hidden_files" => cmd_shell(r#"reg add "HKCU\Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced" /v Hidden /t REG_DWORD /d 2 /f & taskkill /f /im explorer.exe & start explorer.exe"#),
        "show_file_extensions" => cmd_shell(r#"reg add "HKCU\Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced" /v HideFileExt /t REG_DWORD /d 0 /f & taskkill /f /im explorer.exe & start explorer.exe"#),
        "hide_file_extensions" => cmd_shell(r#"reg add "HKCU\Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced" /v HideFileExt /t REG_DWORD /d 1 /f & taskkill /f /im explorer.exe & start explorer.exe"#),
        "debloat_edge" => powershell(r#"
Ensure-Key 'HKLM:\SOFTWARE\Policies\Microsoft\EdgeUpdate'
Set-ItemProperty 'HKLM:\SOFTWARE\Policies\Microsoft\EdgeUpdate' CreateDesktopShortcutDefault 0 -Type DWord
Ensure-Key 'HKLM:\SOFTWARE\Policies\Microsoft\Edge'
$settings = @{ EdgeEnhanceImagesEnabled = 0; PersonalizationReportingEnabled = 0; ShowRecommendationsEnabled = 0; HideFirstRunExperience = 1; UserFeedbackAllowed = 0; ConfigureDoNotTrack = 1; AlternateErrorPagesEnabled = 0; EdgeCollectionsEnabled = 0; EdgeFollowEnabled = 0; EdgeShoppingAssistantEnabled = 0; MicrosoftEdgeInsiderPromotionEnabled = 0; ShowMicrosoftRewards = 0; WebWidgetAllowed = 0; DiagnosticData = 0; CryptoWalletEnabled = 0; WalletDonationEnabled = 0 }
foreach ($key in $settings.Keys) { Set-ItemProperty 'HKLM:\SOFTWARE\Policies\Microsoft\Edge' -Name $key -Value $settings[$key] -Type DWord }
'Edge debloat policies applied.'
"#),
        "adobe_network_block" => powershell(r#"
$hosts = "$env:SystemRoot\System32\drivers\etc\hosts"
$block = @'
#EWT-AdobeNetBlock-start
127.0.0.1 activate.adobe.com
127.0.0.1 practivate.adobe.com
127.0.0.1 lm.licenses.adobe.com
127.0.0.1 lmlicenses.wip4.adobe.com
127.0.0.1 na1r.services.adobe.com
127.0.0.1 hlrcv.stage.adobe.com
127.0.0.1 genuine.adobe.com
127.0.0.1 cc-api-data.adobe.io
#EWT-AdobeNetBlock-end
'@
$content = Get-Content $hosts -Raw -ErrorAction SilentlyContinue
if ($content -notmatch '#EWT-AdobeNetBlock-start') { Add-Content -Path $hosts -Value $block; 'Adobe network block entries added.' } else { 'Adobe network block entries already present.' }
"#),
        "adobe_network_unblock" => powershell(r#"
$hosts = "$env:SystemRoot\System32\drivers\etc\hosts"
$content = Get-Content $hosts -Raw -ErrorAction SilentlyContinue
$content = [regex]::Replace($content, '(?s)\r?\n?#EWT-AdobeNetBlock-start.*?#EWT-AdobeNetBlock-end\r?\n?', "`r`n")
Set-Content -Path $hosts -Value $content -Encoding ASCII
'Adobe network block entries removed.'
"#),
        "adobe_debloat" => powershell(r#"
$services = 'AdobeARMservice','AGMService','AGSService','AdobeUpdateService','Adobe Acrobat Update Service'
foreach ($svc in $services) { Stop-Service $svc -Force -ErrorAction SilentlyContinue; Set-Service $svc -StartupType Disabled -ErrorAction SilentlyContinue }
$tasks = Get-ScheduledTask -ErrorAction SilentlyContinue | Where-Object { $_.TaskName -match 'Adobe|Acrobat' }
foreach ($task in $tasks) { Disable-ScheduledTask -TaskName $task.TaskName -TaskPath $task.TaskPath -ErrorAction SilentlyContinue | Out-Null }
'Adobe updater/helper services and tasks disabled where present.'
"#),
        "remove_onedrive" => powershell(r#"
Stop-Process -Name OneDrive -Force -ErrorAction SilentlyContinue
$paths = @("$env:SystemRoot\SysWOW64\OneDriveSetup.exe", "$env:SystemRoot\System32\OneDriveSetup.exe")
foreach ($p in $paths) { if (Test-Path $p) { Start-Process $p -ArgumentList '/uninstall' -Wait -ErrorAction SilentlyContinue } }
winget uninstall --id Microsoft.OneDrive -e --silent 2>$null
'OneDrive uninstall attempted. Verify files and sync folders manually.'
"#),
        "install_onedrive" => command_output("winget", &args(&["install", "--id", "Microsoft.OneDrive", "-e", "--accept-package-agreements", "--accept-source-agreements"])),
        "run_ooshutup10" => powershell(r#"
$url = 'https://dl5.oo-software.com/files/ooshutup10/OOSU10.exe'
$out = Join-Path $env:TEMP 'OOSU10.exe'
$ProgressPreference = 'SilentlyContinue'
[Net.ServicePointManager]::SecurityProtocol = [Net.ServicePointManager]::SecurityProtocol -bor [Net.SecurityProtocolType]::Tls12
Invoke-WebRequest -Uri $url -OutFile $out -UseBasicParsing -ErrorAction Stop
$signature = Get-AuthenticodeSignature -FilePath $out
if ($signature.Status -ne 'Valid' -or $signature.SignerCertificate.Subject -notmatch 'O&O Software') { Remove-Item $out -Force -ErrorAction SilentlyContinue; throw "Downloaded file is not a valid O&O signed executable ($($signature.Status))." }
Start-Process $out
'O&O ShutUp10++ downloaded from O&O, signature verified and launched.'
"#),
        "time_utc" => cmd_shell(r#"reg add "HKLM\SYSTEM\CurrentControlSet\Control\TimeZoneInformation" /v RealTimeIsUniversal /t REG_DWORD /d 1 /f"#),
        "time_local" => cmd_shell(r#"reg delete "HKLM\SYSTEM\CurrentControlSet\Control\TimeZoneInformation" /v RealTimeIsUniversal /f"#),
        "gpedit_home" => cmd_shell(r#"for %F in ("%SystemRoot%\servicing\Packages\Microsoft-Windows-GroupPolicy-ClientTools-Package~*.mum") do dism /online /norestart /add-package:"%F" & for %F in ("%SystemRoot%\servicing\Packages\Microsoft-Windows-GroupPolicy-ClientExtensions-Package~*.mum") do dism /online /norestart /add-package:"%F""#),
        "password_expire_toggle" => powershell(r#"
$txt = net accounts | Out-String
if ($txt -match 'Maximum password age \(days\):\s+Unlimited') { net accounts /maxpwage:90; 'Password expiration enabled: max age 90 days.' }
else { net accounts /maxpwage:unlimited; 'Password expiration disabled: max age unlimited.' }
"#),
        "hyperv_disable" => cmd_shell(r"bcdedit /set hypervisorlaunchtype off & dism /Online /Disable-Feature:Microsoft-Hyper-V-All /NoRestart & dism /Online /Disable-Feature:VirtualMachinePlatform /NoRestart & dism /Online /Disable-Feature:Windows-Hypervisor-Platform /NoRestart"),

        // NETWORK / DNS
        "flush_dns" => cmd_shell(r"ipconfig /flushdns"),
        "set_dns_cloudflare" => set_dns("1.1.1.1", "1.0.0.1"),
        "set_dns_cloudflare_malware" => set_dns("1.1.1.2", "1.0.0.2"),
        "set_dns_cloudflare_family" => set_dns("1.1.1.3", "1.0.0.3"),
        "set_dns_google" => set_dns("8.8.8.8", "8.8.4.4"),
        "set_dns_opendns" => set_dns("208.67.222.222", "208.67.220.220"),
        "set_dns_quad9" => set_dns("9.9.9.9", "149.112.112.112"),
        "set_dns_adguard" => set_dns("94.140.14.14", "94.140.15.15"),
        "set_dns_adguard_family" => set_dns("94.140.14.15", "94.140.15.16"),
        "set_dns_dns0_open" => set_dns("193.110.81.254", "185.253.5.254"),
        "set_dns_dns0_zero" => set_dns("193.110.81.9", "185.253.5.9"),
        "set_dns_dns0_kids" => set_dns("193.110.81.1", "185.253.5.1"),
        "reset_dns" => reset_dns(),
        "enable_insecure_guest_smb" => powershell(r#"
$ErrorActionPreference = 'Continue'
$log = New-Object System.Collections.Generic.List[string]

Ensure-Key 'HKLM:\SYSTEM\CurrentControlSet\Services\LanmanWorkstation\Parameters'
Set-ItemProperty 'HKLM:\SYSTEM\CurrentControlSet\Services\LanmanWorkstation\Parameters' -Name RequireSecuritySignature -Type DWord -Value 0
$log.Add('RequireSecuritySignature = 0') | Out-Null

Ensure-Key 'HKLM:\SOFTWARE\Policies\Microsoft\Windows\LanmanWorkstation'
Set-ItemProperty 'HKLM:\SOFTWARE\Policies\Microsoft\Windows\LanmanWorkstation' -Name AllowInsecureGuestAuth -Type DWord -Value 1
$log.Add('AllowInsecureGuestAuth (Policies) = 1') | Out-Null

Set-ItemProperty 'HKLM:\SYSTEM\CurrentControlSet\Services\LanmanWorkstation\Parameters' -Name AllowInsecureGuestAuth -Type DWord -Value 1
$log.Add('AllowInsecureGuestAuth (legacy) = 1') | Out-Null

net stop lanmanworkstation /y | Out-Null
$restart = net start lanmanworkstation 2>&1 | Out-String
if ($LASTEXITCODE -eq 0) {
  $log.Add('LanmanWorkstation restarted') | Out-Null
} else {
  $log.Add('LanmanWorkstation restart failed; full Windows restart recommended') | Out-Null
}

"SUCCESS: Network Compatibility Fix fully implemented!`n`n" + ($log -join "`n") + "`n`nIMPORTANT NOTE:`n- gpedit may still show Not Configured (expected).`n- Changes are applied through registry.`n- Test guest share access after Windows restart if needed."
"#),
        "disable_insecure_guest_smb" => powershell(r#"
$ErrorActionPreference = 'Continue'
$log = New-Object System.Collections.Generic.List[string]

Ensure-Key 'HKLM:\SYSTEM\CurrentControlSet\Services\LanmanWorkstation\Parameters'
Set-ItemProperty 'HKLM:\SYSTEM\CurrentControlSet\Services\LanmanWorkstation\Parameters' -Name RequireSecuritySignature -Type DWord -Value 1
$log.Add('RequireSecuritySignature = 1') | Out-Null

Remove-ItemProperty 'HKLM:\SOFTWARE\Policies\Microsoft\Windows\LanmanWorkstation' -Name AllowInsecureGuestAuth -ErrorAction SilentlyContinue
Remove-ItemProperty 'HKLM:\SYSTEM\CurrentControlSet\Services\LanmanWorkstation\Parameters' -Name AllowInsecureGuestAuth -ErrorAction SilentlyContinue
$log.Add('AllowInsecureGuestAuth removed from Policies + legacy') | Out-Null

net stop lanmanworkstation /y | Out-Null
net start lanmanworkstation | Out-Null
$log.Add('LanmanWorkstation restarted') | Out-Null

"SUCCESS: Secure defaults restored!`n`n" + ($log -join "`n") + "`n`nA full Windows restart is recommended."
"#),
        "enable_smb1_client" => cmd_shell(r#"dism /online /Enable-Feature /FeatureName:SMB1Protocol /All /NoRestart"#),
        "disable_smb1_client" => cmd_shell(r#"dism /online /Disable-Feature /FeatureName:SMB1Protocol /NoRestart"#),
        "tcp_offload_on" => cmd_shell(r"netsh int tcp set global taskoffload=enabled"),
        "tcp_offload_off" => cmd_shell(r"netsh int tcp set global taskoffload=disabled"),

        // PERFORMANCE
        "visual_performance" => cmd_shell(r#"reg add "HKCU\Software\Microsoft\Windows\CurrentVersion\Explorer\VisualEffects" /v VisualFXSetting /t REG_DWORD /d 2 /f"#),
        "ultimate_performance" => powershell(r#"
$guid = 'e9a42b02-d5df-448d-aa00-03f14749eb61'
if (-not ((powercfg /list | Out-String) -match $guid)) { powercfg -duplicatescheme $guid $guid | Out-Null }
powercfg /setactive $guid
if ($LASTEXITCODE -ne 0) { throw 'Ultimate Performance is not supported on this system.' }
'Ultimate Performance power plan activated.'
"#),
        "performance_lock" => cmd_shell(r"powercfg /setactive 8c5e7fda-e8bf-4a96-9a85-a6e23a8c635c & powercfg /change standby-timeout-ac 0 & powercfg /change standby-timeout-dc 0 & powercfg /change hibernate-timeout-ac 0 & powercfg /change hibernate-timeout-dc 0 & powercfg /change monitor-timeout-ac 0 & powercfg /change monitor-timeout-dc 0"),
        "restore_balanced_defaults" => cmd_shell(r"powercfg /setactive 381b4222-f694-41f0-9685-ff5bb260df2e & powercfg /change standby-timeout-ac 30 & powercfg /change standby-timeout-dc 15 & powercfg /change hibernate-timeout-ac 180 & powercfg /change hibernate-timeout-dc 60 & powercfg /change monitor-timeout-ac 15 & powercfg /change monitor-timeout-dc 5 & powercfg /change disk-timeout-ac 20 & powercfg /change disk-timeout-dc 10"),
        "high_performance" => cmd_shell(r"powercfg /setactive 8c5e7fda-e8bf-4a96-9a85-a6e23a8c635c"),
        "balanced_power" => cmd_shell(r"powercfg /setactive 381b4222-f694-41f0-9685-ff5bb260df2e"),
        "power_saver" => cmd_shell(r"powercfg /setactive a1841308-3541-4fab-bc81-f71556f20b4a"),
        "hags_on" => cmd_shell(r#"reg add "HKLM\SYSTEM\CurrentControlSet\Control\GraphicsDrivers" /v HwSchMode /t REG_DWORD /d 2 /f"#),
        "hags_off" => cmd_shell(r#"reg add "HKLM\SYSTEM\CurrentControlSet\Control\GraphicsDrivers" /v HwSchMode /t REG_DWORD /d 1 /f"#),
        "vbs_off" => cmd_shell(r#"reg add "HKLM\SYSTEM\CurrentControlSet\Control\DeviceGuard" /v EnableVirtualizationBasedSecurity /t REG_DWORD /d 0 /f & reg add "HKLM\SYSTEM\CurrentControlSet\Control\Lsa" /v LsaCfgFlags /t REG_DWORD /d 0 /f & bcdedit /set hypervisorlaunchtype off"#),
        "vbs_on" => cmd_shell(r#"reg add "HKLM\SYSTEM\CurrentControlSet\Control\DeviceGuard" /v EnableVirtualizationBasedSecurity /t REG_DWORD /d 1 /f & reg add "HKLM\SYSTEM\CurrentControlSet\Control\Lsa" /v LsaCfgFlags /t REG_DWORD /d 1 /f"#),
        "open_startup_apps" => open_ms_settings("ms-settings:startupapps"),
        "relaunch_apps_on" => powershell(r#"Ensure-Key 'HKCU:\Software\Microsoft\Windows NT\CurrentVersion\Winlogon'; Set-ItemProperty 'HKCU:\Software\Microsoft\Windows NT\CurrentVersion\Winlogon' -Name RestartApps -Type DWord -Value 1; 'Relaunch apps enabled.'"#),
        "relaunch_apps_off" => powershell(r#"Ensure-Key 'HKCU:\Software\Microsoft\Windows NT\CurrentVersion\Winlogon'; Set-ItemProperty 'HKCU:\Software\Microsoft\Windows NT\CurrentVersion\Winlogon' -Name RestartApps -Type DWord -Value 0; 'Relaunch apps disabled.'"#),
        "background_apps_off" => cmd_shell(r#"reg add "HKCU\Software\Microsoft\Windows\CurrentVersion\BackgroundAccessApplications" /v GlobalUserDisabled /t REG_DWORD /d 1 /f"#),
        "background_apps_on" => cmd_shell(r#"reg add "HKCU\Software\Microsoft\Windows\CurrentVersion\BackgroundAccessApplications" /v GlobalUserDisabled /t REG_DWORD /d 0 /f"#),
        "transparency_off" => cmd_shell(r#"reg add "HKCU\Software\Microsoft\Windows\CurrentVersion\Themes\Personalize" /v EnableTransparency /t REG_DWORD /d 0 /f"#),
        "transparency_on" => cmd_shell(r#"reg add "HKCU\Software\Microsoft\Windows\CurrentVersion\Themes\Personalize" /v EnableTransparency /t REG_DWORD /d 1 /f"#),
        "game_mode_on" => cmd_shell(r#"reg add "HKCU\Software\Microsoft\GameBar" /v AutoGameModeEnabled /t REG_DWORD /d 1 /f"#),
        "game_mode_off" => cmd_shell(r#"reg add "HKCU\Software\Microsoft\GameBar" /v AutoGameModeEnabled /t REG_DWORD /d 0 /f"#),
        "windowed_optimizations_on" => cmd_shell(r#"reg add "HKCU\Software\Microsoft\DirectX\UserGpuPreferences" /v DirectXUserGlobalSettings /t REG_SZ /d "SwapEffectUpgradeEnable=1;" /f"#),
        "windowed_optimizations_off" => cmd_shell(r#"reg add "HKCU\Software\Microsoft\DirectX\UserGpuPreferences" /v DirectXUserGlobalSettings /t REG_SZ /d "SwapEffectUpgradeEnable=0;" /f"#),
        "background_recording_off" => cmd_shell(r#"reg add "HKCU\Software\Microsoft\Windows\CurrentVersion\GameDVR" /v AppCaptureEnabled /t REG_DWORD /d 0 /f & reg add "HKCU\System\GameConfigStore" /v GameDVR_Enabled /t REG_DWORD /d 0 /f"#),
        "background_recording_on" => cmd_shell(r#"reg add "HKCU\Software\Microsoft\Windows\CurrentVersion\GameDVR" /v AppCaptureEnabled /t REG_DWORD /d 1 /f & reg add "HKCU\System\GameConfigStore" /v GameDVR_Enabled /t REG_DWORD /d 1 /f"#),
        "superfetch_off" => cmd_shell(r"sc stop SysMain & sc config SysMain start= disabled"),
        "superfetch_on" => cmd_shell(r"sc config SysMain start= auto & sc start SysMain"),
        "search_indexing_off" => cmd_shell(r"sc stop WSearch & sc config WSearch start= disabled"),
        "search_indexing_on" => cmd_shell(r"sc config WSearch start= delayed-auto & sc start WSearch"),
        "delivery_optimization_off" => cmd_shell(r#"reg add "HKLM\SOFTWARE\Policies\Microsoft\Windows\DeliveryOptimization" /v DODownloadMode /t REG_DWORD /d 0 /f & sc stop DoSvc & sc config DoSvc start= disabled"#),
        "delivery_optimization_on" => cmd_shell(r#"reg delete "HKLM\SOFTWARE\Policies\Microsoft\Windows\DeliveryOptimization" /v DODownloadMode /f & sc config DoSvc start= delayed-auto & sc start DoSvc"#),
        "updates_default" => cmd_shell(r#"reg delete "HKLM\SOFTWARE\Policies\Microsoft\Windows\WindowsUpdate\AU" /v NoAutoUpdate /f & reg delete "HKLM\SOFTWARE\Policies\Microsoft\Windows\WindowsUpdate\AU" /v AUOptions /f & sc config wuauserv start= demand & sc config bits start= delayed-auto & sc config UsoSvc start= demand & sc config DoSvc start= delayed-auto & sc start wuauserv & sc start bits"#),
        "updates_security" => cmd_shell(r#"reg add "HKLM\SOFTWARE\Policies\Microsoft\Windows\WindowsUpdate\AU" /v NoAutoUpdate /t REG_DWORD /d 0 /f & reg add "HKLM\SOFTWARE\Policies\Microsoft\Windows\WindowsUpdate\AU" /v AUOptions /t REG_DWORD /d 4 /f & sc config wuauserv start= demand & sc config bits start= delayed-auto & sc start wuauserv & sc start bits"#),
        "updates_off" => cmd_shell(r#"reg add "HKLM\SOFTWARE\Policies\Microsoft\Windows\WindowsUpdate\AU" /v NoAutoUpdate /t REG_DWORD /d 1 /f & sc stop UsoSvc & sc stop wuauserv & sc stop bits & sc stop DoSvc & sc config UsoSvc start= disabled & sc config wuauserv start= disabled & sc config bits start= disabled & sc config DoSvc start= disabled"#),

        // CUSTOMIZE PREFERENCES
        "start_with_windows_on" => set_run_at_startup(true),
        "start_with_windows_off" => set_run_at_startup(false),
        "mouse_accel_off" => set_mouse_acceleration(false),
        "mouse_accel_on" => set_mouse_acceleration(true),
        "numlock_on" => set_numlock(true),
        "numlock_off" => set_numlock(false),
        "taskbar_search_hide" => set_taskbar_search(false),
        "taskbar_search_show" => set_taskbar_search(true),
        "widgets_off" => powershell(r#"
Ensure-Key 'HKLM:\SOFTWARE\Policies\Microsoft\Dsh'
Set-ItemProperty 'HKLM:\SOFTWARE\Policies\Microsoft\Dsh' -Name AllowNewsAndInterests -Type DWord -Value 0
Set-ItemProperty 'HKCU:\Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced' -Name TaskbarDa -Type DWord -Value 0 -ErrorAction SilentlyContinue
Stop-Process -Name explorer -Force -ErrorAction SilentlyContinue
'Widgets disabled by policy. Explorer restarted.'
"#),
        "widgets_on" => powershell(r#"
Remove-ItemProperty 'HKLM:\SOFTWARE\Policies\Microsoft\Dsh' -Name AllowNewsAndInterests -ErrorAction SilentlyContinue
Set-ItemProperty 'HKCU:\Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced' -Name TaskbarDa -Type DWord -Value 1 -ErrorAction SilentlyContinue
Stop-Process -Name explorer -Force -ErrorAction SilentlyContinue
'Widgets policy removed. Explorer restarted. If the button is still hidden, enable Widgets in Settings > Personalization > Taskbar.'
"#),
        "snap_on" => cmd_shell(r#"reg add "HKCU\Control Panel\Desktop" /v WindowArrangementActive /t REG_SZ /d 1 /f"#),
        "snap_off" => cmd_shell(r#"reg add "HKCU\Control Panel\Desktop" /v WindowArrangementActive /t REG_SZ /d 0 /f"#),
        "sticky_keys_off" => cmd_shell(r#"reg add "HKCU\Control Panel\Accessibility\StickyKeys" /v Flags /t REG_SZ /d 506 /f"#),
        "sticky_keys_on" => cmd_shell(r#"reg add "HKCU\Control Panel\Accessibility\StickyKeys" /v Flags /t REG_SZ /d 510 /f"#),
        "task_view_off" => powershell(r#"
Ensure-Key 'HKCU:\Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced'
Set-ItemProperty 'HKCU:\Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced' -Name ShowTaskViewButton -Value 0 -Type DWord
& "$env:WINDIR\System32\RUNDLL32.EXE" user32.dll,UpdatePerUserSystemParameters 1, True | Out-Null
'Task View hidden.'
"#),
        "task_view_on" => powershell(r#"
Ensure-Key 'HKCU:\Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced'
Set-ItemProperty 'HKCU:\Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced' -Name ShowTaskViewButton -Value 1 -Type DWord
& "$env:WINDIR\System32\RUNDLL32.EXE" user32.dll,UpdatePerUserSystemParameters 1, True | Out-Null
'Task View shown.'
"#),
        "verbose_logon_on" => cmd_shell(r#"reg add "HKLM\SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\System" /v VerboseStatus /t REG_DWORD /d 1 /f"#),
        "verbose_logon_off" => cmd_shell(r#"reg add "HKLM\SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\System" /v VerboseStatus /t REG_DWORD /d 0 /f"#),
        "bitlocker_suspend" => powershell(r#"Get-BitLockerVolume | ForEach-Object { Suspend-BitLocker -MountPoint $_.MountPoint -RebootCount 0 -ErrorAction SilentlyContinue }; 'BitLocker suspended where applicable.'"#),
        "bitlocker_resume" => powershell(r#"Get-BitLockerVolume | ForEach-Object { Resume-BitLocker -MountPoint $_.MountPoint -ErrorAction SilentlyContinue }; 'BitLocker resumed where applicable.'"#),
        "bitlocker_off_all" => powershell(r#"Get-BitLockerVolume | Where-Object { $_.ProtectionStatus -eq 'On' } | ForEach-Object { Disable-BitLocker -MountPoint $_.MountPoint -ErrorAction SilentlyContinue }; 'BitLocker decryption requested where protection was on.'"#),

        // SETTINGS
        "windows_theme_light" => cmd_shell(r#"reg add "HKCU\Software\Microsoft\Windows\CurrentVersion\Themes\Personalize" /v AppsUseLightTheme /t REG_DWORD /d 1 /f & reg add "HKCU\Software\Microsoft\Windows\CurrentVersion\Themes\Personalize" /v SystemUsesLightTheme /t REG_DWORD /d 1 /f"#),
        "windows_theme_dark" => cmd_shell(r#"reg add "HKCU\Software\Microsoft\Windows\CurrentVersion\Themes\Personalize" /v AppsUseLightTheme /t REG_DWORD /d 0 /f & reg add "HKCU\Software\Microsoft\Windows\CurrentVersion\Themes\Personalize" /v SystemUsesLightTheme /t REG_DWORD /d 0 /f"#),
        "taskbar_alignment_status" => powershell(r#"$v=(Get-ItemProperty 'HKCU:\Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced' -Name TaskbarAl -ErrorAction SilentlyContinue).TaskbarAl; if ($v -eq 0) { 'Taskbar alignment: Left' } elseif ($v -eq 1) { 'Taskbar alignment: Center' } else { 'Taskbar alignment: default/unknown' }"#),
        "taskbar_center" => cmd_shell(r#"reg add "HKCU\Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced" /v TaskbarAl /t REG_DWORD /d 1 /f & taskkill /f /im explorer.exe & start explorer.exe"#),
        "taskbar_left" => cmd_shell(r#"reg add "HKCU\Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced" /v TaskbarAl /t REG_DWORD /d 0 /f & taskkill /f /im explorer.exe & start explorer.exe"#),

        // APPS
        "winget_upgrade_all" => command_output("winget", &args(&["upgrade", "--all", "--accept-package-agreements", "--accept-source-agreements", "--disable-interactivity"])),
        "reinstall_winget" => powershell(r#"
$ErrorActionPreference = 'Continue'
Add-AppxPackage -RegisterByFamilyName -MainPackage Microsoft.DesktopAppInstaller_8wekyb3d8bbwe -ErrorAction SilentlyContinue
if (Get-Command winget -ErrorAction SilentlyContinue) { 'App Installer re-registered. winget is available:'; winget --version }
else { Start-Process 'ms-windows-store://pdp/?ProductId=9NBLGGH4NNS1'; 'winget is still missing. Opened App Installer in Microsoft Store - install or update it there.' }
"#),
        "install_chrome_extensions" => cmd_shell(r#"start "" "https://chromewebstore.google.com/category/extensions" & start "" "https://chromewebstore.google.com/search/uBlock%20Origin" & start "" "https://chromewebstore.google.com/search/Bitwarden""#),

        other => Err(format!("Unknown action_id: {other}")),
    }
}

fn winget_action_sync(operation: String, package_ids: Vec<String>) -> Result<String, String> {
    if package_ids.is_empty() {
        return Err("No packages selected.".into());
    }

    let mut combined = String::new();
    for package_id in package_ids {
        let mut command_args = match operation.as_str() {
            "install" => vec!["install".to_string(), "--id".to_string(), package_id.clone(), "-e".to_string(), "--accept-package-agreements".to_string(), "--accept-source-agreements".to_string(), "--disable-interactivity".to_string()],
            "uninstall" => vec!["uninstall".to_string(), "--id".to_string(), package_id.clone(), "-e".to_string(), "--silent".to_string(), "--accept-source-agreements".to_string(), "--disable-interactivity".to_string()],
            "upgrade" => vec!["upgrade".to_string(), "--id".to_string(), package_id.clone(), "-e".to_string(), "--silent".to_string(), "--accept-package-agreements".to_string(), "--accept-source-agreements".to_string(), "--disable-interactivity".to_string()],
            other => return Err(format!("Unsupported winget operation: {other}")),
        };

        if operation == "install" {
            command_args.push("--silent".to_string());
        }

        combined.push_str(&format!("\n> winget {} {}\n", operation, package_id));
        match command_output("winget", &command_args) {
            Ok(output) => combined.push_str(&output),
            Err(error) => combined.push_str(&format!("ERROR: {error}")),
        }
        combined.push('\n');
    }
    Ok(combined.trim().to_string())
}

const DEPROVISIONED_ROOT: &str = r"HKLM:\SOFTWARE\Microsoft\Windows\CurrentVersion\Appx\AppxAllUserStore\Deprovisioned";

fn is_valid_package_token(value: &str) -> bool {
    !value.is_empty() && value.chars().all(|ch| ch.is_ascii_alphanumeric() || matches!(ch, '.' | '_' | '-'))
}

/// Debloat flow that survives Windows feature updates:
/// 1. add the package family to the AppxAllUserStore\Deprovisioned list,
/// 2. remove the provisioned package so new user profiles do not get it,
/// 3. uninstall it for every existing user.
fn remove_windows_apps_sync(targets: Vec<WindowsAppTarget>) -> Result<String, String> {
    if targets.is_empty() {
        return Err("No Windows apps selected.".into());
    }
    if !is_admin() {
        return Err("Administrator privileges are required to deprovision Windows apps. Please run Eoliann Windows Tools as Administrator.".into());
    }

    let mut entries = Vec::new();
    for target in &targets {
        let name = target.package_name.trim();
        if !is_valid_package_token(name) {
            return Err(format!("Invalid package name: {name}"));
        }
        let family = target.family_name.as_deref().map(str::trim).unwrap_or("");
        if !family.is_empty() && !is_valid_package_token(family) {
            return Err(format!("Invalid package family name: {family}"));
        }
        entries.push(format!("[PSCustomObject]@{{ Name = {}; Family = {} }}", ps_single_quote(name), ps_single_quote(family)));
    }

    let mut script = format!("$targets = @({})\n$root = {}\n", entries.join(", "), ps_single_quote(DEPROVISIONED_ROOT));
    script.push_str(r#"
$ErrorActionPreference = 'Continue'
$log = New-Object System.Collections.Generic.List[string]
$failed = 0
Ensure-Key $root
$installed = @(Get-AppxPackage -AllUsers -ErrorAction SilentlyContinue)
$provisioned = @(Get-AppxProvisionedPackage -Online -ErrorAction SilentlyContinue)

foreach ($target in $targets) {
  $name = $target.Name
  $pkgs = @($installed | Where-Object { $_.Name -eq $name })
  $provs = @($provisioned | Where-Object { $_.DisplayName -eq $name })

  $families = @($pkgs | ForEach-Object { $_.PackageFamilyName }) + @($provs | ForEach-Object { $_.DisplayName + '_' + (($_.PackageName -split '__')[-1]) })
  if ($target.Family) { $families += $target.Family }
  $families = @($families | Where-Object { $_ } | Sort-Object -Unique)

  foreach ($family in $families) {
    $familyKey = Join-Path $root $family
    Ensure-Key $familyKey
    if (Test-Path -LiteralPath $familyKey) { $log.Add("[$name] Added to Deprovisioned list: $family") | Out-Null }
    else { $failed++; $log.Add("[$name] FAILED to add to Deprovisioned list: $family") | Out-Null }
  }

  foreach ($prov in $provs) {
    try {
      Remove-AppxProvisionedPackage -Online -AllUsers -PackageName $prov.PackageName -ErrorAction Stop | Out-Null
      $log.Add("[$name] Removed provisioned package: $($prov.PackageName)") | Out-Null
    } catch {
      try {
        Remove-AppxProvisionedPackage -Online -PackageName $prov.PackageName -ErrorAction Stop | Out-Null
        $log.Add("[$name] Removed provisioned package: $($prov.PackageName)") | Out-Null
      } catch {
        $failed++
        $log.Add("[$name] FAILED to remove provisioned package $($prov.PackageName): $($_.Exception.Message)") | Out-Null
      }
    }
  }

  foreach ($pkg in $pkgs) {
    try {
      Remove-AppxPackage -Package $pkg.PackageFullName -AllUsers -ErrorAction Stop
      $log.Add("[$name] Uninstalled for all users: $($pkg.PackageFullName)") | Out-Null
    } catch {
      try {
        Remove-AppxPackage -Package $pkg.PackageFullName -ErrorAction Stop
        $log.Add("[$name] Uninstalled for current user: $($pkg.PackageFullName)") | Out-Null
      } catch {
        $failed++
        $log.Add("[$name] FAILED to uninstall $($pkg.PackageFullName): $($_.Exception.Message)") | Out-Null
      }
    }
  }

  if ($pkgs.Count -eq 0 -and $provs.Count -eq 0) {
    $log.Add("[$name] Not installed. Blocked from being reinstalled by Windows updates.") | Out-Null
  }
}

$header = if ($failed -eq 0) { "Debloat completed for $($targets.Count) app(s). Removed apps will not be reinstalled by Windows feature updates." } else { "Debloat finished with $failed error(s)." }
$header + "`n" + ($log -join "`n")
"#);
    powershell(&script)
}

fn restore_deprovisioned_app_sync(family_name: String) -> Result<String, String> {
    let family = family_name.trim();
    if !is_valid_package_token(family) {
        return Err(format!("Invalid package family name: {family}"));
    }
    if !is_admin() {
        return Err("Administrator privileges are required to change the Deprovisioned list. Please run Eoliann Windows Tools as Administrator.".into());
    }
    let script = format!("$path = Join-Path {} {}\n", ps_single_quote(DEPROVISIONED_ROOT), ps_single_quote(family))
        + r#"
if (Test-Path -LiteralPath $path) {
  Remove-Item -LiteralPath $path -Recurse -Force -ErrorAction Stop
  "Removed from Deprovisioned list: $(Split-Path $path -Leaf). Reinstall the app from Microsoft Store if you need it now; Windows may also restore it on the next feature update."
} else {
  "Not in Deprovisioned list: $(Split-Path $path -Leaf)"
}
"#;
    powershell(&script)
}

fn get_windows_apps_status_sync() -> Result<WindowsAppsStatus, String> {
    let script = format!("$root = {}\n", ps_single_quote(DEPROVISIONED_ROOT))
        + r#"
$ErrorActionPreference = 'SilentlyContinue'
$packages = @{}
foreach ($p in @(Get-AppxPackage -AllUsers)) {
  if (-not $packages.ContainsKey($p.Name)) {
    $packages[$p.Name] = [PSCustomObject]@{ Name = $p.Name; FamilyName = $p.PackageFamilyName; Installed = $true; Provisioned = $false }
  }
}
foreach ($p in @(Get-AppxProvisionedPackage -Online)) {
  if ($packages.ContainsKey($p.DisplayName)) { $packages[$p.DisplayName].Provisioned = $true }
  else { $packages[$p.DisplayName] = [PSCustomObject]@{ Name = $p.DisplayName; FamilyName = $p.DisplayName + '_' + (($p.PackageName -split '__')[-1]); Installed = $false; Provisioned = $true } }
}
$deprovisioned = @(Get-ChildItem -LiteralPath $root | ForEach-Object { $_.PSChildName })
ConvertTo-Json -InputObject ([PSCustomObject]@{ Packages = @($packages.Values); Deprovisioned = $deprovisioned }) -Depth 4 -Compress
"#;
    let json = powershell(&script)?;
    let parsed: PsWindowsAppsStatus = serde_json::from_str(json.trim())
        .map_err(|error| format!("Failed to parse Windows apps status: {error}"))?;
    Ok(WindowsAppsStatus {
        packages: parsed
            .packages
            .unwrap_or_default()
            .into_iter()
            .filter_map(|item| {
                Some(WindowsAppPackage {
                    name: item.name.filter(|name| !name.is_empty())?,
                    family_name: item.family_name.unwrap_or_default(),
                    installed: item.installed.unwrap_or(false),
                    provisioned: item.provisioned.unwrap_or(false),
                })
            })
            .collect(),
        deprovisioned: parsed.deprovisioned.unwrap_or_default(),
    })
}

fn open_system_tool_sync(tool_id: String) -> Result<String, String> {
    match tool_id.as_str() {
        "taskmgr" => powershell("Start-Process taskmgr.exe; 'Task Manager launched.'"),
        "regedit" => powershell("Start-Process regedit.exe; 'Registry Editor launched.'"),
        "gpedit" => powershell("Start-Process gpedit.msc; 'Group Policy Editor launched.'"),
        "services" => powershell("Start-Process services.msc; 'Services launched.'"),
        "diskmgmt" => powershell("Start-Process diskmgmt.msc; 'Disk Management launched.'"),
        "msinfo32" | "systeminfo" => powershell("Start-Process msinfo32.exe; 'System Information launched.'"),
        "windows_update" => open_ms_settings("ms-settings:windowsupdate"),
        "winx" => send_win_x(),
        "run" => powershell("Start-Process explorer.exe 'shell:::{2559a1f3-21d7-11d4-bdaf-00c04f60b9f0}'; 'Run dialog requested.'"),
        "settings" => open_ms_settings("ms-settings:"),
        "control_panel" => powershell("Start-Process control.exe; 'Control Panel launched.'"),
        "device_manager" => powershell("Start-Process devmgmt.msc; 'Device Manager launched.'"),
        "event_viewer" => powershell("Start-Process eventvwr.msc; 'Event Viewer launched.'"),
        "computer_management" => powershell("Start-Process compmgmt.msc; 'Computer Management launched.'"),
        "local_users" => powershell("Start-Process lusrmgr.msc; 'Local Users and Groups launched where available.'"),
        other => Err(format!("Unknown tool_id: {other}")),
    }
}

fn get_installed_apps_sync() -> Result<String, String> {
    command_output("winget", &args(&["list", "--accept-source-agreements", "--disable-interactivity"]))
}

#[tauri::command]
pub async fn get_windows_apps_status() -> Result<WindowsAppsStatus, String> {
    run_blocking(get_windows_apps_status_sync).await
}

#[tauri::command]
pub async fn remove_windows_apps(targets: Vec<WindowsAppTarget>) -> Result<String, String> {
    run_blocking(move || remove_windows_apps_sync(targets)).await
}

#[tauri::command]
pub async fn restore_deprovisioned_app(family_name: String) -> Result<String, String> {
    run_blocking(move || restore_deprovisioned_app_sync(family_name)).await
}

#[tauri::command]
pub async fn open_external_url(url: String) -> Result<String, String> {
    run_blocking(move || open_external_url_sync(url)).await
}

fn get_smb_guest_status_sync() -> Result<SmbGuestStatus, String> {
    let script = r#"
$ErrorActionPreference = 'SilentlyContinue'

$cfg = Get-SmbClientConfiguration
$policy = Get-ItemProperty 'HKLM:\SOFTWARE\Policies\Microsoft\Windows\LanmanWorkstation' -Name AllowInsecureGuestAuth -ErrorAction SilentlyContinue
$legacy = Get-ItemProperty 'HKLM:\SYSTEM\CurrentControlSet\Services\LanmanWorkstation\Parameters' -Name AllowInsecureGuestAuth -ErrorAction SilentlyContinue
$smb1 = Get-WindowsOptionalFeature -Online -FeatureName SMB1Protocol -ErrorAction SilentlyContinue

[PSCustomObject]@{
  EnableInsecureGuestLogons = if ($null -ne $cfg) { [bool]$cfg.EnableInsecureGuestLogons } else { $null }
  RequireSecuritySignature = if ($null -ne $cfg) { [bool]$cfg.RequireSecuritySignature } else { $null }
  PolicyAllowInsecureGuestAuth = if ($null -ne $policy) { [int]$policy.AllowInsecureGuestAuth } else { $null }
  LegacyAllowInsecureGuestAuth = if ($null -ne $legacy) { [int]$legacy.AllowInsecureGuestAuth } else { $null }
  Smb1Enabled = if ($null -ne $smb1) { $smb1.State -eq 'Enabled' } else { $null }
} | ConvertTo-Json -Compress
"#;

    let json = powershell(script)?;
    let parsed: SmbGuestStatus = serde_json::from_str(json.trim()).map_err(|e| format!("Failed to parse SMB status: {e}"))?;
    Ok(parsed)
}

fn open_external_url_sync(url: String) -> Result<String, String> {
    let trimmed = url.trim();
    if !(trimmed.starts_with("https://") || trimmed.starts_with("http://")) || trimmed.chars().any(|ch| ch.is_control()) {
        return Err("Only http/https URLs are allowed.".into());
    }
    powershell(&format!("Start-Process {}; 'Opened {}'", ps_single_quote(trimmed), trimmed.replace('\'', "''")))
}

#[cfg(windows)]
fn send_win_x() -> Result<String, String> {
    extern "system" {
        fn keybd_event(vk: u8, scan: u8, flags: u32, extra: usize);
    }
    const VK_LWIN: u8 = 0x5B;
    const VK_X: u8 = 0x58;
    const KEYEVENTF_KEYUP: u32 = 0x0002;
    unsafe {
        keybd_event(VK_LWIN, 0, 0, 0);
        keybd_event(VK_X, 0, 0, 0);
        keybd_event(VK_X, 0, KEYEVENTF_KEYUP, 0);
        keybd_event(VK_LWIN, 0, KEYEVENTF_KEYUP, 0);
    }
    Ok("Win + X Quick Link menu opened.".into())
}

#[cfg(not(windows))]
fn send_win_x() -> Result<String, String> {
    Err("Win + X is only available on Windows.".into())
}

fn create_local_user_sync(username: String, password: String, add_to_admins: bool) -> Result<String, String> {
    let username = username.trim();
    if username.is_empty() {
        return Err("Username is required.".into());
    }
    if password.len() < 8 {
        return Err("Password must be at least 8 characters.".into());
    }
    if username.contains('\\') || username.contains('/') || username.contains('"') || username.contains('\'') {
        return Err("Username contains invalid characters.".into());
    }
    let user = ps_single_quote(username);
    let pass = ps_single_quote(&password);
    let add_admin = ps_bool(add_to_admins);
    let script = format!(r#"
$ErrorActionPreference = 'Stop'
$user = {user}
$plain = {pass}
$secure = ConvertTo-SecureString $plain -AsPlainText -Force
if (Get-LocalUser -Name $user -ErrorAction SilentlyContinue) {{ throw "Local user already exists: $user" }}
New-LocalUser -Name $user -Password $secure -FullName $user -Description 'Created by Eoliann Windows Tools' | Out-Null
Add-LocalGroupMember -Group 'Users' -Member $user -ErrorAction SilentlyContinue
if ({add_admin}) {{ Add-LocalGroupMember -Group 'Administrators' -Member $user }}
"Local user created: $user" + $(if ({add_admin}) {{ ' (Administrators)' }} else {{ '' }})
"#);
    powershell(&script)
}

fn install_chrome_extensions_sync(extension_ids: Vec<String>) -> Result<String, String> {
    if extension_ids.is_empty() {
        return Err("No Chrome extensions selected.".into());
    }

    let mut lines: Vec<String> = Vec::new();
    for extension_id in extension_ids {
        let id = extension_id.trim();
        if id.is_empty() {
            continue;
        }
        let key = format!(r"HKCU\Software\Google\Chrome\Extensions\{}", id);
        let result = command_output(
            "reg",
            &args(&[
                "add",
                &key,
                "/v",
                "update_url",
                "/t",
                "REG_SZ",
                "/d",
                "https://clients2.google.com/service/update2/crx",
                "/f",
            ]),
        );
        match result {
            Ok(_) => lines.push(format!("OK: {}", id)),
            Err(err) => lines.push(format!("ERROR: {} -> {}", id, err)),
        }
    }

    if lines.is_empty() {
        return Err("No valid extension IDs provided.".into());
    }

    Ok(format!(
        "Chrome extensions processed. Restart Chrome to load them.\n{}",
        lines.join("\n")
    ))
}
