import { AlertTriangle, X } from "lucide-react";
import type { ToolAction } from "../types";

interface ConfirmDialogProps {
  action: ToolAction | null;
  onCancel: () => void;
  onConfirm: () => void;
}

export default function ConfirmDialog({ action, onCancel, onConfirm }: ConfirmDialogProps) {
  if (!action) return null;
  return (
    <div className="modal-backdrop" role="presentation">
      <div className="confirm-modal" role="dialog" aria-modal="true" aria-label="Confirm action">
        <button className="modal-close" onClick={onCancel}><X size={18} /></button>
        <div className="modal-icon"><AlertTriangle size={26} /></div>
        <h2>Run “{action.title}”?</h2>
        <p>{action.description}</p>
        {action.requiresAdmin && <p className="warning-line">This action usually requires administrator rights.</p>}
        {action.risk === "high" && <p className="danger-line">This is a high-impact action. Create a restore point first.</p>}
        <div className="modal-actions">
          <button className="ghost-button" onClick={onCancel}>Cancel</button>
          <button className="danger-button" onClick={onConfirm}>Run action</button>
        </div>
      </div>
    </div>
  );
}
