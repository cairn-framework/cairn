import { BacklogSections, ChannelItem } from "./channel-bar.js";
import { copy, html } from "./utils.js";

// Over-harness console (todo.overharness-console-ux Task 2/3): one-screen
// READ-ONLY composition of the three steering surfaces, each traceable to
// the CLI command that produces it. Pending presentation belongs to
// todo.pending-queue-briefing and the backlog projection to
// todo.roadmap-derived-view; this surface only composes their renderers.
// It renders what the driver would read and dispatches nothing until the
// control-plane programme decision is signed (dec.user-surfaces stands).

function Lane({ kind, title, source, children }) {
  return html`
    <section class=${`console-lane console-lane-${kind}`} aria-label=${title}>
      <header class="console-lane-head">
        <h2 class="console-lane-title">${title}</h2>
        <span class="console-lane-source">${source}</span>
      </header>
      <div class="console-lane-body">${children}</div>
    </section>`;
}

function FrontierBody({ frontier, onSelect }) {
  // A failed load must never masquerade as the successful empty state:
  // "no ghost nodes exist" is a factual claim about the graph.
  if (frontier?.failed) {
    return html`<p class="plate-meta">${copy("webui.bootstrap-frontier-failed")}</p>`;
  }
  const ready = Array.isArray(frontier?.ready) ? frontier.ready : [];
  const blocked = Array.isArray(frontier?.blocked) ? frontier.blocked : [];
  if (!ready.length && !blocked.length) {
    return html`
      <div class="console-frontier-empty">
        <p class="channel-empty">${copy("webui.empty.frontier")}</p>
        <p class="plate-meta">${copy("webui.empty.frontier-detail")}</p>
      </div>`;
  }
  const entryRow = (entry, badge, detail) => html`
    <article class="channel-item">
      <div class="channel-row">
        <span class="plate-meta">${badge}</span>
        <span class="channel-code" title=${entry?.node}>${entry?.node}</span>
        ${detail ? html`<span class="plate-meta channel-body-text" title=${detail}>${detail}</span>` : null}
        <span class="plate-meta channel-inline-meta">${copy("webui.console.tier").replace("{tier}", String(entry?.tier ?? ""))}</span>
        <button class="query-action channel-focus" type="button" onClick=${() => onSelect(entry?.node)}>
          ${copy("webui.focus-node")}
        </button>
      </div>
    </article>`;
  const blockedDetail = (entry) => {
    const blocking = Array.isArray(entry?.blocking) ? entry.blocking : [];
    return blocking.length ? copy("webui.console.blocked-by").replace("{nodes}", blocking.join(", ")) : "";
  };
  return html`
    ${ready.map((entry) => entryRow(entry, copy("webui.console.frontier-ready"), ""))}
    ${blocked.map((entry) => entryRow(entry, copy("webui.console.frontier-blocked"), blockedDetail(entry)))}`;
}

function Console({ pendingRows, frontier, backlog, onSelect }) {
  return html`
    <div class="console-workspace" aria-label=${copy("webui.console.label")}>
      <${Lane} kind="pending" title=${copy("webui.console.pending")} source="cairn pending">
        ${!pendingRows.length ? html`<p class="channel-empty">${copy("webui.empty.pending")}</p>` : pendingRows.map((row) => html`<${ChannelItem} item=${row} kind="pending" onFocus=${onSelect} />`)}
      </${Lane}>
      <${Lane} kind="frontier" title=${copy("webui.console.frontier")} source="cairn frontier">
        <${FrontierBody} frontier=${frontier} onSelect=${onSelect} />
      </${Lane}>
      <${Lane} kind="roadmap" title=${copy("webui.console.roadmap")} source="cairn roadmap">
        ${!backlog.length ? html`<p class="channel-empty">${copy("webui.empty.backlog")}</p>` : html`<${BacklogSections} items=${backlog} onItem=${onSelect} />`}
      </${Lane}>
    </div>`;
}

export { Console };
