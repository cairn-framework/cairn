---
id: dec.autodocs-head-to-head-arm-b
nodes:
  - cairn.brownfield
status: superseded
date: 2026-07-27
informed_by: [res.autodocs-head-to-head-feasibility]
revisit_triggers:
  - "If AutoDocs supports polyglot repositories AND drops the repository-root layout requirement"
---

# How to resolve AutoDocs-on-itself (Arm B) in the head-to-head

Accepted 2026-07-29 by maintainer ratification (PR #528 sheet W6), taking the
recommended branch: Arm B is dropped and `todo.autodocs-head-to-head` is
rewritten as a one-sided Arm A stress test and reopened. This now binds
`cairn.brownfield`.

## Context

`todo.autodocs-head-to-head` requires both a Cairn run over the AutoDocs
repository (Arm A) and an AutoDocs run over its own repository (Arm B). Arm A is
free and local. Arm B is blocked twice over, per
`res.autodocs-head-to-head-feasibility` drawing on `src.autodocs`:

1. AutoDocs' README "Known Issues" says, as two separate limits, that code must
   live at the repository root and that polyglot repos are not supported yet.
   The AutoDocs repo is both polyglot and nested, so it fails each independently
   and lifting one alone is not enough.
2. Its documented default-provider configuration needs two metered credentials
   and an unbounded per-definition ingest spend.

Blocker 1 is a capability gap upstream. Money does not close it.

## Decision

Drop Arm B and rewrite `todo.autodocs-head-to-head` as a one-sided Arm A
brownfield stress test over the AutoDocs repository.

Revisit when upstream lifts both limits, at which point running Arm B as
specified needs only a spend ruling.

If comparison evidence is wanted sooner, the two available partials both require
relocating a supported single-language subtree to a repository root, so neither
is AutoDocs-on-itself and neither satisfies the Acceptance as written. Choose
between them on what matters more: the paid-provider partial preserves real
summary quality, and the local-provider partial costs nothing but must report
the quality axis as not measured. The local substitution moves all six provider
settings, not just the two base URLs, because the default models are
`google/gemini-2.5-flash` and `text-embedding-3-large` and `EMBEDDINGS_API_KEY`
is validated non-empty at startup.

## Rationale

The target is unsupported regardless of spend, so the comparison the todo asks
for is unavailable at any price today. The cheap arm still delivers the todo's
highest-value goal, stress-testing Cairn brownfield on a real polyglot repo, and
it also unblocks the large-brownfield measurement `res.codeatlas-analysis` item
7 deferred.

Substituting a local model for the quality axis is refused as a default, because
reporting a local model's output as AutoDocs' quality would launder a model swap
into a product claim.

## Consequences

Until this was accepted, `todo.autodocs-head-to-head` stayed `blocked` on
`todo.autodocs-arm-b-ruling`, and Arm A did not land on its own, because
landing half the Acceptance would have read as progress while the binding
criterion stayed unmet.

Whoever accepted this was to set `status: accepted` and state which branch was
chosen; accepting the recommended branch did not by itself unblock the parent,
whose Acceptance still mandated both arms. That protocol was carried out on
2026-07-29: the parent was rewritten to the one-sided form, then reopened. Arm
B as originally specified returns via the parent's revisit trigger, once both
upstream limits no longer apply to the target.
