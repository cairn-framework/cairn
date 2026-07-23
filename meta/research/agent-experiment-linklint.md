---
id: res.agent-experiment-linklint
nodes:
  - cairn.kernel.cli
  - cairn.kernel.query
date: 2026-06-10
method: primary
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
Arm A's agent caught this by smoke-testing against Markdown in its own repo,
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

## Three-arm navigation baseline (2026-07-23)

### Question and method

This follow-up isolated three treatments: target-repository navigation, the
same environment with Cairn's current query surface available, and the Cairn
environment with the canonical `cairn-dev` guidance pack. The frozen
development corpus contained one locate task and one impact-map task from each
of ripgrep and Flask. Every run used `openai-codex/gpt-5.6-sol`, medium
thinking, a 180-second limit, and only `read`, `grep`, `glob`, and `bash`.
Responses were graded blind by two independent graders against frozen
per-task facts and three two-point components. The paired arm order and rubrics
are recorded in `manifests/{pilot-order,final-order,development}.json` inside
the evidence archive. After three trials, the frozen rule
$n=\max(3,\lceil((1.96+0.84)s_d/1)^2\rceil)$ used each comparison's paired
quality-score SD. The larger requirement added one complete paired trial per
task, yielding 48 valid engaged runs. A one-point difference on the six-point
quality scale was the smallest effect of interest.

The first 36 assigned runs all failed before a model turn because the isolated
runner omitted OMP authentication state. Those runs remain the primary
intention-to-treat cohort. A recorded protocol amendment changed only
authentication-state isolation. The later 48 runs are a secondary engaged-run
analysis, not a replacement primary cohort.

The compact public evidence is in
`archive/strongholds/agent-guidance-baseline/`. Its manifest records the pinned
Cairn, runner, ripgrep, and Flask revisions and the SHA-256 digest of the
evidence archive. Sealed confirmation prompts and ground truth remain
unopened and are not in the repository.

### Results

The primary cohort establishes a harness failure, not a treatment effect:
36/36 runs produced no candidate outcome. In the engaged cohort, all 48 runs
completed. Two responses contained material contradictions.

Mean quality was 4.94 for target-only navigation (SD 0.77), 5.13 for the Cairn
surface arm (SD 0.89), and 5.19 for the pack arm (SD 0.83). The paired
Cairn-minus-target improvement was 0.19 points. The paired pack-minus-Cairn
improvement was 0.06 points. Their paired-difference SDs were 0.75 and 0.68
respectively. Neither difference reached the preregistered one-point threshold.
Task-balanced recalled-fact means were 4.19, 4.38, and 4.56 respectively.

The first programme claim is not supported by this experiment. Merely making
the query surface available did not meaningfully beat target-only navigation,
and the Cairn arm invoked Cairn in 0/16 runs. Its small observed quality
difference is therefore descriptive variation rather than evidence of query
benefit.

The second claim separates adoption from outcome. The pack arm retrieved the
guidance in 16/16 runs and invoked Cairn in 16/16 runs, so the pack reliably
caused use of Cairn's capabilities. It did not produce a meaningful measured
quality gain over the Cairn arm. Pack runs also averaged more tool calls
(16.31 versus 13.31), input tokens (47,034 versus 37,229), and elapsed time
(109.1 versus 103.1 seconds).

The earliest failed handoff was runner availability in the primary cohort.
Within valid runs, the Cairn surface arm then failed at invocation.
Required-fact omissions remained. One Cairn-arm response and one pack-arm
response contained a material contradiction. No evidence supports changing
public queries or runtime authority from this sample.

### Reversible follow-up hypotheses

1. A single explicit first-turn instruction to inspect `cairn context` may
   convert surface availability into actual invocation without requiring a
   router.
2. A shorter task-shaped reference that names only the relevant Cairn command
   may retain the pack's 16/16 activation while reducing its extra calls and
   tokens.
3. A follow-up comparison should treat invocation as a manipulation check and
   grade query contribution separately from final-answer quality.

These are experiment candidates only. They grant no implementation,
recommendation, or authority status.
