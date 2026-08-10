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
    const stem = item?.stem || slugFromPath(item?.path);
    return {
      title: item?.title || stem || copy("webui.channel.backlog"),
      meta: item?.status || copy("webui.none"),
      body: stem,
    };
  }

  if (kind === "pending") {
    const raw = item?.subject_hash ? String(item.subject_hash) : "";
    const hex = raw.startsWith("sha256:") ? raw.slice(7) : raw;
    const short = raw ? ` · ${raw.startsWith("sha256:") ? "sha256:" : ""}${hex.slice(0, 8)}` : "";
    const hash = raw ? short : item?.subject_hash_error ? ` · ${copy("webui.channel.pending-hash-error")}` : "";
    const changed = item?.changed_since_review ? ` · ${copy("pending.changed")}` : "";
    const meta =
      copy("webui.channel.pending-meta")
        .replace("{age}", String(item?.age_days))
        .replace("{ratification}", item?.ratification || copy("webui.none")) +
      hash +
      changed;
    return {
      title: item?.id || copy("webui.channel.pending"),
      meta,
      metaTitle: raw ? `${meta} (${raw})` : meta,
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

function pendingList(labelKey, values) {
  if (!Array.isArray(values) || !values.length) {
    return null;
  }
  return html`
    <section class="pending-detail-section">
      <p class="plate-meta pending-detail-label">${copy(labelKey)}</p>
      <ul class="pending-detail-list">${values.map((value) => html`<li>${value}</li>`)}</ul>
    </section>`;
}

function PendingSummary({ item }) {
  return html`<p class="pending-detail-summary">${item?.ruling_summary || copy("webui.channel.pending-no-summary")}</p>`;
}

function PendingTier({ item }) {
  return item?.rubric?.tier ? html`<p class="pending-detail-tier">${copy("webui.channel.pending-tier")}: ${item.rubric.tier}</p>` : null;
}

function PendingDetail({ item }) {
  const evidence = item?.evidence;
  const receipts = Array.isArray(evidence?.receipts) ? evidence.receipts : [];
  return html`
    <div class="pending-detail" data-pending-detail="true">
      <p class="plate-meta pending-detail-label">${copy("webui.channel.pending-ruling")}</p>
      <${PendingSummary} item=${item} />
      ${
        item?.rubric
          ? html`
              <p class="plate-meta pending-detail-label">${copy("webui.channel.pending-briefing")}</p>
              <${PendingTier} item=${item} />
              ${pendingList("webui.channel.pending-unblocks", item.rubric.unblocks)}
              ${pendingList("webui.channel.pending-alignment", item.rubric.alignment)}
              ${pendingList("webui.channel.pending-options", item.rubric.options)}
            `
          : html`<p class="plate-meta">${copy("webui.channel.pending-no-briefing")}</p>`
      }
      ${
        evidence
          ? html`<section class="pending-detail-section pending-detail-evidence">
              <p class="plate-meta pending-detail-label">${copy("webui.channel.pending-evidence")}</p>
              ${
                receipts.length
                  ? html`<ul class="pending-detail-list">
                      ${receipts.map(
                        (receipt) => html`
                          <li>
                            <span>${receipt.stem}</span>
                            <span>${copy("webui.channel.pending-reviewer")}: ${receipt.reviewer || copy("pending.evidence-reviewer-unknown")}</span>
                            <span>${copy("webui.channel.pending-verdict")}: ${receipt.verdict || copy("pending.evidence-verdict-unknown")}</span>
                            <span>${receipt.subject_hash_matches === true ? copy("pending.evidence-match") : receipt.subject_hash_matches === false ? copy("pending.evidence-mismatch") : copy("pending.evidence-unverified")}</span>
                          </li>
                        `,
                      )}
                    </ul>`
                  : html`<p class="plate-meta">${copy("webui.channel.pending-no-evidence")}</p>`
              }
            </section>`
          : null
      }
      ${item?.changed_since_review ? html`<p class="pending-detail-changed">${copy("pending.changed")}</p>` : null}
      <p class="pending-detail-prompt">${copy("webui.channel.pending-next")} ${item?.ruling_prompt || ""}</p>
      <p class="pending-detail-reopen">${copy("webui.channel.pending-reopen")} <code>${item?.reopen_command || ""}</code></p>
    </div>`;
}

function ChannelItem({ item, kind, onFocus }) {
  const label = itemLabel(item, kind);
  const nodeId = item?.node || (Array.isArray(item?.nodes) ? item.nodes[0] : undefined);
  const showFocus = Boolean(nodeId);
  const pending = kind === "pending";
  const [expanded, setExpanded] = useState(false);
  const toggle = () => setExpanded((value) => !value);
  const fullText = [label.title, label.body, label.meta].filter(Boolean).join(" · ");

  return html`
    <article
      class=${clsx("channel-item", label.badge && `tone-${label.badge.tone}`, pending && "pending-item")}
      title=${fullText}
      onClick=${pending ? toggle : undefined}
    >
      <div class="channel-row">
        ${label.badge ? html`<span class=${clsx("channel-severity", label.badge.tone)}>${label.badge.text}</span>` : null}
        <span class=${clsx("channel-code", kind === "backlog" && "channel-title-prose")} title=${label.title}>${label.title}</span>
        <span class="plate-meta channel-body-text" title=${label.body}>${label.body}</span>
        ${label.meta ? html`<span class="plate-meta channel-inline-meta" title=${label.metaTitle || label.meta}>${label.meta}</span>` : null}
        ${
          showFocus
            ? html`<button
                class="query-action channel-focus"
                type="button"
                title=${copy("webui.focus-node")}
                  onClick=${(event) => {
                    event.stopPropagation();
                    onFocus(nodeId);
                  }}
              >
                ${copy("webui.focus-node")}
              </button>`
            : null
        }
        ${
          pending
            ? html`<button
                class="query-action pending-toggle"
                type="button"
                aria-expanded=${expanded}
                onClick=${(event) => {
                  event.stopPropagation();
                  toggle();
                }}
              >
                ${copy(expanded ? "webui.channel.pending-close" : "webui.channel.pending-open")}
              </button>`
            : null
        }
      </div>
      ${
        pending && !expanded
          ? html`<div class="pending-collapsed">
              <${PendingSummary} item=${item} />
              <${PendingTier} item=${item} />
            </div>`
          : null
      }
      ${pending && expanded ? html`<${PendingDetail} item=${item} />` : null}
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

// Groups flattened roadmap items into tier sections holding parent groups
// (`dec.todo-relationship-model` ruling 5: tiers order, parents only group).
// The group key is the effective parent (declared parent, else own stem) so
// a parent todo sits adjacent to its children, in first-seen wire order; a
// header renders only for groups carrying a declared parent edge.
function backlogSections(items) {
  const tiers = new Map();
  for (const item of Array.isArray(items) ? items : []) {
    const tier = Number.isFinite(item?.tier) ? item.tier : 0;
    if (!tiers.has(tier)) tiers.set(tier, []);
    tiers.get(tier).push(item);
  }
  return [...tiers.entries()]
    .sort((left, right) => left[0] - right[0])
    .map(([tier, rows]) => {
      const groups = new Map();
      for (const row of rows) {
        const key = row?.parent || row?.stem || "";
        if (!groups.has(key)) groups.set(key, { parent: "", members: [] });
        const group = groups.get(key);
        group.members.push(row);
        if (row?.parent) group.parent = row.parent;
      }
      return { tier, groups: [...groups.values()] };
    });
}

function BacklogSections({ items, onItem }) {
  return backlogSections(items).map(
    ({ tier, groups }) => html`
      <div class="channel-tier" data-tier=${tier}>
        <p class="channel-code channel-tier-header">${copy("webui.channel.tier-header").replace("{tier}", String(tier))}</p>
        ${groups.map(
          ({ parent, members }) => html`
            <div class=${clsx("channel-group", parent && "has-parent")}>
              ${parent ? html`<p class="plate-meta channel-inline-meta channel-group-header">${copy("webui.channel.group-header").replace("{parent}", parent)}</p>` : null}
              ${members.map((item) => html`<${ChannelItem} item=${item} kind="backlog" onFocus=${onItem} />`)}
            </div>
          `,
        )}
      </div>
    `,
  );
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
        ${!items.length ? html`<p class="channel-empty">${copy(`webui.empty.${active}`)}</p>` : active === "backlog" ? html`<${BacklogSections} items=${items} onItem=${onItem} />` : items.map((item) => html`<${ChannelItem} item=${item} kind=${active} onFocus=${onItem} />`)}
      </div>
    </section>
  `;
}

export { BacklogSections, ChannelBar, ChannelItem };
