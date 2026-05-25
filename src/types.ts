export type Theme = "dark" | "light";

export interface DiskSummary {
  name: string;
  total_gb: number;
  free_gb: number;
}

export interface SystemOverview {
  computer_name: string;
  username: string;
  windows_caption: string;
  version: string;
  architecture: string;
  cpu: string;
  ram_gb: number;
  disks: DiskSummary[];
  uptime: string;
  is_admin: boolean;
  app_version: string;
  winget_available: boolean;
}

export interface DiskHealth {
  friendly_name: string;
  media_type: string;
  health_status: string;
  operational_status: string;
  size_gb: number;
  temperature?: number | null;
  wear?: number | null;
  read_errors?: number | null;
  write_errors?: number | null;
}

export interface RestorePointInfo {
  shadow_id: string;
  sequence_number: number;
  description: string;
  creation_time: string;
}

export interface RestorePointsSummary {
  total_used_space_gb: number;
  points: RestorePointInfo[];
}

export type PageId =
  | "dashboard"
  | "info"
  | "maintenance"
  | "tweaks"
  | "network"
  | "health"
  | "performance"
  | "customize"
  | "apps"
  | "windowsApps"
  | "diskHealth"
  | "quickKeys"
  | "settings"
  | "about";

export type ActionRisk = "low" | "medium" | "high";

export interface ToolAction {
  id: string;
  title: string;
  description: string;
  icon: string;
  category: string;
  requiresAdmin?: boolean;
  risk?: ActionRisk;
}

export interface AppCatalogItem {
  id: string;
  name: string;
  category: string;
  description: string;
}

export interface WindowsAppItem {
  packageName: string;
  name: string;
  category: string;
  description: string;
}
