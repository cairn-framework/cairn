import { clsx, copy, highlightBlueprint, html } from "./utils.js";

function NodeDepthPlate({ node, inRows, outRows, onEdgeSelect }) {
  const paths = Array.isArray(node?.paths) ? node.paths : [];
  const symbols = Array.isArray(node?.symbols) ? node.symbols : [];
  const contracts = Array.isArray(node?.contracts) ? node.contracts : [];

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
        <dt>${copy("webui.path-count")}</dt>
        <dd>${String(paths.length)}</dd>
        <dt>${copy("webui.paths")}</dt>
        <dd>${paths.length ? paths.join(", ") : copy("webui.path-empty")}</dd>
        <dt>${copy("webui.symbols")}</dt>
        <dd>${String(symbols.length)}</dd>
      </dl>
      <section class="edge-group">
        <h4>${copy("webui.in")}</h4>
        ${
          inRows.length
            ? inRows.map(
                (row) => html`
                <button class="edge-row" type="button" onClick=${() => onEdgeSelect(row.id)}>
                  <span class="edge-dir">${copy("webui.in")}</span>
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
                  <span class="edge-dir">${copy("webui.out")}</span>
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
    <section class="lineage-plate">
      <button class="query-action" type="button" onClick=${onBack}>
        ${copy("webui.overview")}
      </button>
      <p class="lineage-title">${title}</p>
      <p class="lineage-meta">${[status, date].filter(Boolean).join(" · ")}</p>
      <pre><code class="blueprint-plate">${snippet}</code></pre>
      ${artefact.path ? html`<p class="plate-meta">${artefact.path}</p>` : null}
    </section>
  `;
}

function LineagePlate({ artefacts = {}, onOpen, selectedItem }) {
  const evidence = Array.isArray(artefacts.evidence) ? artefacts.evidence : [];
  const decisions = Array.isArray(artefacts.decisions) ? artefacts.decisions : [];
  const authority = Array.isArray(artefacts.sources) ? artefacts.sources : [];
  const selected = selectedItem && typeof selectedItem === "object" ? selectedItem : null;

  return html`
    <article class="lineage-plate">
      <h3 class="plate-title">${copy("webui.lineage")}</h3>
      <section class="lineage-stage">
        <p class="lineage-kind">${copy("webui.lineage-rationale")}</p>
        ${
          evidence.length
            ? evidence.map(
                (item) => html`
                <button class="query-chip" type="button" onClick=${() => onOpen(item)}>
                  ${item.title || item.id || copy("webui.artefact")}
                </button>`,
              )
            : html`<p class="plate-meta">${copy("webui.lineage-empty")}</p>`
        }
      </section>
      <section class="lineage-stage">
        <p class="lineage-kind">${copy("webui.lineage-decisions")}</p>
        ${
          decisions.length
            ? decisions.map(
                (item) => html`
                <button class="query-chip" type="button" onClick=${() => onOpen(item)}>
                  ${item.title || item.id || copy("webui.decision")}
                </button>`,
              )
            : html`<p class="plate-meta">${copy("webui.decision-empty")}</p>`
        }
      </section>
      <section class="lineage-stage">
        <p class="lineage-kind">${copy("webui.lineage-authority")}</p>
        ${
          authority.length
            ? authority.map(
                (item) => html`
                <button class="query-chip" type="button" onClick=${() => onOpen(item)}>
                  ${item.title || item.path || item.id || copy("webui.authority")}
                </button>`,
              )
            : html`<p class="plate-meta">${copy("webui.authority-empty")}</p>`
        }
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

function BlueprintPlate({ sourcePath, sourceText, selectionId }) {
  return html`
    <article class="blueprint-plate">
      <h3 class="plate-title">${copy("webui.blueprint-title")}</h3>
      <p class="plate-meta">${sourcePath || copy("webui.blueprint-path-unknown")}</p>
      <pre><code dangerouslySetInnerHTML=${{ __html: highlightBlueprint(sourceText || "", selectionId) }}></code></pre>
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
