import { clsx, html, parseState } from "./utils.js";

/**
 * Data:
 *  - node: normalised graph node
 *  - isSelected: whether node is current selection
 *  - isNeighbour: whether connected to current selection
 *
 * Events:
 *  - onSelect(nodeId)
 */
function NodeModule({ node, isSelected, isNeighbour, onSelect, compact, optionId }) {
  const state = parseState(node.state);
  const fullId = String(node.id || "");
  const shortId = fullId.split(".").pop();
  const nodeName = String(node.name || "").trim();
  const description = String(node.description || "").trim();
  const shouldShowName = nodeName && nodeName !== fullId && nodeName !== shortId;
  const hasDescription = Boolean(description);
  return html`
    <button
      id=${optionId}
      type="button"
      role="option"
      aria-selected=${Boolean(isSelected)}
      tabIndex=${isSelected ? 0 : -1}
      class=${clsx("node-module", state, compact ? "compact" : "", isSelected && "selected", isNeighbour && "focused")}
      data-node-id=${fullId}
      onClick=${() => onSelect(node.id)}
      onKeyDown=${(event) => {
        if (event.key === "Enter" || event.key === " ") {
          event.preventDefault();
          onSelect(node.id);
        }
      }}
      title=${fullId}
      aria-label=${fullId}
    >
      <p class="node-id">${shortId}</p>
      ${
        shouldShowName
          ? html`
            <h3 class="node-name">${nodeName}</h3>
          `
          : null
      }
      ${
        compact
          ? null
          : hasDescription
            ? html`
                <p class="node-description">${description}</p>
              `
            : null
      }
      <span class="state-dot" aria-hidden="true"></span>
    </button>
  `;
}
export { NodeModule };
