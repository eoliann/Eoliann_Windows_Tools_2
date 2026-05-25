import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

const thisFile = fileURLToPath(import.meta.url);
const root = path.resolve(path.dirname(thisFile), "..");

const packageJsonPath = path.join(root, "package.json");
const tauriConfPath = path.join(root, "src-tauri", "tauri.conf.json");
const cargoTomlPath = path.join(root, "src-tauri", "Cargo.toml");

const packageJson = JSON.parse(fs.readFileSync(packageJsonPath, "utf8"));
const version = packageJson.version;

if (!version || typeof version !== "string") {
  throw new Error("package.json version is missing or invalid.");
}

const tauriConf = JSON.parse(fs.readFileSync(tauriConfPath, "utf8"));
tauriConf.version = version;
fs.writeFileSync(tauriConfPath, `${JSON.stringify(tauriConf, null, 2)}\n`, "utf8");

const cargoToml = fs.readFileSync(cargoTomlPath, "utf8");
const updatedCargoToml = cargoToml.replace(
  /^version\s*=\s*"[^"]+"/m,
  `version = "${version}"`
);

fs.writeFileSync(cargoTomlPath, updatedCargoToml, "utf8");
console.log(`Synced version ${version} to tauri.conf.json and Cargo.toml`);
