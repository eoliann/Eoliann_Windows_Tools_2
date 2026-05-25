import { getIcon } from "./icons";

interface StatCardProps {
  icon: string;
  label: string;
  value: string;
  hint?: string;
}

export default function StatCard({ icon, label, value, hint }: StatCardProps) {
  const Icon = getIcon(icon);
  return (
    <div className="stat-card">
      <div className="stat-icon"><Icon size={22} /></div>
      <div>
        <span>{label}</span>
        <strong>{value}</strong>
        {hint && <small>{hint}</small>}
      </div>
    </div>
  );
}
