---
node: cairn.root
status: done
created: 2026-07-15
related: [todo.architecture-modularity-audit, todo.modularity-scan-finding, todo.ui-assets-blueprint-path]
---

# Extend file-size gate beyond Rust sources


## Problem

`scripts/check-file-sizes.sh` only walks `src/**/*.rs` (limit 500 lines,
opt-out via `// cairn:allow-large-module reason: ...`). Non-Rust owned paths
are invisible, so the webui flagships grew to 2013 (`app.js`) and 2729
(`style.css`) lines with zero CI signal. Conventions section 2 states the
500-line cap as a project rule; the gate under-implements it.

## Evidence (res.architecture-modularity-audit, 2026-07-15)

- Gate body: `scripts/check-file-sizes.sh` lines 8-28, `find ... -name '*.rs'`.
- Wired from `scripts/pre-archive-rust-gates.sh` only for Rust archives.
- Measured offenders outside the gate: `src/ui_assets/app.js` (2013),
  `src/ui_assets/style.css` (2729). Both would fail a 500-line check today.
- Root-cause chain in the audit todo: incomplete size gate + incomplete map
  validation + no path claim on `ui_assets` let modularity drift scan-clean.

## Approach (backlog only)

1. Generalise `scripts/check-file-sizes.sh` to also check
   `src/ui_assets/**/*.{js,css}` (and any other non-Rust paths the blueprint
   later claims).
2. Reuse the same 500-line limit. Add an allow-list comment protocol for JS/CSS
   (e.g. first non-blank line `// cairn:allow-large-module reason: ...` for JS,
   `/* cairn:allow-large-module reason: ... */` for CSS).
3. When extending, either land temporary allow-list reasons on the current
   flagships **or** land after / alongside todo.webui-feature-module-split so
   the gate is green without permanent exemptions.
4. Keep the gate deterministic and dependency-free (POSIX sh), matching the
   existing script style.

## Priority

Highest-ROI self-guardrail from the audit: low effort, would have caught the
flagship drift automatically. Prefer landing this before or with the webui
split so the exemption window is short.

## Resolution

Implemented 2026-07-17. `scripts/check-file-sizes.sh` now checks Rust, JavaScript, and CSS sources at 500 lines, excluding `src/ui_assets/vendor/`, with language-appropriate allow markers. The `app.js` exemption is temporary until `todo.webui-feature-module-split` removes it; that todo reviews the `style.css` exemption, which may legitimately remain for its section-scoped monolith. All file walks preserve whitespace-bearing paths and sort deterministically; the existing pre-archive invocation remains unchanged; Makefile and pre-commit have no additional invocation. Pinned Biome 2.4.4, full pre-archive gates, and strict scan passed.
