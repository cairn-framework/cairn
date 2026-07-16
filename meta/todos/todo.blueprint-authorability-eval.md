---
node: cairn.root
status: open
created: 2026-07-16
---

# Blueprint Authorability Eval

Cairn's blueprint syntax, blueprint.delta format, and artefact
frontmatter are increasingly agent-authored (init --from-code, the draft
family, gap, change authoring), with zero measurement of whether models
produce them validly. A2UI's equivalent measurement (production validator
as scorer) drove its v0.9 prompt-first schema rewrite; cairn has no
instrument for such format decisions.

Build a small on-demand harness (for example scripts/authorability-eval/
or a harness/ sibling), declared in cairn.blueprint so scan stays clean:

- Authoring family: 5 to 10 task prompts ("add a module claiming these
  files", "author a blueprint.delta for this refactor", "write a decision
  covering nodes X,Y") run against a temp copy of
  test/fixtures/cairn-bootstrap; apply the model output and score with
  the production tooling: `cairn scan --strict` and `lint --json`.
  Primary metric: convergence cost (iterations and tokens to a clean
  scan under the deterministic repair loop). Secondary: first-shot
  validity, per-format failure hotspots (nested block syntax vs delta
  section markers).
- Navigation family: task prompts with ground truth extracted
  deterministically from map.json ("which node owns file X", "what
  decisions affect Y"), scoring steps and tokens for an agent using
  cairn commands against a grep-only baseline.

Reuse the summariser's LocalCommandBackend pattern for model invocation
and the METRIC-line convention from the existing webui harness. No CI
scheduling, issue filing, or dataset encryption until the harness runs
unattended; those are downstream apparatus, not the instrument.

Owner direction (2026-07-16): the benchmark runs can be driven through
the oh-my-pi harness's autoresearch command/extension rather than a
bespoke runner; scope this todo's harness work to the task prompts,
fixtures, and deterministic scoring, and let oh-my-pi own orchestration.

Motivation: `res.a2ui-analysis` finding 9. Overlap: the navigation family
is the same idea as `todo.agent-effectiveness-benchmarks` on the
codeatlas branch (res.codeatlas-analysis finding 2); one harness should
serve both, do not build two. Sequenced after
`todo.example-corpus-scan-assertions` so the fixture substrate is
trustworthy. Needs a change proposal (new declared module or scripts
entry).
