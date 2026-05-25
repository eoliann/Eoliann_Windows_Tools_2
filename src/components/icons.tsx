import * as Icons from "lucide-react";
import type { LucideIcon } from "lucide-react";

const fallback = Icons.CircleDot;

export function getIcon(name: string): LucideIcon {
  const map = Icons as unknown as Record<string, LucideIcon>;
  return map[name] ?? fallback;
}
