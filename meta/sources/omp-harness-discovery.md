---
id: src.omp-harness-discovery
file: archive/strongholds/pack-omp-adapter-validation.md
verification: verified
sha256: 367ff271cf31b19647f2537e8c25f417076669a7e5f984bf165e14a7d6ff688d
type: repository evidence capture
date: 2026-07-26
---

# OMP: where the harness discovers a project's agent assets

OMP (`omp` 17.1.3, installed at `$HOME/.bun/bin/omp`) ships its own
runtime documentation as internal resources. Read directly 2026-07-26 from
`omp://config-usage.md`, `omp://slash-command-internals.md`, and
`omp://skills.md` on the installed binary, then confirmed by running the
harness non-interactively against fresh temporary projects.

Load-bearing facts for the cairn agent pack:

- **Two scopes, different shapes.** Project assets live under `<cwd>/.omp/...`;
  user assets under `~/.omp/agent/...` (`config-usage.md`: "project:
  `<cwd>/.omp/...`", "user: `~/.omp/agent/...`"). A project installer therefore
  targets `.omp/`, never `.omp/agent/`.
- **Skills.** `<ancestor>/.omp/skills/*/SKILL.md`, scanned one level deep from
  the working directory up to the repository or home boundary, plus
  `~/.omp/agent/skills/*/SKILL.md`. Nested `skills/group/<name>/SKILL.md` is not
  discovered by provider loaders. The native `.omp` provider requires a
  `description` in frontmatter (`skills.md`).
- **Slash commands.** `<cwd>/.omp/commands/*.md` for the project, non-recursive,
  and `~/.omp/agent/commands/*.md` for the user; project wins on a name
  collision (`slash-command-internals.md`).
- **Provider precedence.** `native` (`.omp`) at priority 100, then
  `omp-plugins` 90, then `claude` (`.claude/`) 80, deduplicated by name with
  first match winning. OMP does read a Claude-layout pack, at lower precedence
  than its own.
- **Skill bodies are addressed by URL, not by path.** Referenced assets under a
  skill directory are read as `skill://<name>/...`, so a skill's own references
  need no harness-specific path in their bodies.
