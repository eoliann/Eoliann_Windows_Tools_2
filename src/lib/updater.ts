import { getVersion } from "@tauri-apps/api/app";
import { invoke } from "@tauri-apps/api/core";
import { relaunch } from "@tauri-apps/plugin-process";
import { check } from "@tauri-apps/plugin-updater";

export const releasesUrl = "https://github.com/eoliann/Eoliann_Windows_Tools_2/releases/latest";

export interface UpdateCheckResult {
  available: boolean;
  currentVersion: string;
  latestVersion?: string;
  notes?: string;
  date?: string;
}

interface FallbackUpdateInfo {
  available: boolean;
  current_version: string;
  latest_version: string;
  notes?: string | null;
}

export type UpdateProgress = (message: string) => void;

const CHECK_TIMEOUT_MS = 30000;

const describe = (error: unknown) => (error instanceof Error ? error.message : String(error));

export async function checkForAppUpdate(): Promise<UpdateCheckResult> {
  const currentVersion = await getVersion();
  try {
    const update = await check({ timeout: CHECK_TIMEOUT_MS });
    if (!update) {
      return { available: false, currentVersion };
    }
    return { available: true, currentVersion, latestVersion: update.version, notes: update.body, date: update.date };
  } catch (pluginError) {
    // The updater's HTTP client can be blocked by antivirus/proxy setups; retry through Windows networking.
    try {
      const fallback = await invoke<FallbackUpdateInfo>("check_update_fallback");
      return {
        available: fallback.available,
        currentVersion: fallback.current_version,
        latestVersion: fallback.latest_version,
        notes: fallback.notes ?? undefined
      };
    } catch (fallbackError) {
      throw new Error(`${describe(pluginError)} | Fallback check: ${describe(fallbackError)}`);
    }
  }
}

export async function downloadAndInstallUpdate(onProgress?: UpdateProgress): Promise<void> {
  let pluginError: unknown;
  try {
    const update = await check({ timeout: CHECK_TIMEOUT_MS });
    if (!update) {
      throw new Error("No update available.");
    }
    let total = 0;
    let received = 0;
    await update.downloadAndInstall((event) => {
      if (event.event === "Started") {
        total = event.data.contentLength ?? 0;
        onProgress?.("Downloading update...");
      } else if (event.event === "Progress") {
        received += event.data.chunkLength;
        if (total > 0) onProgress?.(`Downloading update... ${Math.min(100, Math.round((received / total) * 100))}%`);
      } else if (event.event === "Finished") {
        onProgress?.("Installing update...");
      }
    }, { timeout: 10 * 60 * 1000 });
    await relaunch();
    return;
  } catch (error) {
    if (describe(error) === "No update available.") throw error;
    pluginError = error;
  }

  onProgress?.("Built-in updater failed, retrying through Windows networking...");
  try {
    await invoke<string>("install_update_fallback");
  } catch (fallbackError) {
    throw new Error(`${describe(pluginError)} | Fallback install: ${describe(fallbackError)}`);
  }
}
