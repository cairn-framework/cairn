/* Module inspector: the right-hand detail panel for a selected node
 * (blueprint card, prose-nudge banner, artefact sections) plus the
 * map-overview empty state shown when nothing is selected.
 */
import { balanceFromCount, CopyButton, clsx, copy, copyFinding, displayState, Fragment, fillPercent, highlightBlueprint, html, pickNudgeFinding, severityPill, substituteCopy, truncate, useMemo, useState } from "./utils.js";

// ==========================================================================
// Inspector building blocks
// ==========================================================================

function Section({ label, count, defaultOpen = false, children }) {
  const [open, setOpen] = useState(defaultOpen);
  return html`
    <div class=${clsx("ins-section", open && "open")}>
      <button class="ins-section-head" onClick=${() => setOpen((o) => !o)}
        aria-expanded=${open}>
        <span class="chev">${open ? "▾" : "▸"}</span>
        <span class="ins-section-label">${label}</span>
        ${count != null ? html`<span class="ins-section-count">${count}</span>` : null}
      </button>
      ${open ? html`<div class="ins-section-body">${children}</div>` : null}
    </div>
  `;
}

function reconBadge(state) {
  const label = displayState(state || "unknown");
  return html`<span class=${clsx("recon-badge", label)}>${label}</span>`;
}

function renderPath(pathText, state) {
  return html`
    <div class="path-row">
      <span class="path-text">${pathText}</span>
      ${reconBadge(state)}
    </div>
  `;
}

function BlueprintCard({ node, onViewSource }) {
  const snippet = buildBlueprintSnippet(node);
  const state = node.state || "synced";
  return html`
    <div class="blueprint-card">
      <div class="blueprint-head">
        <span class="caps">Blueprint</span>
        <button class="view-src" onClick=${onViewSource}>View source</button>
        ${reconBadge(state)}
      </div>
      <pre class="blueprint-code" dangerouslySetInnerHTML=${{ __html: snippet }}></pre>
    </div>
  `;
}

function buildBlueprintSnippet(node) {
  const kindKeyword = node.kind === "system" ? "System" : node.kind === "container" ? "Container" : node.kind === "module" ? "Module" : "Actor";
  const base = `${kindKeyword} ${node.name || ""} "${node.description || ""}" id "${node.id}"`;
  const lines = [`${base} {`];
  for (const p of node.paths || []) lines.push(`  path "${p}"`);
  for (const c of node.contracts || []) lines.push(`  contract "${c}"`);
  lines.push("}");
  return highlightBlueprint(lines.join("\n"));
}

function ArtefactCard({ artefact }) {
  const status = artefact.status ?? artefact.frontmatter?.status ?? artefact.type;
  const kindClass = artefact.type === "decisions" ? "decision" : artefact.type;
  return html`
    <div class=${clsx("artefact", kindClass, status)}>
      <div class="artefact-head">
        <span class="artefact-id">${artefact.type}</span>
        <span class=${clsx("artefact-status", status)}>${status}</span>
      </div>
      <div class="artefact-title">${artefact.title || artefact.path}</div>
      <div class="artefact-meta">${artefact.path}</div>
      ${artefact.body ? html`<div class="artefact-body">${truncate(artefact.body, 480)}</div>` : null}
    </div>
  `;
}

function BeadCard({ bead }) {
  return html`
    <div class=${clsx("artefact", "bead", bead.status)}>
      <div class="artefact-head">
        <span class="artefact-id">${bead.id}</span>
        <span class=${clsx("artefact-status", bead.status)}>${bead.status}</span>
      </div>
      <div class="artefact-title">${bead.title || bead.id}</div>
      <div class="artefact-meta">${bead.issue_type} · P${bead.priority}</div>
    </div>
  `;
}

function DependencyRow({ entry, onSelect }) {
  return html`
    <button class="dep-row" onClick=${() => onSelect(entry.id)}>
      <span class="dep-name">${entry.name || entry.id}</span>
      ${reconBadge(entry.state || "synced")}
    </button>
  `;
}

function ProseNudgeBanner({ lint, nodeId }) {
  const nudge = useMemo(() => {
    if (!lint || !lint.findings) return null;
    const f = pickNudgeFinding(lint.findings, nodeId);
    if (!f) return null;
    const entry = copyFinding(f.code);
    const vars = { node: f.node || "", path: f.path || "", target: f.target || "" };
    return {
      severity: f.severity,
      heading: entry?.heading || f.code,
      body: substituteCopy(entry?.body || f.message, vars),
      cta: entry?.cta || null,
    };
  }, [lint, nodeId]);

  if (!nudge) return null;

  return html`
    <div class=${clsx("prose-nudge", nudge.severity)}>
      <div class="prose-nudge-heading">
        <span class=${clsx("pill", severityPill(nudge.severity))}><span class="dot"></span>${nudge.severity}</span>
        <strong>${nudge.heading}</strong>
      </div>
      <p class="prose-nudge-body">${nudge.body}</p>
      ${
        nudge.cta
          ? html`<div class="prose-nudge-cta-row">
            <code class="prose-nudge-cta">${nudge.cta}</code>
            <${CopyButton} text=${nudge.cta} />
          </div>`
          : null
      }
    </div>
  `;
}

function ModuleInspector({ node, detail, lint, onSelect, onSelectDecision, onViewBlueprint, onClose }) {
  const { contracts, decisions, todos, beads, research, sources, depends, dependents, symbols } = detail;

  const provCount = (sources?.length || 0) + (research?.length || 0);
  const authCount = (contracts?.length || 0) + (decisions?.length || 0);
  const prov = balanceFromCount(provCount);
  const auth = balanceFromCount(authCount);
  const artefactCount = (contracts?.length || 0) + (decisions?.length || 0) + (todos?.length || 0) + (beads?.length || 0) + (research?.length || 0) + (sources?.length || 0) + (depends?.length || 0) + (dependents?.length || 0) + (symbols?.length || 0);

  const sortedDecisions = (decisions || []).slice().sort((a, b) => {
    const rank = (s) => (s === "proposed" ? 0 : s === "accepted" ? 1 : 2);
    const sa = a.status ?? a.frontmatter?.status ?? "accepted";
    const sb = b.status ?? b.frontmatter?.status ?? "accepted";
    return rank(sa) - rank(sb);
  });

  const containerId = node.parent || "";
  const eyebrowLabel = containerId ? `${node.kind} · ${containerId}` : node.kind;

  const pathEntries = (node.paths || []).map((p) => ({
    path: p,
    state: node.state || "synced",
  }));

  return html`
    <section class="inspector">
      <div class="ins-header">
        <div class="ins-eyebrow">${eyebrowLabel}</div>
        <button class="ins-close" onClick=${onClose} aria-label="Close inspector">×</button>
      </div>
      <h2 class="ins-title">${node.name || node.id}</h2>
      <div class="ins-slug">${node.id}</div>
      ${node.description ? html`<p class="ins-desc">${node.description}</p>` : null}

      <div class="pill-row">
        <span class=${clsx("pill", displayState(node.state))}>
          <span class="dot"></span>${displayState(node.state)}
        </span>
        ${(node.tags || []).map((t) => html`<span class="pill" key=${t}>${t}</span>`)}
      </div>

      <${BlueprintCard} node=${node} onViewSource=${onViewBlueprint}/>

      <${ProseNudgeBanner} lint=${lint} nodeId=${node.id}/>

      <div class="paths-block">
        <div class="paths-head">
          <span class="caps">Paths</span>
          <span class="ins-section-count">${pathEntries.length}</span>
        </div>
        <div class="paths-list">
          ${pathEntries.length === 0 ? html`<div class="row-empty">${copy("empty-states.node-no-paths.body")}</div>` : pathEntries.map((p) => renderPath(p.path, p.state))}
        </div>
      </div>

      ${
        detail.loading
          ? html`<div class="row-empty">${copy("empty-states.node-artefacts-loading.body")}</div>`
          : detail.failed
            ? html`<div class="row-empty">${copy("empty-states.node-artefacts-failed.body")}</div>`
            : html`
      <div class="chain-balance">
        <div class="balance-grid">
          <div class="balance-side prov">
            <div class="balance-kicker">Provenance</div>
            <div class="balance-value">${prov}</div>
          </div>
          <div class="balance-hinge"></div>
          <div class="balance-side auth">
            <div class="balance-kicker">Authority</div>
            <div class="balance-value">${auth}</div>
          </div>
        </div>
        <div class="balance-tracks">
          <div class="balance-track prov">
            <div class="fill" style=${`width:${fillPercent(prov)}`}></div>
          </div>
          <div style="width:12px;height:12px"></div>
          <div class="balance-track auth">
            <div class="fill" style=${`width:${fillPercent(auth)}`}></div>
          </div>
        </div>
      </div>

      <div class="stat-row">
        <div class="stat-cell">
          <div class="stat-n">${decisions?.length || 0}</div>
          <div class="caps">decisions</div>
        </div>
        <div class="stat-cell">
          <div class="stat-n">${contracts?.length || 0}</div>
          <div class="caps">contracts</div>
        </div>
        <div class="stat-cell">
          <div class="stat-n">${todos?.length || 0}</div>
          <div class="caps">todos</div>
        </div>
        <div class="stat-cell">
          <div class="stat-n">${research?.length || 0}</div>
          <div class="caps">research</div>
        </div>
      </div>

      ${
        artefactCount === 0
          ? html`<div class="row-empty">${copy("empty-states.node-no-artefacts.body")}</div>`
          : html`<${Fragment}>
            <${Section} key=${`${node.id}:contracts`} label="Contracts" count=${contracts?.length || 0}>
              ${(contracts || []).length === 0 ? html`<div class="row-empty">${copy("empty-states.node-no-contracts.body")}</div>` : (contracts || []).map((c) => html`<${ArtefactCard} key=${c.path} artefact=${c}/>`)}
            <//>
            <${Section} key=${`${node.id}:decisions`} label="Decisions" count=${decisions?.length || 0} defaultOpen=${sortedDecisions.length > 0}>
              ${
                sortedDecisions.length === 0
                  ? html`<div class="row-empty">${copy("empty-states.node-no-decisions.body")}</div>`
                  : sortedDecisions.map(
                      (d) => html`
                    <button class=${clsx("artefact", "decision", d.status ?? d.frontmatter?.status ?? "accepted")}
                      key=${d.path} onClick=${() => onSelectDecision(d)}>
                      <div class="artefact-head">
                        <span class="artefact-id">decision</span>
                        <span class=${clsx("artefact-status", d.status ?? d.frontmatter?.status ?? "accepted")}>
                          ${d.status ?? d.frontmatter?.status ?? "accepted"}
                        </span>
                      </div>
                      <div class="artefact-title">${d.title || d.path}</div>
                      <div class="artefact-meta">${d.path}</div>
                    </button>
                  `,
                    )
              }
            <//>
            <${Section} key=${`${node.id}:todos`} label="Todos" count=${todos?.length || 0}>
              ${(todos || []).length === 0 ? html`<div class="row-empty">${copy("empty-states.node-no-todos.body")}</div>` : (todos || []).map((t) => html`<${ArtefactCard} key=${t.path} artefact=${t}/>`)}
            <//>
            <${Section} key=${`${node.id}:beads`} label="Beads" count=${beads?.length || 0}>
              ${(beads || []).length === 0 ? html`<div class="row-empty">${copy("empty-states.node-no-beads.body")}</div>` : (beads || []).map((b) => html`<${BeadCard} key=${b.id} bead=${b}/>`)}
            <//>
            <${Section} key=${`${node.id}:research`} label="Research" count=${research?.length || 0}>
              ${(research || []).length === 0 ? html`<div class="row-empty">${copy("empty-states.node-no-research.body")}</div>` : (research || []).map((r) => html`<${ArtefactCard} key=${r.path} artefact=${r}/>`)}
            <//>
            <${Section} key=${`${node.id}:sources`} label="Sources" count=${sources?.length || 0}>
              ${(sources || []).length === 0 ? html`<div class="row-empty">${copy("empty-states.node-no-sources.body")}</div>` : (sources || []).map((s) => html`<${ArtefactCard} key=${s.path} artefact=${s}/>`)}
            <//>
            <${Section} key=${`${node.id}:depends`} label="Depends on" count=${depends?.length || 0}>
              ${(depends || []).length === 0 ? html`<div class="row-empty">${copy("empty-states.node-no-outbound.body")}</div>` : (depends || []).map((d) => html`<${DependencyRow} key=${d.id} entry=${d} onSelect=${onSelect}/>`)}
            <//>
            <${Section} key=${`${node.id}:dependents`} label="Dependents" count=${dependents?.length || 0}>
              ${(dependents || []).length === 0 ? html`<div class="row-empty">${copy("empty-states.node-no-inbound.body")}</div>` : (dependents || []).map((d) => html`<${DependencyRow} key=${d.id} entry=${d} onSelect=${onSelect}/>`)}
            <//>
            <${Section} key=${`${node.id}:symbols`} label="Symbols" count=${(symbols || []).length}>
              ${(symbols || []).length === 0 ? html`<div class="row-empty">${copy("empty-states.node-no-symbols.body")}</div>` : (symbols || []).map((s) => html`<div class="artefact-meta" key=${s.name + s.file}>${s.name} &middot; ${s.kind} &middot; ${s.file}:${s.line}</div>`)}
            <//>
          <//>`
      }
      `
      }
    </section>
  `;
}

function EmptyInspector({ graph, lint, onShowFindings, onOpenCmd }) {
  const nodes = graph ? graph.nodes : [];
  const modules = nodes.filter((n) => n.kind === "module");
  const total = modules.length;
  const ghostCount = modules.filter((n) => n.state === "ghost").length;
  const orphanedCount = modules.filter((n) => n.state === "orphaned").length;

  return html`
    <section class="inspector empty-inspector">
      <div class="ins-eyebrow">Map</div>
      <h2 class="ins-title">${graph?.nodes[0] ? graph.nodes[0].name : "Cairn"}</h2>
      <div class="ins-slug">
        ${graph ? `${graph.nodes.length} nodes · ${graph.edges.length} edges · ${lint?.findings?.length ?? 0} findings` : ""}
      </div>
      ${graph?.nodes[0]?.description ? html`<p class="ins-desc">${graph.nodes[0].description}</p>` : null}

      <button class="overview-action" onClick=${onOpenCmd}>
        <span>${copy("webui.overview-action")}</span>
        <span class="overview-action-keys"><kbd>⌘</kbd><kbd>K</kbd></span>
      </button>

      <div class="stat-grid">
        <div class="stat-cell">
          <div class="stat-n">${total}</div>
          <div class="caps">modules</div>
        </div>
        <div class="stat-cell">
          <div class=${clsx("stat-n", ghostCount > 0 && "ghost")}>${ghostCount}</div>
          <div class="caps">ghost</div>
        </div>
        <div class="stat-cell">
          <div class="stat-n">${orphanedCount}</div>
          <div class="caps">orphaned</div>
        </div>
      </div>

      ${
        lint?.findings && lint.findings.length > 0
          ? html`<button class="findings-link" onClick=${onShowFindings}>
            <span class="caps">${copy("webui.findings-label")}</span>
            <span>${copy("webui.findings-open")}</span>
            <span class="findings-link-count">${lint.findings.length}</span>
          </button>`
          : html`<div class="row-empty">${copy("empty-states.map-clean.body")}</div>`
      }

    </section>
  `;
}

export { EmptyInspector, ModuleInspector };
