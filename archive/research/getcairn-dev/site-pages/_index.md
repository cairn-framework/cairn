# getcairn.dev Site Crawl Index

**Captured:** 2026-04-28
**Crawl scope:** www.getcairn.dev marketing surface plus /docs/ documentation tree.
**Total pages captured:** 48 markdown files (47 distinct URLs; one auth-gated /signin noted).

## Marketing pages

| Source URL | Local path | Status | Words |
|---|---|---|---|
| https://www.getcairn.dev/ | marketing/home.md | OK | 537 |
| https://www.getcairn.dev/about | marketing/about.md | OK | 254 |
| https://www.getcairn.dev/blog | marketing/blog.md | OK | 357 |
| https://www.getcairn.dev/blog/dead-paths | marketing/blog-dead-paths.md | OK | 1185 |
| https://www.getcairn.dev/blog/engineering-math | marketing/blog-engineering-math.md | OK | 2300 |
| https://www.getcairn.dev/blog/four-questions | marketing/blog-four-questions.md | OK | 316 |
| https://www.getcairn.dev/blog/inversion | marketing/blog-inversion.md | OK | 1572 |
| https://www.getcairn.dev/blog/missing-middle | marketing/blog-missing-middle.md | OK | 1844 |
| https://www.getcairn.dev/blog/pipeline | marketing/blog-pipeline.md | OK | 825 |
| https://www.getcairn.dev/blog/pyramid | marketing/blog-pyramid.md | OK | 1477 |
| https://www.getcairn.dev/blog/systems-thinking | marketing/blog-systems-thinking.md | OK | 448 |
| https://www.getcairn.dev/blog/trade-studies | marketing/blog-trade-studies.md | OK | 2024 |
| https://www.getcairn.dev/concepts | marketing/concepts.md | OK | 130 |
| https://www.getcairn.dev/concepts/dendritic-decomposition | marketing/concepts-dendritic-decomposition.md | OK | 140 |
| https://www.getcairn.dev/concepts/dendritic-explorer | marketing/concepts-dendritic-explorer.md | OK | 242 |
| https://www.getcairn.dev/concepts/model-completeness | marketing/concepts-model-completeness.md | OK | 156 |
| https://www.getcairn.dev/concepts/narrative-lens | marketing/concepts-narrative-lens.md | OK | 260 |
| https://www.getcairn.dev/demo | marketing/demo.md | OK | 85 |
| https://www.getcairn.dev/pricing | marketing/pricing.md | OK | 375 |
| https://www.getcairn.dev/privacy | marketing/privacy.md | OK | 588 |
| https://www.getcairn.dev/signin | marketing/signin.md | auth-gated (returned home content) | 48 |
| https://www.getcairn.dev/story | marketing/story.md | OK | 825 |
| https://www.getcairn.dev/terms | marketing/terms.md | OK | 525 |

## Documentation pages

| Source URL | Local path | Status | Words |
|---|---|---|---|
| https://www.getcairn.dev/docs | docs/index.md | OK | 416 |
| https://www.getcairn.dev/docs/ai-governance | docs/ai-governance.md | OK | 430 |
| https://www.getcairn.dev/docs/ai-pipeline | docs/ai-pipeline.md | OK | 211 |
| https://www.getcairn.dev/docs/behavior | docs/behavior.md | OK | 299 |
| https://www.getcairn.dev/docs/changelog | docs/changelog.md | OK | 916 |
| https://www.getcairn.dev/docs/dead-paths | docs/dead-paths.md | OK | 184 |
| https://www.getcairn.dev/docs/decomposition | docs/decomposition.md | OK | 512 |
| https://www.getcairn.dev/docs/editing-interfaces | docs/editing-interfaces.md | OK (page returned generic platform overview; noted in file) | 254 |
| https://www.getcairn.dev/docs/entity-types | docs/entity-types.md | OK | 247 |
| https://www.getcairn.dev/docs/faq | docs/faq.md | OK | 276 |
| https://www.getcairn.dev/docs/four-questions | docs/four-questions.md | OK | 288 |
| https://www.getcairn.dev/docs/glossary | docs/glossary.md | OK | 366 |
| https://www.getcairn.dev/docs/interfaces | docs/interfaces.md | OK | 223 |
| https://www.getcairn.dev/docs/key-concepts | docs/key-concepts.md | OK | 480 |
| https://www.getcairn.dev/docs/lens-paradigm | docs/lens-paradigm.md | OK | 480 |
| https://www.getcairn.dev/docs/lens-workflows | docs/lens-workflows.md | OK | 741 |
| https://www.getcairn.dev/docs/lenses | docs/lenses.md | OK | 220 |
| https://www.getcairn.dev/docs/local-first | docs/local-first.md | OK | 292 |
| https://www.getcairn.dev/docs/manual-vs-ai | docs/manual-vs-ai.md | OK (page returned generic platform overview; noted in file) | 316 |
| https://www.getcairn.dev/docs/nodes-properties | docs/nodes-properties.md | OK | 285 |
| https://www.getcairn.dev/docs/quick-start | docs/quick-start.md | OK | 610 |
| https://www.getcairn.dev/docs/requirements | docs/requirements.md | OK | 305 |
| https://www.getcairn.dev/docs/shortcuts | docs/shortcuts.md | OK | 104 |
| https://www.getcairn.dev/docs/tools | docs/tools.md | OK | 184 |
| https://www.getcairn.dev/docs/verification | docs/verification.md | OK | 294 |

## Notes

- /signin returned the marketing home body; treated as auth-gated and noted in file rather than skipped.
- /docs/manual-vs-ai and /docs/editing-interfaces returned a generic platform-overview body via WebFetch (likely client-side routed content not exposed to the static fetcher). They are captured with that body and a note flagging the issue, so downstream graphifying can decide whether to re-fetch via a JS-aware crawler.
- /app is a product surface (login-gated) and was not crawled.
- No external host redirects were encountered.
- No 404s were encountered.
- All other internal links surfaced from the home, story, blog, concepts, and docs index pages were captured.

## Coverage by section

- Marketing root and policy pages: home, story, about, demo, concepts, blog, pricing, privacy, terms, signin (10 pages).
- Marketing concept demonstrations: dendritic-explorer, dendritic-decomposition, narrative-lens, model-completeness (4 pages).
- Blog posts: 9 essays (dead-paths, pipeline, pyramid, inversion, four-questions, engineering-math, missing-middle, systems-thinking, trade-studies).
- Docs landing plus 24 doc pages spanning quickstart, key concepts, lens paradigm, governance, methodology, modeling primitives, tooling, reference, and changelog.
