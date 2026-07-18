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
 */
function QueryRail({ query, parsed, visibleCount, kindFilter, stateFilter, onQuery, onQueryKey, onKindFilter, onStateFilter, onClear }) {
  const hasFilter = Boolean(query.trim()) || kindFilter !== "all" || stateFilter !== "all" || parsed.kind !== "all" || parsed.state !== "all";

  return html`
    <section class="query-rail" aria-label=${copy("webui.query-rail")} role="search">
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
      />
      <div class="query-summary">
        <span class="query-matches"><strong>${visibleCount}</strong> ${copy("webui.query-matches")}</span>
        ${hasFilter ? html`<button class="query-action query-clear" type="button" onClick=${onClear}>${copy("webui.clear")}</button>` : null}
      </div>
    </section>`;
}

export { QueryRail };
