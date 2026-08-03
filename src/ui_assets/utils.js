/* Shared runtime, copy, and API helpers for the web UI. */

const preactReady = typeof window !== "undefined" && Boolean(window.preact && window.preactHooks && window.htm);

if (!preactReady) {
  console.error("cairn: vendored preact/htm runtime not available");
}

const { h, render } = preactReady ? window.preact : {};
const { useCallback, useEffect, useMemo, useState } = preactReady ? window.preactHooks : {};
const html = preactReady ? window.htm.bind(h) : undefined;

let copyData = {};
let copyLoaded = false;

function normaliseText(raw) {
  return String(raw || "")
    .trim()
    .toLowerCase();
}

function parseState(raw) {
  const state = normaliseText(raw);
  if (state === "planned" || state === "declared") {
    return "ghost";
  }
  return ["synced", "ghost", "orphaned", "drift"].includes(state) ? state : "synced";
}

function parseKind(raw) {
  const value = normaliseText(raw);
  return value || "module";
}

function cleanId(value) {
  return encodeURIComponent(String(value || ""));
}

async function fetchJson(url) {
  const response = await fetch(url);
  if (!response.ok) {
    if (response.status === 404) {
      return null;
    }
    let detail = `request failed: ${url} (${response.status})`;
    try {
      const body = await response.json();
      if (body && typeof body.code === "string" && typeof body.message === "string") {
        detail = `${body.code}: ${body.message}`;
      }
    } catch {
      // Non-JSON body: keep the generic message.
    }
    throw new Error(detail);
  }
  return response.status === 204 ? null : response.json();
}

async function loadCopy() {
  try {
    copyData = (await fetchJson("/assets/copy.json")) || {};
  } catch (error) {
    console.warn("cairn: failed to load copy.json", error);
    copyData = {};
  }
  copyLoaded = true;
}

function copy(key) {
  let value = copyData;
  const parts = String(key).split(".");
  for (const part of parts) {
    value = value?.[part];
    if (value === undefined) {
      if (copyLoaded) {
        console.warn("cairn: missing copy key:", key);
        return key;
      }
      return "";
    }
    if (value === null) {
      return key;
    }
  }
  return typeof value === "string" ? value : key;
}

function clsx(...values) {
  return values.filter(Boolean).join(" ");
}

function escapeHtml(value) {
  return String(value || "")
    .replace(/&/g, "&amp;")
    .replace(/</g, "&lt;")
    .replace(/>/g, "&gt;")
    .replace(/"/g, "&quot;")
    .replace(/'/g, "&#39;");
}

function highlightBlueprint(source, focusNodeId) {
  const focus = String(focusNodeId || "").trim();
  if (!focus) {
    return String(source || "")
      .split("\n")
      .map((line) => {
        const escaped = escapeHtml(line);
        return escaped.replace(/(System|Container|Module|Actor|Decision|Findings|Change)/g, '<span class="blueprint-keyword">$1</span>');
      })
      .join("\n");
  }

  const focusIds = Array.from(new Set([focus, ...focus.split(".").filter(Boolean).slice(-1)])).filter(Boolean);
  return String(source || "")
    .split("\n")
    .map((line) => {
      const escaped = escapeHtml(line);
      let next = escaped.replace(/(System|Container|Module|Actor|Decision|Findings|Change)/g, '<span class="blueprint-keyword">$1</span>');
      if (focusIds.some((id) => line.includes(`"${id}"`))) {
        next = `<span class="blueprint-hit">${next}</span>`;
      }
      return next;
    })
    .join("\n");
}

function normaliseNode(node) {
  return {
    ...node,
    id: String(node?.id || ""),
    name: node?.name || "",
    kind: parseKind(node?.kind),
    state: parseState(node?.state),
    children: Array.isArray(node?.children) ? node.children : [],
    paths: Array.isArray(node?.paths) ? node.paths : [],
    files: Array.isArray(node?.files) ? node.files : [],
    contracts: Array.isArray(node?.contracts) ? node.contracts : [],
  };
}

function normaliseGraph(graph) {
  if (!graph || typeof graph !== "object") {
    return { nodes: [], edges: [] };
  }

  const nodes = Array.isArray(graph.nodes) ? graph.nodes.map((node) => normaliseNode(node)) : [];
  const edges = Array.isArray(graph.edges)
    ? graph.edges
        .map((edge) => ({
          kind: parseKind(edge?.kind || "dependency"),
          from: String(edge?.from || ""),
          to: String(edge?.to || ""),
        }))
        .filter((edge) => edge.from && edge.to)
    : [];

  return { ...graph, nodes, edges };
}

async function fetchGraph() {
  return normaliseGraph((await fetchJson("/api/graph")) || {});
}

async function fetchStatus() {
  return (await fetchJson("/api/status")) || {};
}

async function fetchLint() {
  return (await fetchJson("/api/lint")) || {};
}

async function fetchPending() {
  return (await fetchJson("/api/pending")) || {};
}

async function fetchRoadmap() {
  return (await fetchJson("/api/roadmap")) || {};
}

async function fetchFrontier() {
  return (await fetchJson("/api/frontier")) || {};
}

async function fetchBlueprint() {
  return (await fetchJson("/api/blueprint")) || {};
}

function normaliseArtefactEntries(items) {
  if (!Array.isArray(items)) {
    return [];
  }

  return items
    .map((item) => {
      if (item && typeof item === "object") {
        return item;
      }
      if (item === undefined || item === null) {
        return null;
      }
      return { id: String(item) };
    })
    .filter(Boolean);
}

function fetchContractArtefact(response) {
  if (!response || typeof response !== "object") {
    return [];
  }

  const contracts = normaliseArtefactEntries(response.contracts);
  if (contracts.length) {
    return contracts;
  }

  const rawContract = typeof response.contract === "string" ? response.contract : "";
  if (!rawContract) {
    return [];
  }

  const heading = rawContract.split("\n")[0].trim();
  return [{ id: "contract", path: response.path || heading || "contract", title: "Contract", body: rawContract }];
}

function dedupeByText(items) {
  const seen = new Set();
  return items.filter((item) => {
    const key = `${item?.id || ""}|${item?.path || ""}|${item?.title || ""}`;
    if (seen.has(key)) {
      return false;
    }
    seen.add(key);
    return true;
  });
}

async function fetchNodeArtefacts(nodeId, kind) {
  const response = await fetchJson(`/api/node/${cleanId(nodeId)}/${kind}`);
  if (!response || typeof response !== "object") {
    return [];
  }

  if (kind === "contract") {
    return fetchContractArtefact(response);
  }

  if (kind === "rationale") {
    const decisions = normaliseArtefactEntries(response.decisions);
    const research = normaliseArtefactEntries(response.research);
    const sources = normaliseArtefactEntries(response.sources);
    return dedupeByText([...decisions, ...research, ...sources]);
  }

  return normaliseArtefactEntries(response[kind]);
}

async function fetchDepends(nodeId) {
  const response = await fetchJson(`/api/depends/${cleanId(nodeId)}`);
  if (!response || !Array.isArray(response.nodes)) {
    return [];
  }
  return response.nodes;
}

async function fetchNodeDecisions(nodeId) {
  const response = await fetchJson(`/api/node/${cleanId(nodeId)}/decisions`);
  if (!response || typeof response !== "object") {
    return { decisions: [], decisionIndex: {} };
  }
  const decisionIndex = response.decision_index && typeof response.decision_index === "object" && !Array.isArray(response.decision_index) ? response.decision_index : {};
  return {
    decisions: normaliseArtefactEntries(response.decisions),
    decisionIndex,
  };
}

/**
 * Fetch every evidence surface for one node in parallel. Individual request
 * failures degrade to empty lists so one bad endpoint cannot blank the rail.
 */
async function fetchNodeEvidence(nodeId) {
  const [contracts, decisionData, sources, rationale, depends, dependents] = await Promise.all([
    fetchNodeArtefacts(nodeId, "contract").catch(() => []),
    fetchNodeDecisions(nodeId).catch(() => ({ decisions: [], decisionIndex: {} })),
    fetchNodeArtefacts(nodeId, "sources").catch(() => []),
    fetchNodeArtefacts(nodeId, "rationale").catch(() => []),
    fetchDepends(nodeId).catch(() => []),
    fetchDependents(nodeId).catch(() => []),
  ]);

  const list = (value) => (Array.isArray(value) ? value : []);
  return {
    contracts: list(contracts),
    decisions: list(decisionData.decisions),
    decisionIndex: decisionData.decisionIndex,
    sources: list(sources),
    rationale: list(rationale),
    symbols: [],
    depends: list(depends),
    dependents: list(dependents),
  };
}

async function fetchDependents(nodeId) {
  const response = await fetchJson(`/api/dependents/${cleanId(nodeId)}`);
  if (!response || !Array.isArray(response.nodes)) {
    return [];
  }
  return response.nodes;
}

const SELECTION_STORAGE_KEYS = ["cairn:ui:selection", "cairn:v2:selection"];

/** Restore a persisted selection if it names a real node; empty when none saved. */
function readSelectionSeed(nodes) {
  try {
    for (const key of SELECTION_STORAGE_KEYS) {
      const saved = window.localStorage?.getItem?.(key);
      if (saved && nodes.some((node) => node.id === saved)) {
        return saved;
      }
    }
  } catch {
    // Best effort only.
  }

  return "";
}

/** Persist the current selection for the next visit (best effort). */
function writeSelectionSeed(selectionId) {
  try {
    for (const key of SELECTION_STORAGE_KEYS) {
      window.localStorage?.setItem?.(key, selectionId);
    }
  } catch {
    // Best effort only.
  }
}

/** Enrich raw neighbour references with name and state from the graph. */
function remapNeighbours(nodesById, raw) {
  if (!Array.isArray(raw)) {
    return [];
  }

  return raw
    .map((item) => {
      const id = String(item?.id || item || "");
      if (!id) {
        return null;
      }

      const node = nodesById.get(id);
      return node ? { id, name: node.name, state: node.state } : { id };
    })
    .filter(Boolean);
}

export {
  preactReady,
  h,
  render,
  html,
  useCallback,
  useEffect,
  useMemo,
  useState,
  copy,
  clsx,
  escapeHtml,
  normaliseNode,
  normaliseGraph,
  parseKind,
  parseState,
  highlightBlueprint,
  loadCopy,
  fetchGraph,
  fetchStatus,
  fetchLint,
  fetchPending,
  fetchFrontier,
  fetchRoadmap,
  fetchBlueprint,
  fetchNodeArtefacts,
  fetchNodeEvidence,
  fetchDepends,
  fetchDependents,
  readSelectionSeed,
  writeSelectionSeed,
  remapNeighbours,
};
