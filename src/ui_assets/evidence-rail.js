import { clsx, copy, highlightBlueprint, html, useEffect, useMemo, useState } from "./utils.js";

function NodeDepthPlate({ node, inRows, outRows, onEdgeSelect }) {
  const paths = Array.isArray(node?.paths) ? node.paths : [];
  const symbols = Array.isArray(node?.symbols) ? node.symbols : [];
  const contracts = Array.isArray(node?.contracts) ? node.contracts : [];
  const hasDetails = paths.length > 0 || symbols.length > 0;

  return html`
    <article class="node-depth-plate">
      <h3 class="plate-title">${copy("webui.depth")}</h3>
      <dl class="plate-kv">
        <dt>${copy("webui.node-id")}</dt>
        <dd class="node-id">${node?.id || copy("webui.none")}</dd>
        <dt>${copy("webui.state-label")}</dt>
        <dd>${copy(`webui.states.${node?.state || "synced"}`)}</dd>
        <dt>${copy("webui.kind-label")}</dt>
        <dd>${node?.kind || copy("webui.kind.module")}</dd>
        ${
          paths.length
            ? html`
              <dt>${copy("webui.path-count")}</dt>
              <dd>${String(paths.length)}</dd>
              <dt>${copy("webui.paths")}</dt>
              <dd>${paths.join(", ")}</dd>
            `
            : null
        }
        ${symbols.length ? html`<dt>${copy("webui.symbols")}</dt><dd>${String(symbols.length)}</dd>` : null}
      </dl>
      ${hasDetails ? null : html`<p class="plate-meta">${copy("webui.node-details-empty")}</p>`}
      <section class="edge-group">
        <h4>${copy("webui.in")}</h4>
        ${
          inRows.length
            ? inRows.map(
                (row) => html`
                  <button class="edge-row" type="button" onClick=${() => onEdgeSelect(row.id)}>
                    <span class="edge-target">${row.id}${row.name ? ` · ${row.name}` : ""}</span>
                  </button>`,
              )
            : html`<p class="plate-meta">${copy("empty-states.node-no-inbound.body")}</p>`
        }
      </section>
      <section class="edge-group">
        <h4>${copy("webui.out")}</h4>
        ${
          outRows.length
            ? outRows.map(
                (row) => html`
                  <button class="edge-row" type="button" onClick=${() => onEdgeSelect(row.id)}>
                    <span class="edge-target">${row.id}${row.name ? ` · ${row.name}` : ""}</span>
                  </button>`,
              )
            : html`<p class="plate-meta">${copy("empty-states.node-no-outbound.body")}</p>`
        }
      </section>
      ${
        contracts.length
          ? html`
            <section class="edge-group">
              <h4>${copy("webui.contracts")}</h4>
              ${contracts.map((item) => html`<p class="plate-meta">${item.id || item.path || item.title || copy("webui.artefact")}</p>`)}
            </section>
          `
          : null
      }
    </article>
  `;
}

function EvidenceItemPreview({ artefact, onBack }) {
  if (!artefact) {
    return null;
  }

  const title = artefact.title || artefact.id || artefact.path || copy("webui.artefact");
  const status = artefact.frontmatter?.status || artefact.status || "";
  const date = artefact.frontmatter?.date || artefact.date || "";
  const body = String(artefact.body || copy("webui.artefact-no-body"));
  const snippet = body.length > 800 ? `${body.slice(0, 800)}…` : body;

  return html`
    <section class="lineage-item-preview">
      <button class="query-action" type="button" onClick=${onBack}>
        ${copy("webui.overview")}
      </button>
      <p class="lineage-title">${title}</p>
      <p class="lineage-meta">${[status, date].filter(Boolean).join(" · ")}</p>
      <pre><code>${snippet}</code></pre>
      ${artefact.path ? html`<p class="plate-meta">${artefact.path}</p>` : null}
    </section>
  `;
}

const lineKindLabels = {
  "lineage-rationale": {
    short: "R",
    label: "lineage-rationale",
  },
  "lineage-decisions": {
    short: "D",
    label: "lineage-decisions",
  },
  authority: {
    short: "A",
    label: "authority",
  },
};

function LineageRow({ item, kindLabel, onOpen }) {
  const title = item?.title || item?.id || item?.path || copy("webui.artefact");
  const kind = lineKindLabels[kindLabel] || lineKindLabels["lineage-rationale"];

  return html`
    <button class="lineage-row" type="button" onClick=${() => onOpen(item)}>
      <span class="lineage-row-kind" title=${copy(`webui.${kind.label}`)}>
        ${kind.short}
      </span>
      <span class="lineage-row-title" title=${title}>${title}</span>
    </button>
  `;
}

function LineagePlate({ artefacts = {}, onOpen, selectedItem }) {
  const evidence = Array.isArray(artefacts.evidence) ? artefacts.evidence : [];
  const decisions = Array.isArray(artefacts.decisions) ? artefacts.decisions : [];
  const authority = Array.isArray(artefacts.sources) ? artefacts.sources : [];
  const selected = selectedItem && typeof selectedItem === "object" ? selectedItem : null;

  return html`
    <article class=${clsx("lineage-plate", selected ? "lineage-preview-mode" : "")}>
      <h3 class="plate-title">${copy("webui.lineage")}</h3>
      <section class="lineage-stage">
        <p class="lineage-kind">${copy("webui.lineage-rationale")}</p>
        ${evidence.length ? evidence.map((item) => html`<${LineageRow} item=${item} kindLabel="lineage-rationale" onOpen=${onOpen} />`) : html`<p class="plate-meta">${copy("webui.lineage-empty")}</p>`}
      </section>
      <section class="lineage-stage">
        <p class="lineage-kind">${copy("webui.lineage-decisions")}</p>
        ${decisions.length ? decisions.map((item) => html`<${LineageRow} item=${item} kindLabel="lineage-decisions" onOpen=${onOpen} />`) : html`<p class="plate-meta">${copy("webui.decision-empty")}</p>`}
      </section>
      <section class="lineage-stage">
        <p class="lineage-kind">${copy("webui.lineage-authority")}</p>
        ${authority.length ? authority.map((item) => html`<${LineageRow} item=${item} kindLabel="authority" onOpen=${onOpen} />`) : html`<p class="plate-meta">${copy("webui.authority-empty")}</p>`}
      </section>
      ${selected ? html`<${EvidenceItemPreview} artefact=${selected} onBack=${() => onOpen(null)} />` : null}
    </article>
  `;
}

function EvidenceModeTabs({ mode, onMode }) {
  const modes = ["depth", "lineage", "blueprint"];
  return html`
    <div class="rail-tabbar">
      ${modes.map(
        (name) => html`
          <button
            type="button"
            class=${clsx("rail-tab", mode === name ? "active" : "")}
            onClick=${() => onMode(name)}
          >
            ${copy(`webui.evidence-modes.${name}`)}
          </button>`,
      )}
    </div>
  `;
}

function EvidenceRail({ mode, onMode, selection, inRows, outRows, onNeighbourSelect, artefacts, blueprint, blueprintPath, onLineageOpen, selectedLineageItem }) {
  const emptySelection = !selection;
  const depthNode = emptySelection
    ? null
    : {
        ...selection,
        contracts: artefacts?.contracts || selection.contracts || [],
        symbols: artefacts?.symbols || selection.symbols || [],
      };
  const closeLineageItem = () => onLineageOpen(null);

  return html`
    <aside class="evidence-rail" aria-label=${copy("webui.evidence")} role="complementary">
      <header class="rail-head">
        <p class="rail-title">${copy("webui.evidence")}</p>
        <p class="rail-subtitle">
          ${emptySelection ? copy("webui.select-a-node") : `${copy("webui.depth")} · ${selection.id || copy("webui.none")}`}
        </p>
        <${EvidenceModeTabs} mode=${mode} onMode=${onMode} />
      </header>
      <div class="rail-body">
        ${
          emptySelection
            ? html`<p class="plate-meta">${copy("webui.overview-action")}</p>`
            : mode === "depth"
              ? html`
                <${NodeDepthPlate}
                  node=${depthNode}
                  inRows=${inRows}
                  outRows=${outRows}
                  onEdgeSelect=${onNeighbourSelect}
                />`
              : mode === "blueprint"
                ? html`
                  <${BlueprintPlate}
                    sourcePath=${blueprintPath}
                    sourceText=${blueprint}
                    selectionId=${selection.id}
                  />`
                : html`
                  <${LineagePlate}
                    artefacts=${artefacts}
                    onOpen=${onLineageOpen}
                    selectedItem=${selectedLineageItem}
                    onBack=${closeLineageItem}
                  />`
        }
      </div>
    </aside>
  `;
}

function escapePattern(value) {
  return String(value).replace(/[.*+?^${}()|[\]\\]/g, "\\$&");
}

function blueprintBlock(sourceText, selectionId) {
  const source = String(sourceText || "");
  if (!source) {
    return { text: source, scoped: false };
  }
  const focus = String(selectionId || "");
  if (!focus) {
    return { text: source, scoped: false };
  }

  const lines = source.split("\n");
  const declarationFor = (id) => new RegExp(`^\\s*(?:System|Container|Module|Actor|Decision|Findings|Change)\\b[^\\n]*\\bid\\s+"${escapePattern(id)}"`);
  const fullPattern = declarationFor(focus);
  let start = lines.findIndex((line) => fullPattern.test(line));
  const tail = focus.split(".").pop();
  if (start < 0 && tail && tail !== focus) {
    const tailPattern = declarationFor(tail);
    start = lines.findIndex((line) => tailPattern.test(line));
  }
  if (start < 0) {
    return { text: source, scoped: false };
  }

  const startLine = lines[start] || "";
  const singleLineDecl = (startLine.match(/{/g) || []).length > 0 && (startLine.match(/}/g) || []).length > 0;
  let depth = 0;
  let end = start;
  for (; end < lines.length; end += 1) {
    const line = lines[end];
    depth += (line.match(/{/g) || []).length;
    depth -= (line.match(/}/g) || []).length;
    if ((end > start && depth <= 0) || (end === start && singleLineDecl && depth <= 0)) {
      break;
    }
  }

  return {
    text: lines.slice(start, Math.min(end + 1, lines.length)).join("\n"),
    scoped: true,
  };
}

function BlueprintPlate({ sourcePath, sourceText, selectionId }) {
  const [expanded, setExpanded] = useState(false);
  const scoped = useMemo(() => blueprintBlock(sourceText, selectionId), [sourceText, selectionId]);
  const visibleSource = expanded ? String(sourceText || "") : scoped.text;
  const preId = useMemo(() => {
    const safe = String(selectionId || "all").replace(/[^A-Za-z0-9_-]/g, "_");
    return `blueprint-${safe}`;
  }, [selectionId]);
  const headerTitle = scoped.scoped && selectionId ? `${copy("webui.blueprint-title")} · ${selectionId}` : copy("webui.blueprint-title");
  const scopeLabel = scoped.scoped ? `${copy("webui.blueprint-scoped-to")} ${selectionId}` : copy("webui.blueprint-scope-unavailable");

  useEffect(() => {
    setExpanded(false);
  }, [selectionId, sourceText]);

  useEffect(() => {
    if (!expanded) {
      return undefined;
    }
    const pre = document.getElementById(preId);
    if (!pre) {
      return undefined;
    }
    const hit = pre.querySelector(".blueprint-hit");
    if (!hit) {
      return undefined;
    }
    const frame = requestAnimationFrame(() => {
      hit.scrollIntoView({ block: "center" });
    });
    return () => cancelAnimationFrame(frame);
  }, [expanded, preId, selectionId, sourceText]);

  return html`
    <article class="blueprint-plate">
      <h3 class="plate-title">${headerTitle}</h3>
      <p class="plate-meta">${sourcePath || copy("webui.blueprint-path-unknown")}</p>
      <p class="plate-meta">${scopeLabel}</p>
      <button class="query-action" type="button" onClick=${() => setExpanded((value) => !value)}>
        ${copy(expanded ? "webui.blueprint-collapse" : "webui.blueprint-expand")}
      </button>
      <pre id=${preId}><code dangerouslySetInnerHTML=${{ __html: highlightBlueprint(visibleSource, selectionId) }}></code></pre>
    </article>
  `;
}

function StateLegend() {
  return html`
    <section class="state-legend" aria-label=${copy("webui.legend-label")} role="note">
      <p class="legend-title">${copy("webui.legend-label")}</p>
      <div class="legend-grid">
        <p class="legend-item synced"><span class="legend-key"></span><span class="legend-label"><strong>${copy("webui.states.synced")}</strong> ${copy("webui.state-description.synced")}</span></p>
        <p class="legend-item ghost"><span class="legend-key"></span><span class="legend-label"><strong>${copy("webui.states.ghost")}</strong> ${copy("webui.state-description.ghost")}</span></p>
        <p class="legend-item orphaned"><span class="legend-key"></span><span class="legend-label"><strong>${copy("webui.states.orphaned")}</strong> ${copy("webui.state-description.orphaned")}</span></p>
        <p class="legend-item drift"><span class="legend-key"></span><span class="legend-label"><strong>${copy("webui.states.drift")}</strong> ${copy("webui.state-description.drift")}</span></p>
      </div>
    </section>
  `;
}

export { EvidenceRail, StateLegend, NodeDepthPlate, LineagePlate, BlueprintPlate, EvidenceModeTabs };
