// One-shot capture of the live cairn webui API into frozen fixtures.
//
// Run once during harness setup against a running `cairn ui` server:
//   ./target/release/cairn ui --port <p> --no-open &
//   node harness/capture-fixtures.mjs http://127.0.0.1:<p> harness/fixtures
//
// It mirrors every endpoint the SPA consumes so the replay server can serve a
// deterministic, network-free dataset. Node ids are encoded with the same
// encodeURIComponent the SPA uses, so replay lookups match byte-for-byte.

import { mkdir, writeFile } from "node:fs/promises";
import { dirname, join } from "node:path";

const base = process.argv[2];
const outDir = process.argv[3];
if (!base || !outDir) {
  console.error("usage: node harness/capture-fixtures.mjs <baseUrl> <outDir>");
  process.exit(2);
}

async function fetchText(path) {
  const res = await fetch(base + path);
  return { status: res.status, text: await res.text() };
}

async function save(path, text) {
  const fp = join(outDir, path.replace(/^\//, ""));
  await mkdir(dirname(fp), { recursive: true });
  await writeFile(fp, text);
}

async function capture(path) {
  const { status, text } = await fetchText(path);
  if (status >= 500) throw new Error(`server error ${status} for ${path}`);
  await save(path, text);
  return text;
}

const TOP = ["/assets/copy.json", "/api/meta", "/api/status", "/api/graph", "/api/lint", "/api/blueprint"];
const NODE_SUFFIXES = ["/contract", "/decisions", "/todos", "/research", "/sources", "/beads", "/rationale"];

let count = 0;
for (const path of TOP) {
  await capture(path);
  count += 1;
}

const graphText = await fetchText("/api/graph");
const graph = JSON.parse(graphText.text);
const nodes = Array.isArray(graph.nodes) ? graph.nodes : [];
for (const node of nodes) {
  const id = encodeURIComponent(node.id);
  for (const suffix of NODE_SUFFIXES) {
    await capture(`/api/node/${id}${suffix}`);
    count += 1;
  }
  await capture(`/api/depends/${id}`);
  await capture(`/api/dependents/${id}`);
  count += 2;
}

console.log(`captured ${count} fixtures for ${nodes.length} nodes into ${outDir}`);
