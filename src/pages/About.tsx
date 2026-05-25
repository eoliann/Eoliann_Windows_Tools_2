import { ExternalLink, Gift, Github, HeartHandshake, Info, LockKeyhole, ScrollText, ShieldAlert } from "lucide-react";
import { openExternalUrl } from "../lib/commands";

const repoUrl = "https://github.com/eoliann/Eoliann_Windows_Tools";
const releasesUrl = "https://github.com/eoliann/Eoliann_Windows_Tools/releases";
const supportUrl = "https://github.com/eoliann/Eoliann_Windows_Tools#-support";
const revolutUrl = "https://revolut.me/adriannm9";
const paypalUrl = "https://www.paypal.com/donate/?hosted_button_id=PTH2EXUDS423S";
const kofiUrl = "https://ko-fi.com/eoliann";
const telegramUrl = "https://t.me/eoliannwindowstool";
interface AboutProps {
  appVersion?: string;
}

export default function About({ appVersion }: AboutProps) {
  const open = (url: string) => {
    void openExternalUrl(url);
  };

  return (
    <div className="page-stack">
      <section className="hero-card about-hero">
        <Info size={44} />
        <div>
          <p className="eyebrow">About</p>
          <h2>Eoliann Windows Tools {appVersion ? `v${appVersion}` : ""}</h2>
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
