---
id: src.autodocs
file: https://github.com/TrySita/AutoDocs/tree/795ff04ddf6637cf044424f93c9fa807e08181cc
verification: external
type: tool
date: 2026-07-27
---

# AutoDocs (Sita): dependency-graph-driven documentation generator

Public Apache 2.0 repository, read 2026-07-27 through the GitHub API at the
pinned commit above. **Not cloned and not executed**, which is why this source
is `external` rather than `verified`.

Evidence trail: `README.md`, `.env.example`, and `ingestion/src/api/config.py`
were read through the contents API at the pinned commit. The language byte
counts below come from the `/languages` endpoint, which takes no `ref`
argument; they were recorded while the default branch still resolved to the
pinned SHA.

Language breakdown reported by GitHub: Python 544,359 bytes, TypeScript 316,720,
PLpgSQL 10,212, Shell 7,073, CSS 6,787, Dockerfile 2,797, JavaScript 1,696.

What the tool does, in its own description: parses a repository with tree-sitter
plus SCIP for symbol resolution, builds a code dependency graph over files,
definitions, calls, and imports, topologically sorts it, and walks that order to
generate dependency-aware documentation. It exposes a FastAPI ingestion and
search backend, a Next.js web UI, and an MCP server for agent deep-search.

## Documented limits (README "Known Issues", lines 178 to 182)

Load-bearing, and they disqualify the obvious experiment:

- "In your repositories, code must live at the repository root, not in a nested
  folder."
- "Language support: currently supports TS, JS, and Python; currently working on
  expansion to Go, Kotlin, Java, and Rust."
- "Polyglot repos (multiple languages in one repo): not supported yet, but we're
  actively working on it."

The AutoDocs repository is itself polyglot (Python plus TypeScript) and keeps its
implementation in nested directories (`ingestion/`, `webview/`). By its own
documentation it is therefore not a supported ingestion target for itself.

## Running it

- Toolchain: pnpm 10+ (Node 20+ recommended, not required), uv, and Docker with
  Docker Compose. Quickstart copies `.env.example` to `.env` and brings the stack
  up under Compose.
- Two provider configurations, each one API key plus a model and a base URL:
  - Summaries: `SUMMARIES_API_KEY`, `SUMMARIES_MODEL`, `SUMMARIES_BASE_URL`.
    `.env.example` defaults to `https://openrouter.ai/api/v1` with model
    `google/gemini-2.5-flash`.
  - Embeddings: `EMBEDDINGS_API_KEY`, `EMBEDDINGS_MODEL`, `EMBEDDINGS_BASE_URL`.
    `.env.example` defaults to `https://api.openai.com/v1` with model
    `text-embedding-3-large`.
  Only the two API keys are secrets with no default. The model and base URL
  settings are ordinary configuration. Requests made against the two default
  providers are metered and billable.
- `.env.example` also carries a separate `OPENAI_API_KEY` entry.
- `EMBEDDINGS_API_KEY` is validated at startup, not lazily:
  `ingestion/src/api/config.py:100-106` raises `ConfigError` on an empty value,
  so the API refuses to boot without it. Any local-provider substitution must
  still supply a non-empty token.
- A GitHub PAT is optional, for repo metadata calls and rate limits.
- Summarisation runs per definition across the whole dependency graph, so
  ingestion cost scales with repository size. The documentation quotes no figure
  for a self-ingest, so the spend cannot be bounded from documentation alone.
