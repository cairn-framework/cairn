// Deterministic static replay server for the cairn webui.
//
// Loop-editable surfaces are served LIVE from the working tree so any edit the
// optimisation loop makes to the hand-authored web sources is reflected on the
// next run with no rebuild:
//   /                  -> src/ui_assets/index.html
//   /assets/style.css  -> tokens.css + components.css + ui_assets/style.css
//                         (the exact concatenation src/ui/mod.rs serves)
//   /assets/app.js     -> src/ui_assets/app.js
//   /vendor/*          -> src/ui_assets/vendor/*
//
// The data layer is frozen: /assets/copy.json and every /api/* response are
// served from captured fixtures under harness/fixtures, so the workload is
// identical every run with no live network and no cargo build.

import { createServer } from "node:http";
import { existsSync, readFileSync, statSync } from "node:fs";
import { join, normalize } from "node:path";

const STYLE_HEADER =
  "/* Cairn Graph Explorer stylesheet.\n" +
  "   Concatenated: design-system tokens, design-system components, ui overrides.\n" +
  "   Single source of truth for tokens is docs/design-system/tokens.css. */\n";

/** Reproduce the `/assets/style.css` concatenation from src/ui/mod.rs. */
export function buildStyleCss(root) {
  const tokens = readFileSync(join(root, "docs/design-system/tokens.css"), "utf8");
  const components = readFileSync(join(root, "docs/design-system/components.css"), "utf8");
  const overrides = readFileSync(join(root, "src/ui_assets/style.css"), "utf8");
  return (
    STYLE_HEADER +
    tokens +
    "\n/* ---- design-system components ---- */\n" +
    components +
    "\n/* ---- graph-explorer overrides ---- */\n" +
    overrides
  );
}

function sendFile(res, path, contentType) {
  try {
    const body = readFileSync(path);
    res.writeHead(200, { "content-type": contentType, "cache-control": "no-store" });
    res.end(body);
  } catch {
    res.writeHead(404, { "content-type": "text/plain" });
    res.end("not found");
  }
}

function safeJoin(base, reqPath) {
  const resolved = normalize(join(base, reqPath));
  if (!resolved.startsWith(normalize(base))) return null; // path traversal guard
  return resolved;
}

/**
 * Start the replay server. Returns `{ url, port, close }`. `port: 0` (default)
 * asks the OS for a free port.
 */
export function startReplayServer({ root, fixturesDir, port = 0 }) {
  const assetsDir = join(root, "src/ui_assets");
  const server = createServer((req, res) => {
    const url = new URL(req.url, "http://127.0.0.1");
    const path = decodeURIComponent(url.pathname);

    if (path === "/" || path === "/index.html") {
      return sendFile(res, join(assetsDir, "index.html"), "text/html; charset=utf-8");
    }
    if (path === "/assets/style.css") {
      try {
        res.writeHead(200, { "content-type": "text/css; charset=utf-8", "cache-control": "no-store" });
        return res.end(buildStyleCss(root));
      } catch {
        res.writeHead(500, { "content-type": "text/plain" });
        return res.end("style build failed");
      }
    }
    if (path === "/assets/app.js") {
      return sendFile(res, join(assetsDir, "app.js"), "application/javascript; charset=utf-8");
    }
    if (path.startsWith("/vendor/")) {
      const fp = safeJoin(assetsDir, path);
      if (!fp) {
        res.writeHead(400);
        return res.end("bad path");
      }
      return sendFile(res, fp, "application/javascript; charset=utf-8");
    }
    // Everything else (copy.json, /api/*) replays from frozen fixtures.
    const fp = safeJoin(fixturesDir, path);
    if (fp && existsSync(fp) && statSync(fp).isFile()) {
      return sendFile(res, fp, "application/json; charset=utf-8");
    }
    // The SPA tolerates 404s (treats them as empty), so this is a valid path.
    res.writeHead(404, { "content-type": "application/json; charset=utf-8" });
    res.end('{"error":"fixture not captured"}');
  });

  return new Promise((resolve, reject) => {
    server.on("error", reject);
    server.listen(port, "127.0.0.1", () => {
      const actual = server.address().port;
      resolve({
        url: `http://127.0.0.1:${actual}`,
        port: actual,
        close: () => new Promise((r) => server.close(r)),
      });
    });
  });
}
