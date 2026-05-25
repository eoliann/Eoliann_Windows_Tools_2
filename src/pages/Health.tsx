import ActionGrid from "../components/ActionGrid";
import type { ToolAction } from "../types";
import { healthActions } from "../data/actions";

interface Props {
  runningId?: string | null;
  onRun: (action: ToolAction) => void;
}

export default function Health({ runningId, onRun }: Props) {
  return <ActionGrid actions={healthActions} runningId={runningId} onRun={onRun} />;
}
