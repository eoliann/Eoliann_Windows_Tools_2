import { Activity, Cog, FolderCog, HardDrive, Keyboard, MonitorCog, Network, Play, Search, Settings, Shield, Wrench } from "lucide-react";

interface Props {
  onOpenTool: (toolId: string) => void;
}

const shortcuts = [
  { id: "winx", name: "Win + X", description: "Quick Link menu", icon: Keyboard },
  { id: "run", name: "Win + R", description: "Run dialog", icon: Play },
  { id: "settings", name: "Win + I", description: "Windows Settings", icon: Settings },
  { id: "regedit", name: "regedit", description: "Registry Editor", icon: FolderCog },
  { id: "gpedit", name: "gpedit.msc", description: "Group Policy Editor", icon: Shield },
  { id: "taskmgr", name: "Task Manager", description: "Processes and startup", icon: Activity },
  { id: "services", name: "services.msc", description: "Services console", icon: Cog },
  { id: "msinfo32", name: "msinfo32", description: "System Information", icon: MonitorCog },
  { id: "diskmgmt", name: "diskmgmt.msc", description: "Disk Management", icon: HardDrive },
  { id: "device_manager", name: "devmgmt.msc", description: "Device Manager", icon: Wrench },
  { id: "event_viewer", name: "eventvwr.msc", description: "Event Viewer", icon: Search },
  { id: "computer_management", name: "compmgmt.msc", description: "Computer Management", icon: Network }
];

export default function QuickKeys({ onOpenTool }: Props) {
  return (
    <section className="content-section">
      <div className="section-heading">
        <h2>Quick Keys</h2>
        <span>{shortcuts.length} shortcuts</span>
      </div>
      <div className="quick-grid">
        {shortcuts.map((tool) => {
          const Icon = tool.icon;
          return (
            <button className="quick-action" key={tool.id} onClick={() => onOpenTool(tool.id)}>
              <strong><Icon size={18} /> {tool.name}</strong>
              <span>{tool.description}</span>
            </button>
          );
        })}
      </div>
    </section>
  );
}
