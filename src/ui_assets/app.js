/* Cairn webui v2. Preact + htm, vendored runtime.
 *
 * All colors, spacing, radii, motion, and type come from docs/design-system/tokens.css.
 * Do not hardcode hex or rem values here. All user-visible prose uses plain English
 * punctuation (no em-dashes). Paths and slugs are in IBM Plex Mono; titles are in
 * Source Serif 4; UI labels are in IBM Plex Sans.
 *
 * Data flow:
 *   boot          -> GET /api/graph, GET /api/lint
 *   select node   -> GET /api/node/:id plus six artefact kinds plus depends/dependents
 *   view source   -> GET /api/blueprint
 *
 * Feature modules own their local UI state; this composition root wires
 * shared boot data (graph, lint) and selection state across them.
 */

import { BlueprintModal } from "./blueprint-modal.js";
import { CommandPalette } from "./command-palette.js";
import { DecisionDetail } from "./decision-detail.js";
import { ChangesDrawer } from "./findings-panel.js";
import { GraphCanvas } from "./graph-canvas.js";
import { EmptyInspector, ModuleInspector } from "./inspector.js";
import { buildLayout } from "./layout.js";
import { TopBar } from "./top-bar.js";
import { copy, Fragment, fetchBlueprint, fetchDependents, fetchDepends, fetchGraph, fetchLint, fetchMeta, fetchNodeArtefacts, fetchNodeBeads, fetchNodeSymbols, h, html, loadCopy, preactReady, render, useCallback, useEffect, useMemo, useState } from "./utils.js";

// ==========================================================================
// App root
// ==========================================================================

function App() {
  const [graph, setGraph] = useState(null);
  const [lint, setLint] = useState(null);
  const [meta, setMeta] = useState(null);
  const [error, setError] = useState(null);

  const [selectionId, setSelectionId] = useState(null);
  const [selectedDecision, setSelectedDecision] = useState(null);
  const [detail, setDetail] = useState({});
  const [hoveredId, setHoveredId] = useState(null);
  const [cmdOpen, setCmdOpen] = useState(false);
  const [drawerOpen, setDrawerOpen] = useState(false);
  const [blueprintOpen, setBlueprintOpen] = useState(false);
  const [blueprint, setBlueprint] = useState(null);
  const [blueprintFocus, setBlueprintFocus] = useState(null);
  const [bootTick, setBootTick] = useState(0);

  useEffect(() => {
    let cancelled = false;
    setError(null);
    setGraph(null);
    setLint(null);
    setMeta(null);
    Promise.all([fetchGraph(), fetchLint()])
      .then(([g, l]) => {
        if (cancelled) return;
        if (!g || !Array.isArray(g.nodes) || !Array.isArray(g.edges)) {
          setError(copy("empty-states.map-failed.body"));
          return;
        }
        setGraph(g);
        setLint(l);
        // Fetch metadata only after the map is resolved so its
        // last_reconciled timestamp describes the scan just rendered.
        return fetchMeta().catch(() => null);
      })
      .then((m) => {
        if (cancelled) return;
        if (m) setMeta(m);
      })
      .catch((err) => {
        if (!cancelled) setError(err.message);
      });
    return () => {
      cancelled = true;
    };
  }, [bootTick]);

  useEffect(() => {
    try {
      const saved = localStorage.getItem("cairn:v2:selection");
      if (saved) setSelectionId(saved);
    } catch (_err) {
      // storage disabled; ignore
    }
  }, []);

  useEffect(() => {
    try {
      if (selectionId) localStorage.setItem("cairn:v2:selection", selectionId);
      else localStorage.removeItem("cairn:v2:selection");
    } catch (_err) {
      // ignore
    }
  }, [selectionId]);

  useEffect(() => {
    const onKey = (e) => {
      if ((e.metaKey || e.ctrlKey) && e.key.toLowerCase() === "k") {
        e.preventDefault();
        setCmdOpen((o) => !o);
      }
      if (e.key === "Escape" && !cmdOpen && !blueprintOpen) {
        if (selectedDecision) setSelectedDecision(null);
      }
    };
    window.addEventListener("keydown", onKey);
    return () => window.removeEventListener("keydown", onKey);
  }, [cmdOpen, blueprintOpen, selectedDecision]);

  const nodesById = useMemo(() => {
    const map = new Map();
    if (graph) for (const n of graph.nodes) map.set(n.id, n);
    return map;
  }, [graph]);

  const artefactCountsById = useMemo(() => {
    const map = new Map();
    if (detail && selectionId) {
      map.set(selectionId, {
        provenance: (detail.sources?.length || 0) + (detail.research?.length || 0),
        authority: (detail.contracts?.length || 0) + (detail.decisions?.length || 0),
        decisions: detail.decisions?.length || 0,
        contracts: detail.contracts?.length || 0,
      });
    }
    return map;
  }, [detail, selectionId]);

  const layoutData = useMemo(() => buildLayout(graph, artefactCountsById), [graph, artefactCountsById]);

  const focusNodeIds = useMemo(() => {
    if (!selectionId || !graph) return [];
    const ids = new Set([selectionId]);
    for (const edge of graph.edges) {
      if (edge.kind !== "dependency") continue;
      if (edge.from === selectionId) ids.add(edge.to);
      if (edge.to === selectionId) ids.add(edge.from);
    }
    return [...ids];
  }, [graph, selectionId]);

  useEffect(() => {
    if (!selectionId || !graph) {
      setDetail({});
      return undefined;
    }
    const node = nodesById.get(selectionId);
    if (!node) {
      setDetail({});
      return undefined;
    }
    let cancelled = false;
    setDetail({ loading: true });
    Promise.all([
      fetchNodeArtefacts(selectionId, "contract"),
      fetchNodeArtefacts(selectionId, "decisions"),
      fetchNodeArtefacts(selectionId, "todos"),
      fetchNodeBeads(selectionId).catch(() => []),
      fetchNodeArtefacts(selectionId, "research"),
      fetchNodeArtefacts(selectionId, "sources"),
      fetchDepends(selectionId).catch(() => []),
      fetchDependents(selectionId).catch(() => []),
      fetchNodeSymbols(selectionId).catch(() => []),
    ])
      .then(([contracts, decisions, todos, beads, research, sources, depends, dependents, symbols]) => {
        if (cancelled) return;
        // The deps spine op returns bare node-ID strings; hydrate them from
        // the loaded graph. Object entries (legacy shape) pass through.
        const hydrateDep = (entry) => {
          if (typeof entry !== "string") return entry;
          const record = nodesById.get(entry);
          return record ? { id: record.id, name: record.name, state: record.state } : { id: entry, name: entry, state: "synced" };
        };
        setDetail({
          contracts,
          decisions,
          todos,
          beads,
          research,
          sources,
          depends: depends.map(hydrateDep),
          dependents: dependents.map(hydrateDep),
          symbols,
        });
      })
      .catch(() => {
        if (!cancelled) setDetail({ failed: true });
      });
    return () => {
      cancelled = true;
    };
  }, [selectionId, nodesById, graph]);

  const openBlueprint = useCallback(() => {
    setBlueprintFocus(selectionId);
    setBlueprintOpen(true);
    if (!blueprint) {
      fetchBlueprint()
        .then((bp) => setBlueprint(bp))
        .catch(() => setBlueprint({ source: null, path: null }));
    }
  }, [selectionId, blueprint]);

  const selectedNode = selectionId ? nodesById.get(selectionId) : null;

  const inspector = selectedDecision
    ? html`<${DecisionDetail}
        decision=${selectedDecision}
        node=${selectedNode}
        onBack=${() => setSelectedDecision(null)}
        onSelect=${(id) => {
          setSelectedDecision(null);
          setSelectionId(id);
        }}
      />`
    : selectedNode
      ? html`<${ModuleInspector}
          node=${selectedNode}
          detail=${detail}
          lint=${lint}
          onSelect=${(id) => setSelectionId(id)}
          onSelectDecision=${(d) => setSelectedDecision(d)}
          onViewBlueprint=${openBlueprint}
          onClose=${() => setSelectionId(null)}
        />`
      : html`<${EmptyInspector}
          graph=${graph}
          lint=${lint}
          onShowFindings=${() => setDrawerOpen(true)}
          onOpenCmd=${() => setCmdOpen(true)}
        />`;

  return html`
    <${Fragment}>
      <${TopBar}
        graph=${graph}
        lint=${lint}
        selection=${selectionId ? { id: selectionId } : null}
        nodesById=${nodesById}
        onClear=${(id) => setSelectionId(id || null)}
        onOpenCmd=${() => setCmdOpen(true)}
        onOpenBlueprint=${openBlueprint}
        version=${meta?.version}
      />
      <div class="main">
        ${
          error
            ? html`<section class="graph-canvas canvas-state" aria-label="Architecture map">
              <div class="empty-state">
                <h2 class="empty-state-heading">${copy("empty-states.map-failed.heading")}</h2>
                <p class="empty-state-body">${error}</p>
                <button class="btn secondary" onClick=${() => setBootTick((t) => t + 1)}>${copy("empty-states.map-failed.cta")}</button>
              </div>
            </section>`
            : !graph
              ? html`<section class="graph-canvas canvas-state" aria-label="Architecture map">
                <div class="row-empty">${copy("empty-states.map-loading.body")}</div>
              </section>`
              : html`<${GraphCanvas}
                graph=${graph}
                layoutData=${layoutData}
                selection=${selectionId ? { id: selectionId } : null}
                hoveredId=${hoveredId}
                lint=${lint}
                onSelect=${(id) => setSelectionId(id)}
                onHover=${setHoveredId}
                focusNodeIds=${focusNodeIds}
                focusToken=${`${selectionId}:${focusNodeIds.join(",")}`}
                edgeTrace=${hoveredId}
              />`
        }
        <aside class="inspector-wrap" aria-live="polite">
          ${inspector}
        </aside>
      </div>
      <${ChangesDrawer}
        open=${drawerOpen}
        onToggle=${() => setDrawerOpen((o) => !o)}
        lint=${lint}
        meta=${meta}
        onSelect=${(id) => setSelectionId(id)}
      />
      <${CommandPalette}
        open=${cmdOpen}
        graph=${graph}
        onClose=${() => setCmdOpen(false)}
        onSelect=${(id) => setSelectionId(id)}
        version=${meta?.version}
      />
      <${BlueprintModal}
        open=${blueprintOpen}
        blueprint=${blueprint}
        focusModuleId=${blueprintFocus}
        onClose=${() => setBlueprintOpen(false)}
      />
    <//>
  `;
}

const root = document.getElementById("root");
if (root && preactReady) loadCopy().then(() => render(h(App, {}), root));
