import { clsx, copy, html } from "./utils.js";

/**
 * Data:
 *  - projectName: string
 *  - graphNodes: array of graph nodes
 *  - edges: array of dependency edges
 *  - status: API status payload
 *
 * Events: none
 */

function StatusBezel({ projectName, graphNodes, edges, status }) {
  const dependencyCount = (Array.isArray(edges) ? edges : []).filter((edge) => String(edge?.kind || "").toLowerCase() === "dependency").length;
  const nodes = Array.isArray(graphNodes) ? graphNodes : [];
  const errors = Number(status?.errors || status?.finding_counts?.errors || 0);
  const warnings = Number(status?.warnings || status?.finding_counts?.warnings || 0);
  const infos = Number(status?.infos || status?.finding_counts?.infos || 0);
  const findings = Number(status?.findings || status?.finding_count || 0);
  const interfaceHash = String(status?.interface_hash || "")
    .replace(/\s+/g, "")
    .slice(0, 10);
  const driftMode = errors > 0 || warnings > 0;

  return html`
    <header class="status-bezel" role="status">
      <p class="status-kicker">${projectName || copy("webui.project")}</p>
      <div class="status-grid" aria-label=${copy("webui.metrics")}>
        <span class="status-cell"><strong>${nodes.length}</strong><span>${copy("webui.nodes")}</span></span>
        <span class="status-cell"><strong>${dependencyCount}</strong><span>${copy("webui.dependencies")}</span></span>
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

export { StatusBezel };
