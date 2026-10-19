import { useEffect, useState } from "react";
import ActionGrid from "../components/ActionGrid";
import type { ToolAction } from "../types";
import { networkActions } from "../data/actions";
import { getSmbGuestStatus } from "../lib/commands";
import type { SmbGuestStatus } from "../types";

interface Props {
  runningId?: string | null;
  onRun: (action: ToolAction) => void;
}

export default function Network({ runningId, onRun }: Props) {
  const [statusText, setStatusText] = useState("Status: checking...");
  const [status, setStatus] = useState<SmbGuestStatus | null>(null);

  const statusClass = (ok: boolean | null) => ok === null ? "status-badge neutral" : ok ? "status-badge ok" : "status-badge bad";
  const statusLabel = (ok: boolean | null, on = "Enabled", off = "Disabled") => ok === null ? "Unknown" : ok ? on : off;

  useEffect(() => {
    let mounted = true;

    const loadStatus = async () => {
      try {
        const status = await getSmbGuestStatus();
        if (!mounted) return;
        const enabled = status.enable_insecure_guest_logons === true;
        const requireSignature = status.require_security_signature;
        const policy = status.policy_allow_insecure_guest_auth;
        const legacy = status.legacy_allow_insecure_guest_auth;
        setStatus(status);
        setStatusText(
          `Status: ${enabled ? "enabled" : "disabled"} | EnableInsecureGuestLogons=${String(status.enable_insecure_guest_logons)} | RequireSecuritySignature=${String(requireSignature)} | Policy=${String(policy)} | Legacy=${String(legacy)} | SMB1=${String(status.smb1_enabled)}`
        );
      } catch {
        if (!mounted) return;
        setStatus(null);
        setStatusText("Status: unavailable (failed to read SMB configuration)");
      }
    };

    void loadStatus();

    return () => {
      mounted = false;
    };
  }, []);

  useEffect(() => {
    if (!runningId) {
      void getSmbGuestStatus()
        .then((status) => {
          const enabled = status.enable_insecure_guest_logons === true;
          setStatus(status);
          setStatusText(
            `Status: ${enabled ? "enabled" : "disabled"} | EnableInsecureGuestLogons=${String(status.enable_insecure_guest_logons)} | RequireSecuritySignature=${String(status.require_security_signature)} | Policy=${String(status.policy_allow_insecure_guest_auth)} | Legacy=${String(status.legacy_allow_insecure_guest_auth)} | SMB1=${String(status.smb1_enabled)}`
          );
        })
        .catch(() => {
          setStatus(null);
          setStatusText("Status: unavailable (failed to read SMB configuration)");
        });
    }
  }, [runningId]);

  return (
    <div className="page-stack">
      <ActionGrid actions={networkActions} runningId={runningId} onRun={onRun} />
      <section className="content-section prose-card">
        <h2>SMB guest status</h2>
        {status ? (
          <div className="status-grid">
            <div className="status-row"><span>Guest Logons</span><span className={statusClass(status.enable_insecure_guest_logons ?? null)}>{statusLabel(status.enable_insecure_guest_logons ?? null)}</span></div>
            <div className="status-row"><span>SMB Signing Required</span><span className={statusClass(status.require_security_signature === false ? true : status.require_security_signature === true ? false : null)}>{statusLabel(status.require_security_signature === false ? true : status.require_security_signature === true ? false : null, "No", "Yes")}</span></div>
            <div className="status-row"><span>Policy AllowInsecureGuestAuth</span><span className={statusClass(status.policy_allow_insecure_guest_auth === 1 ? true : status.policy_allow_insecure_guest_auth === 0 ? false : null)}>{status.policy_allow_insecure_guest_auth ?? "Missing"}</span></div>
            <div className="status-row"><span>Legacy AllowInsecureGuestAuth</span><span className={statusClass(status.legacy_allow_insecure_guest_auth === 1 ? true : status.legacy_allow_insecure_guest_auth === 0 ? false : null)}>{status.legacy_allow_insecure_guest_auth ?? "Missing"}</span></div>
            <div className="status-row"><span>SMB1 Optional Feature</span><span className={statusClass(status.smb1_enabled ?? null)}>{statusLabel(status.smb1_enabled ?? null)}</span></div>
          </div>
        ) : null}
        <p>{statusText}</p>
      </section>
    </div>
  );
}
