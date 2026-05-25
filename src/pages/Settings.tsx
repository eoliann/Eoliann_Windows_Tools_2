import { Activity, Cog, ExternalLink, FolderCog, HardDrive, MonitorCog, Settings as SettingsIcon, Shield, UserPlus } from "lucide-react";
import { useState } from "react";
import ActionGrid from "../components/ActionGrid";
import { settingsActions } from "../data/actions";
import type { ToolAction } from "../types";

interface Props {
  runningId?: string | null;
  onRun: (action: ToolAction) => void;
  onOpenTool: (toolId: string) => void;
  onCreateLocalUser: (username: string, password: string, addToAdmins: boolean) => void;
  creatingUser: boolean;
}

const tools = [
  { id: "taskmgr", name: "Task Manager", icon: Activity },
  { id: "regedit", name: "Registry Editor", icon: FolderCog },
  { id: "gpedit", name: "Group Policy", icon: Shield },
  { id: "services", name: "Services", icon: Cog },
  { id: "diskmgmt", name: "Disk Management", icon: HardDrive },
  { id: "msinfo32", name: "System Information", icon: MonitorCog },
  { id: "windows_update", name: "Windows Update", icon: SettingsIcon },
  { id: "local_users", name: "Local Users", icon: UserPlus }
];

export default function Settings({ runningId, onRun, onOpenTool, onCreateLocalUser, creatingUser }: Props) {
  const [username, setUsername] = useState("");
  const [password, setPassword] = useState("");
  const [addToAdmins, setAddToAdmins] = useState(false);

  const submit = () => {
    onCreateLocalUser(username.trim(), password, addToAdmins);
    setPassword("");
  };

  return (
    <div className="page-stack">
      <ActionGrid actions={settingsActions} runningId={runningId} onRun={onRun} />

      <section className="content-section">
        <div className="section-heading">
          <h2>Quick launch</h2>
          <span>Windows tools</span>
        </div>
        <div className="quick-grid">
          {tools.map((tool) => {
            const Icon = tool.icon;
            return (
              <button className="quick-action" key={tool.id} onClick={() => onOpenTool(tool.id)}>
                <strong><Icon size={18} /> {tool.name}</strong>
                <span>Open Windows {tool.name}</span>
              </button>
            );
          })}
        </div>
      </section>

      <section className="content-section prose-card">
        <h2>Create new local user account</h2>
        <p>Creates a local Windows user with a password. Administrator membership is optional and requires an elevated app session.</p>
        <div className="form-grid">
          <label>
            <span>Username</span>
            <input value={username} onChange={(e) => setUsername(e.target.value)} placeholder="local_user" />
          </label>
          <label>
            <span>Password</span>
            <input type="password" value={password} onChange={(e) => setPassword(e.target.value)} placeholder="Minimum 8 characters" />
          </label>
          <label className="checkbox-row">
            <input type="checkbox" checked={addToAdmins} onChange={(e) => setAddToAdmins(e.target.checked)} />
            <span>Add to Administrators group</span>
          </label>
          <button className="primary-button" disabled={creatingUser || !username.trim() || password.length < 8} onClick={submit}>
            <UserPlus size={16} /> {creatingUser ? "Creating..." : "Create local user"}
          </button>
        </div>
      </section>

      <section className="content-section prose-card">
        <h2>Operational notes</h2>
        <p>Run the app as administrator for full functionality. Non-elevated sessions can still read system information and run user-level tweaks.</p>
        <p>Release builds keep the Tauri command list restricted to mapped actions; the UI does not expose arbitrary shell execution.</p>
        <a href="https://github.com/eoliann/Eoliann_Windows_Tools" target="_blank" rel="noreferrer"><ExternalLink size={16} /> Open GitHub repository</a>
      </section>
    </div>
  );
}
