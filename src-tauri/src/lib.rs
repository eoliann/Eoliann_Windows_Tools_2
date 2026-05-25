mod commands;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            commands::get_system_overview,
            commands::get_disk_health,
            commands::run_action,
            commands::winget_action,
            commands::remove_windows_app,
            commands::open_system_tool,
            commands::get_installed_apps,
            commands::get_installed_windows_apps,
            commands::open_external_url,
            commands::create_local_user,
            commands::winget_upgrade_all_progress,
            commands::install_chrome_extensions,
            commands::get_restore_points,
            commands::delete_restore_point,
            commands::delete_restore_point_by_info
        ])
        .run(tauri::generate_context!())
        .expect("error while running Eoliann Windows Tools");
}
