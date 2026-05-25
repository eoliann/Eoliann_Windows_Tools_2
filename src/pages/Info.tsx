import ActionGrid from "../components/ActionGrid";
import type { ToolAction } from "../types";
import { infoActions } from "../data/actions";

interface Props {
  runningId?: string | null;
  onRun: (action: ToolAction) => void;
}

export default function Info({ runningId, onRun }: Props) {
  return <ActionGrid actions={infoActions} runningId={runningId} onRun={onRun} />;
}
