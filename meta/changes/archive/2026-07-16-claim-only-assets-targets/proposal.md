# Proposal: claim-only-assets-targets

## Motivation

Allows claiming non-code directories (like frontend assets under `src/ui_assets`) to resolve `todo.ui-assets-blueprint-path`. Without this, non-code directories trigger `CAIRN_RECONCILE_LANGUAGE_UNKNOWN` warnings, which was verified as a strict-scan blocker on 2026-07-16. This enabling feature has been maintainer-ratified.

## Scope

- Introduce `Language::Assets` variant for explicit target override.
- Reconcile assets targets via an ignore-aware files walk, claiming all files under the path without symbol-extraction or interface hash calculations.

## Out of scope

- Auto-inference of assets (explicit target configuration required).
- Moving files.
