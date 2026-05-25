import ActionGrid from "../components/ActionGrid";
import type { ToolAction } from "../types";
import { networkActions } from "../data/actions";

interface Props {
  runningId?: string | null;
  onRun: (action: ToolAction) => void;
}

export default function Network({ runningId, onRun }: Props) {
  return <ActionGrid actions={networkActions} runningId={runningId} onRun={onRun} />;
}
