# Sources

Captured 2026-05-04. Marked **fetched** when content was retrieved successfully, **walled** when blocked by Cloudflare or 403, **paywalled** when behind a login.

## Primary sources

| Source | URL | Status | What it gave |
|---|---|---|---|
| Cognitive Revolution podcast (Brian Elliott + Sid Pardeshi) | https://www.cognitiverevolution.ai/infinite-code-context-ai-coding-at-enterprise-scale-w-blitzy-ceo-brian-elliott-cto-sid-pardeshi/ | fetched | Most architecture detail. Direct quotes on AST-shaped graph, dynamic agent generation, RAG-as-navigation, multi-vendor reviewers, memory at app layer |
| TWIML "Agent Swarms and Knowledge Graphs for Autonomous Software Development" | https://twimlai.com/podcast/twimlai/agent-swarms-knowledge-graphs-autonomous-software-development | fetched | Confirms hybrid graph+vector framing, agent swarm collaboration, AAP generation, knowledge persistence in graph |
| Blitzy product docs: code-review | https://docs.blitzy.com/project-lifecycle/code-review | walled (pasted by user into chat) | Three-layer review model (AAP / dynamic validation / code-level), risk tiers, post-review actions |

## Secondary sources

| Source | URL | Status | What it gave |
|---|---|---|---|
| Blitzy "How it Works" | https://blitzy.com/how_it_works | fetched (via search snippet) | 3000+ agents, 8-12 hour System 2 thinking, hybrid graph-plus-vector |
| Blitzy "Our Story" | https://blitzy.com/our_story | not fetched | Founder origin (Cambridge, MA, Harvard masters, bakery app, 2023) |
| Blitzy Platform Documentation: Introduction | https://docs.blitzy.com/introduction | walled | Listed in search results; not fetched |
| Blitzy templates: rules-overview | https://docs.blitzy.com/templates/rules-overview.md | walled | Cloudflare challenge even with .md suffix |
| Blitzy llms.txt index | https://docs.blitzy.com/llms.txt | walled | Cloudflare challenge |
| StartupHub article on Sid Pardeshi | https://www.startuphub.ai/ai-news/artificial-intelligence/2026/sid-pardeshi-on-ai-powered-code-generation | 403 | No content extracted |
| AI Podcast Picks substack on Blitzy | https://aipodcastpicks.substack.com/p/blitzy-about-using-ai-for-coding | fetched (empty content) | Summary page only, no architecture detail |
| Uneed review | https://www.uneed.best/blog/blitzy-review | not fetched | Review-style writeup, lower technical signal |

## Founder details

- **Brian Elliott** (CEO): podcast voice, frames the architecture vision, attributes core innovations to Sid.
- **Sid Pardeshi** (CTO): ex-NVIDIA software architect, 27 generative AI patents. Per Brian: *"Blitzy is the instantiation of if you had Sid work at the time of compute."* Less direct architecture commentary in the transcripts I could access; primary contributions cited as taste-based evaluation across model configurations and understanding LLM judge behaviour across vendor families.

## What I tried to fetch and could not

- `docs.blitzy.com/templates/rules-overview.md` — Cloudflare. The user noted markdown URLs sometimes work for direct AI ingestion but in this session both `.md` and `llms.txt` were challenged.
- `docs.blitzy.com/llms.txt` — Cloudflare. This is the index file Blitzy publishes for LLM scrapers; intended for direct AI consumption but firewalled in practice.
- `firecrawl` — out of credits (0/500 remaining this cycle), unable to use as a fallback.

## Tooling notes

- `WebFetch` worked for non-Cloudflare hosts.
- `WebSearch` returned good top results but no full transcripts.
- For future Blitzy research passes, options are: (a) listen to the podcast directly and transcribe, (b) wait for firecrawl credit reset, (c) the user pastes specific Blitzy doc pages into chat as they did with the code-review page.

## Confidence note

Architecture claims in [01-architecture.md](./01-architecture.md) are anchored in direct founder quotes from the Cognitive Revolution episode. Where I paraphrased rather than quoted, the underlying source is the same fetch. The "report card" framing is mine, extrapolating from the transcript phrase about output artifacts; treat as paraphrase, not quote.
