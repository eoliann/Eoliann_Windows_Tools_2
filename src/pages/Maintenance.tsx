import ActionGrid from "../components/ActionGrid";
import RestorePointsPanel from "../components/RestorePointsPanel";
import type { ToolAction } from "../types";
import { maintenanceActions } from "../data/actions";

interface Props {
  runningId?: string | null;
  onRun: (action: ToolAction) => void;
}

export default function Maintenance({ runningId, onRun }: Props) {
  return (
    <div className="page-stack">
      <RestorePointsPanel />
      <ActionGrid actions={maintenanceActions} runningId={runningId} onRun={onRun} />
    </div>
  );
}
