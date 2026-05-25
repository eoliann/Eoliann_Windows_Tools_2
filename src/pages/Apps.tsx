import { Download, PackageCheck, RefreshCw, Search, Trash2 } from "lucide-react";
import { useCallback, useEffect, useMemo, useState } from "react";
import ActionGrid from "../components/ActionGrid";
import { appActions, appCatalog } from "../data/actions";
import { getInstalledApps, installChromeExtensions } from "../lib/commands";
import type { AppCatalogItem, ToolAction } from "../types";

interface AppsProps {
  running: boolean;
  runningId?: string | null;
  onRun: (action: ToolAction) => void;
  onInstall: (ids: string[]) => void;
  onUninstall: (ids: string[]) => void;
  onUpgrade: (ids: string[]) => void;
}

const normalize = (value: string) => value.trim().toLowerCase();

const chromeExtensions = [
  { id: "bgnkhhnnamicmpeenaelnjfhikgbkllg", name: "AdGuard AdBlocker", defaultEnabled: true },
  { id: "cfnpidifppmenkapgihekkeednfoenal", name: "TrafficLight", defaultEnabled: true },
  { id: "khndhdhbebhaddchcgnalcjlaekbbeof", name: "Bitdefender Anti-tracker", defaultEnabled: true },
  { id: "fihnjjcciajhdojfnbdddfaoknhalnja", name: "I don't care about cookies", defaultEnabled: false },
  { id: "oeopbcgkkoapgobdbedcemjljbihmemj", name: "Checker Plus for Gmail", defaultEnabled: false },
  { id: "lpcaedmchfhocbbapmcbpinfpgnhiddi", name: "Google Keep", defaultEnabled: false },
  { id: "jldhpllghnbhlbpcmnajkpdmadaolakh", name: "Todoist for Chrome", defaultEnabled: false },
  { id: "ghgabhipcejejjmhhchfonmamedcbeod", name: "Click&Clean", defaultEnabled: false },
  { id: "nlkaejimjacpillmajjnopmpbkbnocid", name: "YouTube NonStop", defaultEnabled: false },
  { id: "pachckjkecffpdphbpmfolblodfkgbhl", name: "vidIQ Vision for YouTube", defaultEnabled: false }
];

function buildInstalledSet(output: string) {
  const found = new Set<string>();
  const lower = output.toLowerCase();
  for (const app of appCatalog) {
    if (lower.includes(app.id.toLowerCase()) || lower.includes(app.name.toLowerCase())) {
      found.add(app.id);
    }
  }
  return found;
}

export default function Apps({ running, runningId, onRun, onInstall, onUninstall, onUpgrade }: AppsProps) {
  const [query, setQuery] = useState("");
  const [selected, setSelected] = useState<Set<string>>(new Set());
  const [installedIds, setInstalledIds] = useState<Set<string>>(new Set());
  const [checkingInstalled, setCheckingInstalled] = useState(false);
  const [scanError, setScanError] = useState<string | null>(null);
  const [extensionsOutput, setExtensionsOutput] = useState<string>("");
  const [selectedExts, setSelectedExts] = useState<Set<string>>(
    new Set(chromeExtensions.filter((extension) => extension.defaultEnabled).map((extension) => extension.id))
  );

  const scanInstalled = useCallback(async () => {
    setCheckingInstalled(true);
    setScanError(null);
    try {
      const output = await getInstalledApps();
      setInstalledIds(buildInstalledSet(output));
    } catch (error) {
      setScanError(String(error));
    } finally {
      setCheckingInstalled(false);
    }
  }, []);

  useEffect(() => {
    void scanInstalled();
  }, [scanInstalled]);

  useEffect(() => {
    const handler = () => {
      setSelected(new Set());
      void scanInstalled();
    };
    window.addEventListener("ewt-winget-finished", handler);
    return () => window.removeEventListener("ewt-winget-finished", handler);
  }, [scanInstalled]);

  const filtered = useMemo(() => {
    const q = normalize(query);
    return appCatalog.filter((app) => !q || normalize(app.name).includes(q) || normalize(app.category).includes(q) || normalize(app.id).includes(q));
  }, [query]);

  const grouped = useMemo(() => {
    const groups = new Map<string, AppCatalogItem[]>();
    for (const app of filtered) {
      if (!groups.has(app.category)) groups.set(app.category, []);
      groups.get(app.category)!.push(app);
    }
    return Array.from(groups.entries());
  }, [filtered]);

  const selectedIds = [...selected];
  const toggle = (app: AppCatalogItem) => {
    setSelected((prev) => {
      const next = new Set(prev);
      next.has(app.id) ? next.delete(app.id) : next.add(app.id);
      return next;
    });
  };

  const startInstall = () => onInstall(selectedIds);
  const startUpgrade = () => onUpgrade(selectedIds);
  const startUninstall = () => onUninstall(selectedIds);
  const installSelectedExtensions = () => {
    void (async () => {
      try {
        const result = await installChromeExtensions([...selectedExts]);
        setExtensionsOutput(result || "Chrome extensions processed.");
      } catch (error) {
        setExtensionsOutput(`ERROR: ${String(error)}`);
      }
    })();
  };

  return (
    <div className="page-stack">
      <ActionGrid actions={appActions} runningId={runningId} onRun={onRun} />

      <section className="toolbar-card">
        <div className="search-box"><Search size={18} /><input value={query} onChange={(e) => setQuery(e.target.value)} placeholder="Search winget apps, categories or package IDs..." /></div>
        <div className="toolbar-actions">
          <button className="ghost-button" disabled={checkingInstalled || running} onClick={() => void scanInstalled()}><RefreshCw size={16} className={checkingInstalled ? "spin" : ""} /> Scan installed</button>
          <button className="primary-button" disabled={!selected.size || running} onClick={startInstall}><Download size={16} /> Install selected</button>
          <button className="ghost-button" disabled={!selected.size || running} onClick={startUpgrade}><PackageCheck size={16} /> Upgrade selected</button>
          <button className="danger-button" disabled={!selected.size || running} onClick={startUninstall}><Trash2 size={16} /> Uninstall selected</button>
        </div>
      </section>

      <section className="content-section">
        <div className="section-heading">
          <h2>Chrome extensions</h2>
          <span>Select and install</span>
        </div>
        <div className="extensions-links">
          {chromeExtensions.map((extension) => (
            <label key={extension.id} className="catalog-item">
              <input
                type="checkbox"
                checked={selectedExts.has(extension.id)}
                onChange={() => {
                  setSelectedExts((prev) => {
                    const next = new Set(prev);
                    next.has(extension.id) ? next.delete(extension.id) : next.add(extension.id);
                    return next;
                  });
                }}
              />
              <div><strong>{extension.name}</strong><code>{extension.id}</code></div>
            </label>
          ))}
        </div>
        <button className="primary-button" onClick={installSelectedExtensions} disabled={!selectedExts.size}>Install selected extensions</button>
        {extensionsOutput && <p className="notice">{extensionsOutput}</p>}
      </section>

      <section className="content-section">
        <div className="section-heading">
          <h2>Application catalog</h2>
          <span>{selected.size} selected · {filtered.length} shown · {installedIds.size} detected installed</span>
        </div>
        {scanError && <p className="notice warn">Installed-app scan failed: {scanError}</p>}
        {running && <p className="notice">Operation in progress. Details are shown in the global status overlay and log panel.</p>}

        <div className="catalog-sections">
          {grouped.map(([category, apps]) => (
            <div className="catalog-category" key={category}>
              <div className="category-heading"><h3>{category}</h3><span>{apps.length} apps</span></div>
              <div className="catalog-grid">
                {apps.map((app) => {
                  const installed = installedIds.has(app.id);
                  return (
                    <label className={`catalog-item ${selected.has(app.id) ? "selected" : ""} ${installed ? "installed" : ""}`} key={app.id}>
                      <input type="checkbox" checked={selected.has(app.id)} onChange={() => toggle(app)} disabled={running} />
                      <div>
                        <div className="catalog-title-row"><strong>{app.name}</strong>{installed && <em>Installed</em>}</div>
                        <span>{app.category}</span>
                        <p>{app.description}</p>
                        <code>{app.id}</code>
                      </div>
                    </label>
                  );
                })}
              </div>
            </div>
          ))}
        </div>
      </section>
    </div>
  );
}
