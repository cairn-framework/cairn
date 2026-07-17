/* Command palette (Cmd+K): fuzzy node search plus the report-issue action. */
import { clsx, copy, Fragment, html, openReportIssue, useEffect, useRef, useState } from "./utils.js";

// ==========================================================================
// Command palette
// ==========================================================================

function CommandPalette({ open, graph, onClose, onSelect, version }) {
  const [q, setQ] = useState("");
  const [activeIdx, setActiveIdx] = useState(0);
  const inputRef = useRef(null);

  useEffect(() => {
    if (!open) return undefined;
    setQ("");
    setActiveIdx(0);
    const handle = requestAnimationFrame(() => {
      if (inputRef.current) inputRef.current.focus();
    });
    const onKey = (e) => {
      if (e.key === "Escape") onClose();
    };
    window.addEventListener("keydown", onKey);
    return () => {
      cancelAnimationFrame(handle);
      window.removeEventListener("keydown", onKey);
    };
  }, [open, onClose]);

  useEffect(() => {
    if (!open) return;
    const row = document.querySelector(".cmd-palette-results .result-row.active");
    if (row) row.scrollIntoView({ block: "nearest" });
  }, [open, activeIdx]);

  if (!open) return null;
  const ql = q.toLowerCase();
  const nodeMatches = graph
    ? graph.nodes.filter((n) => {
        if (!ql) return false;
        return n.id.toLowerCase().includes(ql) || (n.name || "").toLowerCase().includes(ql) || (n.kind || "").toLowerCase().includes(ql) || (n.state || "").toLowerCase().includes(ql);
      })
    : [];
  const reportActionMatches = ql !== "" && ["report", "issue", "bug", "feedback"].some((kw) => kw.includes(ql));
  const actionLabel = copy("webui.report.palette");
  const actions = reportActionMatches ? [{ __action: "report-issue", label: actionLabel }] : [];
  const shown = [...actions, ...nodeMatches].slice(0, 20);
  const shownNodes = nodeMatches.slice(0, Math.max(0, 20 - actions.length));

  const onInputKey = (e) => {
    if (e.isComposing || e.keyCode === 229) return;
    if (e.key === "ArrowDown") {
      e.preventDefault();
      setActiveIdx((i) => Math.min(i + 1, Math.max(0, shown.length - 1)));
    } else if (e.key === "ArrowUp") {
      e.preventDefault();
      setActiveIdx((i) => Math.max(i - 1, 0));
    } else if (e.key === "Enter") {
      const hit = shown[activeIdx];
      if (hit) {
        if (hit.__action === "report-issue") {
          openReportIssue(version);
        } else {
          onSelect(hit.id);
        }
        onClose();
      }
    }
  };

  return html`
    <div class="modal-scrim" onClick=${onClose}>
      <div class="cmd-palette" onClick=${(e) => e.stopPropagation()}>
        <div class="cmd-palette-head">
          <span class="cmd-label">Query</span>
          <input ref=${inputRef} value=${q}
            onInput=${(e) => {
              setQ(e.target.value);
              setActiveIdx(0);
            }}
            onKeyDown=${onInputKey}
            placeholder="search by name, id, kind, or status"/>
          <kbd>esc</kbd>
        </div>
        ${
          q === ""
            ? html`<div class="cmd-palette-syntax">
              <div class="caps">Query syntax</div>
              <div class="syntax-grid">
                <span class="kw">module · container</span><span class="rest">show by id or name</span>
                <span class="kw">ghost · orphaned</span><span class="rest">list reconciliation gaps</span>
                <span class="kw">synced</span><span class="rest">parts that match the plan</span>
              </div>
            </div>`
            : html`<div class="cmd-palette-results">
              ${
                shown.length === 0
                  ? html`<div class="row-empty" style="padding:var(--s-5)">${copy("empty-states.search-no-matches.body")}</div>`
                  : html`<${Fragment}>
                    ${
                      actions.length === 0
                        ? null
                        : html`<${Fragment}>
                          <div class="caps result-group">Actions</div>
                          ${actions.map(
                            (a, i) => html`
                            <button class=${clsx("result-row", i === activeIdx && "active")} key="action-report-issue"
                              onClick=${() => {
                                openReportIssue(version);
                                onClose();
                              }}>
                              <span class="badge">↗</span>
                              <span class="title">${a.label}</span>
                              <span class="rhs">report</span>
                            </button>
                          `,
                          )}
                        <//>`
                    }
                    ${
                      shownNodes.length === 0
                        ? null
                        : html`<${Fragment}>
                          <div class="caps result-group">Nodes</div>
                          ${shownNodes.map(
                            (n, i) => html`
                            <button class=${clsx("result-row", actions.length + i === activeIdx && "active")} key=${n.id}
                              onClick=${() => {
                                onSelect(n.id);
                                onClose();
                              }}>
                              <span class=${clsx("badge", n.kind === "module" ? "node" : n.kind === "decision" ? "decision" : "node")}>${n.kind}</span>
                              <span class="title">${n.name}</span>
                              <span class="rhs">${n.id}</span>
                            </button>
                          `,
                          )}
                        <//>`
                    }
                  <//>`
              }
            </div>`
        }
      </div>
    </div>
  `;
}

export { CommandPalette };
