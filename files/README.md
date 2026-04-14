# Cairn — handoff bundle (v0.6)

Everything needed to start implementation. Paste into a fresh repo, initialise OpenSpec, and point Claude Code at it.

## What changed in v0.6

Scope correction. Earlier drafts conflated "deferred to v2" with "not part of the product." v0.6 separates capability scope (what Cairn IS) from implementation phasing (what ships first).

Brought back in as v1 capabilities (implemented in later phases):

- Brownfield extraction (`cairn init --from-code`, `cairn refine`) — final phase
- Multi-target modules (`path` accepts a list)
- Edge validation (reconciler compares declared edges against observed dependencies)
- Docstring generation and drift detection (`cairn docstring <node>`)
- `cairn rename` operation with full propagation
- Agent review subtypes (`human`, `agent_introspective`, `agent_cross_model`)
- `cairn status` composed query (replaces need for a session-state artefact)

Introduced **spec maturity levels**: Declared, Designed, Implemented. Some sections are specified at Declared level only (brownfield, docstring generation, agent review subtypes). These are part of v1 but details land when implementation gets closer.

## Files

**`cairn-spec-v0.6.md`** — The specification. Load-bearing document. Read end to end once before implementation. ~8,000 words.

**`cairn-bootstrap.tar.gz`** — Cairn described as a Cairn project. Same structure as v0.5.1 (still valid under v0.6). Contains the framework's own DSL, ADRs, research, sources, and a completed change directory documenting the v0.5 → v0.5.1 adoption. Use as test fixture for the parser's self-hosting test.

**`cairn-mvp-change.tar.gz`** — OpenSpec-format change proposal for the first implementation, updated for v0.6 scope:
- Five CLI commands in scope: `get`, `neighbourhood`, `dependents`, `depends`, `order`
- Multi-target path handling in the parser
- Explicit phase 1 positioning in the proposal
- Updated task list and capability specs

## How to use

```
# 1. Create the implementation repo
mkdir cairn && cd cairn
git init

# 2. Install OpenSpec and initialise
npm install -g @fission-ai/openspec@latest
openspec init
# Select Claude Code from the tool list

# 3. Drop in the change proposal
tar xzf /path/to/cairn-mvp-change.tar.gz
mv cairn-mvp/openspec/changes/cairn-kernel-mvp openspec/changes/
rm -rf cairn-mvp

# 4. Drop the spec in the repo for context
mkdir -p docs
cp /path/to/cairn-spec-v0.6.md docs/spec.md

# 5. Drop the bootstrap in for the self-hosting test
tar xzf /path/to/cairn-bootstrap.tar.gz
mkdir -p test/fixtures
mv cairn-bootstrap test/fixtures/

# 6. Hand off to Claude Code
# In Claude Code: /opsx:apply cairn-kernel-mvp
```

## Notes for the implementation session

- The spec is the authority. If the MVP proposal and the spec disagree, spec wins.
- The MVP is deliberately narrow: parser, graph, five queries. Every capability in section 5 of the spec eventually lands, but phase 1 is strictly about proving the kernel.
- Success is the side-by-side test against OpenSpec-alone, not just green tests.
- If the build takes longer than two focused days, stop and re-evaluate the spec, not just the code.

## Open questions remaining (section 16)

Five. Only one blocks implementation:

- **Name.** *Cairn* is a placeholder. Decide before the first `git commit`. Everything bakes in the name — package.json, CLI binary, config file naming, docs, repo URL.

The other four (shared utilities convention, todo coverage strictness, meta/ layout, detailed agent review schemas) can all wait.

## Reading order for the spec

If you have 20 minutes:
1. Sections 0, 0.1, 2 (vocabulary, maturity levels, framing)
2. Section 3 (the two chains — core architecture)
3. Section 5 (goals, non-goals, deliberate non-features)
4. Section 14 (phased build order) and section 15 (brownfield)

If you have 60 minutes: read the whole thing. It's written to be read end-to-end.
