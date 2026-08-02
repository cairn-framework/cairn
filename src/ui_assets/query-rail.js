import { clsx, copy, html } from "./utils.js";

const KIND_FILTERS = ["all", "system", "container", "module", "actor"];
const STATE_FILTERS = ["all", "synced", "ghost", "orphaned", "drift"];

function FilterGroup({ label, filters, value, copyKey, onChange }) {
  return html`
    <fieldset class="query-filter-group">
      <legend>${label}</legend>
      <div class="query-segmented">
        ${filters.map(
          (filter) =>
            html`<button
              type="button"
              class=${clsx("query-segment", value === filter ? "active" : "")}
              aria-pressed=${value === filter}
              onClick=${() => onChange(filter)}
            >
              ${copy(`${copyKey}.${filter}`)}
            </button>`,
        )}
      </div>
    </fieldset>
  `;
}
/**
 * Data:
 *  - query: raw query string
 *  - parsed: parsed query tokens
 *  - visibleCount: number
 *  - kindFilter, stateFilter
 *
 * Events:
 *  - onQuery(value)
 *  - onQueryKey(event)
 *  - onKindFilter(value)
 *  - onStateFilter(value)
 *  - onClear()
 *  - view, onView(value): workspace view toggle (map | console)
 */

const VIEW_FILTERS = ["map", "console"];
function QueryRail({ query, parsed, visibleCount, kindFilter, stateFilter, view, onQuery, onQueryKey, onKindFilter, onStateFilter, onClear, onView }) {
  const hasFilter = Boolean(query.trim()) || kindFilter !== "all" || stateFilter !== "all" || parsed.kind !== "all" || parsed.state !== "all";
  const mapView = view !== "console";

  // Search, map filters, and the match count operate on the graph canvas
  // only; in console view they would steer a hidden map, so only the view
  // selector renders.
  return html`
    <section class="query-rail" aria-label=${copy("webui.query-rail")} role=${mapView ? "search" : undefined}>
      ${
        mapView
          ? html`
            <label class="query-search">
              <span class="query-search-affordance" aria-hidden="true">/</span>
              <input
                class="query-input"
                type="search"
                value=${query}
                aria-label=${copy("webui.query-placeholder")}
                placeholder=${copy("webui.query-placeholder")}
                onInput=${(event) => onQuery(event.currentTarget.value)}
                onKeyDown=${onQueryKey}
              />
            </label>
            <${FilterGroup}
              label=${copy("webui.kind-label")}
              filters=${KIND_FILTERS}
              value=${kindFilter}
              copyKey="webui.kind"
              onChange=${onKindFilter}
            />
            <${FilterGroup}
              label=${copy("webui.state-label")}
              filters=${STATE_FILTERS}
              value=${stateFilter}
              copyKey="webui.states"
              onChange=${onStateFilter}
            />`
          : null
      }
      <${FilterGroup}
        label=${copy("webui.view-label")}
        filters=${VIEW_FILTERS}
        value=${view}
        copyKey="webui.view"
        onChange=${onView}
      />
      ${
        mapView
          ? html`
            <div class="query-summary">
              <span class="query-matches"><strong>${visibleCount}</strong> ${copy("webui.query-matches")}</span>
              ${hasFilter ? html`<button class="query-action query-clear" type="button" onClick=${onClear}>${copy("webui.clear")}</button>` : null}
            </div>`
          : null
      }
    </section>`;
}

export { QueryRail };
