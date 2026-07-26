---
id: dec.pack-adapter-roots
nodes:
  - cairn.kernel.cli
status: proposed
date: 2026-07-26
informed_by:
  - res.pack-omp-adapter-validation
refines:
  - dec.agent-pack-packaging
---
# An adapter is a pack root, and one install owns one adapter

## Context

`dec.agent-pack-packaging` clause 2 makes adapters pure functions from the
canonical pack to emitted file sets, with per-harness structure held as data.
Adding the second adapter (OMP) forces three questions the Claude-only
implementation never had to answer: what actually differs between harnesses,
where that difference is declared, and which adapter a later verb acts on when
the selector and the ledger disagree.

Measured against a live OMP 17.1.3 host
(`res.pack-omp-adapter-validation`), the only structural difference is the
project directory a harness scans: Claude reads `.claude/`, OMP's `native`
provider reads `<project>/.omp/skills/<name>/SKILL.md` and
`<project>/.omp/commands/*.md`. Skill and command bodies need no per-harness
text once their two Claude-specific path references are written harness
neutrally.

## Decision

1. An adapter is a pack root. `tools/agent-pack/manifest.toml` declares one
   `[[adapters]]` row per harness per entry, and an OMP row is the Claude row
   rooted at `.omp/`. The bundled bodies are identical bytes for both.
2. A render targets exactly one harness. `cairn-agent-pack --harness <name>`
   (default `claude`) renders that harness and fails closed on a harness the
   manifest never declares. This repository checks in only its own harness
   tree, so the shipped CLI carries one copy of the bytes and roots them per
   harness at install time. A test asserts the runtime roots reproduce the
   manifest's OMP rows exactly, so the two descriptions cannot drift.
3. One install owns one adapter. Every verb after `install` acts on the harness
   the ownership ledger records. An explicit `--harness` that disagrees with the
   ledger is refused; a `--harness` with no value is a usage error; with no
   ledger yet, a first install detects the host (an OMP project directory with
   no Claude tree installs the OMP adapter, everything else installs Claude).
4. A campaign resolves under the pack root of the harness its own ledger and
   snapshot name, never a root the caller supplied.

## Rationale

Rendering both harness trees into this repository was the obvious reading of
clause 1, and it is wrong here twice over: `.omp/` is device-local and
untracked in this repository, and the copies would be byte-identical, doubling
the checked-in generated surface and the compiled binary for no behaviour. The
manifest keeps its authority over adapter data; only the rendering target
narrows, and the drift gate moves from "the file is checked in" to "the runtime
mapping equals the declared rows".

Binding later verbs to the ledger is what makes a second harness safe at all.
Without it, a bare `status` on an OMP install reports every Claude destination
missing, and a bare `update` writes a second tree into a ledger that claims a
single harness.

## Consequences

- Adding a third harness is a manifest row plus a table row plus its live
  validation, with no new lifecycle code.
- A project that genuinely wants both trees must uninstall and reinstall; that
  is deliberate, because one ledger describes one adapter.
- `cairn init --wire` now detects the host rather than always writing `.claude/`.
- This decision refines `dec.agent-pack-packaging` and contradicts none of its
  clauses: it fixes implementation scope the packaging decision left open.
