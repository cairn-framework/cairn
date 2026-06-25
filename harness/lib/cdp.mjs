// Minimal Chrome DevTools Protocol client over Node's built-in WebSocket.
//
// Zero npm dependencies: drives the system Chrome/Chromium via --remote-debugging
// so the visual-eval harness needs no node_modules and no network at runtime.
// Node >= 22.4 exposes a global `WebSocket`, which this relies on.

import { spawn } from "node:child_process";
import { existsSync, mkdtempSync, readFileSync, rmSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { setTimeout as sleep } from "node:timers/promises";

const CHROME_CANDIDATES = [
  process.env.CHROME_PATH,
  "/Applications/Google Chrome.app/Contents/MacOS/Google Chrome",
  "/Applications/Chromium.app/Contents/MacOS/Chromium",
  "/Applications/Google Chrome Canary.app/Contents/MacOS/Google Chrome Canary",
  "/Applications/Microsoft Edge.app/Contents/MacOS/Microsoft Edge",
  "/usr/bin/google-chrome",
  "/usr/bin/chromium",
  "/usr/bin/chromium-browser",
].filter(Boolean);

/** Resolve a usable Chrome/Chromium executable or throw. */
export function findChrome() {
  for (const candidate of CHROME_CANDIDATES) {
    if (existsSync(candidate)) return candidate;
  }
  throw new Error(
    "no Chrome/Chromium executable found; set CHROME_PATH to a Chromium binary",
  );
}

/**
 * Launch headless Chrome with a throwaway profile and an OS-assigned debugging
 * port. Returns the browser-level websocket URL plus a teardown handle.
 */
export async function launchChrome({ chromePath } = {}) {
  const exe = chromePath || findChrome();
  const userDataDir = mkdtempSync(join(tmpdir(), "cairn-ux-chrome-"));
  const args = [
    "--headless=new",
    "--disable-gpu",
    "--no-first-run",
    "--no-default-browser-check",
    "--disable-extensions",
    "--disable-background-networking",
    "--disable-component-update",
    "--disable-default-apps",
    "--disable-sync",
    "--metrics-recording-only",
    "--mute-audio",
    "--hide-scrollbars",
    "--force-color-profile=srgb",
    "--font-render-hinting=none",
    "--disable-features=Translate,MediaRouter,OptimizationHints",
    `--user-data-dir=${userDataDir}`,
    "--remote-debugging-port=0",
    "about:blank",
  ];
  const proc = spawn(exe, args, { stdio: ["ignore", "ignore", "ignore"] });

  const portFile = join(userDataDir, "DevToolsActivePort");
  const deadline = Date.now() + 20000;
  let wsUrl = null;
  while (Date.now() < deadline) {
    if (proc.exitCode !== null) {
      throw new Error(`chrome exited early (code ${proc.exitCode})`);
    }
    if (existsSync(portFile)) {
      const contents = readFileSync(portFile, "utf8").split("\n");
      const port = contents[0]?.trim();
      const path = contents[1]?.trim();
      if (port && path) {
        wsUrl = `ws://127.0.0.1:${port}${path}`;
        break;
      }
    }
    await sleep(50);
  }
  if (!wsUrl) {
    proc.kill("SIGKILL");
    throw new Error("timed out waiting for chrome DevTools port");
  }

  const close = () => {
    try {
      proc.kill("SIGKILL");
    } catch {
      /* already gone */
    }
    try {
      rmSync(userDataDir, { recursive: true, force: true });
    } catch {
      /* best effort */
    }
  };
  return { proc, wsUrl, userDataDir, close };
}

/** Thin CDP transport: request/response correlation by id, plus event fan-out. */
export class CDP {
  constructor(ws) {
    this.ws = ws;
    this.nextId = 0;
    this.pending = new Map();
    this.listeners = new Map();
  }

  static connect(wsUrl) {
    return new Promise((resolve, reject) => {
      const ws = new WebSocket(wsUrl);
      const client = new CDP(ws);
      const onError = () => reject(new Error(`websocket error connecting to ${wsUrl}`));
      ws.addEventListener("open", () => {
        ws.removeEventListener("error", onError);
        resolve(client);
      });
      ws.addEventListener("error", onError);
      ws.addEventListener("message", (event) => client._onMessage(event.data));
      ws.addEventListener("close", () => {
        for (const { reject: rej } of client.pending.values()) {
          rej(new Error("websocket closed"));
        }
        client.pending.clear();
      });
    });
  }

  _onMessage(data) {
    let msg;
    try {
      msg = JSON.parse(data);
    } catch {
      return;
    }
    if (msg.id !== undefined && this.pending.has(msg.id)) {
      const { resolve, reject } = this.pending.get(msg.id);
      this.pending.delete(msg.id);
      if (msg.error) {
        reject(new Error(`CDP error: ${msg.error.message} (${JSON.stringify(msg.error)})`));
      } else {
        resolve(msg.result);
      }
      return;
    }
    if (msg.method) {
      const handlers = this.listeners.get(msg.method);
      if (handlers) {
        for (const fn of [...handlers]) fn(msg.params, msg.sessionId);
      }
    }
  }

  send(method, params = {}, sessionId) {
    const id = ++this.nextId;
    const payload = { id, method, params };
    if (sessionId) payload.sessionId = sessionId;
    return new Promise((resolve, reject) => {
      this.pending.set(id, { resolve, reject });
      this.ws.send(JSON.stringify(payload));
    });
  }

  on(method, fn) {
    if (!this.listeners.has(method)) this.listeners.set(method, new Set());
    this.listeners.get(method).add(fn);
    return () => this.listeners.get(method)?.delete(fn);
  }

  /** Resolve once `method` fires (optionally for `sessionId`) or reject on timeout. */
  waitFor(method, { sessionId, timeout = 10000 } = {}) {
    return new Promise((resolve, reject) => {
      const timer = setTimeout(() => {
        off();
        reject(new Error(`timed out waiting for ${method}`));
      }, timeout);
      const off = this.on(method, (params, sid) => {
        if (sessionId && sid !== sessionId) return;
        clearTimeout(timer);
        off();
        resolve(params);
      });
    });
  }

  close() {
    try {
      this.ws.close();
    } catch {
      /* ignore */
    }
  }
}

/** Create a fresh page target and attach a flattened session to it. */
export async function newPageSession(client) {
  const { targetId } = await client.send("Target.createTarget", { url: "about:blank" });
  const { sessionId } = await client.send("Target.attachToTarget", {
    targetId,
    flatten: true,
  });
  return { targetId, sessionId };
}
