---
id: dec.autodocs-head-to-head-arm-b
nodes:
  - cairn.brownfield
status: proposed
date: 2026-07-27
informed_by: [res.autodocs-head-to-head-feasibility]
revisit_triggers:
  - "If AutoDocs supports polyglot repositories AND drops the repository-root layout requirement"
---

# How to resolve AutoDocs-on-itself (Arm B) in the head-to-head

Proposed, not accepted: this needs a maintainer ruling, so nothing here binds
`cairn.brownfield` yet.

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

## Proposed decision

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

Until this is accepted or replaced, `todo.autodocs-head-to-head` stays `blocked`
on `todo.autodocs-arm-b-ruling`, and Arm A does not land on its own, because
landing half the Acceptance would read as progress while the binding criterion
stayed unmet.

Whoever accepts this should set `status: accepted` and state which branch was
chosen. Accepting the recommended branch does not by itself unblock the parent:
its Acceptance still mandates both arms, so it must first be rewritten to the
one-sided form, and only then reopened. A future ruling that authorizes a real
Arm B run would unblock it as written, but only once both upstream limits no
longer apply to the target.
