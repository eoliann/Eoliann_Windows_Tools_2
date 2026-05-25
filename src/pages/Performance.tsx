import ActionGrid from "../components/ActionGrid";
import type { ToolAction } from "../types";
import { performanceActions } from "../data/actions";

interface Props {
  runningId?: string | null;
  onRun: (action: ToolAction) => void;
}

export default function Performance({ runningId, onRun }: Props) {
  return <ActionGrid actions={performanceActions} runningId={runningId} onRun={onRun} />;
}
