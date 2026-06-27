# getcairn.dev research

External-product research file set on **getcairn.dev**, an AI-augmented Model-Based Systems Engineering (MBSE) tool for hardware and systems engineers. Shares only the name "Cairn" with this repo's framework. Captured for graphify-wiki ingestion: every claim is marked as **verified**, **from screenshot N**, **inferred**, or **unknown**.

## Status: v1 awaiting user supplements

This is a v1 doc set produced after two scout passes plus a hands-on UI capture session. The user has flagged that more screenshots and observations are coming. The structure is built to absorb supplements without restructuring: each file owns one concept, and new evidence appends to working-notes.md and slots into the existing files via cross-link.

## How to read

Start with [01-product-overview.md](./01-product-overview.md) for what the product is. If you only have time for one file, read [07-ontology-comparison.md](./07-ontology-comparison.md): it is the artefact closest to a decision. If you want our concrete next steps, read [08-borrow-list.md](./08-borrow-list.md).

## File index

| File | What it covers |
|---|---|
| [01-product-overview.md](./01-product-overview.md) | What getcairn.dev is, audience, market positioning, source-code status |
| [02-workflow-genesis.md](./02-workflow-genesis.md) | The 3-round AI interview, "preserved as provenance", build phase |
| [03-information-architecture.md](./03-information-architecture.md) | The 10 top-nav tabs, 8 sidebar tools, system tree, header chrome |
| [04-node-model.md](./04-node-model.md) | Nodes, properties, budgets, attachments, interfaces, AI-tagging |
| [05-completeness-and-causality.md](./05-completeness-and-causality.md) | Three-axis fidelity radar, causal pyramid, quality scoring |
| [06-command-palette.md](./06-command-palette.md) | Context-aware palette, contextual actions, generation pipeline |
| [07-ontology-comparison.md](./07-ontology-comparison.md) | Their term to our term mapping with screenshot evidence |
| [08-borrow-list.md](./08-borrow-list.md) | What to lift, ranked by leverage, with effort and risk |
| [09-design-influence.md](./09-design-influence.md) | Visual language, type stack, UX patterns worth studying |
| [10-source-attribution.md](./10-source-attribution.md) | URLs fetched, screenshot inventory, dates, verified vs inferred |
| [working-notes.md](./working-notes.md) | Per-screenshot transcriptions, feedstock for the synthesis files |
| `screenshots/` | All UI captures, semantically named |

## Ground rules used while authoring

- Em-dashes are banned in this repo. Replaced throughout with period, colon, comma, or parenthesis.
- Their model is **not** a "two-chain" model. They have a linear traceability flow (Brief → Subsystems → Components → Interfaces → Requirements → Verification). Our model is two chains meeting at a hinge (provenance chain and authority chain). The doc set is careful never to flatten one onto the other.
- Their source is closed (verified via prior scout). No speculation about their internals; UI behaviour is described without claiming the underlying mechanism.
- Every borrow recommendation in [08-borrow-list.md](./08-borrow-list.md) carries an effort tier and at least one explicit downside.

## Source provenance

This file set rests on three layers of evidence, in increasing primacy:

1. **Two prior Uruk-hai scout reports**, synthesised into [../../strongholds/external-cairn-docs-research.md](../../strongholds/external-cairn-docs-research.md). Used as a baseline.
2. **Public docs pages crawled by those scouts**, listed in [10-source-attribution.md](./10-source-attribution.md).
3. **UI screenshots captured by the user during a hands-on trial**, transcribed verbatim in [working-notes.md](./working-notes.md) and saved in `screenshots/`.

When a screenshot contradicts the stronghold, the screenshot wins. Such contradictions are listed at the bottom of working-notes.md so a future scout can reconcile.

## Single highest-leverage observation

The **"Project Genesis · Preserved as provenance"** label observed on the post-interview "Ready to build" screen ([screenshots/03-ready-to-build-project-genesis.png](./screenshots/03-ready-to-build-project-genesis.png)) is the single most interesting design pattern in the entire product. If we adopt it, we should adopt it deliberately: the genesis transcript should be a typed, queryable, mechanically-enforced artefact in the provenance chain, not a UI affordance. See [02-workflow-genesis.md](./02-workflow-genesis.md) and [08-borrow-list.md](./08-borrow-list.md) for the full argument.
