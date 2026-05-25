import { AlertTriangle, CheckCircle2, HardDrive, RefreshCw } from "lucide-react";
import type { DiskHealth as DiskHealthType } from "../types";

interface Props {
  disks: DiskHealthType[];
  loading?: boolean;
  onRefresh?: () => void;
}

export default function DiskHealth({ disks, loading, onRefresh }: Props) {
  return (
    <div className="page-stack">
      <section className="content-section">
        <div className="section-heading">
          <div>
            <h2>Storage health</h2>
            <span>{loading ? "Reading disk health..." : `${disks.length} devices`}</span>
          </div>
          <button className="ghost-button" onClick={onRefresh} disabled={loading}><RefreshCw size={16} className={loading ? "spin" : ""} /> Refresh</button>
        </div>
        <p className="muted section-note">Detailed storage-health data is loaded on this page only, so Dashboard startup remains fast.</p>
        <div className="table-card">
          <table>
            <thead>
              <tr>
                <th>Drive</th>
                <th>Type</th>
                <th>Size</th>
                <th>Health</th>
                <th>Temperature</th>
                <th>Wear</th>
                <th>Errors</th>
              </tr>
            </thead>
            <tbody>
              {disks.map((disk) => {
                const healthy = disk.health_status?.toLowerCase() === "healthy";
                return (
                  <tr key={disk.friendly_name}>
                    <td><span className="table-title"><HardDrive size={16} />{disk.friendly_name}</span></td>
                    <td>{disk.media_type || "Unknown"}</td>
                    <td>{disk.size_gb} GB</td>
                    <td><span className={`health-pill ${healthy ? "ok" : "warn"}`}>{healthy ? <CheckCircle2 size={14} /> : <AlertTriangle size={14} />}{disk.health_status || "Unknown"}</span></td>
                    <td>{disk.temperature ?? "N/A"}</td>
                    <td>{disk.wear ?? "N/A"}</td>
                    <td>R {disk.read_errors ?? 0} / W {disk.write_errors ?? 0}</td>
                  </tr>
                );
              })}
              {!disks.length && <tr><td colSpan={7}>{loading ? "Reading disk data..." : "No disk data returned. Run as administrator or verify Storage module availability."}</td></tr>}
            </tbody>
          </table>
        </div>
      </section>
    </div>
  );
}
