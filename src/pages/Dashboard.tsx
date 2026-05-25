import { AlertTriangle, CheckCircle2, HardDrive, RefreshCw } from "lucide-react";
import type { DiskHealth, SystemOverview, ToolAction } from "../types";
import StatCard from "../components/StatCard";

interface DashboardProps {
  overview: SystemOverview | null;
  diskHealth: DiskHealth[];
  loading: boolean;
  onQuickAction: (action: ToolAction) => void;
}

const quickActions: ToolAction[] = [
  { id: "create_restore_point", title: "Restore point", description: "Create a safety checkpoint before changes.", icon: "ShieldCheck", category: "Quick", requiresAdmin: true, risk: "low" },
  { id: "clean_temp_files", title: "Clean temp", description: "Clean temporary folders.", icon: "Sparkles", category: "Quick", requiresAdmin: true, risk: "medium" },
  { id: "flush_dns", title: "Flush DNS", description: "Clear DNS cache.", icon: "RefreshCw", category: "Quick", risk: "low" },
  { id: "sfc_dism", title: "Repair Windows", description: "Run DISM + SFC.", icon: "Wrench", category: "Quick", requiresAdmin: true, risk: "medium" }
];

export default function Dashboard({ overview, diskHealth, loading, onQuickAction }: DashboardProps) {
  const safe = (value: number) => (Number.isFinite(value) ? value : 0);
  const totalDisk = overview?.disks?.reduce((acc, disk) => acc + safe(disk.total_gb), 0) ?? 0;
  const freeDisk = overview?.disks?.reduce((acc, disk) => acc + safe(disk.free_gb), 0) ?? 0;

  if (!overview && loading) {
    return <div className="empty-state"><RefreshCw className="spin" /> Loading system overview...</div>;
  }

  return (
    <div className="page-stack">
      <section className="hero-card">
        <div>
          <p className="eyebrow">System overview</p>
          <h2>{overview?.windows_caption || "Windows"}</h2>
          <p>{overview?.computer_name || "Unknown PC"} · {overview?.username || "Unknown user"}</p>
        </div>
        <div className="hero-status">
          {overview?.is_admin ? <CheckCircle2 size={20} /> : <AlertTriangle size={20} />}
          {overview?.is_admin ? "Ready for admin tasks" : "Open as administrator for all tools"}
        </div>
      </section>

      <div className="stats-grid">
        <StatCard icon="MonitorCog" label="Windows" value={overview?.version || "Unknown"} hint={overview?.architecture} />
        <StatCard icon="Cpu" label="CPU" value={overview?.cpu || "Unknown"} />
        <StatCard icon="MemoryStick" label="Memory" value={`${overview?.ram_gb ?? 0} GB`} />
        <StatCard icon="HardDrive" label="Storage" value={`${Math.round(freeDisk)} GB free`} hint={`${Math.round(totalDisk)} GB total`} />
        <StatCard icon="Clock3" label="Uptime" value={overview?.uptime || "Unknown"} />
        <StatCard icon="PackageCheck" label="winget" value={overview?.winget_available ? "Available" : "Missing"} />
      </div>

      <section className="content-section">
        <div className="section-heading">
          <h2>Quick actions</h2>
          <span>Common tasks</span>
        </div>
        <div className="quick-grid">
          {quickActions.map((action) => (
            <button key={action.id} className="quick-action" onClick={() => onQuickAction(action)}>
              <strong>{action.title}</strong>
              <span>{action.description}</span>
            </button>
          ))}
        </div>
      </section>

      <section className="content-section">
        <div className="section-heading">
          <h2>Storage summary</h2>
          <span>{overview?.disks?.length ?? 0} fixed drives</span>
        </div>
        <div className="disk-list">
          {(overview?.disks ?? []).map((disk) => (
            <div className="disk-row" key={disk.name}>
              <HardDrive size={18} />
              <div>
                <strong>{disk.name}</strong>
                <span>{Math.round(safe(disk.free_gb))} GB free · {Math.round(safe(disk.total_gb))} GB total</span>
              </div>
              <em>{safe(disk.total_gb) ? `${Math.round((safe(disk.free_gb) / safe(disk.total_gb)) * 100)}% free` : "N/A"}</em>
            </div>
          ))}
          <p className="muted">Detailed disk health is loaded only from the Disk Health page to keep startup responsive.</p>
        </div>
      </section>
    </div>
  );
}
