import { CheckSquare, RefreshCw, Search, ShieldBan, Trash2, Undo2, XSquare } from "lucide-react";
import { useCallback, useEffect, useMemo, useState } from "react";
import { windowsApps } from "../data/actions";
import { getWindowsAppsStatus } from "../lib/commands";
import type { WindowsAppItem, WindowsAppsStatus, WindowsAppTarget } from "../types";

interface Props {
  running: boolean;
  onRemove: (targets: WindowsAppTarget[]) => void;
  onRestore: (familyName: string) => void;
  refreshKey: number;
}

type AppState = "installed" | "provisioned" | "blocked" | "absent";

interface AppRow {
  app: WindowsAppItem;
  familyName: string;
  state: AppState;
  blocked: boolean;
}

const MICROSOFT_PUBLISHER_ID = "8wekyb3d8bbwe";

const stateLabel: Record<AppState, string> = {
  installed: "Installed",
  provisioned: "Provisioned only",
  blocked: "Blocked",
  absent: "Not present"
};

const stateClass: Record<AppState, string> = {
  installed: "status-badge bad",
  provisioned: "status-badge bad",
  blocked: "status-badge ok",
  absent: "status-badge neutral"
};

export default function WindowsApps({ running, onRemove, onRestore, refreshKey }: Props) {
  const [query, setQuery] = useState("");
  const [status, setStatus] = useState<WindowsAppsStatus | null>(null);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [showAll, setShowAll] = useState(false);
  const [selected, setSelected] = useState<Set<string>>(new Set());
  const [confirming, setConfirming] = useState(false);

  const load = useCallback(async () => {
    setLoading(true);
    setError(null);
    try {
      setStatus(await getWindowsAppsStatus());
    } catch (e) {
      setError(String(e));
    } finally {
      setLoading(false);
    }
  }, []);

  useEffect(() => {
    setSelected(new Set());
    setConfirming(false);
    void load();
  }, [load, refreshKey]);

  const rows = useMemo<AppRow[]>(() => {
    const packages = new Map((status?.packages ?? []).map((pkg) => [pkg.name.toLowerCase(), pkg]));
    const deprovisioned = new Set((status?.deprovisioned ?? []).map((name) => name.toLowerCase()));
    return windowsApps.map((app) => {
      const pkg = packages.get(app.packageName.toLowerCase());
      const familyName = pkg?.family_name || app.familyName || `${app.packageName}_${MICROSOFT_PUBLISHER_ID}`;
      const blocked = deprovisioned.has(familyName.toLowerCase());
      const state: AppState = pkg?.installed ? "installed" : pkg?.provisioned ? "provisioned" : blocked ? "blocked" : "absent";
      return { app, familyName, state, blocked };
    });
  }, [status]);

  const present = (row: AppRow) => row.state === "installed" || row.state === "provisioned";

  const filtered = useMemo(() => {
    const q = query.trim().toLowerCase();
    return rows.filter((row) => {
      if (!showAll && !present(row)) return false;
      return !q || row.app.name.toLowerCase().includes(q) || row.app.category.toLowerCase().includes(q) || row.app.packageName.toLowerCase().includes(q);
    });
  }, [rows, query, showAll]);

  const grouped = useMemo(() => {
    const groups = new Map<string, AppRow[]>();
    for (const row of filtered) {
      if (!groups.has(row.app.category)) groups.set(row.app.category, []);
      groups.get(row.app.category)!.push(row);
    }
    return Array.from(groups.entries());
  }, [filtered]);

  const recommendedPresent = rows.filter((row) => row.app.recommended && present(row));
  const knownFamilies = new Set(rows.map((row) => row.familyName.toLowerCase()));
  const otherDeprovisioned = (status?.deprovisioned ?? []).filter((name) => !knownFamilies.has(name.toLowerCase()));
  const blockedRows = rows.filter((row) => row.blocked);

  const toggle = (packageName: string) => {
    setConfirming(false);
    setSelected((prev) => {
      const next = new Set(prev);
      next.has(packageName) ? next.delete(packageName) : next.add(packageName);
      return next;
    });
  };

  const selectRecommended = () => {
    setConfirming(false);
    setSelected(new Set(recommendedPresent.map((row) => row.app.packageName)));
  };

  const selectedRows = rows.filter((row) => selected.has(row.app.packageName));

  const runRemoval = () => {
    setConfirming(false);
    onRemove(selectedRows.map((row) => ({ packageName: row.app.packageName, familyName: row.familyName })));
  };

  return (
    <div className="page-stack">
      <section className="content-section prose-card">
        <h2><ShieldBan size={20} /> Permanent debloat</h2>
        <p>
          Removing an app only uninstalls it; a Windows feature update reinstalls every app that is still provisioned.
          Each selected app is first added to the <code>AppxAllUserStore\Deprovisioned</code> list, then its provisioned package is removed
          and it is uninstalled for all users, so Windows updates do not bring it back.
        </p>
        <p className="muted">Create a restore point first. Apps can be allowed back from the Deprovisioned list below and reinstalled from Microsoft Store.</p>
      </section>

      <section className="toolbar-card">
        <div className="search-box"><Search size={18} /><input value={query} onChange={(e) => setQuery(e.target.value)} placeholder="Search Windows bundled apps..." /></div>
        <div className="toolbar-actions">
          <button className="ghost-button" onClick={() => void load()} disabled={loading || running}><RefreshCw size={16} className={loading ? "spin" : ""} /> Refresh</button>
          <button className="ghost-button" onClick={selectRecommended} disabled={loading || running || !recommendedPresent.length}><CheckSquare size={16} /> Select rarely used ({recommendedPresent.length})</button>
          <button className="ghost-button" onClick={() => { setSelected(new Set()); setConfirming(false); }} disabled={!selected.size || running}><XSquare size={16} /> Clear</button>
          <button className="danger-button" onClick={() => setConfirming(true)} disabled={!selected.size || running || loading}><Trash2 size={16} /> Deprovision &amp; remove ({selected.size})</button>
        </div>
      </section>

      {confirming && (
        <section className="content-section prose-card">
          <h2>Confirm debloat of {selectedRows.length} app(s)</h2>
          <p>{selectedRows.map((row) => row.app.name).join(", ")}</p>
          <p className="notice warn">These apps will be blocked from reinstalling during Windows updates and removed for every user on this PC.</p>
          <div className="toolbar-actions">
            <button className="danger-button" onClick={runRemoval} disabled={running}><Trash2 size={16} /> Deprovision &amp; remove</button>
            <button className="ghost-button" onClick={() => setConfirming(false)}>Cancel</button>
          </div>
        </section>
      )}

      <section className="content-section">
        <div className="section-heading">
          <h2>Bundled Windows apps</h2>
          <span>{loading ? "Reading installed and provisioned packages..." : `${rows.filter(present).length} present · ${blockedRows.length} blocked · ${selected.size} selected`}</span>
        </div>
        <label className="checkbox-row">
          <input type="checkbox" checked={showAll} onChange={(e) => setShowAll(e.target.checked)} />
          <span>Show apps that are not present (they can still be blocked pre-emptively)</span>
        </label>
        {error && <p className="notice warn">Could not read Windows apps: {error}</p>}
        {!loading && !error && !filtered.length && <p className="notice">No bundled apps found{showAll ? "" : " — enable the option above to see the full list"}.</p>}
        <div className="catalog-sections">
          {grouped.map(([category, items]) => (
            <div className="catalog-category" key={category}>
              <div className="category-heading"><h3>{category}</h3><span>{items.length} apps</span></div>
              <div className="catalog-grid">
                {items.map((row) => (
                  <label className={`catalog-item ${selected.has(row.app.packageName) ? "selected" : ""}`} key={row.app.packageName}>
                    <input type="checkbox" checked={selected.has(row.app.packageName)} onChange={() => toggle(row.app.packageName)} disabled={running} />
                    <div>
                      <div className="catalog-title-row">
                        <strong>{row.app.name}</strong>
                        <span className={stateClass[row.state]}>{row.blocked && row.state !== "blocked" ? `${stateLabel[row.state]} · blocked` : stateLabel[row.state]}</span>
                      </div>
                      <span>{row.app.recommended ? "Rarely used" : row.app.category}</span>
                      <p>{row.app.description}</p>
                      <code>{row.familyName}</code>
                    </div>
                  </label>
                ))}
              </div>
            </div>
          ))}
        </div>
      </section>

      <section className="content-section">
        <div className="section-heading">
          <h2>Deprovisioned list</h2>
          <span>{(status?.deprovisioned ?? []).length} entries</span>
        </div>
        <p className="muted section-note">Windows skips these packages when it re-provisions apps during feature updates.</p>
        <div className="catalog-grid">
          {[...blockedRows.map((row) => ({ family: row.familyName, label: row.app.name })), ...otherDeprovisioned.map((family) => ({ family, label: family.split("_")[0] }))].map((entry) => (
            <article className="catalog-item removable" key={entry.family}>
              <div>
                <strong>{entry.label}</strong>
                <code>{entry.family}</code>
              </div>
              <button className="ghost-button" disabled={running} onClick={() => onRestore(entry.family)}><Undo2 size={15} /> Allow reinstall</button>
            </article>
          ))}
        </div>
        {!loading && !(status?.deprovisioned ?? []).length && <p className="notice">The Deprovisioned list is empty.</p>}
      </section>
    </div>
  );
}
