/* Cairn webui app shell: bootstrap data fetch + state wiring between modules. */

import { ChannelBar } from "./channel-bar.js";
import { EvidenceRail, StateLegend } from "./evidence-rail.js";
import { GraphWorkspace } from "./graph-workspace.js";
import { QueryRail } from "./query-rail.js";
import { gridNavigate, mapEdgeRows, matchesQuery, parseQuery } from "./search.js";
import { StatusBezel } from "./status-bezel.js";
import { copy, fetchBlueprint, fetchGraph, fetchLint, fetchNodeEvidence, fetchStatus, html, loadCopy, preactReady, readSelectionSeed, remapNeighbours, render, useCallback, useEffect, useMemo, useState } from "./utils.js";

const DEFAULT_CHANNEL = "findings";

function App() {
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState("");
  const [compact, setCompact] = useState(window.innerWidth <= 990);
  const [bootstrapNonce, setBootstrapNonce] = useState(0);

  const [graph, setGraph] = useState({ nodes: [], edges: [] });
  const [status, setStatus] = useState({});
  const [lint, setLint] = useState({});
  const [blueprint, setBlueprint] = useState({ path: "", source: "" });
  const [notices, setNotices] = useState([]);

  const [query, setQuery] = useState("");
  const [kindFilter, setKindFilter] = useState("all");
  const [stateFilter, setStateFilter] = useState("all");
  const [selectionId, setSelectionId] = useState("");
  const [selectionIndex, setSelectionIndex] = useState(-1);
  const [evidenceMode, setEvidenceMode] = useState("depth");
  const [channel, setChannel] = useState(DEFAULT_CHANNEL);

  const [artefactsByNode, setArtefactsByNode] = useState({});
  const [depends, setDepends] = useState({ in: [], out: [] });
  const [selectedLineageItem, setSelectedLineageItem] = useState(null);

  const onResize = useCallback(() => {
    setCompact(window.innerWidth <= 990);
  }, []);

  useEffect(() => {
    window.addEventListener("resize", onResize);
    return () => window.removeEventListener("resize", onResize);
  }, [onResize]);

  useEffect(() => {
    if (!preactReady) {
      setError(copy("webui.runtime-not-ready"));
      setLoading(false);
      return;
    }

    let active = true;

    async function bootstrap() {
      setError("");
      setLoading(true);
      setNotices([]);

      try {
        await loadCopy();

        const graphPayload = await fetchGraph();
        if (!active) {
          return;
        }

        const optional = await Promise.allSettled([fetchStatus(), fetchLint(), fetchBlueprint()]);

        const statusPayload = optional[0].status === "fulfilled" ? optional[0].value || {} : {};
        const lintPayload = optional[1].status === "fulfilled" ? optional[1].value || {} : {};
        const blueprintPayload = optional[2].status === "fulfilled" ? optional[2].value || { path: "", source: "" } : { path: "", source: "" };

        const nextNotices = [];
        if (optional[0].status === "rejected") {
          nextNotices.push(copy("webui.bootstrap-status-failed"));
        }
        if (optional[1].status === "rejected") {
          nextNotices.push(copy("webui.bootstrap-lint-failed"));
        }
        if (optional[2].status === "rejected") {
          nextNotices.push(copy("webui.bootstrap-blueprint-failed"));
        }

        const nodes = Array.isArray(graphPayload?.nodes) ? graphPayload.nodes : [];
        const edges = Array.isArray(graphPayload?.edges) ? graphPayload.edges : [];

        if (!active) {
          return;
        }

        setGraph({ nodes, edges });
        setStatus(statusPayload);
        setLint(lintPayload);
        setBlueprint(blueprintPayload);
        setNotices(nextNotices);

        const seed = readSelectionSeed(nodes);
        setSelectionId(seed);
        setSelectionIndex(seed ? 0 : -1);
      } catch (cause) {
        if (!active) {
          return;
        }

        setError(cause?.message || copy("empty-states.map-failed.body"));
        setGraph({ nodes: [], edges: [] });
        setNotices([]);
      } finally {
        if (active) {
          setLoading(false);
        }
      }
    }

    bootstrap();

    return () => {
      active = false;
    };
  }, [bootstrapNonce]);

  useEffect(() => {
    if (!selectionId) {
      return;
    }

    try {
      window.localStorage?.setItem?.(STORAGE_KEY, selectionId);
      window.localStorage?.setItem?.(STORAGE_KEY_LEGACY, selectionId);
    } catch {
      // best effort
    }
  }, [selectionId]);

  const nodes = useMemo(() => graph.nodes || [], [graph.nodes]);
  const edges = useMemo(() => graph.edges || [], [graph.edges]);
  const dependencyCount = useMemo(() => edges.filter((edge) => String(edge?.kind || "").toLowerCase() === "dependency").length, [edges]);

  const nodesById = useMemo(() => {
    const map = new Map();
    for (const node of nodes) {
      map.set(node.id, node);
    }
    return map;
  }, [nodes]);

  const parsedQuery = useMemo(() => parseQuery(query), [query]);
  const visibleNodes = useMemo(
    () =>
      nodes.filter((node) =>
        matchesQuery(node, {
          ...parsedQuery,
          kind: kindFilter === "all" ? parsedQuery.kind : kindFilter,
          state: stateFilter === "all" ? parsedQuery.state : stateFilter,
        }),
      ),
    [nodes, parsedQuery.text, parsedQuery.kind, parsedQuery.state, kindFilter, stateFilter],
  );
  const visibleIds = useMemo(() => visibleNodes.map((node) => node.id), [visibleNodes]);

  useEffect(() => {
    if (!visibleIds.length) {
      if (selectionId) {
        setSelectionId("");
      }
      setSelectionIndex(-1);
      return;
    }

    if (!selectionId) {
      setSelectionId(visibleIds[0]);
      setSelectionIndex(0);
      return;
    }

    if (visibleIds.includes(selectionId)) {
      const nextIndex = visibleIds.indexOf(selectionId);
      if (selectionIndex !== nextIndex) {
        setSelectionIndex(nextIndex);
      }
      return;
    }

    setSelectionIndex(0);
    setSelectionId(visibleIds[0]);
  }, [selectionId, selectionIndex, visibleIds]);

  const selection = nodesById.get(selectionId) || null;

  useEffect(() => {
    if (!selectionId) {
      setDepends({ in: [], out: [] });
      return;
    }

    const rows = mapEdgeRows(selectionId, edges);
    setDepends(rows);
  }, [edges, selectionId]);

  useEffect(() => {
    if (!selectionId) {
      return;
    }

    let active = true;

    async function loadArtefacts() {
      const evidence = await fetchNodeEvidence(selectionId);
      if (!active) {
        return;
      }

      const selectedNode = nodesById.get(selectionId);
      const mappedOut = remapNeighbours(nodesById, evidence.depends);
      const mappedIn = remapNeighbours(nodesById, evidence.dependents);
      const rows = mapEdgeRows(selectionId, edges);

      setDepends({
        in: mappedIn.length ? mappedIn : rows.in,
        out: mappedOut.length ? mappedOut : rows.out,
      });

      setArtefactsByNode((current) => ({
        ...current,
        [selectionId]: {
          contracts: evidence.contracts,
          decisions: evidence.decisions,
          sources: evidence.sources,
          evidence: evidence.rationale,
          symbols: evidence.symbols.length ? evidence.symbols : selectedNode?.symbols || [],
        },
      }));
    }

    loadArtefacts().catch(() => {
      if (!active) {
        return;
      }

      const fallback = nodesById.get(selectionId);
      setArtefactsByNode((current) => ({
        ...current,
        [selectionId]: {
          contracts: [],
          decisions: [],
          sources: [],
          evidence: [],
          symbols: Array.isArray(fallback?.symbols) ? fallback.symbols : [],
        },
      }));
      setDepends(mapEdgeRows(selectionId, edges));
    });

    return () => {
      active = false;
    };
  }, [selectionId, edges, nodesById]);

  const findings = Array.isArray(lint.findings) ? lint.findings : [];
  const drift = findings.filter((item) => item?.severity === "error" || item?.severity === "warning");
  const changes = Array.isArray(status.active_changes) ? status.active_changes : [];
  const backlog = Array.isArray(status.open_todos) ? status.open_todos : [];

  const selectionArtefacts = useMemo(() => artefactsByNode[selectionId] || {}, [artefactsByNode, selectionId]);

  const onSelect = useCallback(
    (nodeId, options = {}) => {
      if (!nodeId) {
        return;
      }

      const { clearFilters = false } = options;
      if (clearFilters && (query || kindFilter !== "all" || stateFilter !== "all")) {
        setQuery("");
        setKindFilter("all");
        setStateFilter("all");
      }

      setSelectedLineageItem(null);
      setEvidenceMode("depth");
      setSelectionId(nodeId);
      setSelectionIndex(visibleIds.includes(nodeId) ? visibleIds.indexOf(nodeId) : 0);
    },
    [query, kindFilter, stateFilter, visibleIds],
  );

  const onQueryKey = useCallback(
    (event) => {
      if (!visibleIds.length) {
        return;
      }

      if (event.key === "Enter") {
        setSelectionId(visibleIds[0]);
        setSelectionIndex(0);
        return;
      }

      if (event.key !== "ArrowDown" && event.key !== "ArrowUp") {
        return;
      }

      event.preventDefault();

      const nextIndex = event.key === "ArrowDown" ? (selectionIndex + 1) % visibleIds.length : (selectionIndex - 1 + visibleIds.length) % visibleIds.length;

      setSelectionIndex(nextIndex);
      setSelectionId(visibleIds[nextIndex]);
    },
    [selectionIndex, visibleIds],
  );

  const onCanvasKeyNavigate = useCallback(
    (event, currentSelectionId) => {
      if (!event.key.startsWith("Arrow") || !visibleIds.length || !currentSelectionId) {
        return;
      }

      const columns = compact ? 4 : 6;
      const index = visibleIds.indexOf(currentSelectionId);
      const next = gridNavigate(index, event.key, columns, visibleIds.length);
      if (next === index || next < 0) {
        return;
      }

      event.preventDefault();
      setSelectionIndex(next);
      setSelectionId(visibleIds[next]);
    },
    [compact, visibleIds],
  );

  const onGlobalKey = useCallback(
    (event) => {
      if (event.key !== "Escape") {
        return;
      }

      if (query || kindFilter !== "all" || stateFilter !== "all") {
        setQuery("");
        setKindFilter("all");
        setStateFilter("all");
        setSelectedLineageItem(null);
        setEvidenceMode("depth");
        return;
      }

      if (evidenceMode !== "depth") {
        setEvidenceMode("depth");
        setSelectedLineageItem(null);
        return;
      }

      if (selectionIndex >= 0) {
        setSelectionIndex(0);
        setSelectionId(visibleIds[0] || "");
      }
    },
    [query, kindFilter, stateFilter, evidenceMode, selectionIndex, visibleIds],
  );

  useEffect(() => {
    window.addEventListener("keydown", onGlobalKey);
    return () => window.removeEventListener("keydown", onGlobalKey);
  }, [onGlobalKey]);

  const bringIntoView = useCallback(() => {
    if (!selectionId) {
      return;
    }

    const rowId = `node-${String(selectionId).replace(/[^a-zA-Z0-9_.:-]/g, "-")}`;
    document.getElementById(rowId)?.scrollIntoView({ behavior: "smooth", block: "center", inline: "center" });
  }, [selectionId]);

  const onLineageSelect = useCallback(
    (artefact) => {
      if (!artefact) {
        setSelectedLineageItem(null);
        return;
      }

      const sourceId = artefact.node || artefact.id || artefact.slug;
      if (sourceId && nodesById.has(sourceId)) {
        onSelect(sourceId, { clearFilters: true });
        setSelectedLineageItem(null);
        setEvidenceMode("depth");
        return;
      }

      setSelectedLineageItem(artefact);
      setEvidenceMode("lineage");
    },
    [nodesById, onSelect],
  );

  const onMode = useCallback((mode) => {
    setEvidenceMode(mode);
    if (mode !== "lineage") {
      setSelectedLineageItem(null);
    }
  }, []);

  useEffect(() => {
    if (!selectionId) {
      return;
    }
    const row = document.getElementById(`node-${String(selectionId).replace(/[^a-zA-Z0-9_.:-]/g, "-")}`);
    if (row) {
      row.scrollIntoView({ block: "center", inline: "center" });
    }
  }, [selectionId]);

  if (loading) {
    return html`<main class="instrument-shell"><p class="plate-meta">${copy("empty-states.map-loading.body")}</p></main>`;
  }

  if (error) {
    return html`
      <main class="instrument-shell">
        <p class="plate-meta">${copy("webui.load-error")} ${error}</p>
        <button class="query-action" type="button" onClick=${() => setBootstrapNonce((value) => value + 1)}>
          ${copy("webui.retry")}
        </button>
      </main>
    `;
  }

  return html`
    <main class="instrument-shell" aria-label=${copy("webui.app")} data-compact=${compact}>
      <${StatusBezel} nodeCount=${nodes.length} dependencyCount=${dependencyCount} findings=${findings} />
      ${
        notices.length
          ? html`
            <section class="bootstrap-notices" role="status" aria-live="polite">
              ${notices.map((notice) => html`<p class="plate-meta">${notice}</p>`)}
            </section>`
          : null
      }
      <${QueryRail}
        query=${query}
        parsed=${parsedQuery}
        visibleCount=${visibleIds.length}
        kindFilter=${kindFilter}
        stateFilter=${stateFilter}
        onQuery=${setQuery}
        onQueryKey=${onQueryKey}
        onKindFilter=${setKindFilter}
        onStateFilter=${setStateFilter}
        onBringIntoView=${bringIntoView}
        onClear=${() => {
          setQuery("");
          setKindFilter("all");
          setStateFilter("all");
        }}
      />
      <div class="instrument-workspace">
        <${GraphWorkspace}
          nodes=${visibleNodes}
          edges=${edges}
          compact=${compact}
          selectionId=${selectionId}
          onSelect=${onSelect}
          onCanvasKeyNavigate=${onCanvasKeyNavigate}
        />
        <${EvidenceRail}
          mode=${evidenceMode}
          onMode=${onMode}
          selection=${selection}
          inRows=${depends.in}
          outRows=${depends.out}
          onNeighbourSelect=${(nodeId) => onSelect(nodeId, { clearFilters: true })}
          artefacts=${selectionArtefacts}
          blueprint=${blueprint.source}
          blueprintPath=${blueprint.path}
          onLineageOpen=${onLineageSelect}
          selectedLineageItem=${selectedLineageItem}
        />
      </div>
      <${StateLegend} />
      <${ChannelBar}
        active=${channel}
        findings=${findings}
        drift=${drift}
        changes=${changes}
        backlog=${backlog}
        onChannel=${setChannel}
        onItem=${(nodeId) => onSelect(nodeId, { clearFilters: true })}
      />
    </main>
  `;
}

const root = document.getElementById("root");
if (root && preactReady) {
  render(html`<${App} />`, root);
}
