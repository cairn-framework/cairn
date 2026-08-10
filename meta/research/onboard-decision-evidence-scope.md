---
id: res.onboard-decision-evidence-scope
nodes: [cairn.brownfield]
date: 2026-08-10
method: primary
---

# Discovery candidates are not the file set the invariant evidence class needs

## Question

`dec.brownfield-extraction-mechanism` clause 1 defines a closed evidence set for
`cairn onboard decisions` and says the branch "uses the deterministic
`src/brownfield/discovery.rs` facts as bounded code evidence". Implementing
clause 1 (`todo.brownfield-onboard-decisions-index`) forced a precise reading:
which file set does the invariant-comment class actually scan?

## Method and evidence boundary

Primary code audit of `src/brownfield/discovery.rs`, `src/brownfield/walk.rs`,
and a fixture run of the landed command. It proves what the current traversal
reports; it says nothing about how much invariant prose real repositories carry.

## Result

Candidate evidence is a strict subset of the observed source files, so reading
the invariant class off `DiscoveredCandidate::evidence` silently drops evidence.

`discovery::discover` keeps a directory only if a package root claims it or it
holds at least `MIN_FILES` (3) source files directly (`discovery.rs:86`,
`walk.rs:96-111`). A two-file directory outside every package therefore produces
no candidate and contributes no `evidence` paths, even though `walk::Survey`
already recorded its source files. A `# invariant:` comment in such a directory
is evidence by the ruling's own definition (the marker is the criterion, not the
candidate), so the candidate-derived file set contradicts the closed set it was
meant to supply.

Verified on a fixture carrying `src/tiny/one.py` (one file, no manifest): the
implementation reading candidate evidence omitted its `# invariant:` line; the
implementation reading `walk::Survey::source_files` reports it, unbound, because
no declared path claims it.

The threshold is correct for its own purpose. Proposing a node for a two-file
directory is noise, which is why `dec.brownfield-package-root-discovery` set the
threshold. The mistake is reusing a node-proposal filter as an evidence filter:
the two questions have different bars, and only the walk answers the second.

## Consequence for the plan

`walk.rs` now exposes `Survey::source_files`, and the index calls both
`discovery::discover` (code targets, which are genuinely candidate-shaped) and
`walk::survey` (the invariant scan). Two traversals, one per question.

`todo.brownfield-extraction-external-validation` records evidence counts per
source class. The landed wire labels those classes `document`,
`readme-section`, `invariant-comment`, and `code-target`; the invariant count is
whole-survey, not candidate-scoped, so a per-class count compared against
`cairn init --from-code` candidate counts will not line up, and should not be
expected to.

## Limits

This says nothing about whether the closed evidence set is wide enough. That is
the first `revisit_triggers` entry of `dec.brownfield-extraction-mechanism`
("misses a material ADR-like location"), and only the external-repository run
can fire it.
