/* Shared runtime, copy, and API helpers for the web UI. */

const preactReady = typeof window !== "undefined" && Boolean(window.preact && window.preactHooks && window.htm);

if (!preactReady) {
  console.error("cairn: vendored preact/htm runtime not available");
}

const { h, render } = preactReady ? window.preact : {};
const { useCallback, useEffect, useMemo, useRef, useState } = preactReady ? window.preactHooks : {};
const html = preactReady ? window.htm.bind(h) : undefined;

let copyData = {};

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

function truncate(value, max = 120) {
  const text = String(value || "");
  if (text.length <= max) {
    return text;
  }
  return `${text.slice(0, Math.max(0, max - 1))}\u2026`;
}

async function fetchJson(url) {
  const response = await fetch(url);
  if (!response.ok) {
    if (response.status === 404) {
      return null;
    }
    throw new Error(`request failed: ${url} (${response.status})`);
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
}

function copy(key) {
  let value = copyData;
  const parts = String(key).split(".");
  for (const part of parts) {
    value = value?.[part];
    if (value === undefined) {
      console.warn("cairn: missing copy key:", key);
      return key;
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
  const focus = String(focusNodeId || "");
  return String(source || "")
    .split("\n")
    .map((line) => {
      const escaped = escapeHtml(line);
      let next = escaped.replace(/(System|Container|Module|Actor|Decision|Findings|Change)/g, '<span class="blueprint-keyword">$1</span>');
      if (focus && line.includes(`"${focus}"`)) {
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

async function fetchBlueprint() {
  return (await fetchJson("/api/blueprint")) || {};
}

async function fetchNodeArtefacts(nodeId, kind) {
  const response = await fetchJson(`/api/node/${cleanId(nodeId)}/${kind}`);
  if (!response || typeof response !== "object") {
    return [];
  }

  if (Array.isArray(response.artefacts)) {
    return response.artefacts;
  }

  return [];
}

async function fetchNodeSymbols(nodeId) {
  const response = await fetchJson(`/api/node/${cleanId(nodeId)}/symbols`);
  if (!response || !Array.isArray(response.symbols)) {
    return [];
  }
  return response.symbols;
}

async function fetchDepends(nodeId) {
  const response = await fetchJson(`/api/depends/${cleanId(nodeId)}`);
  if (!response || !Array.isArray(response.nodes)) {
    return [];
  }
  return response.nodes;
}

async function fetchDependents(nodeId) {
  const response = await fetchJson(`/api/dependents/${cleanId(nodeId)}`);
  if (!response || !Array.isArray(response.nodes)) {
    return [];
  }
  return response.nodes;
}

function displayState(node) {
  return parseState(node);
}

function countSeverity(items, target) {
  if (!Array.isArray(items)) {
    return 0;
  }
  return items.filter((item) => String(item.severity || "") === target).length;
}

export {
  preactReady,
  h,
  render,
  html,
  useCallback,
  useEffect,
  useMemo,
  useRef,
  useState,
  copy,
  clsx,
  truncate,
  escapeHtml,
  normaliseNode,
  normaliseGraph,
  parseKind,
  parseState,
  displayState,
  highlightBlueprint,
  loadCopy,
  fetchGraph,
  fetchStatus,
  fetchLint,
  fetchBlueprint,
  fetchNodeArtefacts,
  fetchNodeSymbols,
  fetchDepends,
  fetchDependents,
  countSeverity,
};
