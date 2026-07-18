/* Cairn webui v2 app shell.
 *
 * Composition root: bootstrap data fetch + state wiring between modules.
 */

import { ChannelBar } from "./channel-bar.js";
import { EvidenceRail, StateLegend } from "./evidence-rail.js";
import { GraphWorkSpace } from "./graph-workspace.js";
import { QueryRail } from "./query-rail.js";
import { mapEdgeRows, matchesQuery, parseQuery } from "./search.js";
import { StatusBezel } from "./status-bezel.js";
import { copy, fetchBlueprint, fetchDependents, fetchDepends, fetchGraph, fetchLint, fetchNodeArtefacts, fetchStatus, html, loadCopy, preactReady, render, useCallback, useEffect, useMemo, useState } from "./utils.js";

const DEFAULT_CHANNEL = "findings";
const STORAGE_KEY = "cairn:ui:selection";
const STORAGE_KEY_LEGACY = "cairn:v2:selection";

function readSelectionSeed(nodes) {
  const candidates = [STORAGE_KEY, STORAGE_KEY_LEGACY];

  try {
    for (const key of candidates) {
      const saved = window.localStorage?.getItem?.(key);
      if (saved && nodes.some((node) => node.id === saved)) {
        return saved;
      }
    }
  } catch {
    // best effort only
  }

  return nodes[0]?.id || "";
}

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

function App() {
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState("");
  const [compact, setCompact] = useState(window.innerWidth <= 990);

  const [graph, setGraph] = useState({ nodes: [], edges: [] });
  const [status, setStatus] = useState({});
  const [lint, setLint] = useState({});
  const [blueprint, setBlueprint] = useState({ path: "", source: "" });

  const [query, setQuery] = useState("");
  const [kindFilter, setKindFilter] = useState("all");
  const [stateFilter, setStateFilter] = useState("all");
  const [selectionId, setSelectionId] = useState("");
  const [selectionIndex, setSelectionIndex] = useState(0);
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
      try {
        await loadCopy();

        const [graphPayload, statusPayload, lintPayload, blueprintPayload] = await Promise.all([fetchGraph(), fetchStatus(), fetchLint(), fetchBlueprint()]);

        if (!active) {
          return;
        }

        setGraph({
          nodes: Array.isArray(graphPayload?.nodes) ? graphPayload.nodes : [],
          edges: Array.isArray(graphPayload?.edges) ? graphPayload.edges : [],
        });
        setStatus(statusPayload || {});
        setLint(lintPayload || {});
        setBlueprint(blueprintPayload || { path: "", source: "" });

        const seed = readSelectionSeed(Array.isArray(graphPayload?.nodes) ? graphPayload.nodes : []);
        setSelectionId(seed);
      } catch (cause) {
        if (active) {
          setError(cause?.message || copy("empty-states.map-failed.body"));
        }
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
  }, []);

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
      setSelectionIndex(-1);
      setSelectionId("");
      return;
    }

    if (selectionId && visibleIds.includes(selectionId)) {
      setSelectionIndex(visibleIds.indexOf(selectionId));
      return;
    }

    setSelectionIndex(0);
    setSelectionId(visibleIds[0]);
  }, [selectionId, visibleIds]);

  useEffect(() => {
    if (!selectionId || selectionIndex < 0 || !visibleIds.length) {
      return;
    }

    if (visibleIds[selectionIndex] !== selectionId) {
      setSelectionId(visibleIds[selectionIndex]);
    }
  }, [selectionIndex, visibleIds, selectionId]);

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
      const [contract, decisions, sources, rationale, dependsRaw, dependentsRaw] = await Promise.all([
        fetchNodeArtefacts(selectionId, "contract").catch(() => []),
        fetchNodeArtefacts(selectionId, "decisions").catch(() => []),
        fetchNodeArtefacts(selectionId, "sources").catch(() => []),
        fetchNodeArtefacts(selectionId, "rationale").catch(() => []),
        fetchDepends(selectionId).catch(() => []),
        fetchDependents(selectionId).catch(() => []),
      ]);

      if (!active) {
        return;
      }

      const mappedOut = remapNeighbours(nodesById, dependsRaw);
      const mappedIn = remapNeighbours(nodesById, dependentsRaw);
      const rows = mapEdgeRows(selectionId, edges);

      setDepends({
        in: mappedIn.length ? mappedIn : rows.in,
        out: mappedOut.length ? mappedOut : rows.out,
      });

      setArtefactsByNode((current) => ({
        ...current,
        [selectionId]: {
          contracts: Array.isArray(contract) ? contract : [],
          decisions: Array.isArray(decisions) ? decisions : [],
          sources: Array.isArray(sources) ? sources : [],
          evidence: Array.isArray(rationale) ? rationale : [],
          symbols: [],
        },
      }));
    }

    loadArtefacts().catch(() => {
      if (!active) {
        return;
      }
      setArtefactsByNode((current) => ({
        ...current,
        [selectionId]: {
          contracts: [],
          decisions: [],
          sources: [],
          evidence: [],
          symbols: [],
        },
      }));
      setDepends(mapEdgeRows(selectionId, edges));
    });

    return () => {
      active = false;
    };
  }, [selectionId, edges, nodesById]);

  const findings = Array.isArray(lint.findings) ? lint.findings : [];
  const drift = findings.filter((item) => item.severity === "error" || item.severity === "warning");
  const changes = Array.isArray(lint.changes) ? lint.changes : [];
  const backlog = Array.isArray(lint.todos) && lint.todos.length ? lint.todos : Array.isArray(lint.backlog) ? lint.backlog : [];

  const selectionArtefacts = useMemo(() => artefactsByNode[selectionId] || {}, [artefactsByNode, selectionId]);

  const onSelect = useCallback(
    (nodeId) => {
      if (!nodeId) {
        return;
      }
      setSelectedLineageItem(null);
      setEvidenceMode("depth");
      setSelectionId(nodeId);
      setSelectionIndex((visibleIds.includes(nodeId) ? visibleIds.indexOf(nodeId) : 0) || 0);
    },
    [visibleIds],
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
      if (event.key === "ArrowDown") {
        setSelectionIndex((selectionIndex + 1) % visibleIds.length);
        return;
      }

      setSelectionIndex((selectionIndex - 1 + visibleIds.length) % visibleIds.length);
    },
    [selectionIndex, visibleIds],
  );

  const onCanvasKeyNavigate = useCallback(
    (event, _allNodes, currentSelectionId) => {
      if (!event.key.startsWith("Arrow") || !visibleIds.length || !currentSelectionId) {
        return;
      }

      const columns = compact ? 4 : 6;
      const index = visibleIds.indexOf(currentSelectionId);
      if (index < 0) {
        return;
      }

      const next = (() => {
        if (event.key === "ArrowLeft") return Math.max(0, index - 1);
        if (event.key === "ArrowRight") return Math.min(visibleIds.length - 1, index + 1);
        if (event.key === "ArrowUp") return Math.max(0, index - columns);
        return Math.min(visibleIds.length - 1, index + columns);
      })();

      if (next === index) {
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

      if (query) {
        setQuery("");
        setKindFilter("all");
        setStateFilter("all");
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
    [query, evidenceMode, selectionIndex, visibleIds],
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

      const sourceId = artefact.id || artefact.node || artefact.slug;
      if (sourceId && nodesById.has(sourceId)) {
        setSelectionId(sourceId);
        setSelectedLineageItem(null);
        setEvidenceMode("depth");
        return;
      }

      setSelectedLineageItem(artefact);
      setEvidenceMode("lineage");
    },
    [nodesById],
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
      </main>
    `;
  }

  return html`
    <main class="instrument-shell" aria-label=${copy("webui.app")} data-compact=${compact}>
      <${StatusBezel} graphNodes=${nodes} status=${status} />
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
        <${GraphWorkSpace}
          nodes=${visibleNodes}
          edges=${edges}
          compact=${compact}
          selectionId=${selectionId}
          matches=${visibleIds}
          onSelect=${onSelect}
          onCanvasKeyNavigate=${onCanvasKeyNavigate}
        />
        <${EvidenceRail}
          mode=${evidenceMode}
          onMode=${onMode}
          selection=${selection}
          inRows=${depends.in}
          outRows=${depends.out}
          onNeighbourSelect=${onSelect}
          artefacts=${selectionArtefacts}
          blueprint=${blueprint.source}
          blueprintPath=${blueprint.path}
          onLineageOpen=${onLineageSelect}
          selectedLineageItem=${selectedLineageItem}
          onLineageBack=${() => setSelectedLineageItem(null)}
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
        onItem=${onSelect}
      />
    </main>
  `;
}

const root = document.getElementById("root");
if (root && preactReady) {
  render(html`<${App} />`, root);
}
