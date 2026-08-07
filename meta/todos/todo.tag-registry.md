---
node: cairn.kernel.scanner
status: open
created: 2026-07-16
---

# Tag Registry

Tags are freeform strings the parser merely collects
(`src/blueprint/parser.rs:93-97`), yet three code paths key behavior off
exact tag strings: `no-contract` exemption
(`src/map/contract_coverage.rs:19`), `no-test-coverage` exemption
(`src/map/test_coverage.rs:17`), and brownfield template matching via
MatchRule::HasTag (`src/brownfield/templates.rs:95`). A typo'd exemption
tag silently fails to exempt, and the cairn-dev skill (line 131) wrongly
documents tags as informational only.

Add an opt-in `tags:` section to cairn.config.yaml (small module modeled
on `src/summariser/config.rs`) declaring each known tag with a one-line
description. When the section exists, a scanner check emits an Info
finding (allocate a CK-series code in `docs/registries/error-codes.md`)
for any node tag absent from the declared set. Info tier is load-bearing:
`scan --strict` exits non-zero on Warning, and this must warn, never
block. No registry means no findings, so greenfield projects pay nothing.
Keep the vocabulary open per docs/agent/principles.md principle 2
(tag-extensible, never closed-enum).

Seed the root repo's registry with its current tags, flagging
`no-contract` and `no-test-coverage` as behavior-affecting so their
special status is finally documented in-tool. Optionally thread
descriptions through get/bundle/context render paths later (grows the
task from S to M).

Motivation: `res.a2ui-analysis` finding 6 (a2ui's catalog makes typos
fail fast; the closed-vocabulary part was deliberately not imported).
Also fix the cairn-dev skill line while landing this. No change proposal
needed for the Info-tier core.

## Mission disposition

2026-08-02: keep against dec.cairn-mission. Serves investigable. It keeps tag coverage discoverable for future graph investigations.

2026-08-07 audit (todo.roadmap-assumption-audit): keep as written.
