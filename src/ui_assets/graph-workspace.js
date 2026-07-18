import { NodeModule } from "./node-module.js";
import { clsx, copy, html, parseState, useMemo } from "./utils.js";

const CELL_WIDTH_DESKTOP = 118;
const CELL_HEIGHT_DESKTOP = 104;
const CELL_WIDTH_COMPACT = 72;
const CELL_HEIGHT_COMPACT = 62;
const H_GAP = 10;
const V_GAP = 10;
const PADDING = 10;

function buildNeighbourMap(nodes, edges) {
  const ids = new Set((nodes || []).map((node) => String(node.id)));
  const byNode = new Map();
  const ownership = new Map();
  const normaliseKind = (value) => String(value || "").toLowerCase();

  for (const edge of edges || []) {
    const kind = normaliseKind(edge.kind);
    const from = String(edge.from || "");
    const to = String(edge.to || "");
    if (!from || !to || !ids.has(from) || !ids.has(to)) {
      continue;
    }

    if (kind === "dependency") {
      const source = byNode.get(from) || { incoming: [], outgoing: [] };
      const target = byNode.get(to) || { incoming: [], outgoing: [] };

      source.outgoing.push(to);
      target.incoming.push(from);
      byNode.set(from, source);
      byNode.set(to, target);
    }

    if (kind === "ownership") {
      const owners = ownership.get(to) || [];
      if (!owners.includes(from)) {
        owners.push(from);
      }
      ownership.set(to, owners);
    }
  }

  const normalise = new Map();
  for (const [id, value] of byNode.entries()) {
    normalise.set(id, {
      incoming: [...new Set(value.incoming)].filter((item) => ids.has(item)),
      outgoing: [...new Set(value.outgoing)].filter((item) => ids.has(item)),
    });
  }

  return { adjacency: normalise, ownership };
}

function buildLayout(nodes, compact) {
  const columns = compact ? 4 : 6;
  const rows = Math.ceil(nodes.length / columns) || 1;
  const cellWidth = compact ? CELL_WIDTH_COMPACT : CELL_WIDTH_DESKTOP;
  const cellHeight = compact ? CELL_HEIGHT_COMPACT : CELL_HEIGHT_DESKTOP;

  const positions = new Map(
    nodes.map((node, index) => {
      const row = Math.floor(index / columns);
      const column = index % columns;
      const id = String(node.id);
      const x = PADDING + column * (cellWidth + H_GAP);
      const y = PADDING + row * (cellHeight + V_GAP);
      return [
        id,
        {
          id,
          x,
          y,
          width: cellWidth,
          height: cellHeight,
          cx: x + cellWidth / 2,
          cy: y + cellHeight / 2,
        },
      ];
    }),
  );

  return {
    positions,
    width: columns * (cellWidth + H_GAP) + PADDING * 2 - H_GAP,
    height: rows * (cellHeight + V_GAP) + PADDING * 2 - V_GAP,
    columns,
  };
}

function buildEdgeLines(edges, layout) {
  const edgesToRender = Array.isArray(edges) ? edges : [];

  const dependencyLinks = edgesToRender
    .map((edge) => {
      const from = String(edge.from || "");
      const to = String(edge.to || "");
      if (!from || !to || from === to || String(edge.kind || "dependency").toLowerCase() !== "dependency") {
        return null;
      }

      const source = layout.positions.get(from);
      const target = layout.positions.get(to);
      if (!source || !target) {
        return null;
      }

      return {
        ...edge,
        from,
        to,
        fromX: source.cx,
        fromY: source.cy,
        toX: target.cx,
        toY: target.cy,
      };
    })
    .filter(Boolean);

  return {
    dependencyLinks,
    width: layout.width,
    height: layout.height,
  };
}

function nodeIsNeighbour(adjacency, selected, nodeId) {
  if (!selected) {
    return false;
  }

  if (nodeId === selected) {
    return true;
  }

  const links = adjacency.get(selected) || { incoming: [], outgoing: [] };
  return links.incoming.includes(nodeId) || links.outgoing.includes(nodeId);
}

function GraphWorkspace({ nodes, edges, selectionId, onSelect, onCanvasKeyNavigate, compact }) {
  const map = useMemo(() => buildNeighbourMap(nodes, edges), [nodes, edges]);
  const selected = String(selectionId || "");

  const layout = useMemo(() => buildLayout(nodes, compact), [nodes, compact]);
  const { dependencyLinks } = useMemo(() => buildEdgeLines(edges, layout), [edges, layout]);
  const stageClass = nodes.length ? "has-layout" : "is-empty";
  const activeNodeId = selected ? `node-${selected.replace(/[^a-zA-Z0-9_.:-]/g, "-")}` : "";

  return html`
    <section class="graph-canvas" aria-label=${copy("webui.graph")} role="region">
      <h2 class="graph-head">${copy("webui.graph")}</h2>
      <div
        class=${`graph-stage ${stageClass}`}
        data-columns=${layout.columns}
        role="listbox"
        aria-activedescendant=${activeNodeId}
        tabIndex=${0}
        onKeyDown=${(event) => onCanvasKeyNavigate(event, selected)}
      >
        <div class="graph-content" style=${{ position: "relative", width: `${layout.width}px`, height: `${layout.height}px` }}>
          <svg class="graph-svg" aria-hidden="true" viewBox=${`0 0 ${layout.width} ${layout.height}`}>
            ${dependencyLinks.map(
              (edge) => html`
              <g class="dependency-link" data-from=${edge.from} data-to=${edge.to}>
                <line x1=${edge.fromX} y1=${edge.fromY} x2=${edge.toX} y2=${edge.toY} />
                <text x=${(edge.fromX + edge.toX) / 2} y=${(edge.fromY + edge.toY) / 2 - 2} text-anchor="middle">
                  D
                </text>
              </g>`,
            )}
          </svg>
          ${nodes.map((node) => {
            const position = layout.positions.get(String(node.id));
            const owners = map.ownership.get(node.id) || [];
            const isNeighbour = nodeIsNeighbour(map.adjacency, selected, node.id);
            const ownershipCount = owners.length;

            if (!position) {
              return null;
            }

            const nodeId = `node-${String(node.id).replace(/[^a-zA-Z0-9_.:-]/g, "-")}`;

            return html`
              <div
                class=${clsx("node-shell", isNeighbour ? "focused" : "", node.state === "ghost" ? "ghost" : "")}
                style=${{
                  left: `${position.x}px`,
                  top: `${position.y}px`,
                  width: `${position.width}px`,
                  height: `${position.height}px`,
                  position: "absolute",
                }}
              >
                ${
                  ownershipCount
                    ? html`
                      <span class="ownership-bracket" aria-hidden="true" title=${`${copy("webui.owns")} ${ownershipCount}`}>
                        ${ownershipCount}
                      </span>
                    `
                    : null
                }
                ${
                  parseState(node.state) === "orphaned"
                    ? html`
                      <span class="ownership-marker" aria-hidden="true">
                        ${copy("webui.states.orphaned")}
                      </span>
                    `
                    : null
                }
                <${NodeModule}
                  node=${node}
                  compact=${compact}
                  isSelected=${node.id === selected}
                  isNeighbour=${isNeighbour}
                  onSelect=${onSelect}
                  optionId=${nodeId}
                  ariaSelected=${node.id === selected}
                  isOption=${true}
                />
              </div>
            `;
          })}
        </div>
      </div>
    </section>
  `;
}

export { GraphWorkspace };
