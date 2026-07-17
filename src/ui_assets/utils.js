/* Cairn webui shared utilities: vendored Preact/htm runtime bindings,
 * copy.toml lookups, live-data fetch helpers, and small formatting/display
 * helpers reused across feature modules. No feature-local UI state lives
 * here; see graph-canvas.js, inspector.js, findings-panel.js,
 * command-palette.js, blueprint-modal.js, top-bar.js for that.
 *
 * All colors, spacing, radii, motion, and type come from docs/design-system/tokens.css.
 * Do not hardcode hex or rem values here.
 */

const preactReady = typeof window !== "undefined" && window.preact && window.preactHooks && window.htm;
if (!preactReady) {
  // eslint-disable-next-line no-console
  console.error("cairn: vendored runtime failed to load");
}
const { h, render, Fragment } = preactReady ? window.preact : {};
const { useState, useEffect, useMemo, useRef, useCallback } = preactReady ? window.preactHooks : {};
const html = preactReady ? window.htm.bind(h) : undefined;

let _copyData = null;
async function loadCopy() {
  try {
    _copyData = await fetchJson("/assets/copy.json");
  } catch (e) {
    console.warn("cairn: copy.json failed to load, using fallback keys", e);
    _copyData = {};
  }
}
function copy(key) {
  if (!_copyData) {
    console.warn("cairn: copy data not loaded yet, using key:", key);
    return key;
  }
  const segments = key.split(".");
  let current = _copyData;
  for (const seg of segments) {
    if (current == null || typeof current !== "object") {
      console.warn("cairn: copy key missing:", key);
      return key;
    }
    current = current[seg];
  }
  if (typeof current !== "string") {
    console.warn("cairn: copy key missing:", key);
    return key;
  }
  return current;
}

function copyFinding(code) {
  if (!_copyData) return null;
  const obj = _copyData.findings?.codes || {};
  const entry = obj[code];
  if (!entry || typeof entry !== "object") return null;
  return entry;
}

function substituteCopy(template, vars) {
  return template.replace(/\{(\w+)\}/g, (m, k) => (k in vars ? vars[k] : m));
}

function CopyButton({ text }) {
  const [copied, setCopied] = useState(false);
  const onClick = useCallback(() => {
    if (navigator.clipboard) {
      navigator.clipboard.writeText(text).then(() => {
        setCopied(true);
        setTimeout(() => setCopied(false), 1200);
      });
    } else {
      // Fallback: create a temporary textarea
      const ta = document.createElement("textarea");
      ta.value = text;
      document.body.appendChild(ta);
      ta.select();
      document.execCommand("copy");
      document.body.removeChild(ta);
      setCopied(true);
      setTimeout(() => setCopied(false), 1200);
    }
  }, [text]);
  return html`
    <button class="btn ghost copy-btn" onClick=${onClick}>
      ${copied ? "Copied" : "Copy"}
    </button>
  `;
}

const SEVERITY_RANK = { error: 0, warning: 1, info: 2 };

function pickNudgeFinding(findings, nodeId) {
  if (!findings || !nodeId) return null;
  const nodeFn = findings.filter((f) => f.node === nodeId);
  if (nodeFn.length === 0) return null;
  return nodeFn.reduce((best, f) => {
    const br = SEVERITY_RANK[best.severity] ?? 2;
    const fr = SEVERITY_RANK[f.severity] ?? 2;
    if (fr < br) return f;
    if (fr === br && f.code < best.code) return f;
    return best;
  });
}

function clsx(...values) {
  return values.filter(Boolean).join(" ");
}

function percentEncodeId(id) {
  return encodeURIComponent(id);
}

const ISSUE_BASE = "https://github.com/cairn-framework/cairn/issues/new";

function openReportIssue(version) {
  const safeVersion = version || "unknown";
  const whatHappened = `cairn ${safeVersion} webui: `;
  const query = ["template=bug-report.yml", `version=${encodeURIComponent(safeVersion)}`, `what-happened=${encodeURIComponent(whatHappened)}`].join("&");
  window.open(`${ISSUE_BASE}?${query}`, "_blank", "noopener");
}

async function fetchJson(url, options) {
  const response = await fetch(url, options);
  if (!response.ok && response.status !== 404) {
    let detail = `request failed: ${url} (${response.status})`;
    try {
      const body = await response.json();
      if (body && typeof body.code === "string" && typeof body.message === "string") {
        detail = `${body.code}: ${body.message}`;
      }
    } catch {
      // Non-JSON error body: keep the generic message above.
    }
    throw new Error(detail);
  }
  if (response.status === 204) return null;
  return response.json();
}

function normaliseGraph(graph) {
  // Tolerates both canonical wire shapes (Debug-case strings) and legacy
  // fixture shapes (already lower-case strings) by normalising once at
  // ingest. Non-array nodes/edges are passed through untouched so the
  // boot-time contract check still surfaces malformed payloads.
  if (!graph) return graph;
  const normalised = { ...graph };
  if (Array.isArray(graph.nodes)) {
    normalised.nodes = graph.nodes.map((n) => ({
      ...n,
      kind: String(n.kind || "").toLowerCase(),
      state: n.state ? String(n.state).toLowerCase() : n.state,
    }));
  }
  if (Array.isArray(graph.edges)) {
    normalised.edges = graph.edges.map((e) => ({
      ...e,
      kind: String(e.kind || "").toLowerCase(),
    }));
  }
  return normalised;
}

async function fetchGraph() {
  return normaliseGraph(await fetchJson("/api/graph"));
}

async function fetchLint() {
  return fetchJson("/api/lint");
}

async function fetchMeta() {
  return fetchJson("/api/meta");
}

async function fetchNodeArtefacts(id, kind) {
  const response = await fetchJson(`/api/node/${percentEncodeId(id)}/${kind}`);
  if (!response) return [];
  // Legacy shape: { node, artefacts: [{ type, path, title, frontmatter, body }] }
  if (Array.isArray(response.artefacts)) {
    return response.artefacts.map((entry) => ({
      ...entry,
      status: entry.status ?? entry.frontmatter?.status,
    }));
  }
  // Rationale canonical shape: { node, decisions: [...], research: [...], sources: [...] }
  if (kind === "rationale") {
    const kinds = ["decisions", "research", "sources"];
    return kinds.flatMap((key) => {
      const list = response[key];
      if (!Array.isArray(list)) return [];
      return list.map((entry) => ({ ...entry, type: key }));
    });
  }
  // Canonical shape: { node, <kind>: [...] } (contract uses "contracts").
  const key = kind === "contract" ? "contracts" : kind;
  const list = response[key];
  if (!Array.isArray(list)) return [];
  return list.map((entry) => ({
    ...entry,
    type: kind === "contract" ? "contract" : kind,
  }));
}

async function fetchNodeBeads(id) {
  const response = await fetchJson(`/api/node/${percentEncodeId(id)}/beads`);
  if (!response || !Array.isArray(response.beads)) return [];
  return response.beads;
}

async function fetchNodeSymbols(id) {
  const response = await fetchJson(`/api/node/${percentEncodeId(id)}/symbols`);
  if (!response || !Array.isArray(response.symbols)) return [];
  return response.symbols;
}

async function fetchDepends(id) {
  const response = await fetchJson(`/api/depends/${percentEncodeId(id)}`);
  if (!response || !Array.isArray(response.nodes)) return [];
  return response.nodes;
}

async function fetchDependents(id) {
  const response = await fetchJson(`/api/dependents/${percentEncodeId(id)}`);
  if (!response || !Array.isArray(response.nodes)) return [];
  return response.nodes;
}

async function fetchBlueprint() {
  return fetchJson("/api/blueprint");
}

function escapeHtml(value) {
  return String(value).replace(/&/g, "&amp;").replace(/</g, "&lt;").replace(/>/g, "&gt;").replace(/"/g, "&quot;");
}

// Tokenize a blueprint line into coloured spans. Mirrors the v2 prototype so
// the syntax palette is consistent across the blueprint-card inspector and the
// view-source modal.
function highlightBlueprint(src, highlightModuleId) {
  if (!src) return "";
  const patterns = [
    { re: /^#.*$/, cls: "cm" },
    { re: /"[^"]*"/, cls: "str" },
    { re: /@\w+/, cls: "tag" },
    { re: /\b(System|Container|Module|Actor)\b/, cls: "kw" },
    { re: /\b(path|contract|decisions|research|sources|todos|reviews|id)\b/, cls: "key" },
  ];
  return src
    .split("\n")
    .map((line) => {
      const hit = highlightModuleId && line.includes(`"${highlightModuleId}"`);
      let i = 0;
      let out = "";
      while (i < line.length) {
        let matched = null;
        for (const p of patterns) {
          const re = new RegExp(p.re.source, "g");
          re.lastIndex = i;
          const m = re.exec(line);
          if (m && m.index === i) {
            matched = { text: m[0], cls: p.cls };
            break;
          }
        }
        if (matched) {
          out += `<span class="${matched.cls}">${escapeHtml(matched.text)}</span>`;
          i += matched.text.length;
        } else {
          out += escapeHtml(line[i]);
          i += 1;
        }
      }
      return hit ? `<span class="hi">${out}</span>` : out;
    })
    .join("\n");
}

function truncate(value, limit) {
  if (!value || value.length <= limit) return value || "";
  return `${value.slice(0, limit - 1)}\u2026`;
}

// Clamp any count into the 0 through 5 band used by the chain-balance widget.
function balanceFromCount(count) {
  if (!count || count <= 0) return 0;
  if (count >= 10) return 5;
  return Math.max(1, Math.round(count / 2));
}

function fillPercent(value) {
  return `${Math.min(100, Math.max(0, value * 20))}%`;
}

// Wire state -> display state. Spec §10.1 defines ghost as "planned but
// unimplemented" (healthy), so render it as a calm "planned" affordance.
// Findings overlay independently; they are not folded in here.
function displayState(state) {
  return state === "ghost" ? "planned" : state || "synced";
}

// Maps a finding severity string to the pill modifier class.
// error -> drift (warm-red signal), warning -> orphaned (weathered), info -> settled (mossy-green).
function severityPill(severity) {
  if (severity === "error") return "drift";
  if (severity === "warning") return "orphaned";
  return "settled";
}
// Computes a map of node-id -> highest severity finding for that node.
// Structural errors, interface contradictions, rationale tensions, and
// info observations all surface through this unified overlay.
function nodeSeverityById(lint) {
  const map = new Map();
  if (!lint || !lint.findings) return map;
  const rank = { error: 0, warning: 1, info: 2 };
  for (const f of lint.findings) {
    if (!f.node) continue;
    const current = map.get(f.node);
    if (!current || (rank[f.severity] ?? 2) < (rank[current] ?? 2)) {
      map.set(f.node, f.severity);
    }
  }
  return map;
}

export {
  preactReady,
  h,
  render,
  Fragment,
  html,
  useState,
  useEffect,
  useMemo,
  useRef,
  useCallback,
  loadCopy,
  copy,
  copyFinding,
  substituteCopy,
  CopyButton,
  SEVERITY_RANK,
  pickNudgeFinding,
  clsx,
  percentEncodeId,
  ISSUE_BASE,
  openReportIssue,
  fetchJson,
  normaliseGraph,
  fetchGraph,
  fetchLint,
  fetchMeta,
  fetchNodeArtefacts,
  fetchNodeBeads,
  fetchNodeSymbols,
  fetchDepends,
  fetchDependents,
  fetchBlueprint,
  escapeHtml,
  highlightBlueprint,
  truncate,
  balanceFromCount,
  fillPercent,
  displayState,
  severityPill,
  nodeSeverityById,
};
