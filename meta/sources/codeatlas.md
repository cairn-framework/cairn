---
id: src.codeatlas
file: https://github.com/Aeres-u99/CodeAtlas
verification: external
type: tool
date: 2026-07-16
---

# CodeAtlas: ctags-based symbol index for AI coding agents

Small Go CLI (~1,600 lines, formerly named Hermes) that shells out to
Universal Ctags per file and emits a compressed JSON map whose core is a
flat reverse index: `symbol -> {f: file, l: line}` with `file#name`
qualified keys on collision (`internal/symbols.go:59-75`,
`internal/structs.go`). Repo cloned and read directly 2026-07-16; all
claims below are verified tree facts.

Load-bearing facts for cairn:

- **Symbol-to-location lookup is the whole product.** The prescribed agent
  workflow is `jq -r '.idx["pkg.Symbol"]'` then open `file:line`
  (`.codeatlas/skills/HERMES.md`). Everything else supports that one query.
- **Ships agent playbooks with the tool.** `.codeatlas/skills/` holds a
  master navigation skill (HERMES.md, query recipes plus refinement ladder
  and failure-mode handling), a one-rule guardrail (CAUTION.md: query the
  index, never read the map wholesale), and five task-shaped skills
  (bug investigation, refactoring, architecture discovery, repository
  exploration, feature implementation).
- **Publishes A/B benchmark evidence.** `benchmarks/` holds three
  pinned-SHA reports (Kubernetes, Loki, Terraform) logging per-step agent
  exploration with and without the tool: 50 to 60 percent fewer
  exploration steps, 7 to 50 percent cost reduction, including an honest
  partial-failure case (Terraform, 38-match ambiguous first lookup).
  Method gaps: single self-reported run, no variance, no harness described.
- **Fragile extraction layer.** One ctags subprocess per file in a
  sequential walk; Rust/TS visibility derived by substring-matching the
  ctags pattern text for `pub ` / `export ` (misses `pub(crate)`); the
  `Sig` field is declared but never populated; `.codeatlasignore` is
  resolved against CWD rather than repo root.
- **Compressed output schema.** Single-character JSON keys (`n`/`t`/`l`/`f`)
  because the map is designed to be partially stuffed into agent context.
