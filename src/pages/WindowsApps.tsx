import { Search, Trash2 } from "lucide-react";
import { useEffect, useMemo, useState } from "react";
import { windowsApps } from "../data/actions";
import { getInstalledWindowsApps } from "../lib/commands";

interface Props {
  running: boolean;
  onRemove: (packageName: string) => void;
  refreshKey: number;
}

export default function WindowsApps({ running, onRemove, refreshKey }: Props) {
  const [query, setQuery] = useState("");
  const [installed, setInstalled] = useState<Set<string>>(new Set());

  useEffect(() => {
    const load = async () => {
      try {
        const names = await getInstalledWindowsApps();
        setInstalled(new Set(names.map((n) => n.toLowerCase())));
      } catch {
        setInstalled(new Set());
      }
    };
    void load();
  }, [refreshKey]);

  const filtered = useMemo(() => {
    const q = query.trim().toLowerCase();
    return windowsApps.filter((app) => {
      const visible = !q || app.name.toLowerCase().includes(q) || app.category.toLowerCase().includes(q) || app.packageName.toLowerCase().includes(q);
      if (!visible) return false;
      const needle = app.packageName.toLowerCase();
      return [...installed].some((name) => name.includes(needle));
    });
  }, [query, installed]);

  return (
    <div className="page-stack">
      <section className="toolbar-card">
        <div className="search-box"><Search size={18} /><input value={query} onChange={(e) => setQuery(e.target.value)} placeholder="Search Windows bundled apps..." /></div>
      </section>

      <section className="content-section">
        <div className="section-heading">
          <h2>Bundled Windows apps</h2>
          <span>{filtered.length} detected installed</span>
        </div>
        <div className="catalog-grid">
          {filtered.map((app) => (
            <article className="catalog-item removable" key={app.packageName}>
              <div>
                <strong>{app.name}</strong>
                <span>{app.category}</span>
                <p>{app.description}</p>
                <code>{app.packageName}</code>
              </div>
              <button className="danger-button" disabled={running} onClick={() => onRemove(app.packageName)}><Trash2 size={15} /> Remove</button>
            </article>
          ))}
        </div>
      </section>
    </div>
  );
}
