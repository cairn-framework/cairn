# Cairn Error Code Registry

This file is the single source of truth for all allocated Cairn error codes. Every error code that appears in Rust source MUST have an entry here.

## Format

All codes follow the pattern **`CXNNN`**:

- **C** -- literal prefix (Cairn).
- **X** -- category letter (see table below).
- **NNN** -- zero-padded three-digit sequence number, allocated sequentially within each category starting at 001.

## Categories

| Letter | Subsystem        |
|--------|------------------|
| P      | Parser (blueprint)     |
| K      | Kernel/Map  |
| A      | Artefacts        |
| C      | Changes          |
| H      | Hooks            |
| E      | Edges            |
| T      | Targets          |
| M      | MCP              |
| S      | Summariser       |
| B      | Brownfield       |
| D      | Distribution     |
| O      | CLI output / I/O |
| L      | LSP / Language Server |

## Rules

1. Read this file before adding any code.
2. Append new codes to the appropriate category section below.
3. Never reuse, reassign, or renumber a code once it appears here.
4. Each entry: `CXNNN` -- one-line description -- phase that introduced it.

---

## CP -- Parser

_No codes allocated yet._

## CK -- Kernel/Map

- CK001 -- scanner failed to load project -- phase-7.8 reforge
- CK002 -- blueprint path matches a .gitignore pattern (CAIRN_PATH_GITIGNORED) -- issue #45
- CK003 -- leaf node owns code but declares no contract (CAIRN_CONTRACT_LEAF_UNCOVERED) -- bead cairn-481
- CK004 -- designed spec rule has no emitting enforcer in non-test source (CAIRN_SPEC_RULE_UNIMPLEMENTED) -- bead cairn-iy2

## CA -- Artefacts

- CA001 -- leaf node has no decision artefact (CAIRN_PROVENANCE_NO_DECISION) -- issue #46
- CA003 -- decision artefact exhaustive file claim does not match folder contents -- issue #67
- CA002 -- blueprint shape changed for node but no decision artefact covers it (CAIRN_BLUEPRINT_CHANGE_NO_DECISION) -- issue #68
- CA004 -- decision prose claims to close a spec open-question the registry still lists as unresolved (CAIRN_DECISION_CLAIM_UNRESOLVED) -- cairn-zad
- CA005 -- research artefact not linked from any decision (CAIRN_RESEARCH_ORPHAN) -- bead cairn-ay5

## CC -- Changes

- CC001 -- verification blocked by upstream dependency -- phase-7.5c
- CC002 -- pending suggested-edges entries block --strict validate -- phase-7.6
- CC003 -- failed to enumerate changes directory -- phase-7.8 reforge cycle 3

## CH -- Hooks

- CH001 -- blueprint architectural mutation lacks paired decision artefact -- issue #68
- CH002 -- synced module lacks test coverage (CAIRN_TEST_COVERAGE_MISSING) -- change `cairn-test-coverage-gate`

## CE -- Edges

- CE001 -- Declared blueprint edge has no observed source dependency -- Phase 5
- CE002 -- Observed source dependency has no declared blueprint edge -- Phase 5
- CE003 -- Observed source dependency is ambiguous between multiple node owners -- Phase 5
- CE004 -- Docstring fact references an unknown Cairn node ID -- Phase 5
- CE005 -- Docstring node name contradicts the map -- Phase 5
- CE006 -- Docstring dependency contradicts declared graph edges -- Phase 5
- CE007 -- Docstring tags contradict the map -- Phase 5
- CE008 -- Docstring contains an unknown Cairn fact key -- Phase 5
- CE009 -- Docstring contract pointer contradicts the map -- Phase 5
- CE010 -- Requested docstring language is unsupported -- Phase 5

## CT -- Targets

CT001 -- interface contradiction: multiple targets claim same contract role with divergent interfaces -- phase-6
CT002 -- rationale tension: intentional asymmetry flagged for human review -- phase-6

## CM -- MCP

_No codes allocated yet._

## CS -- Summariser

_No codes allocated yet._

## CB -- Brownfield

_No codes allocated yet._

## CD -- Distribution

_No codes allocated yet._

## CO -- CLI output / I/O

- CO001 -- failed to write CLI output to disk -- phase-7.8 reforge cycle 4

## CL -- LSP / Language Server

- CL001 -- LSP protocol or transport error -- cairn-d7s
