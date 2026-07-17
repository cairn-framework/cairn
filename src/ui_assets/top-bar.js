/* Top bar: brand mark, selection breadcrumb, command-palette trigger, and
 * report-issue / view-blueprint actions.
 */
import { clsx, copy, html, openReportIssue } from "./utils.js";

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

function TopBar({ graph, lint, selection, nodesById, onClear, onOpenCmd, onOpenBlueprint, version }) {
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

  const graphStats = graph ? `${graph.nodes.length} nodes, ${graph.edges.length} edges, ${lint?.findings?.length ?? 0} findings` : "";

  return html`
    <header class="topbar">
      <div class="topbar-left">
        <button class="brand" onClick=${() => onClear(null)} title="Go to map overview">
          <span class="brand-mark"><${CairnMark}/></span>
          <span class="brand-name">Cairn</span>
        </button>
        <nav class="breadcrumb" aria-label="Selection breadcrumb">
          ${crumbs.length === 0 ? html`<span class="crumb">map</span>` : crumbs}
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
        <button class="blueprint-trigger" onClick=${() => openReportIssue(version)} title="Report an issue">
          <span class="caps">${copy("webui.report.topbar")}</span>
        </button>
        <button class="blueprint-trigger blueprint-open-trigger" onClick=${onOpenBlueprint} title="View blueprint source">
          <span class="caps">.blueprint</span>
        </button>
        <div class="avatar">CN</div>
      </div>
    </header>
  `;
}

export { TopBar };
