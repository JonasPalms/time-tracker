import { execSync } from "node:child_process";
import { copyFileSync, mkdirSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

const repoRoot = join(dirname(fileURLToPath(import.meta.url)), "..");
const target =
  process.env.TAURI_ENV_TARGET_TRIPLE?.trim() ||
  execSync("rustc --print host-tuple", { encoding: "utf8" }).trim();
const debug = process.env.TAURI_ENV_DEBUG === "true";
const profile = debug ? "debug" : "release";

const cargoArgs = [
  "build",
  "--manifest-path",
  "src-tauri/mcp-server/Cargo.toml",
  "--bin",
  "timetracker-mcp",
];

if (!debug) {
  cargoArgs.push("--release");
}

if (process.env.TAURI_ENV_TARGET_TRIPLE) {
  cargoArgs.push("--target", target);
}

execSync(`cargo ${cargoArgs.join(" ")}`, { cwd: repoRoot, stdio: "inherit" });

const metadata = JSON.parse(
  execSync(
    "cargo metadata --manifest-path src-tauri/mcp-server/Cargo.toml --format-version 1 --no-deps",
    {
      cwd: repoRoot,
      encoding: "utf8",
    }
  )
);
const artifact = process.env.TAURI_ENV_TARGET_TRIPLE
  ? join(metadata.target_directory, target, profile, "timetracker-mcp")
  : join(metadata.target_directory, profile, "timetracker-mcp");

const binariesDir = join(repoRoot, "src-tauri/binaries");
mkdirSync(binariesDir, { recursive: true });

const dest = join(binariesDir, `timetracker-mcp-${target}`);
copyFileSync(artifact, dest);
copyFileSync(artifact, join(binariesDir, "timetracker-mcp"));
console.error(`Wrote ${dest}`);
