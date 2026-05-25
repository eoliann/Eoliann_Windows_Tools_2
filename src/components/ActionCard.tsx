import { AlertTriangle, LockKeyhole, Play } from "lucide-react";
import type { ToolAction } from "../types";
import { getIcon } from "./icons";

interface ActionCardProps {
  action: ToolAction;
  running?: boolean;
  onRun: (action: ToolAction) => void;
}

export default function ActionCard({ action, running, onRun }: ActionCardProps) {
  const Icon = getIcon(action.icon);
  return (
    <article className={`action-card risk-${action.risk ?? "low"}`}>
      <div className="action-card-header">
        <div className="action-icon"><Icon size={20} /></div>
        <div className="action-tags">
          {action.requiresAdmin && <span className="tag admin"><LockKeyhole size={12} /> Admin</span>}
          {action.risk === "high" && <span className="tag danger"><AlertTriangle size={12} /> High impact</span>}
        </div>
      </div>
      <h3>{action.title}</h3>
      <p>{action.description}</p>
      <button className="run-button" onClick={() => onRun(action)} disabled={running}>
        <Play size={15} />
        {running ? "Running..." : "Run"}
      </button>
    </article>
  );
}
