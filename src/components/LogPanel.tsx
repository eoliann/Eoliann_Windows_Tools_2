import { Clipboard, Terminal, Trash2 } from "lucide-react";

interface LogPanelProps {
  output: string;
  onClear: () => void;
}

export default function LogPanel({ output, onClear }: LogPanelProps) {
  const copy = async () => {
    await navigator.clipboard?.writeText(output);
  };

  return (
    <section className="log-panel">
      <div className="panel-title-row">
        <div className="panel-title"><Terminal size={18} /> Output</div>
        <div className="panel-actions">
          <button className="ghost-button" onClick={copy} disabled={!output}><Clipboard size={15} /> Copy</button>
          <button className="ghost-button" onClick={onClear} disabled={!output}><Trash2 size={15} /> Clear</button>
        </div>
      </div>
      <pre>{output || "No command output yet."}</pre>
    </section>
  );
}
