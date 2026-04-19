/* Cairn Graph Explorer — client */

const UI_SCHEMA_VERSION = 1;

const state = {
  nodes: [],
  edges: [],
  findings: [],
  meta: null,
  selected: null,
  currentNode: null,
  expanded: new Set(),
  // transform
  fitScale: 1,
  scale: 1,
  offsetX: 0,
  offsetY: 0,
  // content layout bounds
  contentBounds: { x: 0, y: 0, w: 0, h: 0 },
  // artefact browser
  artefacts: [],
  artefactIndex: 0,
  artefactExpanded: new Set()
};

const ZOOM_MIN_MULT = 1;
const ZOOM_MAX_MULT = 2.5;
const ZOOM_STEP_IN = 1.15;
const ZOOM_STEP_OUT = 1 / 1.15;

const viewport = document.querySelector("#graph-viewport");
const graph = document.querySelector("#graph");
const panel = document.querySelector("#detail-panel");
const shell = document.querySelector("#shell");
const closeBtn = document.querySelector("#close-panel");
const artefactsMount = document.querySelector("#artefacts");
const metadataMount = document.querySelector("#metadata-blocks");
const layerNavEl = document.querySelector("#layer-nav");
const findingsSection = document.querySelector("#findings-section");
const findingsDetail = document.querySelector("#finding-detail");
const findingsCount = document.querySelector("#findings-count");

/* -------------------- API -------------------- */
async function api(path) {
  const response = await fetch(path);
  if (!response.ok) {
    throw new Error(`${path} returned ${response.status}`);
  }
  return response.json();
}

async function safeApi(path, fallback) {
  try {
    return await api(path);
  } catch (_err) {
    return fallback;
  }
}

/* -------------------- Boot -------------------- */
async function boot() {
  const [meta, graphData, lintData] = await Promise.all([
    safeApi("/api/meta", {}),
    api("/api/graph"),
    safeApi("/api/lint", { findings: [] })
  ]);
  state.meta = meta || {};
  state.nodes = graphData.nodes || [];
  state.edges = graphData.edges || [];
  state.findings = lintData.findings || [];

  if ((meta.schema_version || 0) > UI_SCHEMA_VERSION) {
    const warning = document.querySelector("#warning");
    warning.hidden = false;
    warning.textContent = `Query schema ${meta.schema_version} is newer than this explorer.`;
  }

  renderTopbarMeta();

  for (const node of state.nodes) {
    if (node.kind !== "module") {
      state.expanded.add(node.id);
    }
  }
  renderGraph();
  // Initial fit: the viewport grid column may not have its final width on
  // the first frame after paint. Retry with double-RAF, and if the measured
  // viewport rect is still zero/unstable, try once more — then install a
  // ResizeObserver so any late sizing change (fonts, scrollbars) re-fits.
  scheduleInitialFit();
  installViewportResizeObserver();
}

// Fit only succeeds when `viewport.getBoundingClientRect()` has non-zero width
// and height. The first RAF after `renderGraph()` can fire before the shell
// grid's column-track dimensions have been resolved by layout; force a flush
// by double-rAF, and if still zero, retry once on the next frame.
function scheduleInitialFit(attempt = 0) {
  requestAnimationFrame(() => {
    requestAnimationFrame(() => {
      const rect = viewport.getBoundingClientRect();
      if (rect.width > 0 && rect.height > 0) {
        fitToViewport();
        // Double-tap: one more frame after we've measured once, so any final
        // font-metrics reflow doesn't leave us on a half-computed rect.
        requestAnimationFrame(() => fitToViewport());
        return;
      }
      if (attempt < 8) {
        scheduleInitialFit(attempt + 1);
      }
    });
  });
}

function installViewportResizeObserver() {
  if (typeof ResizeObserver === "undefined") return;
  let firstSettled = false;
  const observer = new ResizeObserver(() => {
    const rect = viewport.getBoundingClientRect();
    if (rect.width <= 0 || rect.height <= 0) return;
    // Preserve user zoom after the very first fit; before the first settled
    // measurement, reset-to-fit is correct (page just loaded).
    if (!firstSettled) {
      firstSettled = true;
      fitToViewport();
    } else {
      // Subsequent viewport shape changes (responsive column toggles that
      // the explicit close-panel refit may have already handled, font
      // loading, OS chrome). Preserve user zoom.
      fitToViewport({ preserveUserScale: true });
    }
  });
  observer.observe(viewport);
}

function renderTopbarMeta() {
  const root = state.nodes.find(n => !n.parent);
  const projectName = state.meta.project_name || (root && root.name) || (root && root.id) || "cairn";
  document.querySelector("#meta-project").textContent = projectName;

  const schema = state.meta.schema_version != null ? `v${state.meta.schema_version}` : "v\u2014";
  document.querySelector("#meta-schema").textContent = schema;

  const genRaw = state.meta.generated_at || state.meta.generated || null;
  const genEl = document.querySelector("#meta-generated");
  if (genRaw) {
    genEl.textContent = formatGenerated(genRaw);
    genEl.setAttribute("title", String(genRaw));
  } else {
    const now = new Date();
    const iso = now.toISOString().replace(/\.\d{3}Z$/, "Z");
    genEl.textContent = iso;
    genEl.setAttribute("title", iso);
  }
}

function formatGenerated(raw) {
  const d = new Date(raw);
  if (isNaN(d.getTime())) {
    return String(raw);
  }
  const diffMs = Date.now() - d.getTime();
  const hours = diffMs / (1000 * 60 * 60);
  if (hours >= 0 && hours < 24) {
    if (diffMs < 60 * 1000) {
      return "just now";
    }
    const minutes = Math.floor(diffMs / 60000);
    if (minutes < 60) {
      return `${minutes}m ago`;
    }
    return `${Math.floor(hours)}h ago`;
  }
  return d.toISOString().replace(/\.\d{3}Z$/, "Z");
}

/* -------------------- Graph rendering -------------------- */
function renderGraph() {
  graph.innerHTML = "";
  const visible = visibleNodes();
  const positions = layout(visible);
  computeContentBounds(positions);
  renderEdges(visible, positions);
  for (const node of visible) {
    renderNode(node, positions.get(node.id));
  }
  applyTransform();
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
  const columnWidth = 240;
  const rowHeight = 150;
  const leftPad = 32;
  const topPad = 32;
  for (const [level, levelNodes] of [...byLevel.entries()].sort((a, b) => a[0] - b[0])) {
    const ordered = levelNodes.sort((a, b) => tagKey(a).localeCompare(tagKey(b)) || a.id.localeCompare(b.id));
    ordered.forEach((node, index) => {
      positions.set(node.id, {
        x: leftPad + index * columnWidth,
        y: topPad + level * rowHeight
      });
    });
  }
  return positions;
}

function computeContentBounds(positions) {
  if (positions.size === 0) {
    state.contentBounds = { x: 0, y: 0, w: 0, h: 0 };
    return;
  }
  let minX = Infinity;
  let minY = Infinity;
  let maxX = -Infinity;
  let maxY = -Infinity;
  const nodeW = 220; // approx for bounds calc (max-width ~260, min 180)
  const nodeH = 92;
  for (const pos of positions.values()) {
    if (pos.x < minX) minX = pos.x;
    if (pos.y < minY) minY = pos.y;
    if (pos.x + nodeW > maxX) maxX = pos.x + nodeW;
    if (pos.y + nodeH > maxY) maxY = pos.y + nodeH;
  }
  state.contentBounds = {
    x: minX,
    y: minY,
    w: maxX - minX,
    h: maxY - minY
  };
}

function renderEdges(visible, positions) {
  const visibleIds = new Set(visible.map(node => node.id));
  const svg = document.createElementNS("http://www.w3.org/2000/svg", "svg");
  svg.classList.add("edge-layer");
  svg.setAttribute("width", "3200");
  svg.setAttribute("height", "2000");
  svg.setAttribute("viewBox", "0 0 3200 2000");
  svg.innerHTML = `
    <defs>
      <marker id="arrowhead" markerWidth="8" markerHeight="8" refX="7" refY="4" orient="auto">
        <polygon points="0 0, 8 4, 0 8"></polygon>
      </marker>
      <marker id="arrowhead-active" class="arrow-active" markerWidth="8" markerHeight="8" refX="7" refY="4" orient="auto">
        <polygon points="0 0, 8 4, 0 8"></polygon>
      </marker>
    </defs>
  `;
  const nodeW = 220;
  const nodeH = 92;
  const selectedId = state.selected;

  for (const edge of state.edges) {
    if (!visibleIds.has(edge.from) || !visibleIds.has(edge.to)) {
      continue;
    }
    const from = positions.get(edge.from);
    const to = positions.get(edge.to);
    const active = selectedId && (edge.from === selectedId || edge.to === selectedId);
    const path = document.createElementNS("http://www.w3.org/2000/svg", "path");
    const startX = from.x + nodeW / 2;
    const startY = from.y + nodeH;
    const endX = to.x + nodeW / 2;
    const endY = to.y;
    path.setAttribute("d", `M ${startX} ${startY} C ${startX} ${startY + 45}, ${endX} ${endY - 45}, ${endX} ${endY}`);
    path.setAttribute("marker-end", active ? "url(#arrowhead-active)" : "url(#arrowhead)");
    path.classList.add("edge", edge.kind || "ownership");
    if (active) {
      path.classList.add("active");
    } else if (selectedId) {
      path.classList.add("faded");
    }
    svg.append(path);
  }
  graph.append(svg);
}

function renderNode(node, position) {
  const element = document.createElement("div");
  element.className = `node kind-${cssSafe(node.kind || "module")}`;
  element.style.left = `${position.x}px`;
  element.style.top = `${position.y}px`;
  if (state.selected === node.id) {
    element.classList.add("active");
  }
  const findings = findingsFor(node.id);
  const primary = findings[0];
  const childCount = (node.children || []).length;
  const hasChildren = childCount > 0;
  if (primary) {
    element.classList.add("has-finding");
  }

  // Main body — click target for selection (everything except chevron tab).
  const body = document.createElement("button");
  body.type = "button";
  body.className = "node-body";
  body.style.display = "flex";
  body.style.flexDirection = "column";
  body.style.gap = "var(--cairn-space-1)";
  body.style.width = "100%";
  body.style.textAlign = "left";
  body.style.color = "inherit";
  body.style.padding = "0";
  const parts = [];
  parts.push(`<span class="node-kind">${safe((node.kind || "node").toUpperCase())}</span>`);
  parts.push(`<span class="node-name">${safe(node.name || node.id || "untitled")}</span>`);
  parts.push(`<span class="node-id">${safe(node.id || "")}</span>`);
  body.innerHTML = parts.join("");
  element.append(body);

  body.addEventListener("click", event => {
    event.stopPropagation();
    if (state.selected === node.id) {
      closePanel();
      return;
    }
    selectNode(node);
  });

  // Chevron tab (top-right) — only for nodes that actually have children.
  if (hasChildren) {
    const expanded = state.expanded.has(node.id);
    const tab = document.createElement("button");
    tab.type = "button";
    tab.className = "node-chevron-tab";
    tab.dataset.expanded = expanded ? "true" : "false";
    tab.title = expanded ? "Collapse" : "Expand";
    tab.setAttribute("aria-label", expanded ? "Collapse children" : "Expand children");
    tab.innerHTML = `<span class="chevron-glyph">\u203A</span>`;
    tab.addEventListener("click", event => {
      event.stopPropagation();
      toggleExpanded(node.id);
    });
    element.append(tab);
  }

  // Finding tab
  if (primary) {
    const sev = severityKey(primary);
    const glyph = sev === "err" ? "\u00D7" : (sev === "note" ? "i" : "!");
    const ftab = document.createElement("span");
    ftab.className = `node-finding-tab severity-${sev}`;
    ftab.title = primary.code || "finding";
    ftab.textContent = glyph;
    element.append(ftab);
  }

  graph.append(element);
}

function toggleExpanded(id) {
  if (state.expanded.has(id)) {
    state.expanded.delete(id);
  } else {
    state.expanded.add(id);
  }
  renderGraph();
  // Content bbox changed — re-fit so composition settles. Double-RAF for
  // parity with open/close paths; single RAF would also work here since the
  // viewport size doesn't change on collapse, only the content bounds.
  requestAnimationFrame(() => {
    requestAnimationFrame(() => fitToViewport());
  });
}

/* -------------------- Transform / zoom / pan -------------------- */
function applyTransform() {
  graph.style.transform = `translate(${state.offsetX}px, ${state.offsetY}px) scale(${state.scale})`;
}

function fitToViewport(opts = {}) {
  const rect = viewport.getBoundingClientRect();
  const bounds = state.contentBounds;
  if (!rect.width || !rect.height || !bounds.w || !bounds.h) {
    return;
  }
  const inset = 48;
  const availW = Math.max(1, rect.width - inset * 2);
  const availH = Math.max(1, rect.height - inset * 2);
  const scaleW = availW / bounds.w;
  const scaleH = availH / bounds.h;
  // 1.0 clamp: don't upscale tiny graphs; keep native type size.
  const fit = Math.min(scaleW, scaleH, 1.0);
  const prevFit = state.fitScale || fit;
  const prevUserRatio = opts.preserveUserScale && prevFit > 0
    ? (state.scale / prevFit)
    : null;
  state.fitScale = fit;
  // If preserving, scale user's relative zoom; else reset to fit.
  state.scale = prevUserRatio ? Math.max(fit, fit * prevUserRatio) : fit;
  // Center content at the effective scale.
  const eff = state.scale;
  const scaledW = bounds.w * eff;
  const scaledH = bounds.h * eff;
  state.offsetX = Math.round((rect.width - scaledW) / 2 - bounds.x * eff);
  state.offsetY = Math.round((rect.height - scaledH) / 2 - bounds.y * eff);
  applyTransform();
}

function zoomBy(mult, animate) {
  const minScale = state.fitScale;
  const maxScale = state.fitScale * ZOOM_MAX_MULT;
  const next = clamp(state.scale * mult, minScale, maxScale);
  const rect = viewport.getBoundingClientRect();
  const cx = rect.width / 2;
  const cy = rect.height / 2;
  // Keep content-point under viewport center fixed.
  const before = state.scale;
  const px = (cx - state.offsetX) / before;
  const py = (cy - state.offsetY) / before;
  state.scale = next;
  state.offsetX = Math.round(cx - px * state.scale);
  state.offsetY = Math.round(cy - py * state.scale);
  if (!animate) {
    graph.classList.add("no-transition");
  }
  applyTransform();
  if (!animate) {
    requestAnimationFrame(() => graph.classList.remove("no-transition"));
  }
  clampPan();
}

function clampPan() {
  if (state.scale <= state.fitScale + 1e-6) {
    // at fit, center
    const rect = viewport.getBoundingClientRect();
    const b = state.contentBounds;
    const scaledW = b.w * state.scale;
    const scaledH = b.h * state.scale;
    state.offsetX = Math.round((rect.width - scaledW) / 2 - b.x * state.scale);
    state.offsetY = Math.round((rect.height - scaledH) / 2 - b.y * state.scale);
    applyTransform();
  }
}

function resetView() {
  // Ensure the 160ms linear transform transition runs, then snap to fit.
  graph.classList.remove("no-transition");
  fitToViewport();
}

function clamp(v, min, max) {
  return Math.max(min, Math.min(max, v));
}

/* -------------------- Detail panel -------------------- */
async function selectNode(node) {
  const wasClosed = panel.hidden;
  state.selected = node.id;
  state.currentNode = node;
  state.artefactExpanded = new Set();

  renderPanelHeader(node);
  renderFindings(node);
  renderMetadataBlocks(node);
  // Hide layer-nav until artefacts are loaded and count > 1 is established.
  layerNavEl.hidden = true;
  panel.hidden = false;
  // Grid reflow: open the panel column instantly.
  shell.classList.remove("panel-closed");
  // Re-fit graph to narrower viewport so composition stays centered.
  // Double-RAF so we measure AFTER the grid column-track change has
  // propagated — single RAF lands on the stale (pre-reflow) width.
  renderGraph();
  viewport.getBoundingClientRect(); // flush
  requestAnimationFrame(() => {
    requestAnimationFrame(() => fitToViewport({ preserveUserScale: true }));
  });

  // Flash focus ring on close button so keyboard users discover Esc.
  if (wasClosed) {
    closeBtn.classList.remove("just-opened");
    // reflow so animation restarts
    void closeBtn.offsetWidth;
    closeBtn.classList.add("just-opened");
    setTimeout(() => closeBtn.classList.remove("just-opened"), 700);
  }

  // async: fetch richer node detail + artefacts
  const detail = await safeApi(`/api/node/${encodeURIComponent(node.id)}`, null);
  if (detail && state.selected === node.id) {
    state.currentNode = detail;
    renderPanelHeader(detail);
    renderMetadataBlocks(detail);
  }
  await loadArtefacts(node.id);
}

function renderPanelHeader(node) {
  document.querySelector("#node-kind").textContent = (node.kind || "node").toUpperCase();
  document.querySelector("#node-name").textContent = node.name || node.id || "untitled";
  const idEl = document.querySelector("#node-id");
  idEl.textContent = node.id || "";
  const descEl = document.querySelector("#node-description");
  descEl.textContent = node.description || "";
}

function renderFindings(node) {
  const items = findingsFor(node.id);
  if (!items.length) {
    findingsSection.hidden = true;
    findingsDetail.innerHTML = "";
    findingsCount.textContent = "";
    return;
  }
  findingsSection.hidden = false;
  findingsCount.textContent = `(${items.length})`;
  findingsDetail.innerHTML = items.map(f => {
    const sev = severityKey(f);
    const lines = [];
    lines.push(`<div class="finding-mark severity-${sev}"></div>`);
    const body = [];
    body.push(`<span class="finding-code">${safe(f.code || "finding")}</span>`);
    if (f.message) {
      body.push(`<span class="finding-message">${safe(f.message)}</span>`);
    }
    if (f.path) {
      body.push(`<span class="finding-path">${safe(f.path)}</span>`);
    }
    lines.push(`<div class="finding-body">${body.join("")}</div>`);
    return `<div class="finding-row">${lines.join("")}</div>`;
  }).join("");
}

function renderMetadataBlocks(node) {
  const blocks = [];
  if (node.tags && node.tags.length) {
    blocks.push(metadataBlock("TAGS", node.tags, { inline: true }));
  }
  if (node.paths && node.paths.length) {
    blocks.push(metadataBlock("PATHS", node.paths));
  }
  if (node.contracts && node.contracts.length) {
    blocks.push(metadataBlock("CONTRACTS", node.contracts));
  }
  if (node.files && node.files.length) {
    blocks.push(metadataBlock("FILES", node.files));
  }
  const depends = edgeNeighbours(node.id, "from");
  if (depends.length) {
    blocks.push(metadataBlock("DEPENDS ON", depends, { linkNodes: true }));
  }
  const dependents = edgeNeighbours(node.id, "to");
  if (dependents.length) {
    blocks.push(metadataBlock("DEPENDENTS", dependents, { linkNodes: true }));
  }
  if (node.state) {
    blocks.push(metadataBlock("STATE", [node.state]));
  }
  metadataMount.innerHTML = blocks.join("");
  // wire node-link clicks
  metadataMount.querySelectorAll("[data-nav-node]").forEach(el => {
    el.addEventListener("click", () => {
      const id = el.getAttribute("data-nav-node");
      const target = state.nodes.find(n => n.id === id);
      if (target) {
        selectNode(target);
      }
    });
  });
}

function metadataBlock(label, values, opts = {}) {
  const inner = values.map(v => {
    if (opts.linkNodes) {
      return `<button type="button" data-nav-node="${safe(v)}">${safe(v)}</button>`;
    }
    return `<span>${safe(v)}</span>`;
  }).join("");
  const listCls = opts.inline ? "metadata-list inline" : "metadata-list";
  return `<section class="metadata-block">
    <header class="metadata-header"><span>${safe(label)}</span><span>(${values.length})</span></header>
    <div class="${listCls}">${inner}</div>
  </section>`;
}

function edgeNeighbours(nodeId, direction) {
  // direction: "from" returns nodes this node points to (depends on)
  //            "to"   returns nodes that point to this node (dependents)
  const out = [];
  const seen = new Set();
  for (const edge of state.edges) {
    if (direction === "from" && edge.from === nodeId && edge.kind !== "ownership") {
      if (!seen.has(edge.to)) {
        seen.add(edge.to);
        out.push(edge.to);
      }
    } else if (direction === "to" && edge.to === nodeId && edge.kind !== "ownership") {
      if (!seen.has(edge.from)) {
        seen.add(edge.from);
        out.push(edge.from);
      }
    }
  }
  return out;
}

/* -------------------- Artefacts -------------------- */
async function loadArtefacts(id) {
  artefactsMount.innerHTML = `<div class="loading-row">loading\u2026</div>`;
  const kinds = ["contract", "decisions", "todos", "research", "sources", "rationale"];
  const responses = await Promise.all(kinds.map(kind =>
    safeApi(`/api/node/${encodeURIComponent(id)}/${kind}`, { artefacts: [] })
  ));
  if (state.selected !== id) {
    return; // selection changed; drop results
  }
  state.artefacts = [];
  const grouped = [];
  responses.forEach((response, index) => {
    const items = (response && response.artefacts) || [];
    if (items.length) {
      grouped.push({ kind: kinds[index], items });
      for (const item of items) {
        state.artefacts.push({ kind: kinds[index], item });
      }
    }
  });
  state.artefactIndex = 0;
  renderArtefacts(grouped);
  renderLayerNav();
}

function renderArtefacts(grouped) {
  if (!grouped || !grouped.length) {
    artefactsMount.innerHTML = "";
    return;
  }
  const fragments = grouped.map(group => {
    const header = `<header class="artefact-group-header">
      <span>${safe(group.kind.toUpperCase())}</span>
      <span class="artefact-group-count">(${group.items.length})</span>
    </header>`;
    const cards = group.items.map((item, idx) => {
      const key = `${group.kind}:${idx}`;
      const open = state.artefactExpanded.has(key);
      const id = item.id || item.code || item.type || `${group.kind}-${idx + 1}`;
      const title = item.title || item.name || item.summary || (item.type ? `${item.type}` : group.kind);
      const metaBits = [];
      if (item.status) metaBits.push(item.status);
      if (item.date) metaBits.push(item.date);
      if (item.author) metaBits.push(item.author);
      const frontmatter = item.frontmatter || {};
      for (const k of ["status", "date", "author"]) {
        if (!metaBits.length && frontmatter[k]) metaBits.push(frontmatter[k]);
      }
      const metaLine = metaBits.length
        ? `<div class="artefact-meta">${metaBits.map(safe).join(" \u00B7 ")}</div>`
        : "";
      const files = Array.isArray(item.files) ? item.files : [];
      const filesBlock = open && files.length
        ? `<div class="artefact-files">
            <span class="artefact-files-label">files:</span>
            ${files.map(f => `<span class="artefact-files-item">${safe(f)}</span>`).join("")}
          </div>`
        : "";
      const body = item.body || item.description || "";
      return `<article class="artefact-card${open ? " is-open" : ""}" data-artefact-key="${safe(key)}">
        <button type="button" class="artefact-head" data-toggle="${safe(key)}">
          <span class="artefact-id">${safe(id)}</span>
          <span class="artefact-title">${safe(title)}</span>
          <span class="artefact-chevron">&rsaquo;</span>
        </button>
        ${metaLine}
        <div class="artefact-body"><div class="artefact-body-inner">${safe(body)}${filesBlock}</div></div>
      </article>`;
    }).join("");
    return `<div class="artefact-group">${header}${cards}</div>`;
  });
  artefactsMount.innerHTML = fragments.join("");
  artefactsMount.querySelectorAll("[data-toggle]").forEach(btn => {
    btn.addEventListener("click", () => {
      const key = btn.getAttribute("data-toggle");
      if (state.artefactExpanded.has(key)) {
        state.artefactExpanded.delete(key);
      } else {
        state.artefactExpanded.add(key);
      }
      renderArtefacts(grouped);
      // sync artefactIndex with last-opened for layer nav
      const flatIdx = state.artefacts.findIndex(a => `${a.kind}:${state.artefacts.filter(b => b.kind === a.kind).indexOf(a)}` === key);
      if (flatIdx >= 0) {
        state.artefactIndex = flatIdx;
        renderLayerNav();
      }
    });
  });
}

function renderLayerNav() {
  const count = state.artefacts.length;
  if (count <= 1) {
    layerNavEl.hidden = true;
    return;
  }
  layerNavEl.hidden = false;
  document.querySelector("#layer-count").textContent = `${state.artefactIndex + 1} / ${count}`;
  const prevBtn = document.querySelector("#prev-layer");
  const nextBtn = document.querySelector("#next-layer");
  prevBtn.disabled = state.artefactIndex <= 0;
  nextBtn.disabled = state.artefactIndex >= count - 1;
}

function advanceArtefact(step) {
  const count = state.artefacts.length;
  if (!count) return;
  const next = clamp(state.artefactIndex + step, 0, count - 1);
  if (next === state.artefactIndex) return;
  state.artefactIndex = next;
  const a = state.artefacts[next];
  if (a) {
    // auto-open the current artefact
    const kindIdx = state.artefacts.filter(b => b.kind === a.kind).indexOf(a);
    state.artefactExpanded = new Set([`${a.kind}:${kindIdx}`]);
    // re-render preserving grouping
    const grouped = regroupArtefacts();
    renderArtefacts(grouped);
  }
  renderLayerNav();
}

function regroupArtefacts() {
  const map = new Map();
  for (const a of state.artefacts) {
    if (!map.has(a.kind)) map.set(a.kind, []);
    map.get(a.kind).push(a.item);
  }
  return Array.from(map.entries()).map(([kind, items]) => ({ kind, items }));
}

function closePanel() {
  state.selected = null;
  state.currentNode = null;
  state.artefacts = [];
  state.artefactExpanded = new Set();
  panel.hidden = true;
  artefactsMount.innerHTML = "";
  metadataMount.innerHTML = "";
  findingsSection.hidden = true;
  layerNavEl.hidden = true;
  // Grid reflow — canvas reclaims the 380px instantly.
  shell.classList.add("panel-closed");
  closeBtn.classList.remove("just-opened");
  closeBtn.style.outline = ""; // clear any lingering inline ring
  renderGraph();
  // After the grid collapses, we must re-fit against the NEW wider viewport.
  // One RAF isn't enough: the grid-template-columns toggle + layout pass has
  // to commit before `viewport.getBoundingClientRect()` reflects the wider
  // column. Force a synchronous layout read and then re-fit on the next
  // frame to guarantee we measure the post-reflow rectangle.
  // eslint-disable-next-line no-unused-expressions
  viewport.getBoundingClientRect(); // flush pending layout
  requestAnimationFrame(() => {
    requestAnimationFrame(() => fitToViewport({ preserveUserScale: true }));
  });
}

/* -------------------- Helpers -------------------- */
function findingsFor(id) {
  return state.findings.filter(f => f.node === id);
}

function severityKey(finding) {
  const s = (finding && finding.severity ? String(finding.severity) : "").toLowerCase();
  if (s.startsWith("err")) return "err";
  if (s.startsWith("warn")) return "warn";
  if (s.startsWith("note") || s.startsWith("info") || s.startsWith("advis")) return "note";
  // fallback by code
  const code = finding && finding.code ? String(finding.code) : "";
  if (code.includes("MISSING") || code.includes("CONTRACT") || code.includes("INTERFACE")) return "warn";
  return "note";
}

function tagKey(node) {
  return (node.tags || []).join(".");
}

function cssSafe(value) {
  return String(value).replace(/[^a-z0-9_-]/gi, "-").toLowerCase();
}

function safe(value) {
  return String(value == null ? "" : value)
    .replaceAll("&", "&amp;")
    .replaceAll("<", "&lt;")
    .replaceAll(">", "&gt;")
    .replaceAll('"', "&quot;");
}

/* -------------------- Wire interactions -------------------- */
closeBtn.addEventListener("click", closePanel);

// Esc closes the panel — but only when the panel is open and no text input is focused.
document.addEventListener("keydown", event => {
  if (event.key !== "Escape") return;
  if (panel.hidden) return;
  const ae = document.activeElement;
  if (ae) {
    const tag = (ae.tagName || "").toLowerCase();
    if (tag === "input" || tag === "textarea" || ae.isContentEditable) {
      return;
    }
  }
  event.preventDefault();
  closePanel();
});

document.querySelector("#next-layer").addEventListener("click", () => advanceArtefact(1));
document.querySelector("#prev-layer").addEventListener("click", () => advanceArtefact(-1));

document.querySelector("#zoom-in").addEventListener("click", () => zoomBy(ZOOM_STEP_IN, false));
document.querySelector("#zoom-out").addEventListener("click", () => zoomBy(ZOOM_STEP_OUT, false));
document.querySelector("#reset-view").addEventListener("click", resetView);

// Click on empty viewport closes panel.
viewport.addEventListener("click", event => {
  if (event.target === viewport || event.target === graph) {
    // only clear if a node isn't being interacted with and the click is truly on empty canvas
    if (state.selected) {
      closePanel();
    }
  }
});

// Pan
let panStart = null;
viewport.addEventListener("pointerdown", event => {
  if (event.target !== viewport && event.target !== graph) {
    return;
  }
  panStart = {
    x: event.clientX,
    y: event.clientY,
    offsetX: state.offsetX,
    offsetY: state.offsetY,
    moved: false
  };
  viewport.setPointerCapture(event.pointerId);
  viewport.classList.add("is-panning");
  graph.classList.add("no-transition");
});

viewport.addEventListener("pointermove", event => {
  if (!panStart) return;
  const dx = event.clientX - panStart.x;
  const dy = event.clientY - panStart.y;
  if (Math.abs(dx) > 2 || Math.abs(dy) > 2) {
    panStart.moved = true;
  }
  // Only permit pan when zoomed beyond fit.
  if (state.scale > state.fitScale + 1e-6) {
    state.offsetX = panStart.offsetX + dx;
    state.offsetY = panStart.offsetY + dy;
    applyTransform();
  }
});

viewport.addEventListener("pointerup", event => {
  if (panStart && viewport.hasPointerCapture(event.pointerId)) {
    viewport.releasePointerCapture(event.pointerId);
  }
  viewport.classList.remove("is-panning");
  graph.classList.remove("no-transition");
  panStart = null;
});

viewport.addEventListener("pointercancel", () => {
  viewport.classList.remove("is-panning");
  graph.classList.remove("no-transition");
  panStart = null;
});

// Resize -> refit (preserve user zoom ratio)
let resizeTimer = null;
window.addEventListener("resize", () => {
  if (resizeTimer) cancelAnimationFrame(resizeTimer);
  resizeTimer = requestAnimationFrame(() => {
    graph.classList.add("no-transition");
    fitToViewport({ preserveUserScale: true });
    requestAnimationFrame(() => graph.classList.remove("no-transition"));
  });
});

/* -------------------- Go -------------------- */
boot().catch(error => {
  const warning = document.querySelector("#warning");
  warning.hidden = false;
  warning.textContent = error.message;
  console.error(error);
});
