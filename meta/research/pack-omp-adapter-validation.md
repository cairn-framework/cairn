---
id: res.pack-omp-adapter-validation
nodes:
  - cairn.kernel.cli
sources:
  - src.omp-harness-discovery
date: 2026-07-26
---

# What a live OMP host actually discovers, and what a second adapter costs

## Question

`todo.agent-pack-omp-adapter` requires an OMP adapter validated against a live
OMP installation, not an adapter row asserted as fact. Two things had to be
measured rather than assumed: where OMP discovers a project pack, and what the
existing Claude-only lifecycle gets wrong the moment a second harness exists.

## Method

Host: `omp` 17.1.3 at `$HOME/.bun/bin/omp`, the same harness family the
baseline study used (omp/17.0.8). Discovery paths were read from the harness's
own documentation (`omp://config-usage.md`, `omp://slash-command-internals.md`,
`omp://skills.md`) and then confirmed by running the harness non-interactively
(`omp -p ... --cwd <project> --no-session --auto-approve`) against fresh
temporary projects, with and without the pack installed.

## What the harness does

- Project skills: `<project>/.omp/skills/<name>/SKILL.md`, scanned one level
  deep by the `native` provider (priority 100). User scope is
  `~/.omp/agent/skills`, which is the wrong target for a project installer.
- Project commands: `<project>/.omp/commands/*.md`, project before user.
- OMP also reads `.claude/skills` through its `claude` provider at priority 80,
  so a Claude install is not invisible to OMP. The OMP adapter earns its place
  by installing into the harness's own project surface at its own precedence,
  and by leaving `.claude/` alone on a host that has no Claude.

## Measurements

1. Fresh project, `cairn pack install --loop` with no selector: the install
   reports the `omp` adapter (detected from the project's `.omp` directory) and
   writes 21 files, which `pack status` then reports as 21 pristine. That the
   destinations are exactly the manifest's OMP rows, and that their bytes equal
   the Claude tree's, is pinned by tests rather than by the live capture
   (`tests/pack_omp_adapter.rs`).
2. Live discovery, pack installed: the host listed the installed cairn skills
   and read the router itself, `skill://cairn-dev`, with no other tool call.
   Routed separately, a bug-investigation task produced exactly two reads, the
   router and then `skill://cairn-dev/references/task-bug-investigation.md`:
   one entry, one just-in-time reference, nothing else.
3. Live discovery, same prompt, no pack installed: no cairn lifecycle skill is
   visible. The installed pack is what the harness discovers, not something
   already present on the host.
4. Live command surface: the host reports `cairn-loop` as a project slash
   command from `.omp/commands/cairn-loop.md` and resolves its step 2 to
   `.omp/skills/cairn-dev/references/loop-mode.md`.
5. Campaign, on the OMP install: `campaign start` pinned the loop entry and its
   six declared closure assets, all under `.omp/`; `verify` passed; appending a
   line to `cairn-loop-scope/SKILL.md` made `verify` exit non-zero with
   `HALT: the installed pack no longer matches this campaign`; running
   `pack update` during the campaign left the edited file untouched and did not
   change what the campaign verified against; `campaign end` released it, and a
   fresh `start` re-pinned only after the tree was pristine again.
6. Two revisions, to separate drift refusal from revision adoption: a campaign
   pinned revision A at bundle `a8246173...`, a genuinely different revision B
   (one canonical body changed, re-rendered, recompiled) was installed over it
   with `pack update`, and the running campaign still verified as `HALT`. Only
   after `campaign end` did a fresh `campaign start` pin B, at bundle
   `b4ec652f...`. A new installed revision reaches a session only through a new
   campaign.

## What the Claude-only lifecycle got wrong

With two harnesses, three behaviours that were unobservable become defects: a
bare `status` after an OMP install would classify the Claude tree and report
every file missing; a bare or crossed `update` would write a second tree into a
ledger that names one harness; and `resolve`/`campaign` ignored the selector
entirely. All three are now bound to the installed harness, and a `--harness`
with no value is a usage error rather than a silent fall back to detection.

## Boundary

This validates an installed and campaign-pinned adapter on one live host. It
does not measure whether the pack changes agent outcomes: that is
`todo.agent-guidance-treatment-evaluation`, which is blocked on owner-supplied
sealed evidence. Publication remains `todo.agent-pack-omp-publication`, gated on
that verdict.

Also recorded, because it cost a session: the shipped `cairn-loop.md` listed its
non-zero exit row before its "no campaign is active here" row, and
`cairn pack campaign verify` exits 1 in that ordinary case. A session read the
rows in order and halted before selecting any unit. The command now keys on the
message rather than the exit code alone.

An earlier attempt at this unit (loop commit `afc770c`, never merged) mapped the
pack to `.omp/agent/`, which is OMP's user scope, and rewrote asset bodies with
a blind `.claude/` replacement while advertising the adapter as supported
without live evidence. It was discarded rather than repaired.
