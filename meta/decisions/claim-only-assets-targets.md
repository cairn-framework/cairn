---
id: dec.claim-only-assets-targets
nodes:
  - cairn.kernel.scanner
  - cairn.ui
status: accepted
date: 2026-07-16
informed_by: []
---

# Ratification of claim-only assets targets

## Context

Blueprint node `cairn.ui` claims paths `./src/ui` (Rust code) and `./src/ui_assets` (HTML/CSS/JS frontend assets). When reconciling `./src/ui_assets`, the scanner defaults to Language::Unknown since directory language inference only supports rust, typescript, python, and go. This triggers a `CAIRN_RECONCILE_LANGUAGE_UNKNOWN` warning, blocking strict-scan execution. This decision is related to `todo.ui-assets-blueprint-path`.

## Decision

Introduce a `Language::Assets` variant representing explicit, claim-only targets. Assets targets never participate in automatic language inference or symbol extraction. Instead, we:
1. Reconcile assets by walking the target directory, ignoring any patterns in `.gitignore` and `.cairnignore`, and claiming all non-ignored files sorted alphabetically.
2. Produce an interface hash of `None` and an empty list of exported symbols.
3. Skip emitting `CAIRN_RECONCILE_LANGUAGE_UNKNOWN` warnings, but continue emitting `CAIRN_RECONCILE_EMPTY_TARGET` if the walked assets target is empty.
4. Support this language override explicitly via the `language: assets` override entry in `cairn.config.yaml`.

## Rationale

This provides a lightweight, configuration-driven way to declare node file ownership for non-code directory components (web frontend assets, documents, static configurations) without inventing language-specific reconcilers or triggering scanner warnings. Since these assets change independently of code, walking files dynamically during cached runs prevents stale graph file ownership claims without requiring heavy cache invalidation hooks.

## Consequences

- Node `cairn.ui` can safely claim `./src/ui_assets` under a `language: assets` targets configuration override.
- File queries like `cairn files` will accurately list claimed web UI assets alongside compile-ready Rust source code.
- Empty folders configured as assets targets will trigger warnings, encouraging cleanup or correct ignore rules.
