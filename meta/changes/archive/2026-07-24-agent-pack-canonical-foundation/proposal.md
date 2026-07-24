# Proposal: agent-pack-canonical-foundation

## Motivation

The shipped Cairn agent guidance currently uses checked-in Claude files as both editable source and compiled output. That prevents a harness-neutral source of truth and leaves no deterministic adapter or validation boundary for later pack lifecycle work.

This change implements the byte-preserving mechanical foundation ratified by `dec.agent-pack-packaging`. It changes where maintainers edit pack assets without changing any shipped guidance bytes or authority.

## Scope

- Add the canonical pack source and data manifest under `tools/agent-pack`.
- Record a bundle version, canonical asset ownership by logical entry id and explicit mode, and pure Claude destination adapter rows.
- Add a dev-only renderer with deterministic check and write modes.
- Reject duplicate normalized destinations, duplicate harness entry-mode producers, lexical escapes, resolved escapes, and symlink escapes before writes.
- Preserve the exact bytes of the five shipped core skills, all three `cairn-dev` references, `/cairn-loop`, recovery, and landing assets.
- Keep generated markers outside content and enumerate generated paths in `.gitattributes`.
- Keep existing `include_str!` consumers compiling the rendered `.claude` bytes.
- Add focused behavioral tests and update blueprint ownership.

## Out of scope

- Pack install, update, status, or uninstall commands.
- Claude bootstrap or first-run lifecycle semantics.
- OMP adapter publication or treatment evaluation.
- Guidance, router, playbook, prompt, or authority changes.
- Scheduling, state machines, workflow edges, or runtime orchestration.
- Unrelated cleanup.
