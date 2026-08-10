---
id: res.candidate-blueprint-absence-classification
nodes:
  - cairn.kernel.hooks
date: 2026-08-10
method: primary
---

# Absence of a candidate blueprint is not by itself "nothing to gate"

Evidence produced while implementing `todo.brownfield-first-hook-blueprint-unstaged`.
It refines that todo's Scope, which is the reason this record exists rather than
the change landing silently against a plan it outgrew.

## What the plan said

The todo offered two options and its first one read:

> Treat "no blueprint in the candidate tree AND none in `HEAD`" as nothing to
> gate, the same way `inside_work_tree` returning false already short-circuits,
> while leaving every other read failure a hard Error.

The `HEAD` clause was there to keep a staged deletion of a previously tracked
blueprint failing closed. That reasoning is correct and is implemented.

## What the run showed

Implemented literally, the carve-out opens a second hole the todo did not
consider. Reproduced on a scratch repository whose `cairn.blueprint` is tracked
in neither the index nor `HEAD`, with `meta/decisions/dec.local.md` at
`status: accepted`, `ratification: local` staged alongside its subject:

```
ratification_findings(root, artefacts, RatificationMode::Index) == []
```

Before the change that state was a hard `CAIRN_HOOK_AFFECTS_SUBSET`. The
bypass is durable, not merely a one-commit window: the decision is already
accepted at tier local by the time a blueprint lands, so the later commit that
adds the blueprint is filtered out by `decision_was_not_local`, which compares
against the merge base. The acceptance never faces the gate at all.

Two independent review lenses reached the same finding from different angles,
and neither needed adversarial intent to reach the state: it is what a
freshly onboarded repository looks like the moment the adopter stages a
generated decision before staging the blueprint.

## The refinement

Silence now requires two conditions, not one. The blueprint is tracked in
neither the candidate tree nor `HEAD`, AND the candidate tree accepts nothing
at tier local anywhere in it. The second probe is deliberately shaped:

- It is repository-wide (`git grep --full-name ... -- ':(top)'`). A candidate
  with no blueprint declares no decisions directory, so there is no pointer to
  scope the search by, and scoping it by the worktree's pointers would let the
  worktree contradict the candidate tree, which is the exact inversion this
  module's existing comment forbids.
- It matches frontmatter KEYS only, never their values. `frontmatter::parse`
  normalises quoting (`status: "accepted"`), indentation, and space before the
  colon, so any lexical value match is a classifier that disagrees with the
  real one. `git grep` narrows; the parser decides.
- Every failure mode stays a refusal: no `git grep` answer, an unreadable blob,
  invalid UTF-8, and an unborn or broken `HEAD` all map to Error rather than to
  proven absence.

## Limits of what this proves

The probe is a correctness guard, not an adversarial boundary. Anyone able to
commit can also decline to run the hook; the gate defends against accident and
against an agent staging an acceptance it never got ratified. What the evidence
establishes is narrower and sufficient: the reachable, no-effort bypass the
literal Scope would have shipped is closed, and every state that previously
refused still refuses unless absence is positively proven.
