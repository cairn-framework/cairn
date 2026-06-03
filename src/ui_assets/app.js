/* Cairn webui v2. Preact + htm, vendored runtime.
 *
 * All colors, spacing, radii, motion, and type come from docs/design-system/tokens.css.
 * Do not hardcode hex or rem values here. All user-visible prose uses plain English
 * punctuation (no em-dashes). Paths and slugs are in IBM Plex Mono; titles are in
 * Source Serif 4; UI labels are in IBM Plex Sans.
 *
 * Data flow:
 *   boot          -> GET /api/graph, GET /api/status, GET /api/lint
 *   select node   -> GET /api/node/:id plus six artefact kinds plus depends/dependents
 *   view source   -> GET /api/blueprint
 *
 * The chain-balance numerics are client-computed as a proxy from the authority
 * and provenance artefact counts: provenance ~= sources + research, authority ~=
 * contracts + decisions, clamped to the 0 through 5 band the widget expects.
 */

(function () {
  "use strict";

  if (typeof window === "undefined" || !window.preact || !window.preactHooks || !window.htm) {
    // eslint-disable-next-line no-console
    console.error("cairn: vendored runtime failed to load");
    return;
  }

  const { h, render, Fragment } = window.preact;
  const { useState, useEffect, useMemo, useRef, useCallback } = window.preactHooks;
  const html = window.htm.bind(h);

  // ==========================================================================
  // Utilities
  // ==========================================================================

  let _copyData = null;
  async function loadCopy() {
    try {
      _copyData = await fetchJson("/assets/copy.json");
    } catch (e) {
      console.warn("cairn: copy.json failed to load, using fallback keys", e);
      _copyData = {};
    }
  }
  function copy(key) {
    if (!_copyData) {
      console.warn("cairn: copy data not loaded yet, using key:", key);
      return key;
    }
    const segments = key.split(".");
    let current = _copyData;
    for (const seg of segments) {
      if (current == null || typeof current !== "object") {
        console.warn("cairn: copy key missing:", key);
        return key;
      }
      current = current[seg];
    }
    if (typeof current !== "string") {
      console.warn("cairn: copy key missing:", key);
      return key;
    }
    return current;
  }

  function copyFinding(code) {
    if (!_copyData) return null;
    const obj = (_copyData.findings || {}).codes || {};
    const entry = obj[code];
    if (!entry || typeof entry !== "object") return null;
    return entry;
  }

  function substituteCopy(template, vars) {
    return template.replace(/\{(\w+)\}/g, (m, k) => (k in vars ? vars[k] : m));
  }

  function CopyButton({ text }) {
    const [copied, setCopied] = useState(false);
    const onClick = useCallback(() => {
      if (navigator.clipboard) {
        navigator.clipboard.writeText(text).then(() => {
          setCopied(true);
          setTimeout(() => setCopied(false), 1200);
        });
      } else {
        // Fallback: create a temporary textarea
        const ta = document.createElement("textarea");
        ta.value = text;
        document.body.appendChild(ta);
        ta.select();
        document.execCommand("copy");
        document.body.removeChild(ta);
        setCopied(true);
        setTimeout(() => setCopied(false), 1200);
      }
    }, [text]);
    return html`
      <button class="btn ghost copy-btn" onClick=${onClick}>
        ${copied ? "Copied" : "Copy"}
      </button>
    `;
  }

  const SEVERITY_RANK = { error: 0, warning: 1, info: 2 };

  function pickNudgeFinding(findings, nodeId) {
    if (!findings || !nodeId) return null;
    const nodeFn = findings.filter((f) => f.node === nodeId);
    if (nodeFn.length === 0) return null;
    return nodeFn.reduce((best, f) => {
      const br = SEVERITY_RANK[best.severity] ?? 2;
      const fr = SEVERITY_RANK[f.severity] ?? 2;
      if (fr < br) return f;
      if (fr === br && f.code < best.code) return f;
      return best;
    });
  }

  function clsx(...values) {
    return values.filter(Boolean).join(" ");
  }

  function percentEncodeId(id) {
    return encodeURIComponent(id);
  }

  async function fetchJson(url, options) {
    const response = await fetch(url, options);
    if (!response.ok && response.status !== 404) {
      throw new Error(`request failed: ${url} (${response.status})`);
    }
    if (response.status === 204) return null;
    return response.json();
  }

  async function fetchGraph() {
    return fetchJson("/api/graph");
  }

  async function fetchStatus() {
    return fetchJson("/api/status");
  }

  async function fetchLint() {
    return fetchJson("/api/lint");
  }

  async function fetchNodeArtefacts(id, kind) {
    const response = await fetchJson(`/api/node/${percentEncodeId(id)}/${kind}`);
    if (!response || !Array.isArray(response.artefacts)) return [];
    return response.artefacts;
  }

  async function fetchDepends(id) {
    const response = await fetchJson(`/api/depends/${percentEncodeId(id)}`);
    if (!response || !Array.isArray(response.nodes)) return [];
    return response.nodes;
  }

  async function fetchDependents(id) {
    const response = await fetchJson(`/api/dependents/${percentEncodeId(id)}`);
    if (!response || !Array.isArray(response.nodes)) return [];
    return response.nodes;
  }

  async function fetchBlueprint() {
    return fetchJson("/api/blueprint");
  }

  function escapeHtml(value) {
    return String(value)
      .replace(/&/g, "&amp;")
      .replace(/</g, "&lt;")
      .replace(/>/g, "&gt;")
      .replace(/"/g, "&quot;");
  }

  // Tokenize a blueprint line into coloured spans. Mirrors the v2 prototype so
  // the syntax palette is consistent across the blueprint-card inspector and the
  // view-source modal.
  function highlightBlueprint(src, highlightModuleId) {
    if (!src) return "";
    const patterns = [
      { re: /^#.*$/, cls: "cm" },
      { re: /"[^"]*"/, cls: "str" },
      { re: /@\w+/, cls: "tag" },
      { re: /\b(System|Container|Module|Actor)\b/, cls: "kw" },
      { re: /\b(path|contract|decisions|research|sources|todos|reviews|id)\b/, cls: "key" },
    ];
    return src
      .split("\n")
      .map((line) => {
        const hit = highlightModuleId && line.includes(`"${highlightModuleId}"`);
        let i = 0;
        let out = "";
        while (i < line.length) {
          let matched = null;
          for (const p of patterns) {
            const re = new RegExp(p.re.source, "g");
            re.lastIndex = i;
            const m = re.exec(line);
            if (m && m.index === i) {
              matched = { text: m[0], cls: p.cls };
              break;
            }
          }
          if (matched) {
            out += `<span class="${matched.cls}">${escapeHtml(matched.text)}</span>`;
            i += matched.text.length;
          } else {
            out += escapeHtml(line[i]);
            i += 1;
          }
        }
        return hit ? `<span class="hi">${out}</span>` : out;
      })
      .join("\n");
  }

  function truncate(value, limit) {
    if (!value || value.length <= limit) return value || "";
    return value.slice(0, limit - 1) + "\u2026";
  }

  // Clamp any count into the 0 through 5 band used by the chain-balance widget.
  function balanceFromCount(count) {
    if (!count || count <= 0) return 0;
    if (count >= 10) return 5;
    return Math.max(1, Math.round(count / 2));
  }

  function fillPercent(value) {
    return `${Math.min(100, Math.max(0, value * 20))}%`;
  }

  // ==========================================================================
  // Layout: hinge placement of System -> Container -> Module
  // ==========================================================================

  const LAYOUT = Object.freeze({
    originX: 900,
    startY: 160,
    groupGap: 72,
    moduleGap: 30,
    moduleWidth: 250,
    moduleHeight: 108,
    containerWidth: 280,
    containerHeight: 74,
    systemWidth: 260,
    systemHeight: 66,
    splayMax: 220,
  });

  function buildLayout(graph, artefactCounts) {
    if (!graph) return { nodes: [], totalHeight: 0 };
    const systems = graph.nodes.filter((n) => n.kind === "system");
    const containers = graph.nodes.filter((n) => n.kind === "container");
    const modules = graph.nodes.filter((n) => n.kind === "module");
    const actors = graph.nodes.filter((n) => n.kind === "actor");
    const system = systems[0] || null;

    const laid = [];
    const { originX, startY, moduleGap, groupGap } = LAYOUT;
    let y = startY;

    if (system) {
      laid.push({
        id: system.id,
        kind: "system",
        data: system,
        x: originX,
        y: 60,
        width: LAYOUT.systemWidth,
        height: LAYOUT.systemHeight,
      });
    }

    const placeModulesFor = (containerId) => {
      const children = modules
        .filter((m) => m.parent === containerId)
        .slice()
        .sort((a, b) => {
          const rank = (state) =>
            state === "ghost" ? 0 : state === "orphaned" ? 1 : 2;
          const rr = rank(a.state) - rank(b.state);
          if (rr !== 0) return rr;
          return (a.name || a.id).localeCompare(b.name || b.id);
        });
      for (const m of children) {
        const counts = artefactCounts.get(m.id) || null;
        const prov = counts ? balanceFromCount(counts.provenance) : 0;
        const auth = counts ? balanceFromCount(counts.authority) : 0;
        const balance = (auth - prov) / 5;
        const x = originX + balance * LAYOUT.splayMax;
        laid.push({
          id: m.id,
          kind: "module",
          data: m,
          counts,
          x,
          y,
          width: LAYOUT.moduleWidth,
          height: LAYOUT.moduleHeight,
        });
        y += LAYOUT.moduleHeight + moduleGap;
      }
    };

    const topContainers = containers.filter(
      (c) => c.parent === (system ? system.id : null) || !c.parent,
    );
    const orphanedContainers = containers.filter((c) => !topContainers.includes(c));

    for (const container of [...topContainers, ...orphanedContainers]) {
      laid.push({
        id: container.id,
        kind: "container",
        data: container,
        x: originX,
        y,
        width: LAYOUT.containerWidth,
        height: LAYOUT.containerHeight,
      });
      y += LAYOUT.containerHeight + 28;
      placeModulesFor(container.id);
      y += groupGap;
    }

    const placedIds = new Set(laid.map((n) => n.id));
    const strayModules = modules.filter((m) => !placedIds.has(m.id));
    if (strayModules.length > 0) {
      laid.push({
        id: "__stray__",
        kind: "divider",
        data: { name: "Uncontained" },
        x: originX,
        y,
        width: LAYOUT.containerWidth,
        height: LAYOUT.containerHeight,
      });
      y += LAYOUT.containerHeight + 28;
      for (const m of strayModules) {
        const counts = artefactCounts.get(m.id) || null;
        laid.push({
          id: m.id,
          kind: "module",
          data: m,
          counts,
          x: originX,
          y,
          width: LAYOUT.moduleWidth,
          height: LAYOUT.moduleHeight,
        });
        y += LAYOUT.moduleHeight + moduleGap;
      }
      y += groupGap;
    }

    for (const a of actors) {
      laid.push({
        id: a.id,
        kind: "actor",
        data: a,
        counts: null,
        x: originX,
        y,
        width: LAYOUT.moduleWidth,
        height: LAYOUT.moduleHeight,
      });
      y += LAYOUT.moduleHeight + moduleGap;
    }

    return { nodes: laid, totalHeight: y + 40 };
  }

  function ownershipPath(from, to) {
    const fx = from.x;
    const fy = from.y + from.height / 2;
    const tx = to.x;
    const ty = to.y - to.height / 2;
    const midY = fy + (ty - fy) * 0.55;
    return `M ${fx} ${fy} C ${fx} ${midY}, ${tx} ${midY}, ${tx} ${ty}`;
  }
  // Approximate midpoint of an ownership bezier curve for label placement.
  function edgeMidpoint(from, to) {
    const fx = from.x;
    const fy = from.y + from.height / 2;
    const tx = to.x;
    const ty = to.y - to.height / 2;
    const midY = fy + (ty - fy) * 0.55;
    // Cubic bezier at t=0.5:
    // x = 0.5*fx + 0.5*tx
    // y = 0.125*fy + 0.75*midY + 0.125*ty
    return {
      x: (fx + tx) / 2,
      y: 0.125 * fy + 0.75 * midY + 0.125 * ty,
    };
  }

  // ==========================================================================
  // Brand mark (stacked stones SVG, matches design-system landing)
  // ==========================================================================

  function CairnMark() {
    return html`
      <svg viewBox="0 0 28 28" width="28" height="28" fill="none">
        <ellipse class="stone stone-base" cx="14" cy="23" rx="11" ry="3.5"
          fill="var(--stone-5)" stroke="var(--seam-carved)" stroke-width="0.6"/>
        <path class="stone stone-mid"
          d="M5.5 18 C 5.5 14.5, 9 13, 14 13 C 19 13, 22.5 14.5, 22.5 18 C 22.5 20, 20 20.5, 14 20.5 C 8 20.5, 5.5 20, 5.5 18 Z"
          fill="var(--stone-4)" stroke="var(--seam-carved)" stroke-width="0.6"/>
        <path class="stone stone-top"
          d="M8 10 C 8 7, 10 5, 14 5 C 18 5, 20 7, 20 10 C 20 12, 17.5 13, 14 13 C 10.5 13, 8 12, 8 10 Z"
          fill="var(--stone-3)" stroke="var(--seam-carved)" stroke-width="0.6"/>
        <ellipse cx="13.5" cy="6" rx="3" ry="0.8" fill="var(--prov-1)" opacity="0.35"/>
      </svg>
    `;
  }

  // ==========================================================================
  // Top bar
  // ==========================================================================

  function TopBar({ status, selection, nodesById, onClear, onOpenCmd, onOpenBlueprint }) {
    const crumbs = [];
    const node = selection ? nodesById.get(selection.id) : null;
    if (node) {
      let cursor = node;
      const chain = [];
      while (cursor) {
        chain.unshift(cursor);
        cursor = cursor.parent ? nodesById.get(cursor.parent) : null;
      }
      for (let i = 0; i < chain.length; i += 1) {
        const isLast = i === chain.length - 1;
        const target = chain[i];
        // Show each ancestor as its short segment. The root system gets its
        // full id (usually just a word, e.g. "cairn"); each descendant shows
        // the trailing id segment so the breadcrumb reads naturally.
        let label = target.id;
        if (i > 0) {
          const parts = target.id.split(".");
          label = parts[parts.length - 1];
        }
        crumbs.push(
          html`<button
            key=${target.id}
            class=${clsx("crumb", isLast && "active")}
            onClick=${() => onClear(target.id)}
          >${label}</button>`,
        );
        if (!isLast) crumbs.push(html`<span class="crumb-sep">.</span>`);
      }
    }

    const graphStats = status
      ? `${status.nodes} nodes, ${status.edges} edges, ${status.findings} findings`
      : "";

    return html`
      <header class="topbar">
        <div class="topbar-left">
          <button class="brand" onClick=${() => onClear(null)} title="Go to map overview">
            <span class="brand-mark"><${CairnMark}/></span>
            <span class="brand-name">Cairn</span>
          </button>
          <nav class="breadcrumb" aria-label="Selection breadcrumb">
            ${crumbs.length === 0
              ? html`<span class="crumb">map</span>`
              : crumbs}
          </nav>
        </div>
        <div class="topbar-center">
          <button class="cmd-trigger" onClick=${onOpenCmd}>
            <span class="cmd-label">Query</span>
            <span class="cmd-placeholder">${graphStats || "search modules, containers, decisions"}</span>
            <span class="cmd-kbd"><kbd>⌘</kbd><kbd>K</kbd></span>
          </button>
        </div>
        <div class="topbar-right">
          <button class="blueprint-trigger" onClick=${onOpenBlueprint} title="View blueprint source">
            <span class="caps">.blueprint</span>
          </button>
          <div class="avatar">CN</div>
        </div>
      </header>
    `;
  }

  // ==========================================================================
  // Graph canvas
  // ==========================================================================

  function SystemNode({ node, selected, onSelect, dimmed, findingSeverity }) {
    const d = node.data;
    const strokeColor = selected
      ? "var(--seam-carved)"
      : findingSeverity === "error"
        ? "var(--ghost)"
        : findingSeverity === "warning"
          ? "var(--orphaned)"
          : findingSeverity === "info"
            ? "var(--settled)"
            : "var(--seam-thin)";
    return html`
      <g class=${clsx("canvas-node", dimmed && "dimmed")}
         transform=${`translate(${node.x - node.width / 2}, ${node.y - node.height / 2})`}
         onClick=${() => onSelect(node)} data-kind="system">
        <rect width=${node.width} height=${node.height} rx="6"
              fill="var(--stone-3)"
              stroke=${strokeColor}
              stroke-width=${selected ? 1.5 : 1}/>
        <rect width=${node.width} height="1" fill="rgba(255,245,220,0.08)"/>
        <text x="14" y="20" font-size="10" font-family="var(--font-mono)"
              fill="var(--ink-mist)" letter-spacing="2.5" style="text-transform:uppercase">SYSTEM</text>
        <text x="14" y="42" font-size="17" font-family="var(--font-serif)"
              fill="var(--ink-char)" font-weight="500" letter-spacing="-0.3"
              style="font-variation-settings: 'opsz' 24">${d.name}</text>
        <text x="14" y="58" font-size="10.5" font-family="var(--font-mono)"
              fill="var(--ink-mist)" letter-spacing="0.5">${d.id}</text>
      </g>
    `;
  }

  function ContainerNode({ node, selected, onSelect, dimmed, findingSeverity }) {
    const d = node.data;
    const strokeColor = selected
      ? "var(--seam-carved)"
      : findingSeverity === "error"
        ? "var(--ghost)"
        : findingSeverity === "warning"
          ? "var(--orphaned)"
          : findingSeverity === "info"
            ? "var(--settled)"
            : "var(--seam-thin)";
    return html`
      <g class=${clsx("canvas-node", dimmed && "dimmed")}
         transform=${`translate(${node.x - node.width / 2}, ${node.y - node.height / 2})`}
         onClick=${() => onSelect(node)} data-kind="container">
        <rect width=${node.width} height=${node.height} rx="6"
              fill="var(--stone-3)"
              stroke=${strokeColor}
              stroke-width=${selected ? 1.5 : 1}/>
        <rect width=${node.width} height="1" fill="rgba(255,245,220,0.08)"/>
        <text x="14" y="20" font-size="10" font-family="var(--font-mono)"
              fill="var(--ink-mist)" letter-spacing="2.5" style="text-transform:uppercase">CONTAINER</text>
        <text x="14" y="44" font-size="17" font-family="var(--font-serif)"
              fill="var(--ink-char)" font-weight="500" letter-spacing="-0.3"
              style="font-variation-settings: 'opsz' 24">${d.name}</text>
        <text x="14" y="62" font-size="10.5" font-family="var(--font-mono)"
              fill="var(--ink-mist)" letter-spacing="0.5">${d.id}</text>
      </g>
    `;
  }

  function ModuleNode({ node, selected, hovered, dimmed, findingSeverity, onSelect, onHover, dependentCount }) {
    const d = node.data;
    const recon = d.state || "synced";
    const breath = recon !== "synced";
    const statusColor =
      recon === "ghost"
        ? "var(--ghost)"
        : recon === "orphaned"
          ? "var(--orphaned)"
          : "var(--synced)";
    const strokeColor = selected
      ? "var(--seam-carved)"
      : findingSeverity === "error"
        ? "var(--ghost)"
        : findingSeverity === "warning"
          ? "var(--orphaned)"
          : findingSeverity === "info"
            ? "var(--settled)"
            : recon === "ghost"
              ? "var(--ghost)"
              : recon === "orphaned"
                ? "var(--orphaned)"
                : "var(--seam-thin)";

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
        <rect x="2" y="3" width=${node.width} height=${node.height} rx="6" fill="rgba(0,0,0,0.3)"/>
        <rect width=${node.width} height=${node.height} rx="6"
              fill=${hovered ? "var(--stone-4)" : "var(--stone-3)"}
              stroke=${strokeColor}
              stroke-width=${selected ? 1.5 : 1}
              stroke-dasharray=${recon === "ghost" ? "4 3" : recon === "orphaned" ? "2 3" : "0"}/>
        <rect width=${node.width} height="1" fill="rgba(255,245,220,0.1)"/>
        <rect x="0" y="0" width="3" height=${node.height}
              fill="var(--prov-2)" opacity=${provStrength * 0.7 + 0.3}/>
        <rect x=${node.width - 3} y="0" width="3" height=${node.height}
              fill="var(--auth-2)" opacity=${authStrength * 0.7 + 0.3}/>
        <text x="14" y="20" font-size="10" font-family="var(--font-mono)"
              fill="var(--ink-mist)" letter-spacing="2" style="text-transform:uppercase">MODULE</text>
        <text x="64" y="20" font-size="10" font-family="var(--font-mono)"
              fill="var(--ink-ghost)" letter-spacing="1">· ${truncate(d.id, 24)}</text>
        <circle cx=${node.width - 16} cy="16" r="3.5" fill=${statusColor}/>
        ${breath
          ? html`<circle cx=${node.width - 16} cy="16" r="6" fill="none" stroke=${statusColor} stroke-width="1" opacity="0.4">
              <animate attributeName="r" values="4;8;4" dur="2.4s" repeatCount="indefinite"/>
              <animate attributeName="opacity" values="0.5;0;0.5" dur="2.4s" repeatCount="indefinite"/>
            </circle>`
          : null}
        ${findingSeverity
          ? html`<rect x=${node.width - 10} y="8" width="6" height="6" rx="1.5"
                fill=${findingSeverity === "error" ? "var(--ghost)" : findingSeverity === "warning" ? "var(--orphaned)" : "var(--settled)"}/>`
          : null}
        <text x="14" y="46" font-size="17" font-family="var(--font-serif)"
              fill="var(--ink-char)" font-weight="500" letter-spacing="-0.3"
              style="font-variation-settings: 'opsz' 20">${truncate(d.name, 22)}</text>
        <text x="14" y="62" font-size="10.5" font-family="var(--font-mono)"
              fill="var(--ink-mist)" letter-spacing="0.4">${truncate(d.id, 28)}</text>
        ${recon !== "synced"
          ? html`<g transform=${`translate(${node.width - 74}, 30)`}>
              <rect x="0" y="0" width="58" height="16" rx="3"
                    fill=${recon === "ghost" ? "var(--ghost-wash)" : "var(--orphan-wash)"}
                    stroke=${recon === "ghost" ? "var(--ghost)" : "var(--orphaned)"}
                    stroke-width="0.75"/>
              <text x="29" y="11" font-size="9" font-family="var(--font-mono)"
                    fill=${recon === "ghost" ? "var(--ghost)" : "var(--orphaned)"}
                    letter-spacing="1.2" text-anchor="middle"
                    style="text-transform:uppercase">${recon}</text>
            </g>`
          : null}
        <line x1="12" y1="78" x2=${node.width - 12} y2="78" stroke="var(--seam-faint)"/>
        <g transform="translate(14, 88)">
          <rect x="0" y="3" width="72" height="3" rx="1.5" fill="rgba(255,255,255,0.04)"/>
          <rect x=${72 - provStrength * 72} y="3" width=${provStrength * 72} height="3"
                rx="1.5" fill="var(--prov-2)"/>
          <circle cx="82" cy="4.5" r="2.5" fill="var(--hinge-1)"/>
          <rect x="92" y="3" width="72" height="3" rx="1.5" fill="rgba(255,255,255,0.04)"/>
          <rect x="92" y="3" width=${authStrength * 72} height="3" rx="1.5" fill="var(--auth-2)"/>
          <text x=${node.width - 28} y="7" font-size="9" font-family="var(--font-mono)"
                fill="var(--ink-faded)" letter-spacing="0.3" text-anchor="end">${
                  dependentCount > 0 ? `${dependentCount} dep` : ""
                }</text>
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
              fill="var(--ink-mist)" letter-spacing="1.5" style="text-transform:uppercase">
          ${node.data.name}
        </text>
      </g>
    `;
  }

  function GraphCanvas({
    graph,
    layoutData,
    selection,
    hoveredId,
    lint,
    onSelect,
    onHover,
    edgeTrace,
  }) {
    const svgRef = useRef(null);
    const [viewport, setViewport] = useState({ x: 0, y: 0, zoom: 1 });
    const [panState, setPanState] = useState(null);
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

    const onMouseDown = (e) => {
      if (e.button !== 0) return;
      setPanState({
        startX: e.clientX,
        startY: e.clientY,
        origX: viewport.x,
        origY: viewport.y,
      });
    };
    const onMouseMove = (e) => {
      if (!panState) return;
      setViewport((v) => ({
        ...v,
        x: panState.origX + (e.clientX - panState.startX),
        y: panState.origY + (e.clientY - panState.startY),
      }));
    };
    const onMouseUp = () => setPanState(null);
    const onWheel = (e) => {
      if (e.ctrlKey || e.metaKey) {
        e.preventDefault();
        const delta = -e.deltaY * 0.002;
        setViewport((v) => ({
          ...v,
          zoom: Math.max(0.4, Math.min(2.0, v.zoom + delta)),
        }));
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
               onMouseDown=${onMouseDown} onMouseMove=${onMouseMove}
               onMouseUp=${onMouseUp} onMouseLeave=${onMouseUp}
               onWheel=${onWheel} aria-label="Architecture map">
        <div class="graph-bg"></div>
        <div class="chain-banner">
          <div class="label prov"><span class="rule"></span>Provenance</div>
          <div class="label hinge"><span class="rule"></span>Hinge<span class="rule"></span></div>
          <div class="label auth">Authority<span class="rule"></span></div>
        </div>
        <svg ref=${svgRef} class="graph-svg" width="100%" height="100%">
          <g transform=${`translate(${viewport.x}, ${viewport.y}) scale(${viewport.zoom})`}>
            <line x1="900" y1="20" x2="900" y2=${totalHeight}
                  stroke="var(--seam-clear)" stroke-dasharray="1 6" opacity="0.6"/>
            ${ownershipEdges.map((e, i) => html`
              <path key=${`o-${i}`} class=${clsx("edge", isTraced(e) && "traced", isDimmed(e) && "dimmed")}
                    d=${e.d}/>
            `)}
            ${dependencyEdges.map((e, i) => html`
              <path key=${`d-${i}`} class=${clsx("edge dependency", isTraced(e) && "traced", isDimmed(e) && "dimmed")}
                    d=${e.d}/>
            `)}
            ${ownershipEdges.map((e, i) => {
              const m = edgeMidpoint(e.from, e.to);
              return html`
                <g key=${`ol-${i}`} class=${clsx("edge-label", isDimmed(e) && "dimmed")}
                   transform=${`translate(${m.x}, ${m.y})`}
                   opacity=${isTraced(e) || !edgeTrace ? 1 : 0.3}>
                  <text font-size="9" font-family="var(--font-mono)" fill="var(--ink-ghost)"
                        text-anchor="middle" dy="-4">${e.description || ""}</text>
                </g>`;
            })}
            ${dependencyEdges.map((e, i) => {
              const m = edgeMidpoint(e.from, e.to);
              return html`
                <g key=${`dl-${i}`} class=${clsx("edge-label", isDimmed(e) && "dimmed")}
                   transform=${`translate(${m.x}, ${m.y})`}
                   opacity=${isTraced(e) || !edgeTrace ? 1 : 0.3}>
                  <text font-size="9" font-family="var(--font-mono)" fill="var(--ink-ghost)"
                        text-anchor="middle" dy="-4">${e.description || ""}</text>
                </g>`;
            })}
            ${nodes.map((n) => {
              const isSelected = selection && selection.id === n.id;
              const isHovered = hoveredId === n.id;
              const findingSeverity = nodeSeverity.get(n.id) || null;
              if (n.kind === "system") return html`<${SystemNode} key=${n.id} node=${n}
                selected=${isSelected} findingSeverity=${findingSeverity} onSelect=${(nd) => onSelect(nd.id)}/>`;
              if (n.kind === "container") return html`<${ContainerNode} key=${n.id} node=${n}
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

        <div class="graph-minimap" title="Overview of reconciliation state">
          ${graph
            ? graph.nodes
                .filter((n) => n.kind === "module")
                .slice(0, 48)
                .map((m) => {
                  const active = selection && selection.id === m.id;
                  const state = m.state || "synced";
                  return html`<div key=${m.id}
                    class=${clsx("mini-dot", state, active && "active")}
                    style="height:22px"
                    onClick=${() => onSelect(m.id)}
                    title=${`${m.name}: ${state}`}></div>`;
                })
            : null}
        </div>

        <div class="graph-legend">
          <span class="sw synced"></span> synced
          <span class="sep"></span>
          <span class="sw ghost"></span> ghost
          <span class="sep"></span>
          <span class="sw orphaned"></span> orphaned
        </div>
      </section>
    `;
  }

  // ==========================================================================
  // Inspector building blocks
  // ==========================================================================

  function Section({ label, count, defaultOpen = false, children }) {
    const [open, setOpen] = useState(defaultOpen);
    return html`
      <div class=${clsx("ins-section", open && "open")}>
        <button class="ins-section-head" onClick=${() => setOpen((o) => !o)}
          aria-expanded=${open}>
          <span class="chev">${open ? "▾" : "▸"}</span>
          <span class="ins-section-label">${label}</span>
          ${count != null ? html`<span class="ins-section-count">${count}</span>` : null}
        </button>
        ${open ? html`<div class="ins-section-body">${children}</div>` : null}
      </div>
    `;
  }

  function reconBadge(state) {
    const label = state || "unknown";
    return html`<span class=${clsx("recon-badge", label)}>${label}</span>`;
  }

  function renderPath(pathText, state) {
    return html`
      <div class="path-row">
        <span class="path-text">${pathText}</span>
        ${reconBadge(state)}
      </div>
    `;
  }

  function BlueprintCard({ node, onViewSource }) {
    const snippet = buildBlueprintSnippet(node);
    const state = node.state || "synced";
    return html`
      <div class="blueprint-card">
        <div class="blueprint-head">
          <span class="caps">Blueprint</span>
          <button class="view-src" onClick=${onViewSource}>View source</button>
          ${reconBadge(state)}
        </div>
        <pre class="blueprint-code" dangerouslySetInnerHTML=${{ __html: snippet }}></pre>
      </div>
    `;
  }

  function buildBlueprintSnippet(node) {
    const kindKeyword =
      node.kind === "system"
        ? "System"
        : node.kind === "container"
          ? "Container"
          : node.kind === "module"
            ? "Module"
            : "Actor";
    const base = `${kindKeyword} ${node.name || ""} "${node.description || ""}" id "${node.id}"`;
    const lines = [base + " {"];
    for (const p of node.paths || []) lines.push(`  path "${p}"`);
    for (const c of node.contracts || []) lines.push(`  contract "${c}"`);
    lines.push("}");
    return highlightBlueprint(lines.join("\n"));
  }

  function ArtefactCard({ artefact }) {
    const status = (artefact.frontmatter && artefact.frontmatter.status) || artefact.type;
    const kindClass = artefact.type === "decisions" ? "decision" : artefact.type;
    return html`
      <div class=${clsx("artefact", kindClass, status)}>
        <div class="artefact-head">
          <span class="artefact-id">${artefact.type}</span>
          <span class=${clsx("artefact-status", status)}>${status}</span>
        </div>
        <div class="artefact-title">${artefact.title || artefact.path}</div>
        <div class="artefact-meta">${artefact.path}</div>
        ${artefact.body
          ? html`<div class="artefact-body">${truncate(artefact.body, 480)}</div>`
          : null}
      </div>
    `;
  }

  function DependencyRow({ entry, onSelect }) {
    return html`
      <button class="dep-row" onClick=${() => onSelect(entry.id)}>
        <span class="dep-name">${entry.name || entry.id}</span>
        ${reconBadge(entry.state || "synced")}
      </button>
    `;
  }

  // Maps a finding severity string to the pill modifier class.
  // error -> ghost (warm-red), warning -> orphaned (weathered), info -> info (mossy-green).
  function severityPill(severity) {
    if (severity === "error") return "ghost";
    if (severity === "warning") return "orphaned";
    return "info";
  }
  // Computes a map of node-id -> highest severity finding for that node.
  // Structural errors, interface contradictions, rationale tensions, and
  // info observations all surface through this unified overlay.
  function nodeSeverityById(lint) {
    const map = new Map();
    if (!lint || !lint.findings) return map;
    const rank = { error: 0, warning: 1, info: 2 };
    for (const f of lint.findings) {
      if (!f.node) continue;
      const current = map.get(f.node);
      if (!current || (rank[f.severity] ?? 2) < (rank[current] ?? 2)) {
        map.set(f.node, f.severity);
      }
    }
    return map;
  }

  // Extracts the code-family prefix used for category filter chips.
  // For alphanumeric codes (e.g. "CT001"): returns the letter prefix ("CT").
  // For underscore codes (e.g. "CAIRN_SOURCE_UNVERIFIED"): returns the first segment ("CAIRN").
  function findingFamily(code) {
    const match = code.match(/^([A-Z]+)\d/);
    if (match) return match[1];
    return code.split("_")[0];
  }

  function ProseNudgeBanner({ lint, nodeId }) {
    const nudge = useMemo(() => {
      if (!lint || !lint.findings) return null;
      const f = pickNudgeFinding(lint.findings, nodeId);
      if (!f) return null;
      const entry = copyFinding(f.code);
      if (!entry) return null;
      const vars = { node: f.node || "", path: f.path || "", target: f.target || "" };
      return {
        severity: f.severity,
        heading: entry.heading || f.code,
        body: substituteCopy(entry.body || f.message, vars),
        cta: entry.cta || null,
      };
    }, [lint, nodeId]);

    if (!nudge) return null;

    return html`
      <div class=${clsx("prose-nudge", nudge.severity)}>
        <div class="prose-nudge-heading">
          <span class=${clsx("pill", severityPill(nudge.severity))}><span class="dot"></span>${nudge.severity}</span>
          <strong>${nudge.heading}</strong>
        </div>
        <p class="prose-nudge-body">${nudge.body}</p>
        ${nudge.cta
          ? html`<div class="prose-nudge-cta-row">
              <code class="prose-nudge-cta">${nudge.cta}</code>
              <${CopyButton} text=${nudge.cta} />
            </div>`
          : null}
      </div>
    `;
  }

  function ModuleInspector({ node, detail, lint, onSelect, onSelectDecision, onViewBlueprint, onClose }) {
    const {
      contracts,
      decisions,
      todos,
      research,
      sources,
      depends,
      dependents,
    } = detail;

    const provCount = (sources?.length || 0) + (research?.length || 0);
    const authCount = (contracts?.length || 0) + (decisions?.length || 0);
    const prov = balanceFromCount(provCount);
    const auth = balanceFromCount(authCount);

    const sortedDecisions = (decisions || []).slice().sort((a, b) => {
      const rank = (s) => (s === "proposed" ? 0 : s === "accepted" ? 1 : 2);
      const sa = (a.frontmatter && a.frontmatter.status) || "accepted";
      const sb = (b.frontmatter && b.frontmatter.status) || "accepted";
      return rank(sa) - rank(sb);
    });

    const containerId = node.parent || "";
    const eyebrowLabel = containerId ? `${node.kind} · ${containerId}` : node.kind;

    const pathEntries = (node.paths || []).map((p) => ({
      path: p,
      state: node.state || "synced",
    }));

    return html`
      <section class="inspector">
        <div class="ins-header">
          <div class="ins-eyebrow">${eyebrowLabel}</div>
          <button class="ins-close" onClick=${onClose} aria-label="Close inspector">×</button>
        </div>
        <h2 class="ins-title">${node.name || node.id}</h2>
        <div class="ins-slug">${node.id}</div>
        ${node.description
          ? html`<p class="ins-desc">${node.description}</p>`
          : null}

        <div class="pill-row">
          <span class=${clsx("pill", node.state || "synced")}>
            <span class="dot"></span>${node.state || "synced"}
          </span>
          ${(node.tags || []).map((t) => html`<span class="pill" key=${t}>${t}</span>`)}
        </div>

        <${BlueprintCard} node=${node} onViewSource=${onViewBlueprint}/>

        <${ProseNudgeBanner} lint=${lint} nodeId=${node.id}/>

        <div class="paths-block">
          <div class="paths-head">
            <span class="caps">Paths</span>
            <span class="ins-section-count">${pathEntries.length}</span>
          </div>
          <div class="paths-list">
            ${pathEntries.length === 0
              ? html`<div class="row-empty">${copy("empty-states.node-no-paths.body")}</div>`
              : pathEntries.map((p) => renderPath(p.path, p.state))}
          </div>
        </div>

        <div class="chain-balance">
          <div class="balance-grid">
            <div class="balance-side prov">
              <div class="balance-kicker">Provenance</div>
              <div class="balance-value">${prov}</div>
            </div>
            <div class="balance-hinge"></div>
            <div class="balance-side auth">
              <div class="balance-kicker">Authority</div>
              <div class="balance-value">${auth}</div>
            </div>
          </div>
          <div class="balance-tracks">
            <div class="balance-track prov">
              <div class="fill" style=${`width:${fillPercent(prov)}`}></div>
            </div>
            <div style="width:12px;height:12px"></div>
            <div class="balance-track auth">
              <div class="fill" style=${`width:${fillPercent(auth)}`}></div>
            </div>
          </div>
        </div>

        <div class="stat-row">
          <div class="stat-cell">
            <div class="stat-n">${decisions?.length || 0}</div>
            <div class="caps">decisions</div>
          </div>
          <div class="stat-cell">
            <div class="stat-n">${contracts?.length || 0}</div>
            <div class="caps">contracts</div>
          </div>
          <div class="stat-cell">
            <div class="stat-n">${todos?.length || 0}</div>
            <div class="caps">todos</div>
          </div>
          <div class="stat-cell">
            <div class="stat-n">${research?.length || 0}</div>
            <div class="caps">research</div>
          </div>
        </div>

        <${Section} label="Contracts" count=${contracts?.length || 0}>
          ${(contracts || []).length === 0
            ? html`<div class="row-empty">${copy("empty-states.node-no-contracts.body")}</div>`
            : (contracts || []).map((c) => html`<${ArtefactCard} key=${c.path} artefact=${c}/>`)}
        <//>

        <${Section} label="Decisions" count=${decisions?.length || 0} defaultOpen=${sortedDecisions.length > 0}>
          ${sortedDecisions.length === 0
            ? html`<div class="row-empty">${copy("empty-states.node-no-decisions.body")}</div>`
            : sortedDecisions.map((d) => html`
                <button class=${clsx("artefact", "decision", (d.frontmatter && d.frontmatter.status) || "accepted")}
                  key=${d.path} onClick=${() => onSelectDecision(d)}>
                  <div class="artefact-head">
                    <span class="artefact-id">decision</span>
                    <span class=${clsx("artefact-status", (d.frontmatter && d.frontmatter.status) || "accepted")}>
                      ${(d.frontmatter && d.frontmatter.status) || "accepted"}
                    </span>
                  </div>
                  <div class="artefact-title">${d.title || d.path}</div>
                  <div class="artefact-meta">${d.path}</div>
                </button>
              `)}
        <//>

        <${Section} label="Todos" count=${todos?.length || 0}>
          ${(todos || []).length === 0
            ? html`<div class="row-empty">${copy("empty-states.node-no-todos.body")}</div>`
            : (todos || []).map((t) => html`<${ArtefactCard} key=${t.path} artefact=${t}/>`)}
        <//>

        <${Section} label="Research" count=${research?.length || 0}>
          ${(research || []).length === 0
            ? html`<div class="row-empty">${copy("empty-states.node-no-research.body")}</div>`
            : (research || []).map((r) => html`<${ArtefactCard} key=${r.path} artefact=${r}/>`)}
        <//>

        <${Section} label="Sources" count=${sources?.length || 0}>
          ${(sources || []).length === 0
            ? html`<div class="row-empty">${copy("empty-states.node-no-sources.body")}</div>`
            : (sources || []).map((s) => html`<${ArtefactCard} key=${s.path} artefact=${s}/>`)}
        <//>

        <${Section} label="Depends on" count=${depends?.length || 0}>
          ${(depends || []).length === 0
            ? html`<div class="row-empty">${copy("empty-states.node-no-outbound.body")}</div>`
            : (depends || []).map((d) => html`<${DependencyRow} key=${d.id} entry=${d} onSelect=${onSelect}/>`)}
        <//>

        <${Section} label="Dependents" count=${dependents?.length || 0}>
          ${(dependents || []).length === 0
            ? html`<div class="row-empty">${copy("empty-states.node-no-inbound.body")}</div>`
            : (dependents || []).map((d) => html`<${DependencyRow} key=${d.id} entry=${d} onSelect=${onSelect}/>`)}
        <//>
      </section>
    `;
  }

  function DecisionDetail({ decision, node, onBack, onSelect }) {
    const status = (decision.frontmatter && decision.frontmatter.status) || "accepted";
    const date = (decision.frontmatter && decision.frontmatter.date) || null;
    const author = (decision.frontmatter && decision.frontmatter.author) || null;

    const body = decision.body || "";
    const conditionMatch = /##\s*(Condition|When this applies)\s*\n([\s\S]*?)(?=\n##\s|\s*$)/i.exec(body);
    const rationaleMatch = /##\s*Rationale\s*\n([\s\S]*?)(?=\n##\s|\s*$)/i.exec(body);
    const condition = conditionMatch ? conditionMatch[2].trim() : null;
    const rationale = rationaleMatch ? rationaleMatch[1].trim() : body.trim();

    return html`
      <section class="inspector decision-detail">
        <button class="pill back-btn" onClick=${onBack}>← ${node ? node.name : "back"}</button>
        <div class="ins-eyebrow">Decision</div>
        <h2 class="ins-title">${decision.title || decision.path}</h2>
        <div class="pill-row">
          <span class=${clsx("pill", status)}><span class="dot"></span>${status}</span>
          ${date ? html`<span class="pill">${date}</span>` : null}
          ${author ? html`<span class="pill">${author}</span>` : null}
        </div>

        ${condition
          ? html`<div class="decision-condition">
              <div class="caps">When this applies</div>
              <div class="condition-text">${condition}</div>
            </div>`
          : null}

        ${rationale
          ? html`<div class="decision-rationale">
              <div class="caps">Rationale</div>
              <p>${rationale}</p>
            </div>`
          : null}

        <div class="hinge-diagram">
          <div class="hinge-side prov">
            <div class="side-label">Provenance. evidence in</div>
            <div class="hinge-item"><span class="n">·</span>sources cited</div>
            <div class="hinge-item"><span class="n">·</span>research syntheses</div>
            <div class="hinge-item"><span class="n">1</span>decision (this)</div>
          </div>
          <div class="hinge-axis">
            <div class="rod"></div>
            <div class="pivot"></div>
          </div>
          <div class="hinge-side auth">
            <div class="side-label">Authority. rules out</div>
            <div class="hinge-item"><span class="n">·</span>blueprint fragment</div>
            <div class="hinge-item"><span class="n">·</span>contracts attached</div>
            <div class="hinge-item"><span class="n">·</span>code reconciled</div>
          </div>
        </div>

        ${node
          ? html`<div class="attached-modules">
              <div class="caps">Attached to</div>
              <button class="attached-module" onClick=${() => onSelect(node.id)}>
                <div class="name">${node.name}</div>
                <div class="slug">${node.id}</div>
              </button>
            </div>`
          : null}
      </section>
    `;
  }

  function EmptyInspector({ graph, status, lint, onSelect, onShowFindings }) {
    const nodes = graph ? graph.nodes : [];
    const modules = nodes.filter((n) => n.kind === "module");
    const total = modules.length;
    const ghostCount = modules.filter((n) => n.state === "ghost").length;
    const orphanedCount = modules.filter((n) => n.state === "orphaned").length;

    return html`
      <section class="inspector empty-inspector">
        <div class="ins-eyebrow">Map</div>
        <h2 class="ins-title">${graph && graph.nodes[0] ? graph.nodes[0].name : "Cairn"}</h2>
        <div class="ins-slug">
          ${status ? `${status.nodes} nodes · ${status.edges} edges · ${status.findings} findings` : ""}
        </div>
        ${graph && graph.nodes[0] && graph.nodes[0].description
          ? html`<p class="ins-desc">${graph.nodes[0].description}</p>`
          : null}

        <div class="stat-grid">
          <div class="stat-cell">
            <div class="stat-n">${total}</div>
            <div class="caps">modules</div>
          </div>
          <div class="stat-cell">
            <div class=${clsx("stat-n", (ghostCount > 0) && "ghost")}>${ghostCount}</div>
            <div class="caps">ghost</div>
          </div>
          <div class="stat-cell">
            <div class="stat-n">${orphanedCount}</div>
            <div class="caps">orphaned</div>
          </div>
        </div>

        ${lint && lint.findings && lint.findings.length > 0
          ? html`<div>
              <div class="caps" style="margin-bottom:var(--s-2)">Recent findings</div>
              <div class="recent-list">
                ${lint.findings.slice(0, 5).map((f) => html`
                  <button class="recent-row" key=${f.code + (f.node || "") + (f.path || "")}
                    onClick=${() => f.node && onSelect(f.node)}>
                    <span class="r-id">${f.code}</span>
                    <span class="recent-title">${f.message}</span>
                    <span class=${clsx("pill", severityPill(f.severity))}>
                      <span class="dot"></span>${f.severity}
                    </span>
                  </button>
                `)}
              </div>
              <button class="btn-text" style="margin-top:var(--s-2)" onClick=${onShowFindings}>View all findings →</button>
            </div>`
          : html`<div class="row-empty">${copy("empty-states.map-clean.body")}</div>`}

        <div class="hint">
          <kbd>⌘</kbd><kbd>K</kbd> query the map. Click any stone to consult it.
        </div>
      </section>
    `;
  }

  // ==========================================================================
  // Findings rollup panel
  // ==========================================================================

  function FindingsPanel({ lint, selectionId, onSelect, onBack }) {
    const [scope, setScope] = useState("map");
    const [activeCategory, setActiveCategory] = useState(null);

    useEffect(() => {
      if (!selectionId && scope === "node") {
        setScope("map");
        setActiveCategory(null);
      }
    }, [selectionId]);

    const scopeFiltered = useMemo(() => {
      if (!lint || !lint.findings) return [];
      if (scope === "node" && selectionId) return lint.findings.filter((f) => f.node === selectionId);
      return lint.findings;
    }, [lint, scope, selectionId]);

    const findings = useMemo(() => {
      if (!activeCategory) return scopeFiltered;
      return scopeFiltered.filter((f) => findingFamily(f.code) === activeCategory);
    }, [scopeFiltered, activeCategory]);

    const buckets = useMemo(() => {
      const c = { error: 0, warning: 0, info: 0 };
      for (const f of findings) c[f.severity in c ? f.severity : "info"] += 1;
      return c;
    }, [findings]);

    const categories = useMemo(() => {
      const set = new Set();
      for (const f of scopeFiltered) set.add(findingFamily(f.code));
      return [...set].sort();
    }, [scopeFiltered]);

    const nodeDisabled = !selectionId;

    return html`
      <section class="inspector findings-panel">
        <div class="findings-header">
          <button class="btn-text" onClick=${onBack}>← Map</button>
          <div class="findings-buckets">
            <span class="pill ghost"><span class="dot"></span>${buckets.error} error</span>
            <span class="pill orphaned"><span class="dot"></span>${buckets.warning} warn</span>
            <span class="pill info"><span class="dot"></span>${buckets.info} info</span>
          </div>
        </div>

        <div class="findings-controls">
          <div class="scope-toggle">
            <button class=${clsx(scope === "map" && "active")} onClick=${() => { setScope("map"); setActiveCategory(null); }}>Whole map</button>
            <button class=${clsx(scope === "node" && !nodeDisabled && "active")} onClick=${() => { setScope("node"); setActiveCategory(null); }} disabled=${nodeDisabled}>Selected node</button>
          </div>
          ${categories.length > 1
            ? html`<div class="category-chips">
                <button class=${clsx("pill", !activeCategory && "synced")} onClick=${() => setActiveCategory(null)}>All</button>
                ${categories.map((c) => html`
                  <button class=${clsx("pill", activeCategory === c && "synced")} key=${c} onClick=${() => setActiveCategory(activeCategory === c ? null : c)}>${c}</button>
                `)}
              </div>`
            : null}
        </div>

        <div class="findings-list">
          ${findings.length === 0
            ? html`<div class="row-empty">${(scope !== "map" || activeCategory) && scopeFiltered.length > 0 ? copy("empty-states.no-filter-matches.body") : copy("empty-states.map-clean.body")}</div>`
            : findings.map((f) => html`
                <button class="recent-row" key=${f.code + (f.node || "") + (f.path || "")}
                  onClick=${() => f.node && onSelect(f.node)}>
                  <span class="r-id">${f.code}</span>
                  <span class="recent-title">${f.message}</span>
                  <span class=${clsx("pill", severityPill(f.severity))}>
                    <span class="dot"></span>${f.severity}
                  </span>
                </button>
              `)}
        </div>
      </section>
    `;
  }

  // ==========================================================================
  // Command palette
  // ==========================================================================

  function CommandPalette({ open, graph, onClose, onSelect }) {
    const [q, setQ] = useState("");
    const inputRef = useRef(null);

    useEffect(() => {
      if (!open) return undefined;
      setQ("");
      const handle = requestAnimationFrame(() => {
        if (inputRef.current) inputRef.current.focus();
      });
      const onKey = (e) => {
        if (e.key === "Escape") onClose();
      };
      window.addEventListener("keydown", onKey);
      return () => {
        cancelAnimationFrame(handle);
        window.removeEventListener("keydown", onKey);
      };
    }, [open, onClose]);

    if (!open) return null;
    const ql = q.toLowerCase();
    const matches = graph
      ? graph.nodes.filter((n) => {
          if (!ql) return false;
          return (
            n.id.toLowerCase().includes(ql) ||
            (n.name || "").toLowerCase().includes(ql) ||
            (n.kind || "").toLowerCase().includes(ql)
          );
        })
      : [];

    return html`
      <div class="modal-scrim" onClick=${onClose}>
        <div class="cmd-palette" onClick=${(e) => e.stopPropagation()}>
          <div class="cmd-palette-head">
            <span class="cmd-label">Query</span>
            <input ref=${inputRef} value=${q} onInput=${(e) => setQ(e.target.value)}
              placeholder="search modules, containers, decisions"/>
            <kbd>esc</kbd>
          </div>
          ${q === ""
            ? html`<div class="cmd-palette-syntax">
                <div class="caps">Query syntax</div>
                <div class="syntax-grid">
                  <span class="kw">module</span><span class="rest">show a module by id or name</span>
                  <span class="kw">container</span><span class="rest">show a container</span>
                  <span class="kw">ghost</span><span class="rest">list reconciliation gaps</span>
                </div>
              </div>`
            : html`<div class="cmd-palette-results">
                ${matches.length === 0
                  ? html`<div class="row-empty" style="padding:var(--s-5)">${copy("empty-states.search-no-matches.body")}</div>`
                  : html`<${Fragment}>
                      <div class="caps result-group">Nodes</div>
                      ${matches.slice(0, 20).map((n) => html`
                        <button class="result-row" key=${n.id}
                          onClick=${() => {
                            onSelect(n.id);
                            onClose();
                          }}>
                          <span class=${clsx("badge", n.kind === "module" ? "node" : n.kind === "decision" ? "decision" : "node")}>${n.kind}</span>
                          <span class="title">${n.name}</span>
                          <span class="rhs">${n.id}</span>
                        </button>
                      `)}
                    <//>`}
              </div>`}
        </div>
      </div>
    `;
  }

  // ==========================================================================
  // Changes drawer (surfaces active changes / findings)
  // ==========================================================================

  function ChangesDrawer({ open, onToggle, lint, onSelect }) {
    const findings = (lint && lint.findings) || [];
    return html`
      <div class="changes-drawer">
        <button class="drawer-handle" onClick=${onToggle}>
          <span class="label">Findings</span>
          <span class="count">${findings.length}</span>
          <span class="sub">reconciliation and integrity notes</span>
          <span class="chev">${open ? "▾" : "▴"}</span>
        </button>
        ${open
          ? findings.length === 0
            ? html`<div class="drawer-empty">${copy("empty-states.map-clean.body")}</div>`
            : html`<div class="drawer-body">
                ${findings.map((f) => html`
                  <button class="change-card"
                    key=${f.code + (f.node || "") + (f.path || "")}
                    onClick=${() => f.node && onSelect(f.node)}>
                    <div class="card-head">
                      <span class="card-id">${f.code}</span>
                      <span class=${clsx("artefact-status", f.severity === "error" ? "proposed" : "accepted")}>
                        ${f.severity}
                      </span>
                    </div>
                    <div class="card-title">${f.message}</div>
                    <div class="card-slug">${f.path || f.node || ""}</div>
                  </button>
                `)}
              </div>`
          : null}
      </div>
    `;
  }

  // ==========================================================================
  // Blueprint source modal
  // ==========================================================================

  function BlueprintModal({ open, blueprint, focusModuleId, onClose }) {
    useEffect(() => {
      if (!open) return undefined;
      const onKey = (e) => {
        if (e.key === "Escape") onClose();
      };
      window.addEventListener("keydown", onKey);
      return () => window.removeEventListener("keydown", onKey);
    }, [open, onClose]);

    if (!open) return null;
    const source = blueprint && blueprint.source;
    const filePath = blueprint && blueprint.path;
    const innerHtml = source
      ? highlightBlueprint(source, focusModuleId)
      : '<span class="cm">Blueprint source is not available.</span>';

    return html`
      <div class="modal-scrim centered" onClick=${onClose}>
        <div class="blueprint-modal" onClick=${(e) => e.stopPropagation()}>
          <div class="modal-head">
            <span class="kicker">Blueprint source</span>
            <span class="file-path">${filePath || "(unknown path)"}</span>
            <button onClick=${onClose}>close ⎋</button>
          </div>
          <div class="modal-body">
            <pre dangerouslySetInnerHTML=${{ __html: innerHtml }}></pre>
          </div>
          <div class="modal-foot">
            <span>Read-only view of the declared map source.</span>
          </div>
        </div>
      </div>
    `;
  }

  // ==========================================================================
  // App root
  // ==========================================================================

  function App() {
    const [graph, setGraph] = useState(null);
    const [status, setStatus] = useState(null);
    const [lint, setLint] = useState(null);
    const [error, setError] = useState(null);

    const [selectionId, setSelectionId] = useState(null);
    const [selectedDecision, setSelectedDecision] = useState(null);
    const [detail, setDetail] = useState({});
    const [hoveredId, setHoveredId] = useState(null);
    const [cmdOpen, setCmdOpen] = useState(false);
    const [drawerOpen, setDrawerOpen] = useState(false);
    const [blueprintOpen, setBlueprintOpen] = useState(false);
    const [blueprint, setBlueprint] = useState(null);
    const [blueprintFocus, setBlueprintFocus] = useState(null);
    const [showFindings, setShowFindings] = useState(false);

    useEffect(() => {
      let cancelled = false;
      Promise.all([fetchGraph(), fetchStatus(), fetchLint()])
        .then(([g, s, l]) => {
          if (cancelled) return;
          setGraph(g);
          setStatus(s);
          setLint(l);
        })
        .catch((err) => {
          if (!cancelled) setError(err.message);
        });
      return () => {
        cancelled = true;
      };
    }, []);

    useEffect(() => {
      try {
        const saved = localStorage.getItem("cairn:v2:selection");
        if (saved) setSelectionId(saved);
      } catch (_err) {
        // storage disabled; ignore
      }
    }, []);

    useEffect(() => {
      try {
        if (selectionId) localStorage.setItem("cairn:v2:selection", selectionId);
        else localStorage.removeItem("cairn:v2:selection");
      } catch (_err) {
        // ignore
      }
    }, [selectionId]);

    useEffect(() => {
      const onKey = (e) => {
        if ((e.metaKey || e.ctrlKey) && e.key.toLowerCase() === "k") {
          e.preventDefault();
          setCmdOpen((o) => !o);
        }
        if (e.key === "Escape" && !cmdOpen && !blueprintOpen) {
          if (selectedDecision) setSelectedDecision(null);
        }
      };
      window.addEventListener("keydown", onKey);
      return () => window.removeEventListener("keydown", onKey);
    }, [cmdOpen, blueprintOpen, selectedDecision]);

    const nodesById = useMemo(() => {
      const map = new Map();
      if (graph) for (const n of graph.nodes) map.set(n.id, n);
      return map;
    }, [graph]);

    const artefactCountsById = useMemo(() => {
      const map = new Map();
      if (detail && selectionId) {
        map.set(selectionId, {
          provenance: (detail.sources?.length || 0) + (detail.research?.length || 0),
          authority: (detail.contracts?.length || 0) + (detail.decisions?.length || 0),
          decisions: detail.decisions?.length || 0,
          contracts: detail.contracts?.length || 0,
        });
      }
      return map;
    }, [detail, selectionId]);

    const layoutData = useMemo(
      () => buildLayout(graph, artefactCountsById),
      [graph, artefactCountsById],
    );

    useEffect(() => {
      if (!selectionId || !graph) {
        setDetail({});
        return undefined;
      }
      const node = nodesById.get(selectionId);
      if (!node) {
        setDetail({});
        return undefined;
      }
      let cancelled = false;
      Promise.all([
        fetchNodeArtefacts(selectionId, "contract"),
        fetchNodeArtefacts(selectionId, "decisions"),
        fetchNodeArtefacts(selectionId, "todos"),
        fetchNodeArtefacts(selectionId, "research"),
        fetchNodeArtefacts(selectionId, "sources"),
        fetchDepends(selectionId).catch(() => []),
        fetchDependents(selectionId).catch(() => []),
      ]).then(([contracts, decisions, todos, research, sources, depends, dependents]) => {
        if (cancelled) return;
        setDetail({
          contracts,
          decisions,
          todos,
          research,
          sources,
          depends,
          dependents,
        });
      });
      return () => {
        cancelled = true;
      };
    }, [selectionId, nodesById, graph]);

    const openBlueprint = useCallback(() => {
      setBlueprintFocus(selectionId);
      setBlueprintOpen(true);
      if (!blueprint) {
        fetchBlueprint()
          .then((bp) => setBlueprint(bp))
          .catch(() => setBlueprint({ source: null, path: null }));
      }
    }, [selectionId, blueprint]);

    const selectedNode = selectionId ? nodesById.get(selectionId) : null;

    const inspector = showFindings
      ? html`<${FindingsPanel}
          lint=${lint}
          selectionId=${selectionId}
          onSelect=${(id) => { setShowFindings(false); setSelectionId(id); }}
          onBack=${() => setShowFindings(false)}
        />`
      : selectedDecision
        ? html`<${DecisionDetail}
            decision=${selectedDecision}
            node=${selectedNode}
            onBack=${() => setSelectedDecision(null)}
            onSelect=${(id) => {
              setSelectedDecision(null);
              setSelectionId(id);
            }}
          />`
        : selectedNode
          ? html`<${ModuleInspector}
              node=${selectedNode}
              detail=${detail}
              lint=${lint}
              onSelect=${(id) => setSelectionId(id)}
              onSelectDecision=${(d) => setSelectedDecision(d)}
              onViewBlueprint=${openBlueprint}
              onClose=${() => setSelectionId(null)}
            />`
          : html`<${EmptyInspector}
              graph=${graph}
              status=${status}
              lint=${lint}
              onSelect=${(id) => setSelectionId(id)}
              onShowFindings=${() => setShowFindings(true)}
            />`;

    return html`
      <${Fragment}>
        <${TopBar}
          status=${status}
          selection=${selectionId ? { id: selectionId } : null}
          nodesById=${nodesById}
          onClear=${(id) => setSelectionId(id || null)}
          onOpenCmd=${() => setCmdOpen(true)}
          onOpenBlueprint=${openBlueprint}
        />
        ${error
          ? html`<div class="status-banner">Failed to load: ${error}</div>`
          : null}
        <div class="main">
          <${GraphCanvas}
            graph=${graph}
            layoutData=${layoutData}
            selection=${selectionId ? { id: selectionId } : null}
            hoveredId=${hoveredId}
            lint=${lint}
            onSelect=${(id) => setSelectionId(id)}
            onHover=${setHoveredId}
            edgeTrace=${hoveredId}
          />
          <aside class="inspector-wrap" aria-live="polite">
            ${inspector}
          </aside>
        </div>
        <${ChangesDrawer}
          open=${drawerOpen}
          onToggle=${() => setDrawerOpen((o) => !o)}
          lint=${lint}
          onSelect=${(id) => setSelectionId(id)}
        />
        <${CommandPalette}
          open=${cmdOpen}
          graph=${graph}
          onClose=${() => setCmdOpen(false)}
          onSelect=${(id) => setSelectionId(id)}
        />
        <${BlueprintModal}
          open=${blueprintOpen}
          blueprint=${blueprint}
          focusModuleId=${blueprintFocus}
          onClose=${() => setBlueprintOpen(false)}
        />
      <//>
    `;
  }

  const root = document.getElementById("root");
  if (root) loadCopy().then(() => render(h(App, {}), root));
})();
