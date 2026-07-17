// Canvas chrome: overlay components that sit on top of the architecture
// map (coach mark, minimap). Kept separate from the graph canvas so both
// modules stay under the file-size guideline.

import { clsx, copy, displayState, html, useState } from "./utils.js";

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

export { ChainCoachMark, Minimap };
