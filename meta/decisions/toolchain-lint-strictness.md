---
id: dec.toolchain-lint-strictness
nodes:
  - cairn.kernel.hooks
  - cairn.kernel.scanner
status: accepted
date: 2026-06-23
---

# Adopt advisory toolchain-lint-strictness findings

## Context

Cairn reconciles the architecture map against reality. A recurring gap in host
projects is the absence of a strict lint configuration for a detected language:
a Rust crate may carry no clippy `-D warnings` configuration, or a JavaScript
surface may have no linter at all. This is one layer beyond pure architecture
(it is project-health), but it is existence/linkage-shaped in the same way as
existing cairn checks such as contract presence and test-coverage linkage
(cairn-a8z).

## Decision

Adopt a new advisory finding, `CAIRN_LINT_NOT_STRICT`, for tracked projects
whose detected primary language lacks a strict lint configuration.

- **Finding code:** `CAIRN_LINT_NOT_STRICT`.
- **Default severity:** `Warning` (non-blocking). Promote to blocking via the
  existing `cairn lint --strict` flag.
- **Scope:** config existence and strictness only. Cairn inspects configuration
  files; it never invokes a linter or formatter.
- **Per-language detection rules (initial):**
  - **Rust:** strict clippy configuration is present. Acceptable signals include
    a `.pre-commit-config.yaml` hook referencing `cargo clippy` with
    `-D warnings` or equivalent, or `Cargo.toml` `[lints.clippy]` setting
    warnings to `deny`. Presence of `cargo fmt` alone is not sufficient.
  - **JavaScript / TypeScript:** an ESLint, Biome, or Oxlint configuration file
    exists with at least one rule set to error (not warn-only) and the config is
    referenced by CI or package scripts.
  - **CSS:** a Stylelint or Biome CSS configuration exists with at least one
    rule set to error.
  - **Other languages:** defer. Do not emit the finding for languages without
    defined detection rules.
- **Node / project link:** the finding is attached to the node whose declared
  paths contain the language evidence. If a node has no detectable primary
  language, no finding is emitted.

## Rationale

This follows the same pattern as cairn-a8z (test-coverage integrity): an
advisory-by-default, opt-in-strict existence check that respects cairn's
boundary of observing reality rather than enforcing workflow. Keeping the check
config-only means cairn stays out of the business of running external tools and
avoids version or platform coupling. The `Warning` default lets project teams
address the gap without blocking commits; `--strict` lets teams that want a hard
gate opt in without adding a new config key.

## Consequences

- A future feature phase will implement the detector in the scanner/reconciler
  path and add the finding to the registry.
- Hook output may include `CAIRN_LINT_NOT_STRICT` under `cairn hook all` once
  implemented, so the finding should be classified as advisory in hook reports.
- Each new language added to the reconciler must either define a strict-lint
  detection rule or explicitly opt out of this check for that language.
- Cairn's own webui assets (`src/ui_assets/`) currently lack a strict CSS/JS
  lint config. If those assets are claimed by a blueprint node, that node would
  be flagged once the finding is implemented unless a strict config is added or
  the node is excluded.
