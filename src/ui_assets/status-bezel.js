import { clsx, copy, html } from "./utils.js";

function countBySeverity(findings) {
  const normalized = Array.isArray(findings) ? findings : [];
  let errors = 0;
  let warnings = 0;
  let infos = 0;

  for (const finding of normalized) {
    const severity = String(finding?.severity || "").toLowerCase();

    if (severity === "error") {
      errors += 1;
      continue;
    }

    if (severity === "warning") {
      warnings += 1;
      continue;
    }

    if (severity === "info") {
      infos += 1;
    }
  }

  const total = errors + warnings + infos;
  return { total, errors, warnings, infos };
}

function StatusBezel({ nodeCount, dependencyCount, findings = [] }) {
  const counts = countBySeverity(findings);
  const driftMode = counts.errors > 0 || counts.warnings > 0;

  return html`
    <header class="status-bezel" role="status">
      <div class="status-grid" aria-label=${copy("webui.metrics")}>
        <span class="status-cell"><strong>${nodeCount}</strong><span>${copy("webui.nodes")}</span></span>
        <span class="status-cell"><strong>${dependencyCount}</strong><span>${copy("webui.dependencies")}</span></span>
        <span class="status-cell"><strong>${counts.total}</strong><span>${copy("webui.findings-count")}</span></span>
      </div>
      <div class="status-grid" role="status">
        <span class=${clsx("status-cell", driftMode ? "drift" : "synced")}>
          <strong>${driftMode ? copy("webui.status-drift") : copy("webui.status-clean")}</strong>
          <span>
            ${counts.errors} ${copy("webui.findings-errors")} · ${counts.warnings} ${copy("webui.findings-warnings")} · ${counts.infos} ${copy("webui.findings-infos")}
          </span>
        </span>
      </div>
    </header>
  `;
}

export { StatusBezel };
