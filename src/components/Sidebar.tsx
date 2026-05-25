import { Activity, Box, ExternalLink, Gauge, HardDrive, HeartPulse, Home, Info, Keyboard, Network, Package, Settings, SlidersHorizontal, Sparkles, Wrench } from "lucide-react";
import type { ComponentType } from "react";
import type { PageId } from "../types";

interface SidebarProps {
  activePage: PageId;
  onChange: (page: PageId) => void;
}

const items: Array<{ id: PageId; label: string; icon: ComponentType<{ size?: number }> }> = [
  { id: "dashboard", label: "Dashboard", icon: Home },
  { id: "info", label: "Info", icon: Info },
  { id: "maintenance", label: "Tools", icon: Sparkles },
  { id: "tweaks", label: "Tweaks", icon: Gauge },
  { id: "network", label: "Network", icon: Network },
  { id: "health", label: "Health", icon: HeartPulse },
  { id: "performance", label: "Performance", icon: Wrench },
  { id: "customize", label: "Customize", icon: SlidersHorizontal },
  { id: "apps", label: "Install", icon: Package },
  { id: "windowsApps", label: "WinApp Removal", icon: Box },
  { id: "diskHealth", label: "Disk Health", icon: HardDrive },
  { id: "quickKeys", label: "Quick Keys", icon: Keyboard },
  { id: "settings", label: "Settings", icon: Settings },
  { id: "about", label: "About", icon: Info }
];

export default function Sidebar({ activePage, onChange }: SidebarProps) {
  return (
    <aside className="sidebar">
      <div className="brand">
        <div className="brand-mark"><img src="/app-icon.png" alt="Eoliann Windows Tools" className="brand-icon" /></div>
        <div>
          <strong>Eoliann</strong>
          <span>Windows Tools</span>
        </div>
      </div>

      <nav className="nav-list" aria-label="Main sections">
        {items.map((item) => {
          const Icon = item.icon;
          return (
            <button key={item.id} className={`nav-item ${activePage === item.id ? "active" : ""}`} onClick={() => onChange(item.id)}>
              <Icon size={17} />
              <span>{item.label}</span>
            </button>
          );
        })}
      </nav>

      <div className="sidebar-footer">
        <div className="footer-title"><Activity size={15} /><span>Local Windows utility</span></div>
        <button className="footer-link" onClick={() => onChange("about")}>Terms · Privacy · Donate <ExternalLink size={13} /></button>
      </div>
    </aside>
  );
}
