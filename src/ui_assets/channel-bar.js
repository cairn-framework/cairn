import { clsx, copy, html, useState } from "./utils.js";

const CHANNELS = ["findings", "drift", "pending", "changes", "backlog"];

function slugFromPath(path) {
  if (!path || typeof path !== "string") {
    return "";
  }

  const clean = path.split("#")[0].trim();
  const slash = clean.split("/").filter(Boolean).pop() || "";
  const stem = slash.replace(/\.md$/i, "");
  const words = stem.split(".").filter(Boolean).join(".");
  return words;
}

function normalizeSeverity(item) {
  const value = String(item?.severity || "info").toLowerCase();
  return value === "warn" ? "warning" : value;
}

function severityRank(item) {
  const rank = { error: 0, warning: 1, info: 2 };
  return rank[normalizeSeverity(item)] ?? rank.info;
}

function severityLabel(item) {
  const value = normalizeSeverity(item);
  if (value === "error") {
    return copy("webui.severity.error");
  }
  if (value === "warning") {
    return copy("webui.severity.warning");
  }
  return copy("webui.severity.info");
}

function itemText(item) {
  if (!item) {
    return "";
  }
  return item.message || item.text || item.description || item.path || item.node || item.id || "";
}

function itemLabel(item, kind) {
  if (kind === "findings") {
    const code = item?.code || item?.id || "";
    return {
      title: code || copy("webui.finding"),
      badge: { text: severityLabel(item), tone: normalizeSeverity(item) },
      body: item?.message || item?.text || item?.description || item?.path || "",
    };
  }

  if (kind === "backlog") {
    return {
      title: slugFromPath(item?.path) || copy("webui.channel.backlog"),
      meta: `${item?.node || copy("webui.none")} · ${item?.status || copy("webui.none")}`,
      body: item?.path || "",
    };
  }

  if (kind === "pending") {
    return {
      title: item?.id || copy("webui.channel.pending"),
      meta: copy("webui.channel.pending-meta")
        .replace("{age}", String(item?.age_days))
        .replace("{ratification}", item?.ratification || copy("webui.none")),
      body: Array.isArray(item?.nodes) ? item.nodes.join(", ") : "",
    };
  }

  if (kind === "changes") {
    return {
      title: item?.id || copy("webui.channel.changes"),
      meta: item?.node ? `${item.node} · ${item?.status || copy("webui.none")}` : "",
      body: item?.path || "",
    };
  }

  if (kind === "drift") {
    return {
      title: item?.code || item?.id || copy("webui.finding"),
      badge: { text: severityLabel(item), tone: normalizeSeverity(item) },
      body: item?.message || item?.text || item?.description || item?.path || "",
    };
  }

  return {
    title: item?.title || item?.id || "",
    body: itemText(item),
  };
}

function ChannelItem({ item, kind, onFocus }) {
  const label = itemLabel(item, kind);
  const showFocus = Boolean(item?.node);
  const nodeId = item?.node;
  const fullText = [label.title, label.body, label.meta].filter(Boolean).join(" · ");

  return html`
    <article class=${clsx("channel-item", label.badge && `tone-${label.badge.tone}`)} title=${fullText}>
      <div class="channel-row">
        ${label.badge ? html`<span class=${clsx("channel-severity", label.badge.tone)}>${label.badge.text}</span>` : null}
        <span class="channel-code" title=${label.title}>${label.title}</span>
        <span class="plate-meta channel-body-text" title=${label.body}>${label.body}</span>
        ${label.meta ? html`<span class="plate-meta channel-inline-meta" title=${label.meta}>${label.meta}</span>` : null}
        ${
          showFocus
            ? html`<button
                class="query-action channel-focus"
                type="button"
                title=${copy("webui.focus-node")}
                onClick=${() => onFocus(nodeId)}
              >
                ${copy("webui.focus-node")}
              </button>`
            : null
        }
      </div>
    </article>`;
}

function sortChannelItems(items, kind) {
  const normalized = Array.isArray(items) ? items : [];
  if (kind !== "findings" && kind !== "drift") {
    return normalized;
  }

  return normalized
    .map((item, index) => ({ item, index }))
    .sort((left, right) => severityRank(left.item) - severityRank(right.item) || left.index - right.index)
    .map(({ item }) => item);
}

function ChannelBar({ active, findings, drift, pending, changes, backlog, onChannel, onItem, defaultCollapsed = false }) {
  const [collapsed, setCollapsed] = useState(Boolean(defaultCollapsed));
  const buckets = {
    findings: findings || [],
    drift: drift || [],
    pending: pending || [],
    changes: changes || [],
    backlog: backlog || [],
  };
  const items = sortChannelItems(buckets[active], active);
  const channelBodyId = "channel-body";
  const toggleLabel = copy(collapsed ? "webui.channel.expand" : "webui.channel.collapse");

  return html`
    <section class=${clsx("channel-bar", collapsed && "is-collapsed")} aria-label=${copy("webui.channel-label")} role="region">
      <div class="channel-tabs">
        <div class="channel-tab-scroll">
          ${CHANNELS.map(
            (name) =>
              html`
                <button type="button" class=${clsx("channel-tab", active === name && "active")} onClick=${() => onChannel(name)}>
                  ${copy(`webui.channel.${name}`)}
                  <span class="query-chip">${(buckets[name] || []).length}</span>
                </button>
              `,
          )}
        </div>
        <button
          class="channel-toggle"
          type="button"
          aria-expanded=${!collapsed}
          aria-controls=${channelBodyId}
          onClick=${() => setCollapsed((value) => !value)}
        >
          ${toggleLabel}
        </button>
      </div>
      <div id=${channelBodyId} class="channel-body" hidden=${collapsed}>
        ${!items.length ? html`<p class="channel-empty">${copy(`webui.empty.${active}`)}</p>` : items.map((item) => html`<${ChannelItem} item=${item} kind=${active} onFocus=${onItem} />`)}
      </div>
    </section>
  `;
}

export { ChannelBar, ChannelItem };
