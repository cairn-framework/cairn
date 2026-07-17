/* Decision detail: drill-down view for a single decision artefact,
 * showing its condition/rationale prose and provenance/authority hinge.
 */
import { clsx, Fragment, html } from "./utils.js";

function DecisionDetail({ decision, node, onBack, onSelect }) {
  const status = decision.status || decision.frontmatter?.status || "accepted";
  const date = decision.date || decision.frontmatter?.date || null;
  const author = decision.frontmatter?.author || null;

  const body = decision.body || "";
  const conditionMatch = /##\s*(Condition|When this applies)\s*\n([\s\S]*?)(?=\n##\s|\s*$)/i.exec(body);
  const rationaleMatch = /##\s*Rationale\s*\n([\s\S]*?)(?=\n##\s|\s*$)/i.exec(body);
  const condition = conditionMatch ? conditionMatch[2].trim() : null;
  const rationale = rationaleMatch ? rationaleMatch[1].trim() : body.trim();
  const fm = decision.frontmatter || {};
  const parseRefs = (v) =>
    !v
      ? []
      : (Array.isArray(v)
          ? v
          : String(v)
              .replace(/^\[|\]$/g, "")
              .split(",")
        )
          .map((s) => String(s).trim())
          .filter(Boolean);
  const informedBy = decision.informed_by || parseRefs(fm.informed_by);
  const supersedes = decision.supersedes || parseRefs(fm.supersedes);
  const related = decision.related || parseRefs(fm.related);
  const revisit = decision.revisit_triggers || parseRefs(fm.revisit_triggers);
  const revisited = decision.revisited || (fm.revisited ? String(fm.revisited).trim() : "");

  return html`
    <section class="inspector decision-detail">
      <button class="pill back-btn" onClick=${onBack}>← ${node ? node.name : "back"}</button>
      <div class="ins-eyebrow">Decision</div>
      <h2 class="ins-title">${decision.title || decision.path}</h2>
      <div class="pill-row">
        <span class=${clsx("pill", status)}><span class="dot"></span>${status}</span>
        ${date ? html`<span class="pill">${date}</span>` : null}
        ${author ? html`<span class="pill">${author}</span>` : null}
      </div>

      ${
        condition
          ? html`<div class="decision-condition">
            <div class="caps">When this applies</div>
            <div class="condition-text">${condition}</div>
          </div>`
          : null
      }

      ${
        rationale
          ? html`<div class="decision-rationale">
            <div class="caps">Rationale</div>
            <p>${rationale}</p>
          </div>`
          : null
      }

      <div class="hinge-diagram">
        <div class="hinge-side prov">
          <div class="side-label">Provenance. evidence in</div>
          ${informedBy.length ? informedBy.map((r) => html`<div class="hinge-item" key=${r}><span class="n">·</span>${r}</div>`) : html`<div class="hinge-item gap-missing"><span class="n">·</span>no sources recorded</div>`}
        </div>
        <div class="hinge-axis">
          <div class="rod"></div>
          <div class="pivot"></div>
        </div>
        <div class="hinge-side auth">
          <div class="side-label">Authority. rules out</div>
          ${
            node
              ? html`<${Fragment}>
                  <div class="hinge-item"><span class="n">·</span>${node.id}</div>
                  <div class="hinge-item"><span class="n">·</span>${node.state || "synced"} on disk</div>
                <//>`
              : html`<div class="hinge-item muted"><span class="n">·</span>no module attached</div>`
          }
        </div>
      </div>

      ${
        supersedes.length || related.length || revisit.length || revisited
          ? html`<div class="decision-lineage">
              ${supersedes.length ? html`<div class="lineage-row"><span class="caps">Supersedes</span><div class="ref-chips">${supersedes.map((r) => html`<span class="pill ref" key=${r}>${r}</span>`)}</div></div>` : null}
              ${related.length ? html`<div class="lineage-row"><span class="caps">Related</span><div class="ref-chips">${related.map((r) => html`<span class="pill ref" key=${r}>${r}</span>`)}</div></div>` : null}
              ${revisit.length ? html`<div class="lineage-row"><span class="caps">Revisit when</span><div class="ref-chips">${revisit.map((r) => html`<span class="pill ref" key=${r}>${r}</span>`)}</div></div>` : null}
              ${revisited ? html`<div class="lineage-row"><span class="caps">Last revisited</span><div class="ref-chips"><span class="pill ref">${revisited}</span></div></div>` : null}
            </div>`
          : null
      }

      ${
        node
          ? html`<div class="attached-modules">
            <div class="caps">Attached to</div>
            <button class="attached-module" onClick=${() => onSelect(node.id)}>
              <div class="name">${node.name}</div>
              <div class="slug">${node.id}</div>
            </button>
          </div>`
          : null
      }
    </section>
  `;
}

export { DecisionDetail };
