import { clsx, copy, html } from "./utils.js";

/**
 * Data:
 *  - projectName: string
 *  - graphNodes: array of graph nodes
 *  - status: API status payload
 *
 * Events: none
 */
function normaliseState(raw) {
  const state = String(raw || "synced").toLowerCase();
  return ["synced", "ghost", "orphaned", "drift"].includes(state) ? state : "synced";
}

function StatusBezel({ projectName, graphNodes, status }) {
  const nodes = Array.isArray(graphNodes) ? graphNodes : [];
  const errors = Number(status?.errors || status?.finding_counts?.errors || 0);
  const warnings = Number(status?.warnings || status?.finding_counts?.warnings || 0);
  const infos = Number(status?.infos || status?.finding_counts?.infos || 0);
  const findings = Number(status?.findings || status?.finding_count || 0);
  const interfaceHash = String(status?.interface_hash || "")
    .replace(/\s+/g, "")
    .slice(0, 10);
  const driftMode = errors > 0 || warnings > 0;

  const stateCounts = {
    synced: 0,
    ghost: 0,
    orphaned: 0,
    drift: 0,
  };

  for (const node of nodes) {
    const state = normaliseState(node?.state);
    stateCounts[state] += 1;
  }

  return html`
    <header class="status-bezel" role="status">
      <p class="status-kicker">${projectName || copy("webui.project")}</p>
      <div class="status-grid" aria-label=${copy("webui.metrics")}>
        <span class="status-cell"><strong>${nodes.length}</strong><span>${copy("webui.nodes")}</span></span>
        <span class="status-cell"><strong>${stateCounts.synced + stateCounts.ghost + stateCounts.orphaned + stateCounts.drift}</strong><span>${copy("webui.count-nodes")}</span></span>
        <span class="status-cell"><strong>${findings}</strong><span>${copy("webui.findings-count")}</span></span>
        <span class="status-cell"><strong>${String(interfaceHash || copy("webui.none"))}</strong><span>${copy("webui.interface-hash")}</span></span>
      </div>
      <div class="status-grid" role="status">
        <span class=${clsx("status-cell", driftMode ? "drift" : "synced")}>
          <strong>${driftMode ? copy("webui.status-drift") : copy("webui.status-synced")}</strong>
          <span>${driftMode ? `${errors} / ${warnings} / ${infos}` : copy("webui.status-clean")}</span>
        </span>
      </div>
    </header>`;
}

function StatusCounts({ nodes }) {
  const sorted = {
    synced: 0,
    ghost: 0,
    orphaned: 0,
    drift: 0,
  };

  for (const node of nodes) {
    sorted[normaliseState(node?.state)] = (sorted[normaliseState(node?.state)] || 0) + 1;
  }

  return html`
    <dl class="status-breakdown">
      <div><dt>${copy("webui.states.synced")}</dt><dd>${sorted.synced}</dd></div>
      <div><dt>${copy("webui.states.ghost")}</dt><dd>${sorted.ghost}</dd></div>
      <div><dt>${copy("webui.states.orphaned")}</dt><dd>${sorted.orphaned}</dd></div>
      <div><dt>${copy("webui.states.drift")}</dt><dd>${sorted.drift}</dd></div>
    </dl>`;
}

export { StatusBezel, StatusCounts };
