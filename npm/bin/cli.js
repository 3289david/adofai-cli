#!/usr/bin/env node

const { execFileSync } = require("child_process");
const path = require("path");
const os = require("os");
const fs = require("fs");

function getBinaryPath() {
  const platform = os.platform();
  const arch = os.arch();

  let binaryName = "adofai";
  if (platform === "win32") binaryName += ".exe";

  const localBinary = path.join(__dirname, "..", "bin", binaryName);
  if (fs.existsSync(localBinary)) {
    return localBinary;
  }

  const installDir = path.join(os.homedir(), ".adofai-cli", "bin");
  const installedBinary = path.join(installDir, binaryName);
  if (fs.existsSync(installedBinary)) {
    return installedBinary;
  }

  console.error("adofai binary not found. Run: npm run postinstall");
  console.error("Or install via Homebrew: brew install danwoo/tap/adofai-cli");
  process.exit(1);
}

const binary = getBinaryPath();
const args = process.argv.slice(2);

try {
  execFileSync(binary, args, { stdio: "inherit" });
} catch (error) {
  process.exit(error.status || 1);
}
