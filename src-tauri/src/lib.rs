mod commands;
mod updater_fallback;

#[tauri::command]
async fn check_update_fallback(app: tauri::AppHandle) -> Result<updater_fallback::FallbackUpdateInfo, String> {
    tauri::async_runtime::spawn_blocking(move || updater_fallback::check(&app))
        .await
        .map_err(|error| format!("Background task failed: {error}"))?
}

#[tauri::command]
async fn install_update_fallback(app: tauri::AppHandle) -> Result<String, String> {
    tauri::async_runtime::spawn_blocking(move || updater_fallback::install(&app))
        .await
        .map_err(|error| format!("Background task failed: {error}"))?
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .invoke_handler(tauri::generate_handler![
            check_update_fallback,
            install_update_fallback,
            commands::get_system_overview,
            commands::get_disk_health,
            commands::run_action,
            commands::winget_action,
            commands::get_windows_apps_status,
            commands::remove_windows_apps,
            commands::restore_deprovisioned_app,
            commands::open_system_tool,
            commands::get_installed_apps,
            commands::open_external_url,
            commands::create_local_user,
            commands::winget_upgrade_all_progress,
            commands::install_chrome_extensions,
            commands::get_restore_points,
            commands::delete_restore_point_by_info,
            commands::get_smb_guest_status
        ])
        .run(tauri::generate_context!())
        .expect("error while running Eoliann Windows Tools");
}
