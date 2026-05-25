import ActionGrid from "../components/ActionGrid";
import type { ToolAction } from "../types";
import { customizeActions } from "../data/actions";

interface Props {
  runningId?: string | null;
  onRun: (action: ToolAction) => void;
}

export default function Customize({ runningId, onRun }: Props) {
  return <ActionGrid actions={customizeActions} runningId={runningId} onRun={onRun} />;
}
