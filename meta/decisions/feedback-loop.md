---
id: dec.feedback-loop
nodes:
  - cairn.kernel.cli
status: accepted
date: 2026-06-10
informed_by:
  - res.agent-experiment-linklint
---

# Close the dogfood loop from host projects via `cairn feedback`

## Context

Cairn dogfoods itself inside this repository, but once it is installed on
other projects there is no channel for friction discovered there to flow
back. Coding agents working in a host repo hit confusing messages, wrong
findings, or missing capabilities, then route around them and the signal is
lost. A blind two-arm agent experiment (see
`meta/research/agent-experiment-linklint.md`) confirmed both the value of the
generated guidance and the absence of any feedback path.

## Decision

Add a `cairn feedback "<message>"` command that records friction locally and
points at the upstream tracker, and make `cairn init` generate agent-facing
guidance that instructs agents to use it.

- `cairn feedback` appends a timestamped entry (with cairn version) to
  `.cairn/feedback.md` in the host project and prints a prefilled
  `https://github.com/cairn-framework/cairn/issues/new` URL. No network
  access, no GitHub credentials required; filing remains a human (or
  authorised agent) action.
- `cairn init` writes `.cairn/AGENTS.md`, a guide meant to be appended to the
  host project's CLAUDE.md or AGENTS.md. It covers orientation commands, the
  scan-before-commit loop, and the instruction to record cairn friction with
  `cairn feedback` before working around it.
- `cairn init` now prints next steps instead of a bare confirmation.

## Consequences

- Every project that adopts cairn becomes a dogfood site: friction
  accumulates in a structured local log that maintainers can triage into
  upstream issues, and the issue URL lowers the cost of filing directly.
- The local-log-first design means feedback works offline and never blocks
  an agent's task on network or auth.
- The upstream repo URL is compiled in; if the canonical repo moves, the
  constant in `src/cli/commands/feedback.rs` must move with it.
