/* Findings rollup panel and the changes drawer that surfaces it. Reads
 * lint findings only from the /api/lint-derived prop; never fetches
 * directly.
 */
import { clsx, copy, findingFamily, html, severityPill, useEffect, useMemo, useState } from "./utils.js";

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
          <span class="pill drift"><span class="dot"></span>${buckets.error} error</span>
          <span class="pill orphaned"><span class="dot"></span>${buckets.warning} warn</span>
          <span class="pill settled"><span class="dot"></span>${buckets.info} info</span>
        </div>
      </div>

      <div class="findings-controls">
        <div class="scope-toggle">
          <button class=${clsx(scope === "map" && "active")} onClick=${() => {
            setScope("map");
            setActiveCategory(null);
          }}>Whole map</button>
          <button class=${clsx(scope === "node" && !nodeDisabled && "active")} onClick=${() => {
            setScope("node");
            setActiveCategory(null);
          }} disabled=${nodeDisabled}>Selected node</button>
        </div>
        ${
          categories.length > 1
            ? html`<div class="category-chips">
              <button class=${clsx("pill", !activeCategory && "synced")} onClick=${() => setActiveCategory(null)}>All</button>
              ${categories.map(
                (c) => html`
                <button class=${clsx("pill", activeCategory === c && "synced")} key=${c} onClick=${() => setActiveCategory(activeCategory === c ? null : c)}>${c}</button>
              `,
              )}
            </div>`
            : null
        }
      </div>

      <div class="findings-list">
        ${
          findings.length === 0
            ? html`<div class="row-empty">${(scope !== "map" || activeCategory) && scopeFiltered.length > 0 ? copy("empty-states.no-filter-matches.body") : copy("empty-states.map-clean.body")}</div>`
            : findings.map(
                (f) => html`
              <button class=${clsx("recent-row", `sev-${f.severity}`)} key=${f.code + (f.node || "") + (f.path || "")}
                onClick=${() => f.node && onSelect(f.node)}>
                <span class="r-id">${f.code}</span>
                <span class="recent-title">${f.message}</span>
                <span class=${clsx("pill", severityPill(f.severity))}>
                  <span class="dot"></span>${f.severity}
                </span>
              </button>
            `,
              )
        }
      </div>
    </section>
  `;
}

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

export { FindingsPanel, ChangesDrawer };
