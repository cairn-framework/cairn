/**
 * The guided-console prototype: a long-running service the maintainer runs
 * on their own server, drives with their own harness, and points at one
 * target project.
 *
 * Scope is the creation journey and nothing else (see
 * `meta/todos/todo.guided-console-prototype.md`). Design authority is
 * `docs/design-system/`, served straight from the repo so the tokens cannot
 * drift from a copy. `src/ui_assets/` is untouched: the production console
 * belongs to `todo.console-signed-widening`.
 */
import { createReadStream } from "node:fs";
import { stat } from "node:fs/promises";
import { createServer } from "node:http";
import { dirname, join, resolve, sep } from "node:path";
import { fileURLToPath } from "node:url";
import { cairnVersion } from "./lib/cairn.mjs";
import { harnessSpec } from "./lib/harness.mjs";
import { Journey } from "./lib/journey.mjs";

const here = dirname(fileURLToPath(import.meta.url));
const PORT = Number(process.env.PORT ?? 4400);
const DATA_DIR = resolve(process.env.CAIRN_PROTOTYPE_DATA ?? join(here, ".data"));
const DESIGN_SYSTEM = resolve(process.env.CAIRN_PROTOTYPE_DESIGN_SYSTEM ?? join(here, "..", "..", "docs", "design-system"));
const UI_DIR = join(here, "ui");

const MIME = {
  ".html": "text/html; charset=utf-8",
  ".css": "text/css; charset=utf-8",
  ".js": "text/javascript; charset=utf-8",
  ".mjs": "text/javascript; charset=utf-8",
  ".svg": "image/svg+xml",
  ".woff2": "font/woff2",
  ".json": "application/json; charset=utf-8",
};

const journey = await new Journey(DATA_DIR).open();

/** What the console is driving, for the working disclosure and the boot log. */
let wiring = null;
async function readWiring() {
  if (wiring) return wiring;
  const { command, args } = harnessSpec();
  let cairn = "not found";
  try {
    cairn = await cairnVersion();
  } catch (cause) {
    cairn = `not found (${cause.message})`;
  }
  // `cairn` is for the boot log only; the console never renders it, so it does
  // not travel on every frame.
  wiring = { shown: { harness: [command, ...args].join(" "), project: journey.projectDir }, cairn };
  return wiring;
}

function sendJson(res, status, body) {
  const payload = JSON.stringify(body);
  res.writeHead(status, { "content-type": "application/json; charset=utf-8", "content-length": Buffer.byteLength(payload) });
  res.end(payload);
}

async function sendFile(res, root, relative) {
  // Resolve first, then require the result to sit under the root with a
  // separator between them: a bare prefix test would also accept a sibling
  // directory whose name merely starts with the root's.
  const full = resolve(root, relative);
  if (full !== root && !full.startsWith(root + sep)) return sendJson(res, 403, { error: "outside the served root" });
  try {
    const info = await stat(full);
    if (!info.isFile()) throw new Error("not a file");
    const ext = full.slice(full.lastIndexOf("."));
    res.writeHead(200, { "content-type": MIME[ext] ?? "application/octet-stream", "cache-control": "no-store" });
    createReadStream(full).pipe(res);
  } catch {
    sendJson(res, 404, { error: `no such file: ${relative}` });
  }
  return undefined;
}

async function readBody(req) {
  const chunks = [];
  for await (const chunk of req) chunks.push(chunk);
  if (chunks.length === 0) return {};
  try {
    return JSON.parse(Buffer.concat(chunks).toString("utf8"));
  } catch {
    throw Object.assign(new Error("body was not JSON"), { status: 400 });
  }
}

async function fullState() {
  return { ...(await journey.snapshot()), wiring: (await readWiring()).shown };
}

/* ---- the event stream: one push per journey change, coalesced ---------- */

const listeners = new Set();
let pushTimer = null;
let pushing = false;
let pushAgain = false;

// Building a frame runs cairn subprocesses. If one is slow while the harness
// is working the project, overlapping pushes would pile up behind it, so only
// one is ever in flight and a change during it collapses into a single retry.
async function pushNow() {
  pushTimer = null;
  if (pushing) {
    pushAgain = true;
    return;
  }
  pushing = true;
  try {
    if (listeners.size === 0) return;
    const frame = `data: ${JSON.stringify(await fullState())}\n\n`;
    for (const res of listeners) {
      // One dead socket must not stop the frame reaching the others.
      try {
        res.write(frame);
      } catch {
        listeners.delete(res);
      }
    }
  } finally {
    pushing = false;
    if (pushAgain) {
      pushAgain = false;
      schedulePush();
    }
  }
}

function schedulePush() {
  if (pushTimer) return;
  pushTimer = setTimeout(() => {
    pushNow().catch(() => {});
  }, 120);
}

journey.on("change", schedulePush);

async function subscribe(req, res) {
  res.writeHead(200, {
    "content-type": "text/event-stream; charset=utf-8",
    "cache-control": "no-store",
    connection: "keep-alive",
  });
  // A write to a socket the client has abandoned reports asynchronously, as an
  // error event rather than a throw. Unhandled, that ends the whole service,
  // which for a console meant to outlive the browser is the wrong death.
  // `beat` is declared before the handler that clears it, because the client
  // can abandon the request during the await below, while cairn is still being
  // read, and the handler must not reach a name that does not exist yet.
  let beat = null;
  const drop = () => {
    clearInterval(beat);
    beat = null;
    listeners.delete(res);
  };
  res.on("error", drop);
  req.on("error", drop);
  req.on("close", drop);

  const first = `data: ${JSON.stringify(await fullState())}\n\n`;
  if (res.writableEnded || res.destroyed) return;
  res.write(first);
  listeners.add(res);
  beat = setInterval(() => res.write(": beat\n\n"), 20000);
}

/* ---- routes ------------------------------------------------------------ */

const server = createServer(async (req, res) => {
  const url = new URL(req.url, `http://${req.headers.host ?? "localhost"}`);
  const path = url.pathname;

  try {
    if (req.method === "GET") {
      if (path === "/") return await sendFile(res, UI_DIR, "index.html");
      if (path === "/console.css" || path === "/app.js") return await sendFile(res, UI_DIR, path.slice(1));
      if (path.startsWith("/design-system/")) return await sendFile(res, DESIGN_SYSTEM, path.slice("/design-system/".length));
      if (path === "/api/state") return sendJson(res, 200, await fullState());
      if (path === "/api/events") return await subscribe(req, res);
    }

    if (req.method === "POST") {
      if (path === "/api/describe") {
        const { description } = await readBody(req);
        if (typeof description !== "string" || description.trim() === "") {
          return sendJson(res, 400, { error: "describe what you want built" });
        }
        journey.describe(description.trim()).catch(() => {});
        return sendJson(res, 202, { accepted: true });
      }
      if (path === "/api/answer") {
        const { question, option } = await readBody(req);
        await journey.answer(question, option);
        return sendJson(res, 202, { accepted: true });
      }
      if (path === "/api/run") {
        // Deliberately refuses. See RUN_NOT_WIRED in lib/journey.mjs.
        return sendJson(res, 409, (await journey.snapshot()).run);
      }
      if (path === "/api/reset") {
        await journey.reset();
        return sendJson(res, 200, { reset: true });
      }
    }

    return sendJson(res, 404, { error: `no route for ${req.method} ${path}` });
  } catch (cause) {
    return sendJson(res, cause.status ?? 500, { error: cause.message });
  }
});

server.listen(PORT, "0.0.0.0", async () => {
  const { shown, cairn } = await readWiring();
  process.stdout.write([`guided console prototype listening on http://0.0.0.0:${PORT}`, `  target project : ${shown.project}`, `  harness        : ${shown.harness}`, `  cairn          : ${cairn}`, `  design system  : ${DESIGN_SYSTEM}`, ""].join("\n"));
});

for (const signal of ["SIGINT", "SIGTERM"]) {
  process.on(signal, () => {
    server.close(() => process.exit(0));
  });
}
