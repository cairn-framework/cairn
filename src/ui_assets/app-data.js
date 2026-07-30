import { mapEdgeRows } from "./search.js";
import { copy, fetchBlueprint, fetchGraph, fetchLint, fetchNodeEvidence, fetchPending, fetchStatus, loadCopy, remapNeighbours } from "./utils.js";

async function loadBootstrapData(isActive) {
  await loadCopy();
  const graphPayload = await fetchGraph();
  if (isActive && !isActive()) {
    return { cancelled: true };
  }

  const optional = await Promise.allSettled([fetchStatus(), fetchLint(), fetchBlueprint(), fetchPending()]);

  const statusPayload = optional[0].status === "fulfilled" ? optional[0].value || {} : {};
  const lintPayload = optional[1].status === "fulfilled" ? optional[1].value || {} : {};
  const blueprintPayload = optional[2].status === "fulfilled" ? optional[2].value || { path: "", source: "" } : { path: "", source: "" };
  const pendingPayload = optional[3].status === "fulfilled" ? optional[3].value || {} : {};

  const notices = [];
  if (optional[0].status === "rejected") {
    notices.push(copy("webui.bootstrap-status-failed"));
  }
  if (optional[1].status === "rejected") {
    notices.push(copy("webui.bootstrap-lint-failed"));
  }
  if (optional[2].status === "rejected") {
    notices.push(copy("webui.bootstrap-blueprint-failed"));
  }
  if (optional[3].status === "rejected") {
    notices.push(copy("webui.bootstrap-pending-failed"));
  }

  return {
    cancelled: false,
    graph: {
      nodes: Array.isArray(graphPayload?.nodes) ? graphPayload.nodes : [],
      edges: Array.isArray(graphPayload?.edges) ? graphPayload.edges : [],
    },
    status: statusPayload,
    lint: lintPayload,
    pending: pendingPayload,
    blueprint: blueprintPayload,
    notices,
  };
}

async function loadNodeArtefacts(selectionId, nodesById, edges) {
  const evidence = await fetchNodeEvidence(selectionId);
  const selectedNode = nodesById.get(selectionId);
  const mappedOut = remapNeighbours(nodesById, evidence.depends);
  const mappedIn = remapNeighbours(nodesById, evidence.dependents);
  const rows = mapEdgeRows(selectionId, edges);

  return {
    depends: {
      in: mappedIn.length ? mappedIn : rows.in,
      out: mappedOut.length ? mappedOut : rows.out,
    },
    artefacts: {
      contracts: evidence.contracts,
      decisions: evidence.decisions,
      sources: evidence.sources,
      evidence: evidence.rationale,
      symbols: evidence.symbols.length ? evidence.symbols : selectedNode?.symbols || [],
    },
  };
}

export { loadBootstrapData, loadNodeArtefacts };
