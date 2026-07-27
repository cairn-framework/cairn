---
node: cairn.brownfield
status: blocked
created: 2026-07-11
---

# Brownfield head-to-head: Cairn on AutoDocs versus AutoDocs on itself

Experiment (2026-07-11): clone TrySita/AutoDocs locally, run Cairn's brownfield onboarding on it (Arm A), and separately run AutoDocs against its own repository (Arm B). Compare the two outputs on the same codebase.

Three goals:

- Stress-test Cairn brownfield on a real polyglot repo (Python ingestion service plus TypeScript Next.js app, Docker Compose, MCP surface).
- Use Cairn's map as the navigation aid for our own competitor study of their codebase, so the study itself validates the tool.
- Head-to-head: what does Cairn's authored architecture/intent map capture that their generated dependency docs miss, and the reverse (summary quality, freshness, blast radius, provenance).

Scope and boundaries:

- Local and private; publish nothing project-specific without maintainer clearance (see business adoption plan in the george repo).
- Record setup friction, runtime, node counts, findings, false positives for both tools.
- Output: a research artefact (meta/research) with the comparison table and any Cairn defects or borrow-candidates it exposes; file follow-up todos per defect.

Acceptance: both runs completed and documented; comparison note written with at least three concrete Cairn improvements or confirmations; no unsolicited external contact.

## Review disposition (2026-07-11)

Backlog review recommends **narrow + defer**. The webui/dashboard comparison
sub-goal is already delivered by `todo.webui-simplicity-review` (the AutoDocs
dashboard was cloned and compared this session), so drop that from scope. What
remains is the brownfield-map versus generated-dependency-docs comparison, which
partly overlaps `todo.discovery-import-edges` and the first-run work. This is a
research artefact with no user-facing deliverable and carries external-contact
risk, so it is **low priority at current scale**: keep as a deferred research
note, run only alongside a real brownfield validation pass, and never make
external contact from it.

## Blocked (2026-07-27)

blocked on sub-todos: todo.autodocs-arm-b-ruling (node cairn.brownfield)

The 2026-07-11 scope narrowing above still stands: the webui/dashboard sub-goal
stays dropped. Only that disposition's **deferral** is superseded. "Low priority
at current scale" was a priority argument, and the real obstacle turned out to be
concrete: Arm B is blocked by an upstream capability gap (AutoDocs supports
neither polyglot nor nested-layout repos, and its own repo is both, so lifting
either limit alone is not enough), plus metered spend on the documented
default-provider configuration. Arm A is deliberately not landed alone, because
half a binding Acceptance would read as progress.

Evidence: `res.autodocs-head-to-head-feasibility` (source `src.autodocs`).
Recommendation pending maintainer ratification: `dec.autodocs-head-to-head-arm-b`,
status `proposed`, which recommends dropping Arm B and rewriting this todo as a
one-sided Arm A stress test.
