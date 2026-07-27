---
id: res.autodocs-head-to-head-feasibility
nodes:
  - cairn.brownfield
sources: [src.autodocs]
method: primary
date: 2026-07-27
---

# Why AutoDocs-on-itself (Arm B) cannot run, and the four ways out

`todo.autodocs-head-to-head` binds two arms on one codebase. Arm A runs Cairn's
brownfield onboarding over the AutoDocs repository. Arm B runs AutoDocs over its
own repository. Its Acceptance requires "both runs completed and documented", so
neither arm satisfies the unit alone.

This note records why Arm B stalls, the four resolutions considered, and the
recommendation awaiting a maintainer ruling, so no later session re-derives the
same blocker.

## Arm B has two independent blockers

**1. Upstream does not support the target, on two counts.** AutoDocs' README
"Known Issues" states separately that code must live at the repository root
rather than a nested folder, and that polyglot repos are "not supported yet".
The AutoDocs repository is polyglot (Python 544KB plus TypeScript 317KB) *and*
keeps its implementation under `ingestion/` and `webview/`. It fails both
conditions independently, so lifting either one alone still leaves Arm B
unsupported. No credential and no feasibility check changes this: a check would
confirm the documented limits, not cure them.

**2. The documented default-provider configuration costs metered money.**
`SUMMARIES_API_KEY` (default provider OpenRouter) and `EMBEDDINGS_API_KEY`
(default provider OpenAI) are secrets with no default, and
`ingestion/src/api/config.py:100-106` refuses to boot without the latter.
Summarisation runs per definition over the whole graph, so cost scales with
repository size and is unbounded by the documentation. This gates the default,
full-quality configuration, not literally every possible run: option 3 below
substitutes local providers and avoids the spend at a cost to what is measured.

Arm A has neither problem. The repository is public and Apache 2.0, and Cairn
brownfield onboarding is local compute over a working tree: a clone plus one
`cairn init --from-code` pass.

Blocker 1 is the reason the recommendation below is not simply "pay for it".

## Options considered

**1. Authorize the spend and run Arm B as written.** Would answer every axis the
todo names: summary quality, freshness, blast radius, provenance. It is the only
option that measures every named quality axis at full fidelity. But it is **not
sufficient on its own**: blocker 1 stops it first, and blocker 1 does not lift
until both the polyglot and the nested-layout restrictions stop applying to the
target. Until then this option is only available in a rewritten form, against a
relocated single-language subtree placed at a repository root, which is no
longer AutoDocs-on-itself as the Acceptance specifies.

**2. Drop Arm B and rewrite the parent as a one-sided brownfield stress test.**
Zero cost, and it still delivers the todo's highest-value goal, stress-testing
Cairn brownfield on a real polyglot repo. It also unblocks the measurement that
`res.codeatlas-analysis` item 7 deferred "until a large brownfield dogfood repo
exists to measure against". What it gives up is the comparison itself. Blocker 1
raises this option's standing sharply: a comparison upstream cannot currently
perform is not a comparison we are declining out of thrift.

**3. Point both providers at a local OpenAI-compatible server.** Removes blocker
2 only, and it is more than a base-URL change: `SUMMARIES_MODEL` and
`EMBEDDINGS_MODEL` default to `google/gemini-2.5-flash` and
`text-embedding-3-large`, which a local server will not serve, and
`EMBEDDINGS_API_KEY` must stay non-empty or the API will not start. So all six
provider settings move, with dummy tokens standing in for the keys. It also
changes what is measured: AutoDocs summary quality is a function of the model
behind that key, so a weak local model produces a comparison unfair to them and
misleading to us. Usable for the mechanical axes only (setup friction, runtime,
graph coverage, counts), with the quality axis reported as not measured. Blocker
1 still applies, so it needs the same relocated-subtree rewrite.

**4. Compare against their published output instead of running the tool.**
Sidesteps both blockers. Rejected on evidence, not on contact risk: it cannot
show setup friction, runtime, freshness behaviour, or false-positive rates.
Publishing a project-specific conclusion drawn this way would additionally need
maintainer clearance.

## Two partials exist, and they trade against each other

Neither satisfies Arm B as written, since both require relocating a supported
single-language subtree to a repository root, and so neither is
AutoDocs-on-itself:

- **Paid subtree partial** (option 1 in rewritten form): preserves real summary
  quality, costs authorized spend.
- **Local-provider subtree partial** (option 3): costs nothing, forfeits the
  quality axis.

## Recommendation, pending maintainer ruling

Option 2. The head-to-head as specified asks AutoDocs to do something its own
documentation says it cannot yet do, on two independent counts, so no spend
ruling makes the original comparison achievable. Rewrite the parent as an Arm A
brownfield stress test, which is free, immediately runnable, and already the
todo's highest-value goal.

Revisit when upstream lifts **both** limits, polyglot support and the
repository-root layout requirement, at which point option 1 becomes the right
answer and needs only a spend ruling.

If the maintainer wants comparison evidence before then, choose between the two
partials above on whether the summary-quality axis matters more than the spend.
Either write-up must state that the target was a relocated subtree, and the
local-provider variant must mark summary quality as not measured rather than
reporting a local model's output as AutoDocs' quality.
