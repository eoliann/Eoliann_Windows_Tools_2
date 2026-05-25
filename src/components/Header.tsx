import { RefreshCw, ShieldAlert, ShieldCheck, SunMoon } from "lucide-react";
import type { SystemOverview, Theme } from "../types";

interface HeaderProps {
  title: string;
  subtitle: string;
  overview?: SystemOverview | null;
  theme: Theme;
  onToggleTheme: () => void;
  onRefresh: () => void;
  loading?: boolean;
}

export default function Header({ title, subtitle, overview, theme, onToggleTheme, onRefresh, loading }: HeaderProps) {
  return (
    <header className="topbar">
      <div>
        <p className="eyebrow">EWT</p>
        <h1>{title}</h1>
        <p className="muted">{subtitle}</p>
      </div>

      <div className="topbar-actions">
        {overview && (
          <div className={`status-pill ${overview.is_admin ? "ok" : "warn"}`}>
            {overview.is_admin ? <ShieldCheck size={16} /> : <ShieldAlert size={16} />}
            {overview.is_admin ? "Administrator" : "Standard user"}
          </div>
        )}
        <button className="icon-button" onClick={onToggleTheme} title={`Switch to ${theme === "dark" ? "light" : "dark"} theme`}>
          <SunMoon size={18} />
        </button>
        <button className="icon-button" onClick={onRefresh} title="Refresh" disabled={loading}>
          <RefreshCw size={18} className={loading ? "spin" : ""} />
        </button>
      </div>
    </header>
  );
}
