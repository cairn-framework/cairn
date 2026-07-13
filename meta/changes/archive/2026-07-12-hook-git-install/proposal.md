# Proposal: hook-git-install

## Motivation

Cairn exposes reconciliation hooks, but users must wire them into Git
manually. That leaves the commit gate absent in otherwise Cairn-managed
repositories.

## Scope

- Add explicit `cairn hook install`, `cairn hook uninstall`, and
  `cairn hook status` lifecycle commands.
- Install an executable Cairn `pre-commit` hook by default, with `--pre-push`
  selecting the other supported hook.
- Resolve `core.hooksPath`, refuse to overwrite non-Cairn hooks, remain
  idempotent, and report pre-commit framework conflicts.
- Support human and `--json` output with deterministic behaviour.

## Out of scope

- Silent hook installation during `cairn init`.
- Replacing or editing `.pre-commit-config.yaml`.
- Installing hooks other than `pre-commit` and `pre-push`.

