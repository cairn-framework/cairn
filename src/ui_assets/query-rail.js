import { clsx, copy, html } from "./utils.js";

const KIND_FILTERS = ["all", "system", "container", "module", "actor"];
const STATE_FILTERS = ["all", "synced", "ghost", "orphaned", "drift"];

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
 *  - onBringIntoView()
 *  - onClear()
 */
function QueryRail({ query, parsed, visibleCount, kindFilter, stateFilter, onQuery, onQueryKey, onKindFilter, onStateFilter, onBringIntoView, onClear }) {
  return html`
    <section class="query-rail" aria-label=${copy("webui.query-rail")} role="search">
      <span class="query-chip" aria-hidden="true">/</span>
      <input
        class="query-input"
        type="search"
        value=${query}
        placeholder=${copy("webui.query-placeholder")}
        onInput=${(event) => onQuery(event.currentTarget.value)}
        onKeyDown=${onQueryKey}
      />
      <span class="query-chip">${copy("webui.query-hint")}</span>
      ${KIND_FILTERS.map(
        (kind) =>
          html`<button
          type="button"
          class=${clsx("query-chip", kindFilter === kind ? "active" : "")}
          onClick=${() => onKindFilter(kind)}
        >
          ${copy(`webui.kind.${kind}`)}
        </button>`,
      )}
      ${STATE_FILTERS.map(
        (state) =>
          html`<button
          type="button"
          class=${clsx("query-chip", stateFilter === state ? "active" : "")}
          onClick=${() => onStateFilter(state)}
        >
          ${state === "all" ? copy("webui.states.all") : copy(`webui.states.${state}`)}
        </button>`,
      )}
      <span class="query-chip">${copy("webui.query-matches")}: ${visibleCount}</span>
      ${parsed.state !== "all" || parsed.kind !== "all" ? html`<span class="query-chip">${copy("webui.filters-active")} ${parsed.state !== "all" ? copy(`webui.states.${parsed.state}`) : copy(`webui.kind.${parsed.kind}`)}</span>` : null}
      <button class="query-action" type="button" onClick=${onBringIntoView}>${copy("webui.bring-into-view")}</button>
      <button class="query-action" type="button" onClick=${onClear}>${copy("webui.clear")}</button>
    </section>`;
}

export { QueryRail };
