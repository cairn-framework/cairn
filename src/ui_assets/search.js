/* Query parsing and edge helper utilities for the web UI. */

function parseQuery(raw) {
  const source = String(raw || "")
    .trim()
    .toLowerCase();
  if (!source) {
    return { text: "", state: "all", kind: "all" };
  }

  const tokens = source.split(/\s+/);
  let state = "all";
  let kind = "all";
  const textTokens = [];

  for (const token of tokens) {
    if (token.startsWith("state:") && token.length > 6) {
      state = token.slice(6);
      continue;
    }

    if (token.startsWith("kind:") && token.length > 5) {
      kind = token.slice(5);
      continue;
    }

    textTokens.push(token);
  }

  return {
    text: textTokens.join(" ").trim(),
    state,
    kind,
  };
}

function matchesQuery(node, parsed) {
  const haystack = [node.id, node.name, node.description, node.state, node.kind, ...(Array.isArray(node.paths) ? node.paths : []), ...(Array.isArray(node.files) ? node.files : [])].map((value) => String(value || "").toLowerCase()).join(" ");

  const kind = String(node.kind || "all").toLowerCase();
  const state = String(node.state || "synced").toLowerCase();

  if (parsed.kind && parsed.kind !== "all" && kind !== parsed.kind) {
    return false;
  }

  if (parsed.state && parsed.state !== "all" && state !== parsed.state) {
    return false;
  }

  if (!parsed.text) {
    return true;
  }

  return haystack.includes(parsed.text);
}

function dependencyLists(nodeId, edges) {
  const incoming = new Set();
  const outgoing = new Set();

  for (const edge of edges || []) {
    if (edge.kind !== "dependency") {
      continue;
    }

    if (String(edge.to) === String(nodeId)) {
      incoming.add(String(edge.from));
    }

    if (String(edge.from) === String(nodeId)) {
      outgoing.add(String(edge.to));
    }
  }

  return {
    in: [...incoming],
    out: [...outgoing],
  };
}

function mapEdgeRows(nodeId, edges) {
  const deps = dependencyLists(nodeId, edges);
  return {
    in: (deps.in || []).map((id) => ({ id })),
    out: (deps.out || []).map((id) => ({ id })),
  };
}

/**
 * Row-major grid arrow navigation. Returns the destination index, or the
 * current index when the move would cross a row boundary or leave the grid.
 */
function gridNavigate(index, key, columns, total) {
  if (index < 0) {
    return -1;
  }

  const row = Math.floor(index / columns);
  const column = index % columns;
  switch (key) {
    case "ArrowLeft":
      return column === 0 ? index : index - 1;
    case "ArrowRight":
      return column === columns - 1 || index + 1 >= total ? index : index + 1;
    case "ArrowUp":
      return row === 0 ? index : index - columns;
    case "ArrowDown":
      return index + columns >= total ? index : index + columns;
    default:
      return index;
  }
}

export { parseQuery, matchesQuery, mapEdgeRows, gridNavigate };
