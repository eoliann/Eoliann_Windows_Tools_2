import { invoke } from "@tauri-apps/api/core";
import type { DiskHealth, RestorePointsSummary, SmbGuestStatus, SystemOverview, WindowsAppsStatus, WindowsAppTarget } from "../types";

export async function getSystemOverview(): Promise<SystemOverview> {
  return invoke<SystemOverview>("get_system_overview");
}

export async function getDiskHealth(): Promise<DiskHealth[]> {
  return invoke<DiskHealth[]>("get_disk_health");
}

export async function runAction(actionId: string): Promise<string> {
  return invoke<string>("run_action", { actionId });
}

export async function wingetAction(operation: "install" | "uninstall" | "upgrade", packageIds: string[]): Promise<string> {
  return invoke<string>("winget_action", { operation, packageIds });
}

export async function removeWindowsApps(targets: WindowsAppTarget[]): Promise<string> {
  return invoke<string>("remove_windows_apps", { targets });
}

export async function restoreDeprovisionedApp(familyName: string): Promise<string> {
  return invoke<string>("restore_deprovisioned_app", { familyName });
}

export async function getWindowsAppsStatus(): Promise<WindowsAppsStatus> {
  return invoke<WindowsAppsStatus>("get_windows_apps_status");
}

export async function openSystemTool(toolId: string): Promise<string> {
  return invoke<string>("open_system_tool", { toolId });
}

export async function getInstalledApps(): Promise<string> {
  return invoke<string>("get_installed_apps");
}

export async function openExternalUrl(url: string): Promise<string> {
  return invoke<string>("open_external_url", { url });
}

export async function installChromeExtensions(extensionIds: string[]): Promise<string> {
  return invoke<string>("install_chrome_extensions", { extensionIds });
}

export async function wingetUpgradeAllProgress(): Promise<string> {
  return invoke<string>("winget_upgrade_all_progress");
}

export async function createLocalUser(username: string, password: string, addToAdmins: boolean): Promise<string> {
  return invoke<string>("create_local_user", { username, password, addToAdmins });
}

export async function getRestorePoints(): Promise<RestorePointsSummary> {
  return invoke<RestorePointsSummary>("get_restore_points");
}

export async function deleteRestorePoint(sequenceNumber: number): Promise<string> {
  return invoke<string>("delete_restore_point_by_info", { args: { sequenceNumber } });
}

export async function getSmbGuestStatus(): Promise<SmbGuestStatus> {
  return invoke<SmbGuestStatus>("get_smb_guest_status");
}
