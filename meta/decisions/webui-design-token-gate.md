---
id: dec.webui-design-token-gate
nodes:
  - cairn.ui
status: accepted
date: 2026-06-23
---

# Webui design-token conformance gate

## Context

The webui stylesheet (`src/ui_assets/style.css`) rides on the design-system
tokens (`docs/design-system/tokens.css`) and must source every colour and
rem-based size from a `var(--token)`. CLAUDE.md, AGENTS.md, and the stylesheet
header all state "do not hardcode hex or rem values", but nothing enforced it:
biome (added by cairn-xiw) formats and lints the asset with its recommended
rule set, which has no rule for "use a token instead of a literal colour or
rem". The invariant was documented and currently honoured, yet a regression
would pass every gate.

## Decision

Add a standalone repository gate, `scripts/check-design-tokens.sh`, that fails
when `style.css` contains a hardcoded hex colour or rem value (CSS comments are
stripped first). Wire it into the same three places that already gate the webui
asset: the pre-commit config, the CI `webui` job, and the Makefile `check`
target. Cover its behaviour with `tests/check_design_tokens.rs`.

## Rationale

This is a project-health gate that lives alongside `scripts/check-file-sizes.sh`,
not a cairn feature. It deliberately stays outside cairn's kernel: per
`dec.toolchain-lint-strictness`, cairn inspects lint *configuration existence*
and never invokes a linter or formatter. The `CAIRN_LINT_NOT_STRICT` finding
checks that a strict config exists; it does not (and should not) encode a
project's specific token-conformance rule. A repo script is the right home for
that rule, mirroring how the Rust file-size limit is enforced.

Scope is limited to `style.css`: it is the hand-written stylesheet bound to the
tokens. `app.js` is behaviour, not styling, and the marketing surfaces under
`docs/landing/` are out of this iteration's scope.

## Consequences

- A hardcoded hex colour or rem value in `style.css` now blocks commit, push,
  and CI, with the offending file and line reported.
- A future token-conformance need for `app.js` or `docs/landing/` extends this
  script (parameterised via `CAIRN_DESIGN_TOKENS_TARGET`) rather than adding a
  new mechanism.
- The gate is config-free and dependency-free (POSIX sh + awk + grep), keeping
  the repo's low-dependency, single-binary ethos intact.
