---
node: cairn.kernel.map
status: open
created: 2026-07-28
---

# Module Size Limit Adopter Scope

## Priority

P2 adoption friction. It degrades the signal-to-noise of `cairn scan` for exactly
the brownfield audience the first-run path is aimed at. This is a
baseline-and-configuration ergonomics problem, not an unfixable check: a per-file
escape hatch exists.

## Problem

`CAIRN_MODULE_OVERSIZED` applies this repository's own 500-line house rule to
every adopter's codebase at a fixed threshold. `MODULE_SIZE_LIMIT_LINES` is a
hardcoded `const` (`src/map/module_size.rs:44`), deliberately mirrored
byte-for-byte with `scripts/check-file-sizes.sh`. The documented escape is
per-file: a `cairn:allow-large-module reason: <reason>` marker on the first
non-blank line (`src/map/module_size.rs:3-8`, `:41-43`). What does not exist is a
project-level knob for THIS check: `scanner::config::Config`
(`src/scanner/config/mod.rs:49`) already carries generic surfaces
(ignores, context, rules, artefact types, targets, intentional
asymmetries, gates, a decision-accumulation threshold), but nothing for
module size: no threshold override, no per-code severity control, and no
recorded baseline. Any design should extend those existing generic
patterns rather than invent a parallel config.

Measured on the MAG repository (onboarded with the released 0.9.0 installer
binary; the scan reported here was run with a `main` build at `00c212a`, and the
rule is unchanged between them: `MODULE_SIZE_LIMIT_LINES` is 500 and `Config`
carries no threshold or severity field at the `v0.9.0` tag too) against a
reviewed 23-node blueprint: 30 of its 32 scan findings are
`CAIRN_MODULE_OVERSIZED`. They are Warnings, so `cairn scan --strict` cannot pass
until the adopter either splits unrelated files or annotates each one with a
marker justifying a rule they never adopted. The reconciliation findings that
matter in that project are buried underneath. One flagged file is a 8378-line test
module, another a 3350-line benchmark; neither is a statement about the
architecture cairn was asked to reconcile.

The check is right for this repository. The question is whether a house style
guideline belongs in the same finding stream as reconciliation results (orphans,
ghosts, drift, contract and decision violations), which are statements about the
adopter's declared intent versus their code, and what the ergonomic on-ramp is for
a large existing codebase meeting it for the first time.

## Scope

- Rule on where the module-size guideline belongs for an adopter project. Options,
  not a decision: make the threshold configurable in root-level `cairn.config.yaml` (the
  scanner's only recognised config surface; today it lacks any
  module-size or severity fields) with 500 as
  the default; let a project opt a finding code down to Info or off; accept a
  recorded baseline so pre-existing violations are acknowledged once and only new
  ones warn; or scope the check to opt-in projects, with this repository as an
  opt-in case. Any of these needs a decision artefact before code.
- Whichever way it goes, keep the shell-gate parity contract stated at the top of
  `src/map/module_size.rs` intact: the default threshold and the marker protocol
  must still match `scripts/check-file-sizes.sh` for this repository.
- If per-code severity overrides or baselines are the answer, design them as a
  general surface across finding codes rather than a special case for this one, and
  weigh them against the fail-closed posture of `cairn scan --strict`.

## Acceptance

- A decision artefact records the ruling and its reasoning.
- A large adopter project can reach a green `cairn scan --strict` without editing
  or annotating source files that its own conventions consider fine, or the
  decision states plainly why it should not be able to.
- This repository's own scan behaviour and its `scripts/check-file-sizes.sh`
  parity are unchanged by the fix.

Measured while checking a brownfield onboarding report (MAG) against this
repository.

2026-08-07 audit (todo.roadmap-assumption-audit): keep; adopter order after todo.ratification-candidate-pointer. Health pass: see res.roadmap-audit-health; the adopter-scope question is unchanged.
