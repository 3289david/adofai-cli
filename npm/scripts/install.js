#!/usr/bin/env node

const https = require("https");
const fs = require("fs");
const path = require("path");
const os = require("os");
const { execSync } = require("child_process");

const REPO = "3289david/adofai-cli";
const VERSION = "v1.0.0";

function getPlatformTarget() {
  const platform = os.platform();
  const arch = os.arch();

  const targets = {
    "darwin-arm64": "aarch64-apple-darwin",
    "darwin-x64": "x86_64-apple-darwin",
    "linux-x64": "x86_64-unknown-linux-gnu",
    "linux-arm64": "aarch64-unknown-linux-gnu",
    "win32-x64": "x86_64-pc-windows-msvc",
  };

  const key = `${platform}-${arch}`;
  return targets[key] || null;
}

function downloadBinary(url, dest) {
  return new Promise((resolve, reject) => {
    const file = fs.createWriteStream(dest);
    https.get(url, (response) => {
      if (response.statusCode === 302 || response.statusCode === 301) {
        downloadBinary(response.headers.location, dest).then(resolve).catch(reject);
        return;
      }
      response.pipe(file);
      file.on("finish", () => {
        file.close();
        resolve();
      });
    }).on("error", reject);
  });
}

async function install() {
  const target = getPlatformTarget();
  if (!target) {
    console.log("Pre-built binary not available for your platform.");
    console.log("Please build from source: cargo install adofai-cli");
    return;
  }

  const binaryName = os.platform() === "win32" ? "adofai.exe" : "adofai";
  const assetName = `adofai-cli-${target}${os.platform() === "win32" ? ".exe" : ""}`;
  const url = `https://github.com/${REPO}/releases/download/${VERSION}/${assetName}`;

  const installDir = path.join(os.homedir(), ".adofai-cli", "bin");
  fs.mkdirSync(installDir, { recursive: true });

  const dest = path.join(installDir, binaryName);

  console.log(`Downloading adofai-cli for ${target}...`);

  try {
    await downloadBinary(url, dest);
    fs.chmodSync(dest, 0o755);
    console.log(`Installed to ${dest}`);
    console.log("Run 'adofai --help' to get started!");
  } catch (err) {
    console.log("Failed to download binary. Trying cargo install...");
    try {
      execSync("cargo install adofai-cli", { stdio: "inherit" });
    } catch {
      console.error("Installation failed. Please install Rust and run: cargo install adofai-cli");
    }
  }
}

install();
