---
id: src.graphify
file: https://github.com/Graphify-Labs/graphify
verification: external
type: tool
date: 2026-07-13
---

# Graphify: agent-skill install surface across 20+ harnesses

Python CLI (PyPI package `graphifyy`, command `graphify`, MIT) that ships an
AI-assistant skill into 20+ platforms (Claude Code, Codex, Cursor, Gemini CLI,
Copilot, OpenCode, Kiro, Pi, and more). README verified directly 2026-07-13;
all claims below are README/tree facts.

Load-bearing findings for cairn's pack design:

- **Generic verb with a platform selector**: `graphify install
  [--platform <name>] [--project]`, defaulting to Claude Code with platform
  auto-detection. `--project` scopes the install to the current repo
  (`.claude/skills/graphify/SKILL.md` or `.agents/skills/graphify/SKILL.md`
  plus a `references/` sidecar) and prints a `git add` hint.
- **Surface duplication anti-pattern, visible in the README**: alongside the
  generic verb, ~22 per-platform command families coexist (`graphify claude
  install`, `graphify codex install`, `graphify cursor install`, each with
  `uninstall`). Two ways to do everything; the instruction surface grows with
  every platform. Cairn adopts the generic verb only.
- **Adapter differences are real, structural, and small**: hook platforms
  (Claude Code, Gemini) get PreToolUse-style hooks; instruction-file platforms
  (Codex, Cursor, OpenCode) get persistent instruction files (`AGENTS.md`,
  `.cursor/rules/*.mdc`). Per-platform always-on fragments ship as data files
  in the package (`graphify/always_on/agents-md.md`, `claude-md.md`,
  `gemini-md.md`, `kiro-steering.md`, `vscode-instructions.md`).
- **Cross-framework target**: `--platform agents` (alias `skills`) writes the
  Agent-Skills spec locations (`~/.agents/skills/`, `./.agents/skills/`) for
  any spec-compliant framework.
- **Refresh-on-upgrade pattern**: `graphify hook install` embeds the current
  interpreter path into hook scripts and must be re-run after upgrade; global
  `graphify uninstall` (with `--purge`) plus per-platform uninstalls handle
  retirement.
