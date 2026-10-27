import { ChevronDown, Download, PackageCheck, RefreshCw, Search, Trash2 } from "lucide-react";
import { useCallback, useEffect, useMemo, useRef, useState } from "react";
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

const catalogIdsByLower = new Map(appCatalog.map((app) => [app.id.toLowerCase(), app.id]));
// Longest first, so "Mozilla.Firefox.ESR.x" resolves to Firefox ESR rather than Firefox.
const catalogIdsLongestFirst = [...catalogIdsByLower.keys()].sort((a, b) => b.length - a.length);

// "google.chrome.exe" -> "Google.Chrome": winget sometimes lists a variant of the catalog id.
function catalogIdForToken(token: string) {
  const exact = catalogIdsByLower.get(token);
  if (exact) return exact;
  const prefix = catalogIdsLongestFirst.find((id) => token.startsWith(`${id}.`));
  return prefix ? catalogIdsByLower.get(prefix) : undefined;
}

const escapeRegExp = (value: string) => value.replace(/[.*+?^${}()|[\]\\]/g, "\\$&");

// Matches `winget list` rows against the catalog. A row whose Id column is a catalog id
// belongs to that app only; rows without a known id (ARP\..., MSIX\..., Store ids) fall
// back to a whole-word match of the app name in the Name column, so "Git" does not match
// "GitHub Desktop" and "Vim" does not match "Neovim".
function buildInstalledSet(output: string) {
  const found = new Set<string>();
  const unmatchedNames: string[] = [];
  for (const raw of output.split(/\r?\n/)) {
    const row = raw.slice(raw.lastIndexOf("\r") + 1).trim().toLowerCase();
    if (!row) continue;
    const ids = row.split(/\s+/).map(catalogIdForToken).filter((id): id is string => Boolean(id));
    if (ids.length) ids.forEach((id) => found.add(id));
    else unmatchedNames.push(row.split(/\s{2,}/)[0]);
  }
  for (const app of appCatalog) {
    if (found.has(app.id)) continue;
    const pattern = new RegExp(`(^|[^a-z0-9])${escapeRegExp(app.name.toLowerCase())}($|[^a-z0-9])`);
    if (unmatchedNames.some((name) => pattern.test(name))) found.add(app.id);
  }
  return found;
}

export default function Apps({ running, runningId, onRun, onInstall, onUninstall, onUpgrade }: AppsProps) {
  const [query, setQuery] = useState("");
  const [selected, setSelected] = useState<Set<string>>(new Set());
  const [installedIds, setInstalledIds] = useState<Set<string>>(new Set());
  const [checkingInstalled, setCheckingInstalled] = useState(false);
  const [scanError, setScanError] = useState<string | null>(null);
  const [skippedNotice, setSkippedNotice] = useState<string | null>(null);
  const scanRequest = useRef(0);
  const [extensionsOutput, setExtensionsOutput] = useState<string>("");
  const [selectedExts, setSelectedExts] = useState<Set<string>>(
    new Set(chromeExtensions.filter((extension) => extension.defaultEnabled).map((extension) => extension.id))
  );

  const scanInstalled = useCallback(async () => {
    // Only the latest scan may update state, so a slow earlier scan cannot overwrite
    // the status refreshed after an install/uninstall.
    const request = ++scanRequest.current;
    setCheckingInstalled(true);
    setScanError(null);
    try {
      const output = await getInstalledApps();
      if (request === scanRequest.current) setInstalledIds(buildInstalledSet(output));
    } catch (error) {
      if (request === scanRequest.current) setScanError(String(error));
    } finally {
      if (request === scanRequest.current) setCheckingInstalled(false);
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
  const installableIds = selectedIds.filter((id) => !installedIds.has(id));
  const toggle = (app: AppCatalogItem) => {
    setSelected((prev) => {
      const next = new Set(prev);
      next.has(app.id) ? next.delete(app.id) : next.add(app.id);
      return next;
    });
  };

  const startInstall = () => {
    const skipped = appCatalog.filter((app) => selected.has(app.id) && installedIds.has(app.id));
    const skippedNames = [...new Set(skipped.map((app) => app.name))];
    setSkippedNotice(
      skippedNames.length
        ? `Skipped ${skippedNames.length} app(s): ${skippedNames.join(", ")}. Reason: already installed. Use "Upgrade selected" to update them.`
        : null
    );
    if (installableIds.length) onInstall(installableIds);
  };
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
          <button
            className="primary-button"
            disabled={!installableIds.length || running}
            title={selected.size && !installableIds.length ? "All selected apps are already installed" : undefined}
            onClick={startInstall}
          >
            <Download size={16} /> Install selected
          </button>
          <button className="ghost-button" disabled={!selected.size || running} onClick={startUpgrade}><PackageCheck size={16} /> Upgrade selected</button>
          <button className="danger-button" disabled={!selected.size || running} onClick={startUninstall}><Trash2 size={16} /> Uninstall selected</button>
        </div>
      </section>

      <section className="content-section">
        <details className="accordion-card">
          <summary className="accordion-summary">
            <span><strong>Chrome extensions</strong> <span>({selectedExts.size} selected)</span></span>
            <ChevronDown size={16} className="accordion-chevron" />
          </summary>
          <p className="notice">Select and install</p>
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
        </details>
      </section>

      <section className="content-section">
        <div className="section-heading">
          <h2>Application catalog</h2>
          <span>{selected.size} selected · {filtered.length} shown · {installedIds.size} detected installed</span>
        </div>
        {scanError && <p className="notice warn">Installed-app scan failed: {scanError}</p>}
        {skippedNotice && <p className="notice warn">{skippedNotice}</p>}
        {checkingInstalled && !running && <p className="notice">Refreshing installed status...</p>}
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
