/* Pure layout math for the graph workspace: layered dependency layout,
 * edge routing, and adjacency. No rendering. */

const NODE_WIDTH = 104;
const H_GAP = 12;
const V_GAP = 10;
const PADDING = 16;
const GROUP_INSET = 10;
const GROUP_LABEL_HEIGHT = 24;
const LAYER_ROWS = 6;
const MIN_NODE_WIDTH = 90;
const MAX_NODE_WIDTH = 230;
const MIN_NODE_HEIGHT = 56;
const MAX_NODE_HEIGHT = 92;

function normaliseKind(value) {
  return String(value || "").toLowerCase();
}

function clamp(value, min, max) {
  return Math.min(max, Math.max(min, value));
}

function estimateNodeWidth(availableWidth, columns) {
  if (availableWidth <= 0 || !columns) {
    return NODE_WIDTH;
  }

  const usable = Math.max(0, availableWidth);
  const cols = Math.max(1, columns);
  const gapTotal = H_GAP * (cols - 1);
  const widthFromSpace = Math.floor((usable - gapTotal) / cols);
  return clamp(widthFromSpace, MIN_NODE_WIDTH, MAX_NODE_WIDTH);
}

function estimateNodeHeight(width) {
  return clamp(Math.round(54 + width * 0.17), MIN_NODE_HEIGHT, MAX_NODE_HEIGHT);
}

function graphSets(nodes, edges) {
  const allNodes = Array.isArray(nodes) ? nodes : [];
  const ids = new Set(allNodes.map((node) => String(node.id)));

  const modules = allNodes.filter((node) => {
    const kind = normaliseKind(node.kind);
    return kind === "module" || (kind !== "system" && kind !== "container" && String(node.id) !== "cairn");
  });
  const containers = allNodes.filter((node) => normaliseKind(node.kind) === "container" || String(node.id) === "cairn.kernel");

  const dependency = [];
  const ownership = [];

  for (const edge of edges || []) {
    const from = String(edge.from || "");
    const to = String(edge.to || "");
    const kind = normaliseKind(edge.kind);
    if (!ids.has(from) || !ids.has(to) || from === to) {
      continue;
    }
    if (kind === "dependency") {
      dependency.push({ ...edge, from, to });
    } else if (kind === "ownership") {
      ownership.push({ ...edge, from, to });
    }
  }

  const moduleIds = new Set(modules.map((node) => String(node.id)));
  const outgoing = new Map(modules.map((node) => [String(node.id), []]));
  const incoming = new Map(modules.map((node) => [String(node.id), []]));

  for (const edge of dependency) {
    if (!moduleIds.has(edge.from) || !moduleIds.has(edge.to)) {
      continue;
    }
    outgoing.get(edge.from).push(edge.to);
    incoming.get(edge.to).push(edge.from);
  }

  const members = new Map(containers.map((node) => [String(node.id), new Set()]));
  for (const module of modules) {
    const id = String(module.id);
    for (const container of containers) {
      const containerId = String(container.id);
      if (id.startsWith(`${containerId}.`)) {
        members.get(containerId).add(id);
      }
    }
  }

  for (const edge of ownership) {
    if (members.has(edge.from) && moduleIds.has(edge.to)) {
      members.get(edge.from).add(edge.to);
    }
  }

  return { modules, containers, dependency, outgoing, incoming, members };
}

function longestDepth(id, outgoing, visiting = new Set(), memo = new Map()) {
  if (memo.has(id)) {
    return memo.get(id);
  }
  if (visiting.has(id)) {
    return 0;
  }
  visiting.add(id);
  const next = outgoing.get(id) || [];
  const depth = Math.max(0, ...next.map((target) => longestDepth(target, outgoing, visiting, memo) + 1));
  visiting.delete(id);
  memo.set(id, depth);
  return depth;
}

function buildLayout(nodes, edges, compact, stageWidth = 0) {
  const sets = graphSets(nodes, edges);
  const baseRowLimit = compact ? 3 : LAYER_ROWS;
  const compactAvailableWidth = compactGraphLayoutWidth(compact, stageWidth);
  const availableWidth = compact ? compactAvailableWidth : Math.max(0, stageWidth - PADDING * 2);
  const depths = new Map();
  for (const module of sets.modules) {
    depths.set(String(module.id), longestDepth(String(module.id), sets.outgoing));
  }
  const maxDepth = Math.max(0, ...depths.values());

  const layersByRaw = new Map();
  const isolated = [];

  for (const module of sets.modules) {
    const id = String(module.id);
    const hasIn = (sets.incoming.get(id) || []).length > 0;
    const hasOut = (sets.outgoing.get(id) || []).length > 0;

    if (hasIn || hasOut) {
      const rawLayer = maxDepth - (depths.get(id) || 0);
      layersByRaw.set(id, rawLayer);
      continue;
    }

    isolated.push(id);
  }

  isolated.sort((a, b) => a.localeCompare(b));
  const isolatedSet = new Set(isolated);

  const rawLayerMax = Math.max(...layersByRaw.values(), 0);
  const desiredColumns = Math.max(1, compact ? 1 : Math.max(1, Math.floor((availableWidth + H_GAP) / (MIN_NODE_WIDTH + H_GAP))));
  const maxColumns = compact ? 1 : Math.max(1, Math.min(layersByRaw.size, desiredColumns));
  const displayFactor = maxColumns <= 1 ? 0 : rawLayerMax / (maxColumns - 1);

  const layers = new Map();
  for (const module of sets.modules) {
    const id = String(module.id);
    if (isolatedSet.has(id)) {
      continue;
    }
    const rawLayer = layersByRaw.get(id) || 0;
    const layer = compact ? rawLayer : maxColumns <= 1 || rawLayerMax === 0 ? 0 : Math.round(rawLayer / displayFactor);

    const bucket = layers.get(layer) || [];
    bucket.push(module);
    layers.set(layer, bucket);
  }

  const orderedLayers = [...layers.keys()].sort((a, b) => a - b);
  let rowLimit = baseRowLimit;
  let bucketsByLayer = [];
  let columns = 1;

  const resolveBuckets = (activeRowLimit) => {
    const mapped = orderedLayers.map((layer) => {
      const bucket = (layers.get(layer) || []).sort((a, b) => String(a.id).localeCompare(String(b.id)));
      const span = Math.max(1, Math.ceil(bucket.length / Math.max(1, activeRowLimit)));
      return {
        layer,
        bucket,
        span,
      };
    });
    const spanTotal = Math.max(
      1,
      mapped.reduce((acc, item) => acc + item.span, 0),
    );
    return { mapped, spanTotal };
  };

  while (true) {
    const result = resolveBuckets(rowLimit);
    bucketsByLayer = result.mapped;
    columns = result.spanTotal;
    if (compact || columns <= maxColumns || rowLimit >= orderedLayers.length + 1) {
      break;
    }
    rowLimit += 1;
  }

  const nodeWidth = compact ? Math.max(MIN_NODE_WIDTH, compactAvailableWidth) : stageWidth > 0 ? estimateNodeWidth(availableWidth, columns) : NODE_WIDTH;
  const nodeHeight = compact ? MIN_NODE_HEIGHT : estimateNodeHeight(nodeWidth);
  const nodeStrideX = nodeWidth + H_GAP;
  const nodeStrideY = nodeHeight + V_GAP;

  const positions = new Map();
  let maxY = PADDING;
  let maxX = PADDING;
  let globalColumn = 0;
  let compactIndex = 0;
  for (const layer of bucketsByLayer) {
    const { layer: layerKey, bucket, span } = layer;
    const layerStartY = compact ? compactIndex * nodeStrideY : PADDING;
    const layerBaseX = compact ? PADDING : PADDING + globalColumn * nodeStrideX;

    bucket.forEach((node, index) => {
      const localColumn = compact ? 0 : Math.floor(index / rowLimit);
      const row = compact ? index : index - localColumn * rowLimit;
      const x = compact ? PADDING : layerBaseX + localColumn * nodeStrideX;
      const y = compact ? layerStartY + index * nodeStrideY : PADDING + row * nodeStrideY;
      const id = String(node.id);
      positions.set(id, {
        id,
        x,
        y,
        width: nodeWidth,
        height: nodeHeight,
        cx: x + nodeWidth / 2,
        cy: y + nodeHeight / 2,
        layer: layerKey,
        row,
        localColumn,
      });
      compactIndex += 1;
      maxY = Math.max(maxY, y + nodeHeight);
      maxX = Math.max(maxX, x + nodeWidth);
    });

    if (compact) {
      continue;
    }

    globalColumn += span;
  }

  if (isolated.length && !compact) {
    const connectedWidth = Math.max(maxX - PADDING, nodeStrideX);
    const bandColumns = Math.max(1, Math.min(isolated.length, Math.floor((Math.max(connectedWidth, availableWidth) + H_GAP) / nodeStrideX)));
    const bandTop = maxY + V_GAP * 3;
    isolated.forEach((id, index) => {
      const row = Math.floor(index / bandColumns);
      const column = index % bandColumns;
      const x = PADDING + column * nodeStrideX;
      const y = bandTop + row * nodeStrideY;
      positions.set(id, {
        id,
        x,
        y,
        width: nodeWidth,
        height: nodeHeight,
        cx: x + nodeWidth / 2,
        cy: y + nodeHeight / 2,
        layer: rawLayerMax + 1,
        row,
        localColumn: column,
      });
      maxY = Math.max(maxY, y + nodeHeight);
      maxX = Math.max(maxX, x + nodeWidth);
    });
  } else if (isolated.length) {
    isolated.forEach((id) => {
      const y = PADDING + compactIndex * nodeStrideY;
      positions.set(id, {
        id,
        x: PADDING,
        y,
        width: nodeWidth,
        height: nodeHeight,
        cx: PADDING + nodeWidth / 2,
        cy: y + nodeHeight / 2,
        layer: rawLayerMax + 1,
        row: compactIndex,
        localColumn: 0,
      });
      compactIndex += 1;
      maxY = Math.max(maxY, y + nodeHeight);
      maxX = Math.max(maxX, PADDING + nodeWidth);
    });
  }

  const groups = [];
  for (const container of sets.containers) {
    const id = String(container.id);
    const members = [...(sets.members.get(id) || [])].map((memberId) => positions.get(memberId)).filter(Boolean);
    if (!members.length) {
      continue;
    }

    const left = Math.max(PADDING, Math.min(...members.map((member) => member.x)) - GROUP_INSET);
    const top = Math.max(PADDING, Math.min(...members.map((member) => member.y)) - GROUP_LABEL_HEIGHT);
    const right = Math.max(...members.map((member) => member.x + member.width)) + GROUP_INSET;
    const bottom = Math.max(...members.map((member) => member.y + member.height)) + GROUP_INSET;

    groups.push({
      id,
      label: String(container.name || id.split(".").pop()),
      x: left,
      y: top,
      width: right - left,
      height: Math.max(bottom - top, 0),
    });
  }

  const stageTargetWidth = Math.max(PADDING * 2, maxX + PADDING);
  const stageTargetHeight = Math.max(PADDING * 2, maxY + PADDING);
  return {
    ...sets,
    positions,
    groups,
    columns,
    width: stageTargetWidth,
    height: stageTargetHeight,
  };
}

function edgePath(source, target, compact = false) {
  if (compact) {
    const startX = source.cx;
    const startY = source.y + source.height;
    const endX = target.cx;
    const endY = target.y;
    const guide = Math.max(18, Math.min(80, Math.abs(endY - startY) * 0.4));
    const firstY = endY >= startY ? startY + guide : startY - guide;
    const secondY = endY >= startY ? endY - guide : endY + guide;
    return `M ${startX} ${startY} C ${startX} ${firstY}, ${endX} ${secondY}, ${endX} ${endY}`;
  }

  const targetIsRight = target.x >= source.x;
  const startX = targetIsRight ? source.x + source.width : source.x;
  const startY = source.cy;
  const endX = targetIsRight ? target.x : target.x + target.width;
  const endY = target.cy;
  const guide = Math.max(22, Math.min(78, Math.abs(endX - startX) * 0.4));
  const firstX = targetIsRight ? startX + guide : startX - guide;
  const secondX = targetIsRight ? endX - guide : endX + guide;
  return `M ${startX} ${startY} C ${firstX} ${startY}, ${secondX} ${endY}, ${endX} ${endY}`;
}

function buildEdgeLines(edges, layout, compact) {
  return (Array.isArray(edges) ? edges : [])
    .filter((edge) => normaliseKind(edge.kind) === "dependency")
    .map((edge) => {
      const from = String(edge.from || "");
      const to = String(edge.to || "");
      const source = layout.positions.get(from);
      const target = layout.positions.get(to);
      if (!source || !target || from === to) {
        return null;
      }
      return { ...edge, from, to, path: edgePath(source, target, compact) };
    })
    .filter(Boolean);
}

function nodeIsNeighbour(adjacency, selected, nodeId) {
  if (!selected || nodeId === selected) {
    return Boolean(selected && nodeId === selected);
  }
  const links = adjacency.get(selected) || { incoming: [], outgoing: [] };
  return links.incoming.includes(nodeId) || links.outgoing.includes(nodeId);
}

function buildAdjacency(layout) {
  const adjacency = new Map();
  for (const module of layout.modules) {
    const id = String(module.id);
    adjacency.set(id, {
      incoming: [...new Set(layout.incoming.get(id) || [])],
      outgoing: [...new Set(layout.outgoing.get(id) || [])],
    });
  }
  return adjacency;
}
function compactGraphLayoutWidth(compact, stageWidth) {
  return compact ? Math.max(0, stageWidth - PADDING * 2) : 0;
}

export { buildLayout, buildEdgeLines, buildAdjacency, nodeIsNeighbour, normaliseKind, clamp };
