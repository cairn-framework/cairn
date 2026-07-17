/* Blueprint source modal: read-only syntax-highlighted view of the
 * declared map source, with focus-scroll to the selected module.
 */
import { highlightBlueprint, html, useEffect } from "./utils.js";

// ==========================================================================
// Blueprint source modal
// ==========================================================================

function BlueprintModal({ open, blueprint, focusModuleId, onClose }) {
  useEffect(() => {
    if (!open) return undefined;
    const onKey = (e) => {
      if (e.key === "Escape") onClose();
    };
    window.addEventListener("keydown", onKey);
    return () => window.removeEventListener("keydown", onKey);
  }, [open, onClose]);

  useEffect(() => {
    if (!open) return undefined;
    const raf = requestAnimationFrame(() => {
      const hi = document.querySelector(".blueprint-modal .modal-body .hi");
      if (hi) hi.scrollIntoView({ block: "center" });
    });
    return () => cancelAnimationFrame(raf);
  }, [open, focusModuleId]);

  if (!open) return null;
  const source = blueprint?.source;
  const filePath = blueprint?.path;
  const innerHtml = source ? highlightBlueprint(source, focusModuleId) : '<span class="cm">Blueprint source is not available.</span>';

  return html`
    <div class="modal-scrim centered" onClick=${onClose}>
      <div class="blueprint-modal" onClick=${(e) => e.stopPropagation()}>
        <div class="modal-head">
          <span class="kicker">Blueprint source</span>
          <span class="file-path">${filePath || "(unknown path)"}</span>
          <button onClick=${onClose}>close ⎋</button>
        </div>
        <div class="modal-body">
          <pre dangerouslySetInnerHTML=${{ __html: innerHtml }}></pre>
        </div>
        <div class="modal-foot">
          <span>Read-only view of the declared map source.</span>
        </div>
      </div>
    </div>
  `;
}

export { BlueprintModal };
