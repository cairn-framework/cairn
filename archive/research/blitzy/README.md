# Blitzy research

External-product research on **Blitzy** (blitzy.com), an autonomous enterprise coding platform that orchestrates thousands of AI agents over a hybrid graph + vector context layer. Captured during the phase 7.6 / 7.7 / 7.8 / 8 / 9 / 10 planning window to inform CAIRN's longer-horizon orchestration story and to extract short-term borrows for the cflx-driven workflow.

## Status: v1 from podcast + docs trawl, 2026-05-04

Compiled from a single research pass:
- Cognitive Revolution interview (Brian Elliott + Sid Pardeshi, ~2 hours)
- TWIML "Agent Swarms and Knowledge Graphs" episode
- Blitzy product docs (introduction, how-it-works, code-review)
- Secondary coverage (StartupHub, AI Podcast Picks)

The Blitzy docs site is partly Cloudflare-walled, so primary architecture detail comes from the long-form podcast transcripts rather than from official docs.

## How to read

If you have one minute, read [02-borrow-list.md](./02-borrow-list.md). If you want the architecture, read [01-architecture.md](./01-architecture.md).

## File index

| File | What it covers |
|---|---|
| [01-architecture.md](./01-architecture.md) | Hybrid graph+vector, dynamic agent generation, RAG-as-navigation, validation stack, multi-vendor reviewer ensemble |
| [02-borrow-list.md](./02-borrow-list.md) | What to borrow now (cflx-era) versus what to defer to CAIRN-native orchestration |
| [03-sources.md](./03-sources.md) | URLs fetched, what each gave, what was inaccessible |

## Why this is filed in research, not in spec

CAIRN's spec deliberately rejects flattening the two-chain topology, even when external products would suggest a flatter shape. Blitzy's architecture validates several of CAIRN's existing bets (provenance over flat-RAG, framework-layer memory over fine-tuning, graph-as-navigation) but does not prescribe spec changes. Treat these notes as competitive context for design judgement on phases 7.6 through 10, not as authority over `docs/spec.md`.
