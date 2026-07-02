#!/usr/bin/env node
// Records a ~30s webui screencast against examples/demo and converts it to
// docs/assets/demo/webui.mp4 + webui.gif. Committed so the media is
// regenerable; not run in CI.
//
// puppeteer is intentionally NOT a project dependency (this is a Rust
// crate). Install it in a scratch directory so `npm install` can never
// walk up and mutate an unrelated node_modules, then point NODE_PATH at
// it:
//
//   mkdir -p /tmp/cairn-webui-rec && cd /tmp/cairn-webui-rec
//   npm init -y >/dev/null && npm install puppeteer
//   cd /path/to/cairn
//   NODE_PATH=/tmp/cairn-webui-rec/node_modules node scripts/record-webui-demo.mjs
//
// Requires a release build (`cargo build --release`) and `ffmpeg` on PATH.
// Override CAIRN_BIN_DIR if the binary lives somewhere other than
// target/release.

import { createRequire } from "node:module";
import { spawn, execFileSync } from "node:child_process";
import { setTimeout as sleep } from "node:timers/promises";
import path from "node:path";
import { fileURLToPath } from "node:url";

const require = createRequire(import.meta.url);
const puppeteer = require("puppeteer");

const REPO_ROOT = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "..");
const CAIRN_BIN = path.join(process.env.CAIRN_BIN_DIR || path.join(REPO_ROOT, "target", "release"), "cairn");
const DEMO_DIR = path.join(REPO_ROOT, "examples", "demo");
const OUT_DIR = path.join(REPO_ROOT, "docs", "assets", "demo");
const PORT = 4321;

async function waitForServer(url, timeoutMs = 8000) {
  const start = Date.now();
  while (Date.now() - start < timeoutMs) {
    try {
      const res = await fetch(url);
      if (res.ok) return;
    } catch {
      // not up yet, keep polling
    }
    await sleep(200);
  }
  throw new Error(`server did not come up at ${url} within ${timeoutMs}ms`);
}

async function record() {
  const server = spawn(CAIRN_BIN, ["ui", "--port", String(PORT), "--no-open"], {
    cwd: DEMO_DIR,
    stdio: "ignore",
  });

  try {
    await waitForServer(`http://127.0.0.1:${PORT}/api/meta`);

    const browser = await puppeteer.launch({
      headless: true,
      defaultViewport: { width: 1280, height: 800 },
    });
    const page = await browser.newPage();
    await page.goto(`http://127.0.0.1:${PORT}`, { waitUntil: "networkidle0" });
    await page.waitForSelector(".canvas-node", { timeout: 8000 }).catch(() => {});
    await sleep(800);

    const webmPath = path.join(OUT_DIR, "webui.webm");
    const recorder = await page.screencast({ path: webmPath });

    // 1. Graph is loaded; hold a beat so the viewer orients.
    await sleep(3500);

    // 2. Click a content-rich node.
    await page.evaluate(() => {
      const nodes = Array.from(document.querySelectorAll(".canvas-node"));
      const target = nodes.find((n) => n.textContent.includes("api")) || nodes[0];
      target?.dispatchEvent(new MouseEvent("click", { bubbles: true }));
    });
    await sleep(4000);

    // 3. Scroll the inspector to show more of the node detail.
    await page.evaluate(() => {
      document.querySelector(".inspector-wrap")?.scrollTo({ top: 300, behavior: "smooth" });
    });
    await sleep(3500);

    // 4. Open the command palette and type "report" to show the Report an
    // issue action filtering live, then close it.
    await page.keyboard.down("Meta");
    await page.keyboard.press("KeyK");
    await page.keyboard.up("Meta");
    await sleep(800);
    await page.keyboard.type("report", { delay: 90 });
    await sleep(1800);
    await page.keyboard.press("Escape");
    await sleep(800);

    // 5. Open the blueprint modal.
    await page.evaluate(() => {
      const btn = Array.from(document.querySelectorAll(".blueprint-trigger")).find((b) => /blueprint/i.test(b.textContent));
      btn?.dispatchEvent(new MouseEvent("click", { bubbles: true }));
    });
    await sleep(4000);

    await recorder.stop();
    await browser.close();
  } finally {
    server.kill();
  }
}

function convert() {
  const webm = path.join(OUT_DIR, "webui.webm");
  const mp4 = path.join(OUT_DIR, "webui.mp4");
  const gif = path.join(OUT_DIR, "webui.gif");
  const palette = path.join(OUT_DIR, "webui-palette.png");

  console.log("Converting webm -> mp4/gif via ffmpeg...");
  execFileSync("ffmpeg", ["-y", "-i", webm, "-vf", "scale=1200:-1", "-movflags", "+faststart", mp4], { stdio: "inherit" });
  execFileSync("ffmpeg", ["-y", "-i", webm, "-vf", "fps=12,scale=900:-1:flags=lanczos,palettegen", palette], {
    stdio: "inherit",
  });
  execFileSync("ffmpeg", ["-y", "-i", webm, "-i", palette, "-lavfi", "fps=12,scale=900:-1:flags=lanczos[x];[x][1:v]paletteuse", gif], { stdio: "inherit" });

  execFileSync("rm", [webm, palette]);
  console.log(`Wrote ${mp4} and ${gif}`);
}

async function main() {
  await record();
  convert();
}

main().catch((err) => {
  console.error(err);
  process.exitCode = 1;
});
