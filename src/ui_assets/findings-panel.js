/* Findings drawer: the single canonical surface for lint findings and
 * reconciliation notes. Reads lint findings from the /api/lint-derived prop.
 */
import { clsx, copy, html } from "./utils.js";

// ==========================================================================
// Findings rollup panel
// ==========================================================================

// ==========================================================================
// Changes drawer (surfaces active changes / findings)
// ==========================================================================

function ChangesDrawer({ open, onToggle, lint, onSelect }) {
  const findings = lint?.findings || [];
  return html`
    <div class="changes-drawer">
      <button class="drawer-handle" onClick=${onToggle}>
        <span class="label">Findings</span>
        <span class="count">${findings.length}</span>
        <span class="sub">reconciliation and integrity notes</span>
        <span class="chev">${open ? "▾" : "▴"}</span>
      </button>
      ${
        open
          ? findings.length === 0
            ? html`<div class="drawer-empty">${copy("empty-states.map-clean.body")}</div>`
            : html`<div class="drawer-body">
              ${findings.map(
                (f) => html`
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
              `,
              )}
            </div>`
          : null
      }
    </div>
  `;
}

export { ChangesDrawer };
