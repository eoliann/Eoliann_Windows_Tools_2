import { RefreshCw, Trash2 } from "lucide-react";
import { useCallback, useEffect, useState } from "react";
import { deleteRestorePointByInfo, getRestorePoints } from "../lib/commands";
import type { RestorePointInfo } from "../types";

export default function RestorePointsPanel() {
  const [loading, setLoading] = useState(false);
  const [deleting, setDeleting] = useState(false);
  const [usedGb, setUsedGb] = useState(0);
  const [points, setPoints] = useState<RestorePointInfo[]>([]);
  const [selectedIndex, setSelectedIndex] = useState<number>(-1);
  const [error, setError] = useState<string | null>(null);

  const refresh = useCallback(async () => {
    setLoading(true);
    setError(null);
    try {
      const summary = await getRestorePoints();
      setUsedGb(Number.isFinite(summary.total_used_space_gb) ? summary.total_used_space_gb : 0);
      setPoints(summary.points ?? []);
      setSelectedIndex(-1);
    } catch (e) {
      setError(String(e));
    } finally {
      setLoading(false);
    }
  }, []);

  useEffect(() => {
    void refresh();
  }, [refresh]);

  const removeSelected = async () => {
    if (selectedIndex < 0 || selectedIndex >= points.length) return;
    setDeleting(true);
    setError(null);
    try {
      const selectedPoint = points[selectedIndex];
      await deleteRestorePointByInfo(selectedPoint.shadow_id ?? "", selectedPoint.sequence_number ?? 0);
      await refresh();
    } catch (e) {
      setError(String(e));
    } finally {
      setDeleting(false);
    }
  };

  return (
    <section className="content-section">
      <div className="section-heading">
        <h2>System restore</h2>
        <span>Refresh and delete selected</span>
      </div>
      <p className="muted">Total used space: {usedGb.toFixed(2)} GB</p>
      <p className="muted">Available restore points: {points.length}</p>
      <div className="restore-controls">
        <select value={selectedIndex} onChange={(e) => setSelectedIndex(Number(e.target.value))} disabled={loading || deleting || !points.length}>
          <option value={-1}>Select restore point</option>
          {points.map((point, index) => (
            <option key={`${point.shadow_id}-${point.sequence_number}-${index}`} value={index}>
              {point.creation_time} - {point.description}
            </option>
          ))}
        </select>
        <button className="ghost-button" onClick={() => void refresh()} disabled={loading || deleting}>
          <RefreshCw size={16} className={loading ? "spin" : ""} /> Refresh
        </button>
        <button className="danger-button" onClick={() => void removeSelected()} disabled={loading || deleting || selectedIndex < 0 || selectedIndex >= points.length}>
          <Trash2 size={16} /> Delete selected
        </button>
      </div>
      {error && <p className="notice warn">Restore points error: {error}</p>}
    </section>
  );
}
