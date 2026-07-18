import { clsx, copy, html, parseState } from "./utils.js";

/**
 * Data:
 *  - node: normalised graph node
 *  - isSelected: whether node is current selection
 *  - isMatch: whether query matched
 *  - isNeighbour: whether connected to current selection
 *
 * Events:
 *  - onSelect(nodeId)
 */
function NodeModule({ node, isSelected, isMatch, isNeighbour, onSelect, compact }) {
  const state = parseState(node.state);
  const shortId = String(node.id || "").slice(0, 42);
  const shortName = String(node.name || copy("webui.no-name")).slice(0, compact ? 30 : 60);

  return html`
    <button
      type="button"
      class=${clsx("node-module", state, compact ? "compact" : "", isSelected && "selected", isMatch && "matched", isNeighbour && "focused")}
      onClick=${() => onSelect(node.id)}
      onKeyDown=${(event) => {
        if (event.key === "Enter" || event.key === " ") {
          event.preventDefault();
          onSelect(node.id);
        }
      }}
      title=${node.id}
      aria-label=${`${shortName} ${shortId}`}
    >
      <p class="node-id">${shortId}</p>
      <h3 class="node-name">${shortName || copy("webui.no-name")}</h3>
      <p class="node-meta">${copy("webui.state-label")}: ${copy(`webui.states.${state}`)}</p>
      ${
        compact
          ? null
          : html`
              <p class="node-meta">${node.description || copy("webui.no-description")}</p>
            `
      }
      <div class="node-counts">
        <span class="node-count">${copy("webui.paths-count")}: ${String((node.paths || []).length)}</span>
        <span class="node-count">${copy("webui.files-count")}: ${String((node.files || []).length)}</span>
      </div>
      <span class="state-dot" aria-hidden="true"></span>
    </button>
  `;
}

export { NodeModule };
