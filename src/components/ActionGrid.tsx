import type { ToolAction } from "../types";
import ActionCard from "./ActionCard";

interface ActionGridProps {
  actions: ToolAction[];
  runningId?: string | null;
  onRun: (action: ToolAction) => void;
}

export default function ActionGrid({ actions, runningId, onRun }: ActionGridProps) {
  const groups = actions.reduce<Record<string, ToolAction[]>>((acc, action) => {
    acc[action.category] = acc[action.category] ?? [];
    acc[action.category].push(action);
    return acc;
  }, {});

  return (
    <div className="action-sections">
      {Object.entries(groups).map(([category, items]) => (
        <section className="content-section" key={category}>
          <div className="section-heading">
            <h2>{category}</h2>
            <span>{items.length} actions</span>
          </div>
          <div className="action-grid">
            {items.map((action) => (
              <ActionCard key={action.id} action={action} running={runningId === action.id} onRun={onRun} />
            ))}
          </div>
        </section>
      ))}
    </div>
  );
}
