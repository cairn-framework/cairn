import { clsx, copy, html } from "./utils.js";

const CHANNELS = ["findings", "drift", "changes", "backlog"];

function itemText(item) {
  if (!item) {
    return "";
  }

  return item.message || item.text || item.description || item.title || item.path || item.node || item.id || "";
}

function findingBadge(item) {
  const severity = String(item.severity || "info");
  if (item.code) {
    return `${item.code}`;
  }
  return severity;
}

function ChannelItem({ item, onFocus }) {
  const nodeId = item.node || item.slug;
  return html`
    <article class="channel-item">
      <p class="channel-title">${item.title || item.id || copy("webui.finding")}</p>
      <p class="plate-meta">${findingBadge(item)} ${item.severity ? `· ${item.severity}` : ""}</p>
      <p>${itemText(item)}</p>
      ${
        nodeId
          ? html`<button class="query-action" type="button" onClick=${() => onFocus(nodeId)}>
            ${copy("webui.focus-node")}
          </button>`
          : null
      }
    </article>`;
}

function ChannelBar({ active, findings, drift, changes, backlog, onChannel, onItem }) {
  const buckets = {
    findings,
    drift,
    changes: changes || [],
    backlog: backlog || [],
  };
  const items = buckets[active] || [];

  return html`
      <section class="channel-bar" aria-label=${copy("webui.channel-label")} role="region">
      <div class="channel-tabs">
        ${CHANNELS.map(
          (name) =>
            html`
            <button
              type="button"
              class=${clsx("channel-tab", active === name ? "active" : "")}
              onClick=${() => onChannel(name)}
            >
              ${copy(`webui.channel.${name}`)}
              <span class="query-chip">${(buckets[name] || []).length}</span>
            </button>`,
        )}
      </div>
      <div class="channel-body">
        ${!items.length ? html`<p class="channel-empty">${copy(`webui.empty.${active}`)}</p>` : items.map((item) => html`<${ChannelItem} item=${item} onFocus=${onItem} />`)}
      </div>
    </section>`;
}

export { ChannelBar, ChannelItem };
