---
type: research
id: res.agent-experiment-linklint
node: cairn
date: 2026-06-10
---

# Blind two-arm agent experiment: building a small CLI with and without cairn

## Setup

Two identical fresh git repositories received the same product brief
(`SPEC.md` for "linklint", a markdown broken-link checker with an explicit
four-module architecture intent) and the same base working notes. Arm A
additionally got what a real adopting repo would have: `cairn init` output, a
hand-authored `cairn.blueprint` declaring the four intended modules as ghost
nodes, and a CLAUDE.md section pointing agents at `cairn context`, `cairn
scan`, and the keep-the-blueprint-in-sync rule. Arm B got no cairn at all.

One coding agent was launched per repo with an identical, neutral prompt.
Neither agent was told it was part of a comparison. Both were asked, after
finishing, to report any friction caused by the repo's tooling or docs.

## Results

Both arms shipped working tools meeting the full quality bar (fmt, clippy
`-D warnings`, tests green) with near-identical module structure; the
architecture intent in SPEC.md was a strong enough signal on its own to
produce the four-module shape. Surface metrics were close: arm A 875 LOC and
41 tests, arm B 794 LOC and 35 tests.

The behavioural difference showed up in correctness. On a shared fixture,
arm B reported two false positives (link syntax inside inline code spans and
fenced code blocks was treated as a real link); arm A handled both correctly.
Arm A's agent caught this by smoke-testing against markdown in its own repo,
where the example link syntax in SPEC.md sat inside backticks. Attribution is
soft (single run per arm; could be agent variance), but arm A's workflow
included more verification passes: its `.cairn/log.md` recorded four scans,
and the ghost-to-synced transition was explicitly used as a to-do list
("`cairn context` showed the four Ghost modules as a literal to-do list").

## Usability findings for cairn

1. **Ghost modules work as scaffolding.** Declaring intended modules before
   code exists gave the agent an orientation artefact it actively used. The
   blueprint-as-skeleton pattern is worth documenting as a greenfield
   workflow.
2. **Starter guidance must mention test directories.** Arm A's only real
   friction: SPEC.md required tests, the blueprint only declared `src/`
   paths, and the agent had to make a judgment call before extending the
   blueprint with a `./tests` path. Fixed: the init starter blueprint and the
   generated `.cairn/AGENTS.md` now call out test directories explicitly.
3. **No feedback channel existed.** Friction observed in a host project had
   nowhere to go. Fixed: `cairn feedback` plus the generated agent guide
   (see `meta/decisions/feedback-loop.md`).
4. **CLI behaved as documented.** The arm A agent reported zero confusion
   from cairn itself: "the cairn CLI behaved exactly as documented."

## Caveats

- n=1 per arm; no statistical claim. The correctness delta is suggestive,
  not conclusive.
- Both agents inherited ambient context from the cairn repo's own CLAUDE.md
  (a harness artefact, symmetric across arms). The arm B agent flagged this
  as the main source of potential confusion, not the task repo itself.
- The blueprint in arm A was hand-authored to match the spec; a sloppier
  blueprint would presumably help less.
