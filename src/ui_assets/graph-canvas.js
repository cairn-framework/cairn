/* Graph canvas: pannable/zoomable SVG map of System/Container/Module/Actor
 * nodes with ownership and dependency edges. Local viewport/pan state only.
 */

import { edgeMidpoint, ownershipPath } from "./layout.js";
import { balanceFromCount, clsx, copy, displayState, html, nodeSeverityById, truncate, useEffect, useMemo, useRef, useState } from "./utils.js";

// ==========================================================================
// Graph canvas
// ==========================================================================

function SystemNode({ node, selected, onSelect, dimmed, findingSeverity }) {
  const d = node.data;
  const strokeColor = selected ? "var(--seam-carved)" : findingSeverity === "error" ? "var(--drift)" : findingSeverity === "warning" ? "var(--orphaned)" : findingSeverity === "info" ? "var(--settled)" : "var(--seam-thin)";
  return html`
    <g class=${clsx("canvas-node", dimmed && "dimmed")}
       transform=${`translate(${node.x - node.width / 2}, ${node.y - node.height / 2})`}
       onClick=${() => onSelect(node)} data-kind="system">
      <rect x="0" y=${(node.height - 110) / 2} width=${node.width} height="110" fill="transparent" pointer-events="all"/>
      <rect width=${node.width} height=${node.height} rx="6"
            fill="var(--stone-3)"
            stroke=${strokeColor}
            stroke-width=${selected ? 1.5 : 1}/>
      <rect width=${node.width} height="1" fill="rgba(255,245,220,0.08)"/>
      <text x="14" y="20" font-size="10" font-family="var(--font-mono)"
            fill="var(--ink-faded)" letter-spacing="2.5" style="text-transform:uppercase">SYSTEM</text>
      <text x="14" y="42" font-size="17" font-family="var(--font-serif)"
            fill="var(--ink-char)" font-weight="500" letter-spacing="-0.3"
            style="font-variation-settings: 'opsz' 24">${d.name}</text>
      <text x="14" y="58" font-size="10.5" font-family="var(--font-mono)"
            fill="var(--ink-faded)" letter-spacing="0.5">${d.id}</text>
    </g>
  `;
}

function ContainerNode({ node, selected, onSelect, dimmed, findingSeverity }) {
  const d = node.data;
  const strokeColor = selected ? "var(--seam-carved)" : findingSeverity === "error" ? "var(--drift)" : findingSeverity === "warning" ? "var(--orphaned)" : findingSeverity === "info" ? "var(--settled)" : "var(--seam-thin)";
  return html`
    <g class=${clsx("canvas-node", dimmed && "dimmed")}
       transform=${`translate(${node.x - node.width / 2}, ${node.y - node.height / 2})`}
       onClick=${() => onSelect(node)} data-kind="container">
      <rect x="0" y=${(node.height - 110) / 2} width=${node.width} height="110" fill="transparent" pointer-events="all"/>
      <rect width=${node.width} height=${node.height} rx="6"
            fill="var(--stone-3)"
            stroke=${strokeColor}
            stroke-width=${selected ? 1.5 : 1}/>
      <rect width=${node.width} height="1" fill="rgba(255,245,220,0.08)"/>
      <text x="14" y="20" font-size="10" font-family="var(--font-mono)"
            fill="var(--ink-faded)" letter-spacing="2.5" style="text-transform:uppercase">CONTAINER</text>
      <text x="14" y="44" font-size="17" font-family="var(--font-serif)"
            fill="var(--ink-char)" font-weight="500" letter-spacing="-0.3"
            style="font-variation-settings: 'opsz' 24">${d.name}</text>
      <text x="14" y="62" font-size="10.5" font-family="var(--font-mono)"
            fill="var(--ink-faded)" letter-spacing="0.5">${d.id}</text>
    </g>
  `;
}

function ModuleNode({ node, selected, hovered, dimmed, findingSeverity, onSelect, onHover, dependentCount }) {
  const d = node.data;
  const recon = d.state || "synced";
  const base = displayState(recon);
  const breath = findingSeverity === "error" || findingSeverity === "warning" || base === "orphaned";
  const statusColor = base === "planned" ? "var(--planned)" : base === "orphaned" ? "var(--orphaned)" : "var(--synced)";
  const strokeColor = selected ? "var(--seam-carved)" : findingSeverity === "error" ? "var(--drift)" : findingSeverity === "warning" ? "var(--orphaned)" : findingSeverity === "info" ? "var(--settled)" : base === "planned" ? "var(--planned)" : base === "orphaned" ? "var(--orphaned)" : "var(--seam-thin)";

  const counts = node.counts || { provenance: 0, authority: 0, decisions: 0, contracts: 0 };
  const provStrength = Math.max(0.15, balanceFromCount(counts.provenance) / 5);
  const authStrength = Math.max(0.15, balanceFromCount(counts.authority) / 5);

  return html`
    <g class=${clsx("canvas-node", breath && "breathing", dimmed && "dimmed")}
       transform=${`translate(${node.x - node.width / 2}, ${node.y - node.height / 2})`}
       onClick=${() => onSelect(node)}
       onMouseEnter=${() => onHover(node.id)}
       onMouseLeave=${() => onHover(null)}
       data-kind="module">
      <rect x="0" y=${(node.height - 110) / 2} width=${node.width} height="110" fill="transparent" pointer-events="all"/>
      <rect x="2" y="3" width=${node.width} height=${node.height} rx="6" fill="rgba(0,0,0,0.3)"/>
      <rect width=${node.width} height=${node.height} rx="6"
            fill=${hovered ? "var(--stone-4)" : "var(--stone-3)"}
            stroke=${strokeColor}
            stroke-width=${selected ? 1.5 : 1}
            stroke-dasharray=${base === "orphaned" ? "2 3" : "0"}/>
      <rect width=${node.width} height="1" fill="rgba(255,245,220,0.1)"/>
      <rect x="0" y="0" width="3" height=${node.height}
            fill="var(--prov-2)" opacity=${provStrength * 0.7 + 0.3}/>
      <rect x=${node.width - 3} y="0" width="3" height=${node.height}
            fill="var(--auth-2)" opacity=${authStrength * 0.7 + 0.3}/>
      <text x="14" y="20" font-size="10" font-family="var(--font-mono)"
            fill="var(--ink-faded)" letter-spacing="2" style="text-transform:uppercase">MODULE</text>
      <text x="64" y="20" font-size="10" font-family="var(--font-mono)"
            fill="var(--ink-faded)" letter-spacing="1">· ${truncate(d.id, 24)}</text>
      <circle cx=${node.width - 16} cy="16" r="3.5" fill=${statusColor}/>
      ${
        breath
          ? html`<circle cx=${node.width - 16} cy="16" r="6" fill="none" stroke=${statusColor} stroke-width="1" opacity="0.4">
            <animate attributeName="r" values="4;8;4" dur="2.4s" repeatCount="indefinite"/>
            <animate attributeName="opacity" values="0.5;0;0.5" dur="2.4s" repeatCount="indefinite"/>
          </circle>`
          : null
      }
      ${
        findingSeverity
          ? html`<rect x=${node.width - 10} y="8" width="6" height="6" rx="1.5"
              fill=${findingSeverity === "error" ? "var(--drift)" : findingSeverity === "warning" ? "var(--orphaned)" : "var(--settled)"}/>`
          : null
      }
      <text x="14" y="46" font-size="17" font-family="var(--font-serif)"
            fill="var(--ink-char)" font-weight="500" letter-spacing="-0.3"
            style="font-variation-settings: 'opsz' 20">${truncate(d.name, 22)}</text>
      <text x="14" y="62" font-size="10.5" font-family="var(--font-mono)"
            fill="var(--ink-faded)" letter-spacing="0.4">${truncate(d.id, 28)}</text>
      ${
        base !== "synced"
          ? html`<g transform=${`translate(${node.width - 74}, 30)`}>
            <rect x="0" y="0" width="58" height="16" rx="3"
                  fill=${base === "planned" ? "var(--planned-wash)" : "var(--orphan-wash)"}
                  stroke=${base === "planned" ? "var(--planned)" : "var(--orphaned)"}
                  stroke-width="0.75"/>
            <text x="29" y="11" font-size="9" font-family="var(--font-mono)"
                  fill=${base === "planned" ? "var(--planned)" : "var(--orphaned)"}
                  letter-spacing="1.2" text-anchor="middle"
                  style="text-transform:uppercase">${base}</text>
          </g>`
          : null
      }
      <line x1="12" y1="78" x2=${node.width - 12} y2="78" stroke="var(--seam-faint)"/>
      <g transform="translate(14, 88)">
        <rect x="0" y="3" width="72" height="3" rx="1.5" fill="rgba(255,255,255,0.04)"/>
        <rect x=${72 - provStrength * 72} y="3" width=${provStrength * 72} height="3"
              rx="1.5" fill="var(--prov-2)"/>
        <circle cx="82" cy="4.5" r="2.5" fill="var(--hinge-1)"/>
        <rect x="92" y="3" width="72" height="3" rx="1.5" fill="rgba(255,255,255,0.04)"/>
        <rect x="92" y="3" width=${authStrength * 72} height="3" rx="1.5" fill="var(--auth-2)"/>
        <text x=${node.width - 28} y="7" font-size="9" font-family="var(--font-mono)"
              fill="var(--ink-faded)" letter-spacing="0.3" text-anchor="end">${dependentCount > 0 ? `${dependentCount} dep` : ""}</text>
      </g>
    </g>
  `;
}

function DividerNode({ node }) {
  return html`
    <g transform=${`translate(${node.x - node.width / 2}, ${node.y - node.height / 2})`}>
      <rect width=${node.width} height=${node.height} rx="6"
            fill="transparent" stroke="var(--seam-thin)" stroke-dasharray="4 4"/>
      <text x="14" y="32" font-size="11" font-family="var(--font-mono)"
            fill="var(--ink-faded)" letter-spacing="1.5" style="text-transform:uppercase">
        ${node.data.name}
      </text>
    </g>
  `;
}

function ChainCoachMark() {
  const [open, setOpen] = useState(() => {
    try {
      return localStorage.getItem("cairn:v2:chain-coach") !== "dismissed";
    } catch (_err) {
      return true;
    }
  });
  if (!open) return null;
  const dismiss = () => {
    setOpen(false);
    try {
      localStorage.setItem("cairn:v2:chain-coach", "dismissed");
    } catch (_err) {
      // storage disabled; the coach-mark remains dismissed for this session
    }
  };
  return html`
    <div class="chain-banner coach-mark" role="note">
      <div class="coach-mark-copy">${copy("webui.chain-coach-body")}</div>
      <div class="coach-mark-legend">
        <div class="label prov"><span class="rule"></span>Provenance</div>
        <div class="label hinge"><span class="rule"></span>Hinge<span class="rule"></span></div>
        <div class="label auth">Authority<span class="rule"></span></div>
      </div>
      <button class="coach-mark-dismiss" onClick=${dismiss}>${copy("webui.chain-coach-dismiss")}</button>
    </div>
  `;
}

function Minimap({ graph, selection, onSelect }) {
  const [hoveredId, setHoveredId] = useState(null);
  return html`
    <div class="graph-minimap" aria-label=${copy("webui.minimap-label")}>
      ${
        graph
          ? graph.nodes
              .filter((n) => n.kind === "module")
              .slice(0, 48)
              .map((m) => {
                const active = selection && selection.id === m.id;
                const state = displayState(m.state);
                return html`
                  <div class="mini-item" key=${m.id}
                    onMouseEnter=${() => setHoveredId(m.id)}
                    onMouseLeave=${() => setHoveredId(null)}>
                    <button class=${clsx("mini-dot", state, active && "active")}
                      type="button"
                      style="height:22px"
                      aria-label=${`${m.name}: ${state}`}
                      onFocus=${() => setHoveredId(m.id)}
                      onClick=${() => onSelect(m.id)}
                      onBlur=${() => setHoveredId(null)}
                      title=${`${m.name}: ${state}`}></button>
                    ${hoveredId === m.id ? html`<span class="mini-label">${m.name}</span>` : null}
                  </div>
                `;
              })
          : null
      }
    </div>
  `;
}
function GraphCanvas({ graph, layoutData, selection, hoveredId, lint, onSelect, onHover, edgeTrace, focusNodeIds = [], focusToken }) {
  const svgRef = useRef(null);
  const [viewport, setViewport] = useState({ x: 0, y: 0, zoom: 1 });
  const [panState, setPanState] = useState(null);
  const pointersRef = useRef(new Map());
  const gestureRef = useRef(null);
  const dragRef = useRef(false);
  const nodeSeverity = useMemo(() => nodeSeverityById(lint), [lint]);

  const { nodes, totalHeight } = layoutData;
  const nodesById = useMemo(() => {
    const map = new Map();
    for (const n of nodes) map.set(n.id, n);
    return map;
  }, [nodes]);

  useEffect(() => {
    if (!svgRef.current) return;
    const rect = svgRef.current.getBoundingClientRect();
    setViewport({ x: rect.width / 2 - 900, y: 40, zoom: 1 });
  }, [graph]);
  useEffect(() => {
    if (!svgRef.current || !focusToken || focusNodeIds.length === 0) return;
    const targets = focusNodeIds.map((id) => nodesById.get(id)).filter(Boolean);
    if (targets.length === 0) return;
    const rect = svgRef.current.getBoundingClientRect();
    const minX = Math.min(...targets.map((n) => n.x - n.width / 2));
    const maxX = Math.max(...targets.map((n) => n.x + n.width / 2));
    const minY = Math.min(...targets.map((n) => n.y - n.height / 2));
    const maxY = Math.max(...targets.map((n) => n.y + n.height / 2));
    const spanX = Math.max(1, maxX - minX);
    const spanY = Math.max(1, maxY - minY);
    const zoom = Math.max(0.4, Math.min(1.4, Math.min((rect.width - 48) / spanX, (rect.height - 96) / spanY)));
    setViewport({
      x: rect.width / 2 - ((minX + maxX) / 2) * zoom,
      y: rect.height / 2 - ((minY + maxY) / 2) * zoom,
      zoom,
    });
  }, [focusNodeIds, focusToken, nodesById]);

  const ownershipEdges = useMemo(() => {
    if (!graph) return [];
    return graph.edges
      .filter((e) => e.kind === "ownership")
      .map((e) => {
        const from = nodesById.get(e.from);
        const to = nodesById.get(e.to);
        if (!from || !to) return null;
        return { ...e, from, to, d: ownershipPath(from, to) };
      })
      .filter(Boolean);
  }, [graph, nodesById]);

  const dependencyEdges = useMemo(() => {
    if (!graph) return [];
    return graph.edges
      .filter((e) => e.kind === "dependency")
      .map((e) => {
        const from = nodesById.get(e.from);
        const to = nodesById.get(e.to);
        if (!from || !to) return null;
        return { ...e, from, to, d: ownershipPath(from, to) };
      })
      .filter(Boolean);
  }, [graph, nodesById]);

  const dependentCountById = useMemo(() => {
    const map = new Map();
    if (!graph) return map;
    for (const e of graph.edges) {
      if (e.kind !== "dependency") continue;
      map.set(e.to, (map.get(e.to) || 0) + 1);
    }
    return map;
  }, [graph]);

  const onPointerDown = (e) => {
    if (e.pointerType === "mouse" && e.button !== 0) return;
    pointersRef.current.set(e.pointerId, { x: e.clientX, y: e.clientY });
    dragRef.current = false;
    setPanState({ startX: e.clientX, startY: e.clientY, origX: viewport.x, origY: viewport.y });
    if (pointersRef.current.size === 2) {
      const [a, b] = [...pointersRef.current.values()];
      gestureRef.current = {
        distance: Math.max(1, Math.hypot(a.x - b.x, a.y - b.y)),
        zoom: viewport.zoom,
        x: viewport.x,
        y: viewport.y,
        centreX: (a.x + b.x) / 2,
        centreY: (a.y + b.y) / 2,
      };
    }
  };
  const onPointerMove = (e) => {
    if (!pointersRef.current.has(e.pointerId)) return;
    if (panState && !dragRef.current && Math.hypot(e.clientX - panState.startX, e.clientY - panState.startY) > 4) {
      e.currentTarget.setPointerCapture?.(e.pointerId);
      dragRef.current = true;
    }
    pointersRef.current.set(e.pointerId, { x: e.clientX, y: e.clientY });
    if (pointersRef.current.size >= 2 && gestureRef.current) {
      const [a, b] = [...pointersRef.current.values()];
      const gesture = gestureRef.current;
      const distance = Math.max(1, Math.hypot(a.x - b.x, a.y - b.y));
      const zoom = Math.max(0.4, Math.min(2, gesture.zoom * (distance / gesture.distance)));
      const rect = e.currentTarget.getBoundingClientRect();
      const localX = (a.x + b.x) / 2 - rect.left;
      const localY = (a.y + b.y) / 2 - rect.top;
      const startLocalX = gesture.centreX - rect.left;
      const startLocalY = gesture.centreY - rect.top;
      setViewport({
        zoom,
        x: localX - (startLocalX - gesture.x) * (zoom / gesture.zoom),
        y: localY - (startLocalY - gesture.y) * (zoom / gesture.zoom),
      });
      return;
    }
    if (!panState) return;
    setViewport((v) => ({
      ...v,
      x: panState.origX + (e.clientX - panState.startX),
      y: panState.origY + (e.clientY - panState.startY),
    }));
  };
  const onPointerUp = (e) => {
    pointersRef.current.delete(e.pointerId);
    gestureRef.current = null;
    if (pointersRef.current.size === 0) {
      setPanState(null);
      dragRef.current = false;
    } else {
      const [remaining] = [...pointersRef.current.values()];
      setPanState({ startX: remaining.x, startY: remaining.y, origX: viewport.x, origY: viewport.y });
    }
  };
  const onWheel = (e) => {
    if (e.ctrlKey || e.metaKey) {
      e.preventDefault();
      const delta = -e.deltaY * 0.002;
      setViewport((v) => ({ ...v, zoom: Math.max(0.4, Math.min(2.0, v.zoom + delta)) }));
    } else {
      setViewport((v) => ({ ...v, x: v.x - e.deltaX, y: v.y - e.deltaY }));
    }
  };

  const fit = () => {
    if (!svgRef.current) return;
    const rect = svgRef.current.getBoundingClientRect();
    setViewport({ x: rect.width / 2 - 900, y: 40, zoom: 1 });
  };

  const isTraced = (edge) => {
    if (!edgeTrace) return false;
    return edge.from.id === edgeTrace || edge.to.id === edgeTrace;
  };
  const isDimmed = (edge) => {
    if (!edgeTrace) return false;
    return edge.from.id !== edgeTrace && edge.to.id !== edgeTrace;
  };

  return html`
    <section class=${clsx("graph-canvas", panState && "panning")}
             onPointerDown=${onPointerDown} onPointerMove=${onPointerMove}
             onPointerUp=${onPointerUp} onPointerCancel=${onPointerUp}
             onWheel=${onWheel} aria-label="Architecture map">
      <div class="graph-bg"></div>
      <${ChainCoachMark}/>
      <svg ref=${svgRef} class="graph-svg" width="100%" height="100%">
        <g transform=${`translate(${viewport.x}, ${viewport.y}) scale(${viewport.zoom})`}>
          <line x1="900" y1="20" x2="900" y2=${totalHeight}
                stroke="var(--seam-clear)" stroke-dasharray="1 6" opacity="0.6"/>
          ${ownershipEdges.map(
            (e, i) => html`
            <path key=${`o-${i}`} class=${clsx("edge", isTraced(e) && "traced", isDimmed(e) && "dimmed")}
                  d=${e.d}/>
          `,
          )}
          ${dependencyEdges.map(
            (e, i) => html`
            <path key=${`d-${i}`} class=${clsx("edge dependency", isTraced(e) && "traced", isDimmed(e) && "dimmed")}
                  d=${e.d}/>
          `,
          )}
          ${ownershipEdges.map((e, i) => {
            const m = edgeMidpoint(e.from, e.to);
            return html`
              <g key=${`ol-${i}`} class=${clsx("edge-label", isTraced(e) && "traced", isDimmed(e) && "dimmed")}
                 transform=${`translate(${m.x}, ${m.y})`}
                 opacity=${isTraced(e) || !edgeTrace ? 1 : 0.3}>
                <text font-size="9" font-family="var(--font-mono)" fill="var(--ink-faded)"
                      text-anchor="middle" dy="-4">${e.description || ""}</text>
              </g>`;
          })}
          ${dependencyEdges.map((e, i) => {
            const m = edgeMidpoint(e.from, e.to);
            return html`
              <g key=${`dl-${i}`} class=${clsx("edge-label", isTraced(e) && "traced", isDimmed(e) && "dimmed")}
                 transform=${`translate(${m.x}, ${m.y})`}
                 opacity=${isTraced(e) || !edgeTrace ? 1 : 0.3}>
                <text font-size="9" font-family="var(--font-mono)" fill="var(--ink-faded)"
                      text-anchor="middle" dy="-4">${e.description || ""}</text>
              </g>`;
          })}
          ${nodes.map((n) => {
            const isSelected = selection && selection.id === n.id;
            const isHovered = hoveredId === n.id;
            const findingSeverity = nodeSeverity.get(n.id) || null;
            if (n.kind === "system")
              return html`<${SystemNode} key=${n.id} node=${n}
              selected=${isSelected} findingSeverity=${findingSeverity} onSelect=${(nd) => onSelect(nd.id)}/>`;
            if (n.kind === "container")
              return html`<${ContainerNode} key=${n.id} node=${n}
              selected=${isSelected} findingSeverity=${findingSeverity} onSelect=${(nd) => onSelect(nd.id)}/>`;
            if (n.kind === "divider") return html`<${DividerNode} key=${n.id} node=${n}/>`;
            return html`<${ModuleNode} key=${n.id} node=${n}
              selected=${isSelected} hovered=${isHovered}
              findingSeverity=${findingSeverity}
              onSelect=${(nd) => onSelect(nd.id)}
              onHover=${onHover}
              dependentCount=${dependentCountById.get(n.id) || 0}/>`;
          })}
        </g>
      </svg>

      <div class="graph-zoom" role="group" aria-label="Canvas zoom">
        <button title="Zoom in" aria-label="Zoom in"
          onClick=${() => setViewport((v) => ({ ...v, zoom: Math.min(2.0, v.zoom + 0.1) }))}>+</button>
        <div class="zoom-val">${Math.round(viewport.zoom * 100)}%</div>
        <button title="Zoom out" aria-label="Zoom out"
          onClick=${() => setViewport((v) => ({ ...v, zoom: Math.max(0.4, v.zoom - 0.1) }))}>−</button>
        <div class="sep"></div>
        <button class="reset" onClick=${fit}>fit</button>
      </div>

      <${Minimap} graph=${graph} selection=${selection} onSelect=${onSelect}/>

      <div class="graph-legend">
        <span class="sw synced"></span> synced
        <span class="sep"></span>
        <span class="sw planned"></span> planned
        <span class="sep"></span>
        <span class="sw orphaned"></span> orphaned
      </div>
    </section>
  `;
}

export { GraphCanvas };
