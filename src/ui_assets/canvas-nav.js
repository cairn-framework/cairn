/* Spatial keyboard navigation over the graph canvas: picks the next node id
 * for an arrow key from the rendered node positions. */

function parseNodeNumber(value, fallback) {
  const parsed = Number.parseFloat(String(value || ""));
  return Number.isFinite(parsed) ? parsed : fallback;
}

function collectCanvasNodes(panel, visibleIds) {
  return Array.from(panel.querySelectorAll(".node-shell .node-module[data-node-id]"))
    .map((button) => {
      const shell = button.closest(".node-shell");
      if (!shell) {
        return null;
      }

      const id = button.getAttribute("data-node-id") || "";
      const left = parseNodeNumber(shell.dataset.x, NaN);
      const top = parseNodeNumber(shell.dataset.y, NaN);
      const layer = Number.parseInt(shell.dataset.layer || "0", 10);
      return {
        id,
        x: Number.isFinite(left) ? left : parseNodeNumber(button.getAttribute("style")?.match(/left:\s*([^;]+);?/)?.[1], NaN),
        y: Number.isFinite(top) ? top : parseNodeNumber(button.getAttribute("style")?.match(/top:\s*([^;]+);?/)?.[1], NaN),
        layer: Number.isNaN(layer) ? 0 : layer,
      };
    })
    .filter((node) => node?.id && visibleIds.includes(node.id))
    .map((node) => ({
      ...node,
      x: Number.isFinite(node.x) ? node.x : 0,
      y: Number.isFinite(node.y) ? node.y : 0,
    }));
}

function directionalScore(key, source, candidate) {
  const deltaX = candidate.x - source.x;
  const deltaY = candidate.y - source.y;
  const deltaLayer = (candidate.layer - source.layer) * 1000;

  if (key === "ArrowRight") {
    return deltaX > 0 ? deltaX * 8 + Math.abs(deltaY) + Math.abs(deltaLayer) : null;
  }
  if (key === "ArrowLeft") {
    return deltaX < 0 ? -deltaX * 8 + Math.abs(deltaY) + Math.abs(deltaLayer) : null;
  }
  if (key === "ArrowDown") {
    return deltaY > 0 ? deltaY * 8 + Math.abs(deltaX) + Math.abs(deltaLayer) : null;
  }
  if (key === "ArrowUp") {
    return deltaY < 0 ? -deltaY * 8 + Math.abs(deltaX) + Math.abs(deltaLayer) : null;
  }
  return null;
}

function fallbackScore(key, source, candidate) {
  const deltaLayer = (candidate.layer - source.layer) * 900;
  if (key === "ArrowRight") {
    return candidate.layer >= source.layer ? deltaLayer + Math.abs(candidate.y - source.y) : Infinity;
  }
  if (key === "ArrowLeft") {
    return candidate.layer <= source.layer ? -deltaLayer + Math.abs(candidate.y - source.y) : Infinity;
  }
  if (key === "ArrowDown") {
    return candidate.y >= source.y ? candidate.y - source.y + Math.abs(candidate.layer - source.layer) : Infinity;
  }
  if (key === "ArrowUp") {
    return candidate.y <= source.y ? source.y - candidate.y + Math.abs(candidate.layer - source.layer) : Infinity;
  }
  return Infinity;
}

/** Resolve the next node id for an arrow key press on the canvas, or null. */
function pickCanvasTarget(event, panel, visibleIds, currentSelectionId) {
  if (!event.key.startsWith("Arrow") || !visibleIds.length || !panel) {
    return null;
  }

  const nodes = collectCanvasNodes(panel, visibleIds);
  if (!nodes.length) {
    return null;
  }

  const source = nodes.find((node) => node.id === currentSelectionId) || nodes.find((node) => node.id === visibleIds[0]) || nodes[0];
  if (!source) {
    return null;
  }

  const candidates = nodes.filter((node) => node.id !== source.id);
  if (!candidates.length) {
    return null;
  }

  let nextNode = null;
  let nextMetric = Infinity;
  for (const candidate of candidates) {
    const score = directionalScore(event.key, source, candidate);
    if (score !== null && score < nextMetric) {
      nextMetric = score;
      nextNode = candidate;
    }
  }

  if (!nextNode) {
    for (const candidate of candidates) {
      const metric = fallbackScore(event.key, source, candidate);
      if (metric < nextMetric) {
        nextMetric = metric;
        nextNode = candidate;
      }
    }
  }

  return nextNode ? nextNode.id : null;
}

export { pickCanvasTarget };
