const UI_SCHEMA_VERSION = 1;
const state = {
  nodes: [],
  edges: [],
  findings: [],
  selected: null,
  expanded: new Set(),
  scale: 1,
  offsetX: 0,
  offsetY: 0,
  artefacts: [],
  artefactIndex: 0
};

const graph = document.querySelector("#graph");
const panel = document.querySelector("#detail-panel");
const artefacts = document.querySelector("#artefacts");

async function api(path) {
  const response = await fetch(path);
  if (!response.ok) {
    throw new Error(`${path} returned ${response.status}`);
  }
  return response.json();
}

async function boot() {
  const meta = await api("/api/meta");
  if (meta.schema_version > UI_SCHEMA_VERSION) {
    const warning = document.querySelector("#warning");
    warning.hidden = false;
    warning.textContent = `Query schema ${meta.schema_version} is newer than this explorer.`;
  }
  const [graphData, lintData] = await Promise.all([api("/api/graph"), api("/api/lint")]);
  state.nodes = graphData.nodes || [];
  state.edges = graphData.edges || [];
  state.findings = lintData.findings || [];
  for (const node of state.nodes) {
    if (node.kind !== "module") {
      state.expanded.add(node.id);
    }
  }
  renderGraph();
}

function renderGraph() {
  graph.innerHTML = "";
  const visible = visibleNodes();
  const positions = layout(visible);
  renderEdges(visible, positions);
  for (const node of visible) {
    renderNode(node, positions.get(node.id));
  }
}

function visibleNodes() {
  const total = state.nodes.length;
  return state.nodes.filter(node => {
    if (total >= 200 && node.kind === "module" && !state.expanded.has(node.parent)) {
      return false;
    }
    let parent = node.parent;
    while (parent) {
      if (!state.expanded.has(parent)) {
        return false;
      }
      const parentNode = state.nodes.find(candidate => candidate.id === parent);
      parent = parentNode ? parentNode.parent : null;
    }
    return true;
  });
}

function layout(nodes) {
  const levels = new Map([["system", 0], ["actor", 0], ["container", 1], ["module", 2]]);
  const byLevel = new Map();
  for (const node of nodes) {
    const level = levels.get(node.kind) ?? 3;
    if (!byLevel.has(level)) {
      byLevel.set(level, []);
    }
    byLevel.get(level).push(node);
  }
  const positions = new Map();
  for (const [level, levelNodes] of [...byLevel.entries()].sort((a, b) => a[0] - b[0])) {
    const ordered = levelNodes.sort((a, b) => tagKey(a).localeCompare(tagKey(b)) || a.id.localeCompare(b.id));
    ordered.forEach((node, index) => {
      positions.set(node.id, {
        x: 80 + index * 230 + state.offsetX,
        y: 70 + level * 190 + state.offsetY
      });
    });
  }
  return positions;
}

function renderEdges(visible, positions) {
  const visibleIds = new Set(visible.map(node => node.id));
  const svg = document.createElementNS("http://www.w3.org/2000/svg", "svg");
  svg.classList.add("edge-layer");
  svg.setAttribute("width", "2400");
  svg.setAttribute("height", "1400");
  svg.setAttribute("viewBox", `0 0 ${2400 / state.scale} ${1400 / state.scale}`);
  svg.innerHTML = `
    <defs>
      <marker id="arrowhead" markerWidth="10" markerHeight="7" refX="9" refY="3.5" orient="auto">
        <polygon points="0 0, 10 3.5, 0 7"></polygon>
      </marker>
    </defs>
  `;
  for (const edge of state.edges) {
    if (!visibleIds.has(edge.from) || !visibleIds.has(edge.to)) {
      continue;
    }
    const from = positions.get(edge.from);
    const to = positions.get(edge.to);
    const active = state.selected && (edge.from === state.selected || edge.to === state.selected);
    const path = document.createElementNS("http://www.w3.org/2000/svg", "path");
    const startX = from.x + 90;
    const startY = from.y + 82;
    const endX = to.x + 90;
    const endY = to.y;
    path.setAttribute("d", `M ${startX} ${startY} C ${startX} ${startY + 55}, ${endX} ${endY - 55}, ${endX} ${endY}`);
    path.setAttribute("marker-end", "url(#arrowhead)");
    path.classList.add("edge", edge.kind);
    if (active) {
      path.classList.add("active");
    }
    svg.append(path);
    const label = document.createElementNS("http://www.w3.org/2000/svg", "text");
    label.textContent = edge.description || edge.kind;
    label.setAttribute("x", (startX + endX) / 2);
    label.setAttribute("y", (startY + endY) / 2);
    label.classList.add("edge-label");
    if (active) {
      label.classList.add("active");
    }
    svg.append(label);
  }
  graph.append(svg);
}

function renderNode(node, position) {
  const element = document.createElement("button");
  element.type = "button";
  element.className = "node";
  element.style.left = `${position.x}px`;
  element.style.top = `${position.y}px`;
  element.style.transform = `scale(${state.scale})`;
  element.style.transformOrigin = "top left";
  element.style.borderColor = tagColor(node);
  element.style.background = tagBackground(node);
  if (state.selected === node.id) {
    element.classList.add("active");
  }
  const findings = findingsFor(node.id);
  const childCount = (node.children || []).length;
  element.innerHTML = `
    <div class="badge-row">
      <span class="type-badge">${safe(node.kind)}</span>
      ${childCount ? `<span class="child-badge">${state.expanded.has(node.id) ? "collapse" : `${childCount} hidden`}</span>` : ""}
      ${findings.map(finding => `<span class="finding-badge ${findingClass(finding)}">${safe(finding.code)}</span>`).join("")}
    </div>
    <div class="node-title">${safe(node.name || "Not available")}</div>
    <div class="node-id">${safe(node.id || "Not available")}</div>
  `;
  element.addEventListener("click", event => {
    event.stopPropagation();
    if (event.target.classList.contains("finding-badge")) {
      showFinding(node, findings[0]);
      return;
    }
    if (state.selected === node.id) {
      closePanel();
    } else {
      selectNode(node);
    }
    if (childCount) {
      toggleExpanded(node.id);
    }
  });
  graph.append(element);
}

function toggleExpanded(id) {
  if (state.expanded.has(id)) {
    state.expanded.delete(id);
  } else {
    state.expanded.add(id);
  }
  renderGraph();
}

async function selectNode(node) {
  state.selected = node.id;
  document.querySelector("#node-kind").textContent = node.kind || "Not available";
  document.querySelector("#node-name").textContent = node.name || "Not available";
  document.querySelector("#node-id").textContent = node.id || "Not available";
  document.querySelector("#node-description").textContent = node.description || "Not available";
  document.querySelector("#finding-detail").textContent = "";
  panel.hidden = false;
  await loadArtefacts(node.id);
  renderGraph();
}

async function loadArtefacts(id) {
  const kinds = ["contract", "decisions", "todos", "research", "sources", "rationale"];
  const responses = await Promise.all(kinds.map(kind => api(`/api/node/${encodeURIComponent(id)}/${kind}`).catch(() => ({ artefacts: [] }))));
  state.artefacts = responses.flatMap((response, index) => (response.artefacts || []).map(item => ({ kind: kinds[index], ...item })));
  state.artefactIndex = Math.max(0, state.artefacts.findIndex(item => item.type === "contract"));
  if (state.artefactIndex < 0) {
    state.artefactIndex = 0;
  }
  renderArtefacts();
}

function renderArtefacts() {
  artefacts.innerHTML = "";
  const count = state.artefacts.length;
  document.querySelector("#layer-count").textContent = count ? `${state.artefactIndex + 1} / ${count}` : "0 / 0";
  state.artefacts.forEach((item, index) => {
    const section = document.createElement("section");
    section.className = "accordion";
    const open = index === state.artefactIndex;
    section.innerHTML = `
      <button type="button"><span>${safe(item.title || item.type || "Artefact")}</span><span>${safe(item.type || item.kind || "unknown")}</span></button>
      <div class="artefact-body" ${open ? "" : "hidden"}>
        <div class="frontmatter">${safe(JSON.stringify(item.frontmatter || {}, null, 2))}</div>
        ${safe(item.body || "Not available")}
      </div>
    `;
    section.querySelector("button").addEventListener("click", () => {
      state.artefactIndex = index;
      renderArtefacts();
    });
    artefacts.append(section);
  });
}

function showFinding(node, finding) {
  state.selected = node.id;
  panel.hidden = false;
  document.querySelector("#node-kind").textContent = node.kind || "Not available";
  document.querySelector("#node-name").textContent = node.name || "Not available";
  document.querySelector("#node-id").textContent = node.id || "Not available";
  document.querySelector("#node-description").textContent = node.description || "Not available";
  document.querySelector("#finding-detail").textContent = finding ? `${finding.code}: ${finding.message}` : "No finding detail";
  renderGraph();
}

function closePanel() {
  state.selected = null;
  panel.hidden = true;
  state.artefacts = [];
  artefacts.innerHTML = "";
  renderGraph();
}

function findingsFor(id) {
  return state.findings.filter(finding => finding.node === id);
}

function findingClass(finding) {
  if (finding.severity === "Error") {
    return "error";
  }
  if ((finding.code || "").includes("CONTRACT") || (finding.code || "").includes("INTERFACE")) {
    return "warning";
  }
  return "advisory";
}

function tagKey(node) {
  return (node.tags || []).join(".");
}

function tagColor(node) {
  const tag = (node.tags || [node.kind || "node"])[0] || "node";
  let hash = 0;
  for (const char of tag) {
    hash = (hash * 31 + char.charCodeAt(0)) % 360;
  }
  return `hsl(${hash} 45% 38%)`;
}

function tagBackground(node) {
  const tag = (node.tags || [node.kind || "node"])[0] || "node";
  let hash = 0;
  for (const char of tag) {
    hash = (hash * 31 + char.charCodeAt(0)) % 360;
  }
  return `linear-gradient(180deg, hsl(${hash} 55% 96%), #ffffff)`;
}

function safe(value) {
  return String(value)
    .replaceAll("&", "&amp;")
    .replaceAll("<", "&lt;")
    .replaceAll(">", "&gt;")
    .replaceAll('"', "&quot;");
}

document.querySelector("#close-panel").addEventListener("click", closePanel);
document.querySelector("#next-layer").addEventListener("click", () => {
  if (state.artefacts.length) {
    state.artefactIndex = (state.artefactIndex + 1) % state.artefacts.length;
    renderArtefacts();
  }
});
document.querySelector("#prev-layer").addEventListener("click", () => {
  if (state.artefacts.length) {
    state.artefactIndex = (state.artefactIndex + state.artefacts.length - 1) % state.artefacts.length;
    renderArtefacts();
  }
});
document.querySelector("#zoom-in").addEventListener("click", () => {
  state.scale = Math.min(1.7, state.scale + 0.1);
  renderGraph();
});
document.querySelector("#zoom-out").addEventListener("click", () => {
  state.scale = Math.max(0.5, state.scale - 0.1);
  renderGraph();
});
document.querySelector("#reset-view").addEventListener("click", () => {
  state.scale = 1;
  state.offsetX = 0;
  state.offsetY = 0;
  renderGraph();
});
graph.addEventListener("click", closePanel);
let panStart = null;
graph.addEventListener("pointerdown", event => {
  if (event.target !== graph) {
    return;
  }
  panStart = {
    x: event.clientX,
    y: event.clientY,
    offsetX: state.offsetX,
    offsetY: state.offsetY
  };
  graph.setPointerCapture(event.pointerId);
});
graph.addEventListener("pointermove", event => {
  if (!panStart) {
    return;
  }
  state.offsetX = panStart.offsetX + event.clientX - panStart.x;
  state.offsetY = panStart.offsetY + event.clientY - panStart.y;
  renderGraph();
});
graph.addEventListener("pointerup", event => {
  panStart = null;
  if (graph.hasPointerCapture(event.pointerId)) {
    graph.releasePointerCapture(event.pointerId);
  }
});
boot().catch(error => {
  const warning = document.querySelector("#warning");
  warning.hidden = false;
  warning.textContent = error.message;
});
