#!/usr/bin/env bash
set -euo pipefail

# Cairn webui visual-eval benchmark harness.
#
# Renders the graph-explorer webui in headless Chrome from its live hand-authored
# sources (index.html, app.js, and the tokens+components+style.css concatenation)
# against a frozen, network-free API dataset (harness/fixtures). It screenshots a
# fixed set of viewport/interaction scenarios (desktop overview, node inspector,
# findings drawer, tablet, mobile), runs a deterministic visual eval on each
# (WCAG contrast, responsive overflow/offscreen, text clipping, tap-target size,
# design-token palette conformance, plus a pixel-level blank/clutter check on the
# screenshot), and aggregates the defects into one score.
#
# Primary metric:   ux_defect_score   (lower is better; 0 is the goal)
# Secondary metrics: per-dimension defect counts and render-health counters.
#
# Deterministic: fixed viewports, frozen data, web fonts blocked, reduced motion
# emulated, animations disabled before capture. No live network, no cargo build.
#
# Screenshots land in harness/out/screenshots/ and a machine-readable breakdown
# (per scenario, with the exact offending colour/element signatures) in
# harness/out/report.json so the optimisation loop knows what to fix.

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR"

command -v node >/dev/null 2>&1 || { echo "autoresearch: node not found on PATH" >&2; exit 1; }

if [ ! -f harness/fixtures/api/graph ]; then
  echo "autoresearch: missing harness/fixtures; capture them with harness/capture-fixtures.mjs" >&2
  exit 1
fi

exec node harness/eval.mjs
