import { clsx, copy, html } from "./utils.js";

function countBySeverity(findings) {
  const normalized = Array.isArray(findings) ? findings : [];
  let errors = 0;
  let warnings = 0;
  let infos = 0;

  for (const finding of normalized) {
    const severity = String(finding?.severity || "info").toLowerCase();
    const normalizedSeverity = severity === "warn" ? "warning" : severity;
    if (normalizedSeverity === "error") {
      errors += 1;
      continue;
    }
    if (normalizedSeverity === "warning") {
      warnings += 1;
      continue;
    }
    if (normalizedSeverity === "info") {
      infos += 1;
    }
  }

  return { errors, warnings, infos, total: errors + warnings + infos };
}

function SeverityChip({ tone, children }) {
  return html`<span class=${clsx("status-severity", tone)}>${children}</span>`;
}

function DriftIndicator({ counts }) {
  const drift = counts.errors + counts.warnings;
  const parts = [];
  if (counts.errors) parts.push(`${counts.errors} ${copy("webui.findings-errors")}`);
  if (counts.warnings) parts.push(`${counts.warnings} ${copy("webui.findings-warnings")}`);
  if (counts.infos) parts.push(`${counts.infos} ${copy("webui.findings-infos")}`);

  const tone = drift ? (counts.errors ? "error" : "warning") : "info";
  return html`
    <p class="status-annunciator status-annunciator-chip" role="status">
      <${SeverityChip} tone=${tone}>${copy(drift ? "webui.status-drift" : "webui.status-clean")}</${SeverityChip}>
      ${drift ? html`<span class="status-annunciator-summary">${parts.join(" · ")}</span>` : null}
    </p>
  `;
}

function StatusBezel({ nodeCount, dependencyCount, findings = [], blueprintPath }) {
  const counts = countBySeverity(findings);
  const projectName = copy("webui.project-name");
  const pathLabel = blueprintPath ? blueprintPath : copy("webui.blueprint-path-unknown");

  return html`
    <header class="status-bezel" role="status">
      <p class="status-kicker">
        <strong>${projectName}</strong>
        <span>${pathLabel}</span>
      </p>
      <div class="status-grid" aria-label=${copy("webui.metrics")}>
        <span class="status-cell"><strong>${nodeCount}</strong><span>${copy("webui.nodes")}</span></span>
        <span class="status-cell"><strong>${dependencyCount}</strong><span>${copy("webui.dependencies")}</span></span>
      </div>
      <${DriftIndicator} counts=${counts} />
    </header>
  `;
}

export { StatusBezel };
