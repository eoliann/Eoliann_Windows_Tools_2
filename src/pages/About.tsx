import { ExternalLink, Gift, Github, HeartHandshake, Info, LockKeyhole, ScrollText, ShieldAlert } from "lucide-react";
import { useState } from "react";
import { openExternalUrl } from "../lib/commands";
import { checkForAppUpdate, downloadAndInstallUpdate, releasesUrl as latestReleaseUrl } from "../lib/updater";

const repoUrl = "https://github.com/eoliann/Eoliann_Windows_Tools_2";
const releasesUrl = "https://github.com/eoliann/Eoliann_Windows_Tools_2/releases";
const supportUrl = "https://github.com/eoliann/Eoliann_Windows_Tools_2#-support";
const revolutUrl = "https://revolut.me/adriannm9";
const paypalUrl = "https://www.paypal.com/donate/?hosted_button_id=PTH2EXUDS423S";
const kofiUrl = "https://ko-fi.com/eoliann";
const telegramUrl = "https://t.me/eoliannwindowstool";
interface AboutProps {
  appVersion?: string;
}

export default function About({ appVersion }: AboutProps) {
  const [updateStatus, setUpdateStatus] = useState("");
  const [checkingUpdate, setCheckingUpdate] = useState(false);
  const [installingUpdate, setInstallingUpdate] = useState(false);
  const [updateFailed, setUpdateFailed] = useState(false);

  const open = (url: string) => {
    void openExternalUrl(url);
  };

  const checkUpdate = async () => {
    setCheckingUpdate(true);
    setUpdateFailed(false);
    setUpdateStatus("Checking for updates...");
    try {
      const result = await checkForAppUpdate();
      if (!result.available) {
        setUpdateStatus(`No updates available. Current version: ${result.currentVersion}`);
        return;
      }
      setUpdateStatus(`Update available: ${result.currentVersion} -> ${result.latestVersion}. Click Install update.`);
    } catch (error) {
      setUpdateFailed(true);
      setUpdateStatus(`Update check failed: ${error instanceof Error ? error.message : String(error)}`);
    } finally {
      setCheckingUpdate(false);
    }
  };

  const installUpdate = async () => {
    setInstallingUpdate(true);
    setUpdateFailed(false);
    setUpdateStatus("Downloading and installing update...");
    try {
      await downloadAndInstallUpdate(setUpdateStatus);
    } catch (error) {
      setUpdateFailed(true);
      setUpdateStatus(`Update install failed: ${error instanceof Error ? error.message : String(error)}`);
      setInstallingUpdate(false);
    }
  };

  return (
    <div className="page-stack">
      <section className="hero-card about-hero">
        <Info size={44} />
        <div>
          <p className="eyebrow">About</p>
          <h2>Eoliann Windows Tools 2 {appVersion ? `v${appVersion}` : ""}</h2>
          <p>Developer: Eoliann. Windows 11 toolkit for practical administration, cleanup, diagnostics and tweaks.</p>
        </div>
      </section>

      <section className="content-section prose-card">
        <h2>Project</h2>
        <p>
          A quick toolkit for Windows 11, built with a local-first architecture and focused on useful administration commands,
          install flows, diagnostics and quality-of-life tweaks.
        </p>
        <div className="links-grid">
          <button className="ghost-button" onClick={() => open(repoUrl)}><Github size={18} /> GitHub repository <ExternalLink size={14} /></button>
          <button className="ghost-button" onClick={() => open(releasesUrl)}><ExternalLink size={18} /> Releases <ExternalLink size={14} /></button>
          <button className="ghost-button" onClick={() => open(telegramUrl)}><ExternalLink size={18} /> Telegram group <ExternalLink size={14} /></button>
        </div>
      </section>

      <section className="content-section prose-card">
        <h2>Updates</h2>
        <p>Checks GitHub Releases and installs signed updates directly in the app.</p>
        <div className="links-grid">
          <button className="ghost-button" onClick={() => void checkUpdate()} disabled={checkingUpdate || installingUpdate}>
            {checkingUpdate ? "Checking..." : "Check for updates"}
          </button>
          <button className="primary-button" onClick={() => void installUpdate()} disabled={checkingUpdate || installingUpdate}>
            {installingUpdate ? "Installing..." : "Install update"}
          </button>
        </div>
        {updateStatus ? <p className={updateFailed ? "notice warn" : undefined}>{updateStatus}</p> : null}
        {updateFailed ? (
          <button className="ghost-button" onClick={() => open(latestReleaseUrl)}>
            <ExternalLink size={16} /> Download the latest version manually
          </button>
        ) : null}
      </section>

      <section className="legal-grid">
        <article className="legal-card">
          <h3><ScrollText size={18} /> Terms & Conditions</h3>
          <p>By using Eoliann Windows Tools, you agree that this software can execute system-level actions on your local Windows device.</p>
          <ul>
            <li>Some actions can change registry, services, boot behavior, policies and installed packages.</li>
            <li>You are responsible for reviewing actions before running them.</li>
            <li>Create restore points and backups before high-impact operations.</li>
            <li>Third-party apps installed via winget follow their own licenses and terms.</li>
            <li>The project is provided as-is; behavior may vary by Windows edition/build.</li>
          </ul>
        </article>

        <article className="legal-card">
          <h3><LockKeyhole size={18} /> Privacy Policy</h3>
          <p>EWT is local-first. It does not require account login and does not send built-in telemetry from the app UI/backend itself.</p>
          <ul>
            <li>System information and command output are read/displayed locally.</li>
            <li>Logs are kept in the local app session unless you export/share them manually.</li>
            <li>External tools/services (winget, Microsoft endpoints, third-party utilities) can contact their own servers when used.</li>
            <li>Generated files (like battery reports) are saved only to paths you choose.</li>
          </ul>
        </article>

        <article className="donate-card">
          <h3><HeartHandshake size={18} /> Donate</h3>
          <p>Support the project through the official channels from the repository.</p>
          <div className="page-stack">
            <button className="primary-button" onClick={() => open(revolutUrl)}><Gift size={16} /> Donate via Revolut</button>
            <button className="ghost-button" onClick={() => open(paypalUrl)}><Gift size={16} /> Donate via PayPal</button>
            <button className="ghost-button" onClick={() => open(kofiUrl)}><Gift size={16} /> Donate via Ko-fi</button>
            <button className="ghost-button" onClick={() => open(supportUrl)}><ExternalLink size={16} /> Open Support section</button>
          </div>
        </article>
      </section>

      <section className="content-section prose-card">
        <h2><ShieldAlert size={20} /> Community</h2>
        <p>
          Join the official Telegram group for updates, feedback and support:
        </p>
        <ul>
          <li><button className="ghost-button" onClick={() => open(telegramUrl)}>Telegram group <ExternalLink size={14} /></button></li>
        </ul>
      </section>
    </div>
  );
}
