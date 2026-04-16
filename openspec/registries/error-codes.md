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
| P      | Parser (DSL)     |
| K      | Kernel/Ontology  |
| A      | Artefacts        |
| C      | Changes          |
| H      | Hooks            |
| E      | Edges            |
| T      | Targets          |
| M      | MCP              |
| S      | Summariser       |
| B      | Brownfield       |
| D      | Distribution     |

## Rules

1. Read this file before adding any code.
2. Append new codes to the appropriate category section below.
3. Never reuse, reassign, or renumber a code once it appears here.
4. Each entry: `CXNNN` -- one-line description -- phase that introduced it.

---

## CP -- Parser

_No codes allocated yet._

## CK -- Kernel/Ontology

_No codes allocated yet._

## CA -- Artefacts

_No codes allocated yet._

## CC -- Changes

_No codes allocated yet._

## CH -- Hooks

_No codes allocated yet._

## CE -- Edges

_No codes allocated yet._

## CT -- Targets

_No codes allocated yet._

## CM -- MCP

_No codes allocated yet._

## CS -- Summariser

_No codes allocated yet._

## CB -- Brownfield

_No codes allocated yet._

## CD -- Distribution

_No codes allocated yet._
