import ActionGrid from "../components/ActionGrid";
import type { ToolAction } from "../types";
import { tweakActions } from "../data/actions";

interface Props {
  runningId?: string | null;
  onRun: (action: ToolAction) => void;
}

export default function Tweaks({ runningId, onRun }: Props) {
  return <ActionGrid actions={tweakActions} runningId={runningId} onRun={onRun} />;
}
