---
id: res.agent-pack-packaging-survey
nodes:
  - cairn.kernel.cli
sources: [src.graphify, src.superpowers-plugin, src.agentskills-spec]
method: secondary
date: 2026-07-13
---

# Agent skill packaging: cross-ecosystem survey

Two scout passes (2026-07-13) surveyed how developer CLIs package assets into
user repositories and detect drift afterwards: Claude Code plugins
(superpowers), shadcn/ui, pre-commit, lefthook, cargo-dist, Prisma, openspec,
and graphify (the closest precedent). Conducted to inform the agent-pack
packaging decision for cairn's shipped skills.

Verification boundary: the three typed sources above were inspected directly.
Claims about shadcn, pre-commit, lefthook, cargo-dist, Prisma, and openspec
were verified in-session by scout agents against the upstream repositories and
docs but are not retained as typed sources; treat them as dated survey
observations, re-verify before load-bearing reuse.

## Converged findings

- **Single metadata file at repo root, never per-file frontmatter.** The
  surveyed tools keep their repo-level metadata in one file, though what it
  records varies: shadcn `components.json` and pre-commit configs are
  configuration (not installed-file state); Prisma `migration_lock.toml` is a
  version/provider marker. The full ownership-ledger shape (per-file hashes
  plus install timestamps) is directly evidenced only by this repo's own
  `.agents/.skill-lock.json` (version, per-skill `skillFolderHash`,
  `installedAt`/`updatedAt`, `source`), which is the precedent the manifest
  contract actually rests on.
- **Tool version and bundle version are separate.** pre-commit
  (`minimum_pre_commit_version` vs per-hook `rev`) and cargo-dist
  (`cargo-dist-version` vs manifest `dist_version`) split them; shadcn
  conflates them and regrets it: `shadcn@latest` silently changes fetched
  component code with no pinning in `components.json` and no way to tell which
  version of a component was installed.
- **Drift checks run every time and warn, never block** (pre-commit's model;
  the strongest single precedent for the every-run comparison).
- **Migration notes ship compiled into the binary** keyed by version range,
  matching cairn's `include_str!` zero-fetch posture.
- **Per-skill version frontmatter is not a thing** (agentskills.io: version is
  pack-level).
- **Three-way file handling**: pristine (hash matches manifest) is overwritten
  on update; modified is reported, never overwritten; missing is backfilled.
  Removal is retire-if-pristine. A directory that exists without a manifest is
  a legacy install: scan, match against the bundled pack, write the manifest.
- **Vendor and user directories stay separate** across ecosystems. Claude Code
  discovery expects flat `.claude/skills/<name>/SKILL.md`; nested vendor
  subdirectories are unverified against live discovery, so the
  manifest-as-ownership-ledger (touch only manifest-listed files at matching
  hashes) is what delivers clobber-safety, regardless of directory layout.
- **Graphify is the closest precedent** (README verified, see src.graphify):
  one generic install verb with `--platform` selector and auto-detection,
  project-vs-user scoping, per-platform always-on fragments shipped as data
  files, and a hook-platform vs instruction-file-platform adapter split. Its
  README also shows the surface-duplication anti-pattern to avoid: ~22
  per-platform command families coexisting with the generic verb. Earlier
  session notes attributed `platforms.toml`, `.graphify_version`, and golden
  trees to this repo; those paths are NOT verifiable on main and were dropped.
