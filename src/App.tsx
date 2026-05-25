import { useCallback, useEffect, useMemo, useRef, useState } from "react";
import { Loader2 } from "lucide-react";
import { listen } from "@tauri-apps/api/event";
import Header from "./components/Header";
import LogPanel from "./components/LogPanel";
import Sidebar from "./components/Sidebar";
import ConfirmDialog from "./components/ConfirmDialog";
import Dashboard from "./pages/Dashboard";
import Info from "./pages/Info";
import Maintenance from "./pages/Maintenance";
import Tweaks from "./pages/Tweaks";
import Network from "./pages/Network";
import Health from "./pages/Health";
import Performance from "./pages/Performance";
import Customize from "./pages/Customize";
import Apps from "./pages/Apps";
import WindowsApps from "./pages/WindowsApps";
import DiskHealth from "./pages/DiskHealth";
import QuickKeys from "./pages/QuickKeys";
import Settings from "./pages/Settings";
import About from "./pages/About";
import { allActions } from "./data/actions";
import type { DiskHealth as DiskHealthType, PageId, SystemOverview, Theme, ToolAction } from "./types";
import { createLocalUser, getDiskHealth, getSystemOverview, openSystemTool, removeWindowsApp, runAction, wingetAction, wingetUpgradeAllProgress } from "./lib/commands";

const pageMeta: Record<PageId, { title: string; subtitle: string }> = {
  dashboard: { title: "Dashboard", subtitle: "Fast system status and quick actions. Detailed disk data loads only when requested." },
  info: { title: "Info", subtitle: "whoami, ipconfig, systeminfo, tasklist and system reports." },
  maintenance: { title: "Tools", subtitle: "Cleanup, diagnostics, repair and restore tools." },
  tweaks: { title: "Tweaks", subtitle: "Explorer, privacy, AI, Edge, Adobe, OneDrive, security and advanced Windows tweaks." },
  network: { title: "Network", subtitle: "DNS providers, Winsock/IP reset, SMB compatibility and TCP offload." },
  health: { title: "Health", subtitle: "Power, cleanup, diagnostics, hibernation and system restore actions." },
  performance: { title: "Performance", subtitle: "Power plans, graphics, VBS, startup, visual, gaming and service settings." },
  customize: { title: "Customize Preferences", subtitle: "Mouse, NumLock, taskbar, Snap, Sticky Keys, verbose logon and BitLocker." },
  apps: { title: "Install", subtitle: "Install, uninstall and update applications through winget." },
  windowsApps: { title: "Windows App Removal", subtitle: "Remove bundled Microsoft Store app packages by category." },
  diskHealth: { title: "Disk Health", subtitle: "Storage reliability data from Windows Storage cmdlets." },
  quickKeys: { title: "Quick Keys", subtitle: "Shortcuts and fast launchers for important Windows consoles." },
  settings: { title: "Settings", subtitle: "Windows theme, taskbar alignment, local users and native tools." },
  about: { title: "About", subtitle: "Project information, terms, privacy and donation links." }
};

const actionTitle = (id: string | null) => {
  if (!id) return "";
  if (id.startsWith("winget-install")) return "Installing selected applications";
  if (id.startsWith("winget-uninstall")) return "Uninstalling selected applications";
  if (id.startsWith("winget-upgrade")) return "Updating selected applications";
  if (id.startsWith("remove-")) return "Removing bundled Windows app";
  if (id === "create-local-user") return "Creating local user";
  return allActions.find((action) => action.id === id)?.title ?? id;
};

export default function App() {
  const [activePage, setActivePage] = useState<PageId>("dashboard");
  const [theme, setTheme] = useState<Theme>(() => (localStorage.getItem("ewt-theme") as Theme) || "dark");
  const [overview, setOverview] = useState<SystemOverview | null>(null);
  const [diskHealth, setDiskHealth] = useState<DiskHealthType[]>([]);
  const [loading, setLoading] = useState(false);
  const [diskLoading, setDiskLoading] = useState(false);
  const [runningId, setRunningId] = useState<string | null>(null);
  const [pendingAction, setPendingAction] = useState<ToolAction | null>(null);
  const [output, setOutput] = useState("");
  const [wingetPercent, setWingetPercent] = useState(0);
  const [wingetMessage, setWingetMessage] = useState("");
  const [windowsAppsRefreshKey, setWindowsAppsRefreshKey] = useState(0);
  const contentRef = useRef<HTMLDivElement>(null);

  const meta = useMemo(() => pageMeta[activePage], [activePage]);
  const runningTitle = actionTitle(runningId);

  useEffect(() => {
    document.documentElement.dataset.theme = theme;
    localStorage.setItem("ewt-theme", theme);
  }, [theme]);

  useEffect(() => {
    contentRef.current?.scrollTo({ top: 0, left: 0, behavior: "instant" as ScrollBehavior });
  }, [activePage]);

  const appendOutput = useCallback((text: string) => {
    const stamp = new Date().toLocaleTimeString();
    setOutput((prev) => `${prev}${prev ? "\n" : ""}[${stamp}] ${text}`);
  }, []);

  const refreshOverview = useCallback(async () => {
    setLoading(true);
    try {
      setOverview(await getSystemOverview());
    } catch (error) {
      appendOutput(`System overview failed: ${String(error)}`);
    } finally {
      setLoading(false);
    }
  }, [appendOutput]);

  const refreshDiskHealth = useCallback(async () => {
    setDiskLoading(true);
    try {
      setDiskHealth(await getDiskHealth());
    } catch (error) {
      appendOutput(`Disk health failed: ${String(error)}`);
    } finally {
      setDiskLoading(false);
    }
  }, [appendOutput]);

  const refreshWindowsApps = useCallback(() => {
    setWindowsAppsRefreshKey((prev) => prev + 1);
  }, []);

  const refreshCurrentPage = useCallback(async () => {
    if (activePage === "diskHealth") {
      await refreshDiskHealth();
      return;
    }
    if (activePage === "windowsApps") {
      refreshWindowsApps();
      return;
    }
    await refreshOverview();
  }, [activePage, refreshDiskHealth, refreshOverview, refreshWindowsApps]);

  useEffect(() => {
    void refreshOverview();
  }, [refreshOverview]);

  useEffect(() => {
    if (activePage === "diskHealth" && !diskHealth.length && !diskLoading) {
      void refreshDiskHealth();
    }
  }, [activePage, diskHealth.length, diskLoading, refreshDiskHealth]);

  useEffect(() => {
    const unlistenPromise = listen<{ percent: number; message: string }>("winget-progress", (event) => {
      setWingetPercent(event.payload.percent ?? 0);
      setWingetMessage(event.payload.message ?? "");
    });
    return () => {
      void unlistenPromise.then((unlisten) => unlisten());
    };
  }, []);

  const requestRun = (action: ToolAction) => {
    if (runningId) return;
    if (action.risk === "high" || action.requiresAdmin) {
      setPendingAction(action);
      return;
    }
    void executeAction(action);
  };

  const executeAction = async (action: ToolAction) => {
    setPendingAction(null);
    setRunningId(action.id);
    appendOutput(`Running: ${action.title}`);
    try {
      const result = action.id === "winget_upgrade_all" ? await wingetUpgradeAllProgress() : await runAction(action.id);
      appendOutput(result || "Action completed.");
    } catch (error) {
      appendOutput(`ERROR: ${String(error)}`);
    } finally {
      setRunningId(null);
      if (action.id === "winget_upgrade_all") {
        setWingetPercent(0);
        setWingetMessage("");
      }
    }
  };

  const runWinget = async (operation: "install" | "uninstall" | "upgrade", ids: string[]) => {
    if (!ids.length || runningId) return;
    setRunningId(`winget-${operation}`);
    appendOutput(`winget ${operation}: ${ids.join(", ")}`);
    try {
      const result = await wingetAction(operation, ids);
      appendOutput(result || "winget action completed.");
      window.dispatchEvent(new CustomEvent("ewt-winget-finished", { detail: { operation, ids } }));
    } catch (error) {
      appendOutput(`ERROR: ${String(error)}`);
    } finally {
      setRunningId(null);
    }
  };

  const removeApp = async (packageName: string) => {
    if (runningId) return;
    setRunningId(`remove-${packageName}`);
    appendOutput(`Removing Windows app: ${packageName}`);
    try {
      const result = await removeWindowsApp(packageName);
      appendOutput(result || "Package removal completed.");
      refreshWindowsApps();
    } catch (error) {
      appendOutput(`ERROR: ${String(error)}`);
    } finally {
      setRunningId(null);
    }
  };

  const launchTool = async (toolId: string) => {
    appendOutput(`Opening Windows tool: ${toolId}`);
    try {
      const result = await openSystemTool(toolId);
      appendOutput(result || "Tool launched.");
    } catch (error) {
      appendOutput(`ERROR: ${String(error)}`);
    }
  };

  const createUser = async (username: string, password: string, addToAdmins: boolean) => {
    if (runningId) return;
    setRunningId("create-local-user");
    appendOutput(`Creating local user: ${username}${addToAdmins ? " as administrator" : ""}`);
    try {
      const result = await createLocalUser(username, password, addToAdmins);
      appendOutput(result || "Local user created.");
    } catch (error) {
      appendOutput(`ERROR: ${String(error)}`);
    } finally {
      setRunningId(null);
    }
  };

  return (
    <div className="app-shell">
      <Sidebar activePage={activePage} onChange={setActivePage} />
      <main className="main-panel">
        <Header
          title={meta.title}
          subtitle={meta.subtitle}
          overview={overview}
          theme={theme}
          onToggleTheme={() => setTheme(theme === "dark" ? "light" : "dark")}
          onRefresh={refreshCurrentPage}
          loading={loading || diskLoading}
        />

        <div className="content-layout">
          <div className="content-area" ref={contentRef}>
            {activePage === "dashboard" && <Dashboard overview={overview} diskHealth={diskHealth} loading={loading} onQuickAction={requestRun} />}
            {activePage === "info" && <Info runningId={runningId} onRun={requestRun} />}
            {activePage === "maintenance" && <Maintenance runningId={runningId} onRun={requestRun} />}
            {activePage === "tweaks" && <Tweaks runningId={runningId} onRun={requestRun} />}
            {activePage === "network" && <Network runningId={runningId} onRun={requestRun} />}
            {activePage === "health" && <Health runningId={runningId} onRun={requestRun} />}
            {activePage === "performance" && <Performance runningId={runningId} onRun={requestRun} />}
            {activePage === "customize" && <Customize runningId={runningId} onRun={requestRun} />}
            {activePage === "apps" && <Apps running={Boolean(runningId)} runningId={runningId} onRun={requestRun} onInstall={(ids) => runWinget("install", ids)} onUninstall={(ids) => runWinget("uninstall", ids)} onUpgrade={(ids) => runWinget("upgrade", ids)} />}
            {activePage === "windowsApps" && <WindowsApps running={Boolean(runningId)} onRemove={removeApp} refreshKey={windowsAppsRefreshKey} />}
            {activePage === "diskHealth" && <DiskHealth disks={diskHealth} loading={diskLoading} onRefresh={refreshDiskHealth} />}
            {activePage === "quickKeys" && <QuickKeys onOpenTool={launchTool} />}
            {activePage === "settings" && <Settings runningId={runningId} onRun={requestRun} onOpenTool={launchTool} onCreateLocalUser={createUser} creatingUser={runningId === "create-local-user"} />}
            {activePage === "about" && <About appVersion={overview?.app_version} />}
          </div>
          <LogPanel output={output} onClear={() => setOutput("")} />
        </div>
      </main>
      {runningId && (
        <div className="operation-overlay" role="status" aria-live="polite">
          <div className="operation-card">
            <Loader2 className="spin" size={34} />
            <h2>{runningTitle}</h2>
            <p>The operation is running. Do not close the application until the status disappears.</p>
            <span>{runningId}</span>
            {runningId === "winget_upgrade_all" && (
              <>
                <div className="progress-track"><div className="progress-fill" style={{ width: `${wingetPercent}%` }} /></div>
                <strong>{wingetPercent}%</strong>
                <p>{wingetMessage || "Working..."}</p>
              </>
            )}
          </div>
        </div>
      )}
      <ConfirmDialog action={pendingAction} onCancel={() => setPendingAction(null)} onConfirm={() => pendingAction && executeAction(pendingAction)} />
    </div>
  );
}
