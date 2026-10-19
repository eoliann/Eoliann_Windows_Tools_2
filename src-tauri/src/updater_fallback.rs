//! Fallback update path used when the updater plugin cannot reach GitHub (for example when an
//! antivirus, a PAC proxy or a filtering gateway blocks its built-in HTTP client).
//! Downloads go through Windows' own networking stack (PowerShell / .NET, which honours the
//! system proxy configuration), and the installer is only started after its minisign signature
//! has been verified with the same public key the updater plugin uses.

use base64::Engine;
use minisign_verify::{PublicKey, Signature};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::process::Command;

const APP_VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Debug, Serialize)]
pub struct FallbackUpdateInfo {
    pub available: bool,
    pub current_version: String,
    pub latest_version: String,
    pub notes: Option<String>,
}

#[derive(Debug, Deserialize)]
struct LatestJson {
    version: String,
    notes: Option<String>,
    platforms: std::collections::HashMap<String, PlatformEntry>,
}

#[derive(Debug, Deserialize)]
struct PlatformEntry {
    signature: String,
    url: String,
}

fn updater_config(app: &tauri::AppHandle) -> Result<(String, String), String> {
    let updater = app
        .config()
        .plugins
        .0
        .get("updater")
        .ok_or_else(|| "Updater configuration is missing.".to_string())?;
    let endpoint = updater
        .get("endpoints")
        .and_then(|value| value.as_array())
        .and_then(|list| list.first())
        .and_then(|value| value.as_str())
        .ok_or_else(|| "Updater endpoint is missing.".to_string())?
        .to_string();
    let pubkey = updater
        .get("pubkey")
        .and_then(|value| value.as_str())
        .ok_or_else(|| "Updater public key is missing.".to_string())?
        .to_string();
    Ok((endpoint, pubkey))
}

fn hidden_powershell(script: &str) -> Result<String, String> {
    let mut command = Command::new("powershell");
    command.args(["-NoProfile", "-NonInteractive", "-ExecutionPolicy", "Bypass", "-Command", script]);
    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        const CREATE_NO_WINDOW: u32 = 0x08000000;
        command.creation_flags(CREATE_NO_WINDOW);
    }
    let output = command.output().map_err(|error| format!("Failed to start PowerShell: {error}"))?;
    let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
    if output.status.success() {
        Ok(stdout)
    } else {
        Err(if stderr.is_empty() { stdout } else { stderr })
    }
}

fn ps_quote(value: &str) -> String {
    format!("'{}'", value.replace('\'', "''"))
}

const PS_WEB_PRELUDE: &str = r#"
$ErrorActionPreference = 'Stop'
$ProgressPreference = 'SilentlyContinue'
[Net.ServicePointManager]::SecurityProtocol = [Net.ServicePointManager]::SecurityProtocol -bor [Net.SecurityProtocolType]::Tls12
$proxy = [System.Net.WebRequest]::GetSystemWebProxy()
$proxy.Credentials = [System.Net.CredentialCache]::DefaultNetworkCredentials
[System.Net.WebRequest]::DefaultWebProxy = $proxy
$headers = @{ 'User-Agent' = 'EoliannWindowsTools-Updater' }
"#;

fn download_text(url: &str) -> Result<String, String> {
    let script = format!(
        "{PS_WEB_PRELUDE}\n$response = Invoke-WebRequest -Uri {} -Headers $headers -UseBasicParsing -TimeoutSec 60\n[System.Text.Encoding]::UTF8.GetString($response.RawContentStream.ToArray())",
        ps_quote(url)
    );
    hidden_powershell(&script).map_err(|error| format!("Could not download {url}: {error}"))
}

fn download_file(url: &str, destination: &PathBuf) -> Result<(), String> {
    let script = format!(
        "{PS_WEB_PRELUDE}\nInvoke-WebRequest -Uri {} -Headers $headers -UseBasicParsing -TimeoutSec 600 -OutFile {}",
        ps_quote(url),
        ps_quote(&destination.to_string_lossy())
    );
    hidden_powershell(&script).map(|_| ()).map_err(|error| format!("Could not download {url}: {error}"))
}

fn fetch_latest(endpoint: &str) -> Result<(LatestJson, PlatformEntry), String> {
    let text = download_text(endpoint)?;
    let mut latest: LatestJson = serde_json::from_str(text.trim_start_matches('\u{feff}').trim())
        .map_err(|error| format!("Invalid update manifest: {error}"))?;
    let entry = latest
        .platforms
        .remove("windows-x86_64-nsis")
        .or_else(|| latest.platforms.remove("windows-x86_64"))
        .ok_or_else(|| "The update manifest has no Windows x64 installer.".to_string())?;
    Ok((latest, entry))
}

fn parse_version(value: &str) -> Vec<u64> {
    value
        .trim()
        .trim_start_matches('v')
        .split(['-', '+'])
        .next()
        .unwrap_or("")
        .split('.')
        .map(|part| part.parse::<u64>().unwrap_or(0))
        .collect()
}

fn is_newer(latest: &str, current: &str) -> bool {
    let (mut a, mut b) = (parse_version(latest), parse_version(current));
    let len = a.len().max(b.len());
    a.resize(len, 0);
    b.resize(len, 0);
    a > b
}

fn decode_base64_text(value: &str) -> Result<String, String> {
    let bytes = base64::engine::general_purpose::STANDARD
        .decode(value.trim())
        .map_err(|error| format!("Invalid base64 data: {error}"))?;
    String::from_utf8(bytes).map_err(|_| "Invalid signature encoding.".to_string())
}

fn verify_signature(data: &[u8], signature: &str, pubkey: &str) -> Result<(), String> {
    let public_key = PublicKey::decode(&decode_base64_text(pubkey)?).map_err(|error| format!("Invalid public key: {error}"))?;
    let signature = Signature::decode(&decode_base64_text(signature)?).map_err(|error| format!("Invalid signature: {error}"))?;
    public_key
        .verify(data, &signature, true)
        .map_err(|error| format!("Installer signature verification failed: {error}"))
}

pub fn check(app: &tauri::AppHandle) -> Result<FallbackUpdateInfo, String> {
    let (endpoint, _) = updater_config(app)?;
    let (latest, _) = fetch_latest(&endpoint)?;
    Ok(FallbackUpdateInfo {
        available: is_newer(&latest.version, APP_VERSION),
        current_version: APP_VERSION.to_string(),
        latest_version: latest.version,
        notes: latest.notes,
    })
}

pub fn install(app: &tauri::AppHandle) -> Result<String, String> {
    let (endpoint, pubkey) = updater_config(app)?;
    let (latest, entry) = fetch_latest(&endpoint)?;
    if !is_newer(&latest.version, APP_VERSION) {
        return Err(format!("No update available. Current version: {APP_VERSION}"));
    }

    let folder = std::env::temp_dir().join("EWT-update");
    std::fs::create_dir_all(&folder).map_err(|error| format!("Could not create {}: {error}", folder.display()))?;
    let installer = folder.join(format!("EoliannWindowsTools_{}_x64-setup.exe", latest.version));

    // GitHub stores asset names with spaces replaced by dots, so try both spellings.
    let mut urls = vec![entry.url.clone()];
    let dotted = entry.url.replace("%20", ".");
    if dotted != entry.url {
        urls.push(dotted);
    }
    let mut last_error = String::new();
    let mut downloaded = false;
    for url in &urls {
        match download_file(url, &installer) {
            Ok(()) => {
                downloaded = true;
                break;
            }
            Err(error) => last_error = error,
        }
    }
    if !downloaded {
        return Err(last_error);
    }

    let bytes = std::fs::read(&installer).map_err(|error| format!("Could not read the downloaded installer: {error}"))?;
    if let Err(error) = verify_signature(&bytes, &entry.signature, &pubkey) {
        let _ = std::fs::remove_file(&installer);
        return Err(error);
    }

    Command::new(&installer)
        .args(["/P", "/R", "/UPDATE"])
        .spawn()
        .map_err(|error| format!("Could not start the installer: {error}"))?;

    let handle = app.clone();
    std::thread::spawn(move || {
        std::thread::sleep(std::time::Duration::from_millis(800));
        handle.exit(0);
    });
    Ok(format!("Installing version {}...", latest.version))
}

#[cfg(test)]
mod tests {
    use super::is_newer;

    #[test]
    fn compares_versions() {
        assert!(is_newer("1.1.0", "1.0.4"));
        assert!(is_newer("1.0.10", "1.0.9"));
        assert!(!is_newer("1.0.3", "1.0.3"));
        assert!(!is_newer("1.0.2", "1.0.3"));
        assert!(is_newer("v2.0", "1.9.9"));
    }
}
