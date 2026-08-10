import { loadBootstrapData, loadNodeArtefacts } from "./app-data.js";
import { pickCanvasTarget } from "./canvas-nav.js";
import { ChannelBar } from "./channel-bar.js";
import { Console } from "./console.js";
import { EvidenceRail, StateLegend } from "./evidence-rail.js";
import { GraphWorkspace } from "./graph-workspace.js";
import { QueryRail } from "./query-rail.js";
import { mapEdgeRows, matchesQuery, parseQuery } from "./search.js";
import { StatusBezel } from "./status-bezel.js";
import { copy, html, preactReady, readSelectionSeed, render, useCallback, useEffect, useMemo, useState, writeSelectionSeed } from "./utils.js";

const DEFAULT_CHANNEL = "findings";

function App() {
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState("");
  const [compact, setCompact] = useState(window.innerWidth <= 990);
  const [bootstrapNonce, setBootstrapNonce] = useState(0);

  const [graph, setGraph] = useState({ nodes: [], edges: [] });
  const [status, setStatus] = useState({});
  const [lint, setLint] = useState({});
  const [pending, setPending] = useState({});
  const [roadmap, setRoadmap] = useState({});
  const [frontier, setFrontier] = useState({});
  const [blueprint, setBlueprint] = useState({ path: "", source: "" });
  const [notices, setNotices] = useState([]);

  const [query, setQuery] = useState("");
  const [kindFilter, setKindFilter] = useState("all");
  const [stateFilter, setStateFilter] = useState("all");
  const [selectionId, setSelectionId] = useState("");
  const [selectionIndex, setSelectionIndex] = useState(-1);
  const [evidenceMode, setEvidenceMode] = useState("depth");
  const [channel, setChannel] = useState(DEFAULT_CHANNEL);
  const [view, setView] = useState("map");

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
        const bootstrap = await loadBootstrapData(() => active);
        if (bootstrap.cancelled) {
          return;
        }

        const { graph, status, lint, pending, roadmap, frontier, blueprint, notices: nextNotices } = bootstrap;
        if (!active) {
          return;
        }

        setGraph(graph);
        setStatus(status);
        setLint(lint);
        setPending(pending);
        setRoadmap(roadmap);
        setFrontier(frontier);
        setBlueprint(blueprint);
        setNotices(nextNotices);

        const seed = readSelectionSeed(graph.nodes);
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
    if (loading) {
      return;
    }

    writeSelectionSeed(selectionId || "");
  }, [loading, selectionId]);

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
  // Containers and the system node render as frames/title, not cards; the
  // match count reflects selectable cards so it always equals the canvas.
  const visibleCardCount = useMemo(() => visibleNodes.filter((node) => !["system", "container"].includes(String(node.kind || "").toLowerCase())).length, [visibleNodes]);

  useEffect(() => {
    if (!visibleIds.length) {
      if (selectionId) {
        setSelectionId("");
      }
      setSelectionIndex(-1);
      return;
    }

    if (!selectionId) {
      setSelectionIndex(-1);
      return;
    }

    if (visibleIds.includes(selectionId)) {
      const nextIndex = visibleIds.indexOf(selectionId);
      if (selectionIndex !== nextIndex) {
        setSelectionIndex(nextIndex);
      }
      return;
    }

    setSelectionIndex(-1);
    setSelectionId("");
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
      const { depends: nextDepends, artefacts } = await loadNodeArtefacts(selectionId, nodesById, edges);
      if (!active) {
        return;
      }

      setDepends(nextDepends);
      setArtefactsByNode((current) => ({
        ...current,
        [selectionId]: artefacts,
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
          decisionIndex: {},
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
  const roadmapTiers = Array.isArray(roadmap.tiers) ? roadmap.tiers : [];
  const backlog = roadmapTiers.flatMap((tier) => (Array.isArray(tier?.items) ? tier.items : []).map((item) => ({ ...item, tier: tier?.tier })));
  const pendingRows = Array.isArray(pending.pending) ? pending.pending : [];

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

  // One navigation vocabulary for lane and channel actions: showing a node
  // always lands on the map view, even from the console.
  const navigateToNode = useCallback(
    (nodeId) => {
      setView("map");
      onSelect(nodeId, { clearFilters: true });
    },
    [onSelect],
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
      const nextId = pickCanvasTarget(event, event.currentTarget, visibleIds, currentSelectionId);
      if (!nextId) {
        return;
      }

      event.preventDefault();
      onSelect(nextId, { clearFilters: false });
    },
    [visibleIds, onSelect],
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

      if (selectionIndex >= 0 || selectionId) {
        setSelectionIndex(-1);
        setSelectionId("");
      }
    },
    [query, kindFilter, stateFilter, evidenceMode, selectionIndex, selectionId, visibleIds],
  );

  useEffect(() => {
    window.addEventListener("keydown", onGlobalKey);
    return () => window.removeEventListener("keydown", onGlobalKey);
  }, [onGlobalKey]);

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
      <${StatusBezel} nodeCount=${nodes.length} dependencyCount=${dependencyCount} findings=${findings} blueprintPath=${blueprint.path} nextRecommended=${status.next_recommended} />
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
        view=${view}
        onView=${setView}
        parsed=${parsedQuery}
        visibleCount=${visibleCardCount}
        kindFilter=${kindFilter}
        stateFilter=${stateFilter}
        onQuery=${setQuery}
        onQueryKey=${onQueryKey}
        onKindFilter=${setKindFilter}
        onStateFilter=${setStateFilter}
        onClear=${() => {
          setQuery("");
          setKindFilter("all");
          setStateFilter("all");
        }}
      />
      <div class="instrument-workspace" data-evidence-state=${selectionId ? "expanded" : "collapsed"}>
        ${
          view === "console"
            ? html`<${Console}
                pendingRows=${pendingRows}
                frontier=${frontier}
                backlog=${backlog}
                onSelect=${navigateToNode}
              />`
            : html`
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
              />`
        }
      </div>
      <${StateLegend} />
      <${ChannelBar}
        active=${channel}
        findings=${findings}
        drift=${drift}
        pending=${pendingRows}
        changes=${changes}
        backlog=${backlog}
        onChannel=${setChannel}
        onItem=${navigateToNode}
        defaultCollapsed=${compact}
      />
    </main>
  `;
}

const root = document.getElementById("root");
if (root && preactReady) {
  render(html`<${App} />`, root);
}
