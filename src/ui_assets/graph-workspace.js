import { buildAdjacency, buildEdgeLines, buildLayout, clamp, nodeIsNeighbour, normaliseKind } from "./graph-layout.js";
import { NodeModule } from "./node-module.js";
import { clsx, copy, html, useEffect, useMemo, useState } from "./utils.js";

function GraphWorkspace({ nodes, edges, selectionId, onSelect, onCanvasKeyNavigate, compact }) {
  const [stageSize, setStageSize] = useState({ width: 0, height: 0 });
  const stageWidth = stageSize.width;
  const selected = String(selectionId || "");
  const layout = useMemo(() => buildLayout(nodes, edges, compact, stageWidth), [nodes, edges, compact, stageWidth]);
  const adjacency = useMemo(() => buildAdjacency(layout), [layout]);
  const dependencyLinks = useMemo(() => buildEdgeLines(edges, layout, compact), [edges, layout, compact]);
  const activeSelection = adjacency.has(selected) ? selected : "";
  const stageClass = layout.modules.length ? "has-layout" : "is-empty";
  const activeNodeId = activeSelection ? `node-${activeSelection.replace(/[^a-zA-Z0-9_.:-]/g, "-")}` : "";
  const system = (nodes || []).find((node) => normaliseKind(node.kind) === "system" || String(node.id) === "cairn");
  const graphTitle = system ? String(system.name || system.id || copy("webui.project-name")) : copy("webui.project-name");
  const architectureLabel = copy("webui.architecture-map");
  const graphSeparator = copy("webui.graph-separator");

  useEffect(() => {
    const stage = document.querySelector("[data-graph-workspace='true']");
    if (!stage) {
      return;
    }

    const update = () => {
      const nextWidth = Math.max(0, stage.clientWidth);
      const nextHeight = Math.max(0, stage.clientHeight);
      setStageSize((current) => (current.width === nextWidth && current.height === nextHeight ? current : { width: nextWidth, height: nextHeight }));
    };

    update();
    if (typeof ResizeObserver === "undefined") {
      return;
    }

    const observer = new ResizeObserver(() => update());
    observer.observe(stage);
    return () => {
      observer.disconnect();
    };
  }, [compact]);

  return html`
    <section class="graph-canvas" aria-label=${copy("webui.graph")} role="region">
      <h2 class="graph-head">
        <span class="graph-title">${graphTitle}</span>
        <span class="graph-separator" aria-hidden="true">${graphSeparator}</span>
        <span class="graph-label">${architectureLabel}</span>
      </h2>
      <div
        class=${`graph-stage ${stageClass}`}
        data-compact=${compact}
        data-graph-workspace=${true}
        role="listbox"
        aria-activedescendant=${activeNodeId}
        tabIndex=${0}
        onKeyDown=${(event) => onCanvasKeyNavigate(event, selected)}
      >
        ${(() => {
          const fitScale = compact ? 1 : !stageSize.width || !stageSize.height || !layout.width || !layout.height ? 1 : clamp(Math.min(stageSize.width / layout.width, stageSize.height / layout.height), 1, 1.5);
          const graphFitStyle = compact ? { width: "100%", height: `${layout.height}px`, margin: 0 } : { width: `${layout.width * fitScale}px`, height: `${layout.height * fitScale}px`, margin: "auto" };
          const graphContentStyle = compact
            ? { position: "relative", width: "100%", height: `${layout.height}px`, transform: "none", transformOrigin: "0 0" }
            : {
                position: "relative",
                width: `${layout.width}px`,
                height: `${layout.height}px`,
                transform: `scale(${fitScale})`,
                transformOrigin: "0 0",
              };
          return html`
            <div class="graph-fit" style=${graphFitStyle}>
              <div class="graph-content" style=${graphContentStyle}>
                <svg class="graph-svg" aria-hidden="true" viewBox=${`0 0 ${layout.width} ${layout.height}`}>
                  <defs>
                    <marker id="dependency-arrow" markerWidth="6" markerHeight="6" refX="5" refY="3" orient="auto" markerUnits="strokeWidth">
                      <path d="M 0 0 L 6 3 L 0 6 z" />
                    </marker>
                  </defs>
                  ${dependencyLinks.map((edge) => {
                    const direction = edge.from === activeSelection ? "out" : edge.to === activeSelection ? "in" : "";
                    const isSelectedEdge = !activeSelection ? false : edge.from === activeSelection || edge.to === activeSelection;
                    return html`
                      <g
                        class=${clsx("dependency-link", direction && `is-${direction}`, activeSelection && !isSelectedEdge && "is-dimmed")}
                        data-from=${edge.from}
                        data-to=${edge.to}
                      >
                        <path d=${edge.path} marker-end="url(#dependency-arrow)" />
                      </g>
                    `;
                  })}
                </svg>
                ${layout.groups.map(
                  (group) => html`
                    <div
                      class="ownership-group"
                      aria-label=${group.label}
                      style=${{
                        left: `${group.x}px`,
                        top: `${group.y}px`,
                        width: `${group.width}px`,
                        height: `${group.height}px`,
                      }}
                    >
                      <span class="ownership-group-label">${group.label}</span>
                    </div>
                  `,
                )}
                ${layout.modules.map((node) => {
                  const id = String(node.id);
                  const position = layout.positions.get(id);
                  if (!position) {
                    return null;
                  }
                  const isNeighbour = nodeIsNeighbour(adjacency, activeSelection, id);
                  const isSelected = id === activeSelection;
                  const dimmed = Boolean(activeSelection) && !isNeighbour;
                  const nodeId = `node-${id.replace(/[^a-zA-Z0-9_.:-]/g, "-")}`;
                  return html`
                    <div
                      class=${clsx("node-shell", isNeighbour && "focused", dimmed && "dimmed", isSelected && "selected", node.state === "ghost" && "ghost")}
                      style=${{
                        left: `${position.x}px`,
                        top: `${position.y}px`,
                        width: `${position.width}px`,
                        height: `${position.height}px`,
                        position: "absolute",
                      }}
                      data-layer=${position.layer}
                      data-row=${position.row}
                      data-column=${position.localColumn}
                      data-x=${position.x}
                      data-y=${position.y}
                    >
                      <${NodeModule}
                        node=${node}
                        compact=${compact}
                        isSelected=${isSelected}
                        isNeighbour=${isNeighbour}
                        onSelect=${onSelect}
                        optionId=${nodeId}
                        ariaSelected=${isSelected}
                        isOption=${true}
                      />
                    </div>
                  `;
                })}
              </div>
            </div>
          `;
        })()}
      </div>
    </section>
  `;
}

export { GraphWorkspace };
