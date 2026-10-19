import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

const thisFile = fileURLToPath(import.meta.url);
const root = path.resolve(path.dirname(thisFile), "..");

const packageJsonPath = path.join(root, "package.json");
const packageJson = JSON.parse(fs.readFileSync(packageJsonPath, "utf8"));
const version = packageJson.version;

if (!version || typeof version !== "string") {
  throw new Error("package.json version is missing or invalid.");
}

const repoSlug = process.env.EWT_REPO_SLUG || "eoliann/Eoliann_Windows_Tools_2";
const channel = process.env.EWT_UPDATER_CHANNEL || "latest";
const notes = process.env.EWT_UPDATER_NOTES || `Release ${version}`;
const bundleRoot = path.join(root, "src-tauri", "target", "release", "bundle");
const nsisDir = path.join(bundleRoot, "nsis");

if (!fs.existsSync(nsisDir)) {
  throw new Error(`NSIS bundle directory not found: ${nsisDir}. Run \"npm run tauri build\" first.`);
}

const files = fs.readdirSync(nsisDir);
const setupFile = files.find((name) => name.toLowerCase().endsWith("-setup.exe") && name.includes(version));

if (!setupFile) {
  throw new Error(`No NSIS setup exe found for version ${version} in ${nsisDir}.`);
}

const sigPath = path.join(nsisDir, `${setupFile}.sig`);
if (!fs.existsSync(sigPath)) {
  throw new Error(`Signature file not found: ${sigPath}. Ensure updater signing is configured and rebuild.`);
}

const signature = fs.readFileSync(sigPath, "utf8").trim();
if (!signature) {
  throw new Error(`Signature file is empty: ${sigPath}`);
}

// GitHub replaces spaces in uploaded asset names with dots ("Eoliann Windows Tools_1.0.0_x64-setup.exe"
// becomes "Eoliann.Windows.Tools_1.0.0_x64-setup.exe"), so the URL must use the stored name.
const assetName = setupFile.replace(/ /g, ".");
const downloadUrl = `https://github.com/${repoSlug}/releases/download/${version}/${encodeURIComponent(assetName)}`;
const pubDate = new Date().toISOString();

const platformEntry = { signature, url: downloadUrl };
const latestJson = {
  version,
  notes,
  pub_date: pubDate,
  platforms: {
    "windows-x86_64-nsis": platformEntry,
    "windows-x86_64": platformEntry
  }
};

const outputPath = path.join(nsisDir, "latest.json");
fs.writeFileSync(outputPath, `${JSON.stringify(latestJson, null, 2)}\n`, "utf8");

console.log(`Generated ${outputPath}`);
console.log(`Upload to GitHub release (${channel} channel):`);
console.log(`- ${setupFile} (published by GitHub as ${assetName})`);
console.log(`- ${setupFile}.sig`);
console.log("- latest.json");
