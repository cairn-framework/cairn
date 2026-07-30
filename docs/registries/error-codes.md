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

- CP001 -- blueprint file uses legacy `.dsl` extension, no longer accepted (CAIRN_BLUEPRINT_LEGACY_EXTENSION) -- pre-registry provenance unknown -- audited 2026-07-16
- CP002 -- blueprint file could not be read from disk (CAIRN_IO_READ_BLUEPRINT) -- pre-registry provenance unknown -- audited 2026-07-16
- CP003 -- parser encountered an unexpected token (CAIRN_PARSE_UNEXPECTED_TOKEN) -- pre-registry provenance unknown -- audited 2026-07-16
- CP004 -- lexer hit an unterminated string literal (CAIRN_PARSE_UNTERMINATED_STRING) -- pre-registry provenance unknown -- audited 2026-07-16

## CK -- Kernel/Map

- CK001 -- scanner failed to load project -- phase-7.8 reforge
- CK002 -- blueprint path matches a .gitignore pattern (CAIRN_PATH_GITIGNORED) -- issue #45
- CK003 -- leaf node owns code but declares no contract (CAIRN_CONTRACT_LEAF_UNCOVERED) -- bead cairn-481
- CK004 -- designed spec rule has no emitting enforcer in non-test source (CAIRN_SPEC_RULE_UNIMPLEMENTED) -- bead cairn-iy2
- CK005 -- backlog bead references an unknown node via its cairn-node label (CAIRN_BACKLOG_ORPHAN_NODE) -- pre-registry provenance unknown -- audited 2026-07-16
- CK006 -- failed to read cairn.config.yaml (CAIRN_CONFIG_READ_FAILED) -- pre-registry provenance unknown -- audited 2026-07-16
- CK007 -- cairn.config.yaml declares an unknown configuration key (CAIRN_CONFIG_UNKNOWN_KEY) -- pre-registry provenance unknown -- audited 2026-07-16
- CK008 -- contract declares an interface entry not found among the node's extracted symbols (CAIRN_CONTRACT_INTERFACE_DRIFT) -- pre-registry provenance unknown -- audited 2026-07-16
- CK009 -- node declares a contract role with no contract artefact covering it (CAIRN_CONTRACT_MISSING) -- pre-registry provenance unknown -- audited 2026-07-16
- CK010 -- failed to read a `.cairnignore` file (CAIRN_IGNORE_READ_FAILED) -- pre-registry provenance unknown -- audited 2026-07-16
- CK011 -- two nodes declare the same node id (CAIRN_INTEGRITY_DUPLICATE_ID) -- pre-registry provenance unknown -- audited 2026-07-16
- CK012 -- an edge references a node id that does not exist (CAIRN_INTEGRITY_INVALID_EDGE_ENDPOINT) -- pre-registry provenance unknown -- audited 2026-07-16
- CK013 -- a node id fails blueprint id validation (CAIRN_INTEGRITY_INVALID_ID) -- pre-registry provenance unknown -- audited 2026-07-16
- CK014 -- two nodes claim the same file path (CAIRN_INTEGRITY_PATH_TIE) -- pre-registry provenance unknown -- audited 2026-07-16
- CK015 -- containment or dependency constraints form a cycle (CAIRN_ORDER_CYCLE) -- pre-registry provenance unknown -- audited 2026-07-16
- CK016 -- reconciled target discovered zero files (CAIRN_RECONCILE_EMPTY_TARGET) -- pre-registry provenance unknown -- audited 2026-07-16
- CK017 -- Go tree-sitter grammar failed to load (CAIRN_RECONCILE_GO_LANGUAGE) -- pre-registry provenance unknown -- audited 2026-07-16
- CK018 -- target has no declared or inferable language (CAIRN_RECONCILE_LANGUAGE_UNKNOWN) -- pre-registry provenance unknown -- audited 2026-07-16
- CK019 -- source file is not owned by any eligible node (CAIRN_RECONCILE_ORPHANED_FILE) -- pre-registry provenance unknown -- audited 2026-07-16
- CK020 -- tree-sitter failed to parse a Go source file (CAIRN_RECONCILE_PARSE_GO) -- pre-registry provenance unknown -- audited 2026-07-16
- CK021 -- tree-sitter failed to parse a Python source file (CAIRN_RECONCILE_PARSE_PYTHON) -- pre-registry provenance unknown -- audited 2026-07-16
- CK022 -- tree-sitter failed to parse a Rust source file (CAIRN_RECONCILE_PARSE_RUST) -- pre-registry provenance unknown -- audited 2026-07-16
- CK023 -- tree-sitter failed to parse a TypeScript source file (CAIRN_RECONCILE_PARSE_TYPESCRIPT) -- pre-registry provenance unknown -- audited 2026-07-16
- CK024 -- Python tree-sitter grammar failed to load (CAIRN_RECONCILE_PYTHON_LANGUAGE) -- pre-registry provenance unknown -- audited 2026-07-16
- CK025 -- failed to read a reconciled source directory (CAIRN_RECONCILE_READ_DIR) -- pre-registry provenance unknown -- audited 2026-07-16
- CK026 -- failed to read a directory entry while reconciling (CAIRN_RECONCILE_READ_DIR_ENTRY) -- pre-registry provenance unknown -- audited 2026-07-16
- CK027 -- failed to read a reconciled source file (CAIRN_RECONCILE_READ_SOURCE) -- pre-registry provenance unknown -- audited 2026-07-16
- CK028 -- Rust tree-sitter grammar failed to load (CAIRN_RECONCILE_RUST_LANGUAGE) -- pre-registry provenance unknown -- audited 2026-07-16
- CK029 -- TypeScript tree-sitter grammar failed to load (CAIRN_RECONCILE_TYPESCRIPT_LANGUAGE) -- pre-registry provenance unknown -- audited 2026-07-16
- CK030 -- workspace member's root or blueprint failed to load (CAIRN_WORKSPACE_MEMBER_MISSING) -- pre-registry provenance unknown -- audited 2026-07-16
- CK031 -- claimed source/asset file exceeds the 500-line module-size guideline with no allow-list marker (CAIRN_MODULE_OVERSIZED) -- todo.modularity-scan-finding
- CK032 -- spec rule defers to a decision that is not accepted: missing, proposed, deprecated, or superseded (CAIRN_SPEC_RULE_DEFERRED_DECISION_INVALID) -- todo.spec-rule-deferred-cell-check
- CK033 -- contract has not been reviewed against its node's current declared shape (CAIRN_CONTRACT_NODE_SHAPE_DRIFT) -- todo.contract-blueprint-staleness

## CA -- Artefacts

- CA001 -- leaf node has no decision artefact (CAIRN_PROVENANCE_NO_DECISION) -- issue #46
- CA003 -- decision artefact exhaustive file claim does not match folder contents -- issue #67
- CA002 -- blueprint shape changed for node but no decision artefact covers it (CAIRN_BLUEPRINT_CHANGE_NO_DECISION) -- issue #68
- CA004 -- decision prose claims to close a spec open-question the registry still lists as unresolved (CAIRN_DECISION_CLAIM_UNRESOLVED) -- cairn-zad
- CA005 -- research artefact not linked from any decision (CAIRN_RESEARCH_ORPHAN) -- bead cairn-ay5
- CA006 -- failed to read an artefact directory named by a registry pointer (CAIRN_ARTEFACT_DIR_READ_FAILED) -- pre-registry provenance unknown -- audited 2026-07-16
- CA007 -- artefact file is missing a required frontmatter field (CAIRN_ARTEFACT_MISSING_FIELD) -- pre-registry provenance unknown -- audited 2026-07-16
- CA008 -- research artefact declares no `nodes` (CAIRN_ARTEFACT_MISSING_NODES) -- pre-registry provenance unknown -- audited 2026-07-16
- CA009 -- artefact registry pointer names a path with no file on disk (CAIRN_ARTEFACT_POINTER_MISSING) -- pre-registry provenance unknown -- audited 2026-07-16
- CA010 -- failed to read an artefact file named by a registry pointer (CAIRN_ARTEFACT_READ_FAILED) -- pre-registry provenance unknown -- audited 2026-07-16
- CA011 -- research artefact's `nodes` references an unknown node (CAIRN_ARTEFACT_UNKNOWN_NODE) -- pre-registry provenance unknown -- audited 2026-07-16
- CA012 -- contract artefact lacks node frontmatter (CAIRN_CONTRACT_MISSING_NODE) -- pre-registry provenance unknown -- audited 2026-07-16
- CA013 -- failed to read a contract artefact (CAIRN_CONTRACT_READ_FAILED) -- pre-registry provenance unknown -- audited 2026-07-16
- CA014 -- contract artefact references an unknown node (CAIRN_CONTRACT_UNKNOWN_NODE) -- pre-registry provenance unknown -- audited 2026-07-16
- CA015 -- contract artefact declared by one node references a different node (CAIRN_CONTRACT_WRONG_NODE) -- pre-registry provenance unknown -- audited 2026-07-16
- CA016 -- decision artefact declares no `nodes` (CAIRN_DECISION_MISSING_NODES) -- pre-registry provenance unknown -- audited 2026-07-16
- CA017 -- decision references only unknown nodes and lacks an explicit orphan reason (CAIRN_DECISION_ORPHANED) -- pre-registry provenance unknown -- audited 2026-07-16
- CA018 -- decision references an unknown `supersedes`, `refines`, or `related` target (CAIRN_DECISION_REFERENCE_UNKNOWN) -- pre-registry provenance unknown -- audited 2026-07-16
- CA019 -- decision artefact frontmatter declares an invalid `status` (CAIRN_DECISION_STATUS_INVALID) -- pre-registry provenance unknown -- audited 2026-07-16
- CA020 -- decision that supersedes another has a status inconsistent with that relationship (CAIRN_DECISION_SUPERSEDES_STATUS) -- pre-registry provenance unknown -- audited 2026-07-16
- CA021 -- decision references provenance that does not resolve to a known artefact (CAIRN_DECISION_UNKNOWN_PROVENANCE) -- pre-registry provenance unknown -- audited 2026-07-16
- CA022 -- decision's gap is not marked resolved (CAIRN_GAP_UNRESOLVED) -- pre-registry provenance unknown -- audited 2026-07-16
- CA023 -- research artefact declares an invalid `method` (CAIRN_RESEARCH_METHOD_INVALID) -- pre-registry provenance unknown -- audited 2026-07-16
- CA024 -- research artefact declares no `sources` (CAIRN_RESEARCH_MISSING_SOURCES) -- pre-registry provenance unknown -- audited 2026-07-16
- CA025 -- research artefact's `sources` references an unknown source (CAIRN_RESEARCH_UNKNOWN_SOURCE) -- pre-registry provenance unknown -- audited 2026-07-16
- CA026 -- review artefact frontmatter declares an invalid `review_type` (CAIRN_REVIEW_TYPE_INVALID) -- pre-registry provenance unknown -- audited 2026-07-16
- CA027 -- review artefact references an unknown node (CAIRN_REVIEW_UNKNOWN_NODE) -- pre-registry provenance unknown -- audited 2026-07-16
- CA028 -- external source's path is not a URL (CAIRN_SOURCE_EXTERNAL_URL) -- pre-registry provenance unknown -- audited 2026-07-16
- CA029 -- validator received an indexed source id absent from the loaded source artefacts; production derives both from the same set, so this currently fires only under test-supplied indexes (CAIRN_SOURCE_INDEX_GAP) -- pre-registry provenance unknown -- audited 2026-07-16
- CA030 -- source artefact is not referenced by any research `sources` or decision `informed_by` (CAIRN_SOURCE_ORPHAN) -- pre-registry provenance unknown -- audited 2026-07-16
- CA031 -- source path resolution failed: a verified source could not be read, or a tracked source's path does not resolve inside the repository root (CAIRN_SOURCE_READ_FAILED) -- pre-registry provenance unknown; generalised by todo.source-tracked-verification-mode -- audited 2026-07-16
- CA032 -- verified source's sha256 does not match its recorded digest (CAIRN_SOURCE_SHA256_MISMATCH) -- pre-registry provenance unknown -- audited 2026-07-16
- CA033 -- verified source declares no sha256 digest (CAIRN_SOURCE_SHA256_MISSING) -- pre-registry provenance unknown -- audited 2026-07-16
- CA034 -- source lacks the verification metadata required for its kind (CAIRN_SOURCE_UNVERIFIED) -- pre-registry provenance unknown -- audited 2026-07-16
- CA035 -- source's `verification` frontmatter is malformed (CAIRN_SOURCE_VERIFICATION_INVALID) -- pre-registry provenance unknown -- audited 2026-07-16
- CA036 -- todo artefact declares an unknown `node` (CAIRN_TODO_ORPHAN_NODE) -- pre-registry provenance unknown -- audited 2026-07-16
- CA037 -- todo artefact frontmatter declares an invalid `status` (CAIRN_TODO_STATUS_INVALID) -- pre-registry provenance unknown -- audited 2026-07-16
- CA038 -- artefact filename does not follow the naming rule for its kind: id-derived for decisions, research, and sources, `todo.<slug>.md` for todos (CAIRN_ARTEFACT_FILENAME_DRIFT) -- todo.artefact-filename-convention
- CA039 -- node carries more accepted decisions than the consolidation threshold (CAIRN_DECISION_ACCUMULATION) -- todo.decision-accumulation-finding
- CA040 -- tracked source declares a sha256 digest (CAIRN_SOURCE_SHA256_UNEXPECTED) -- todo.source-tracked-verification-mode
- CA041 -- todo `defers:` reference matches no emitted finding (CAIRN_TODO_DEFERS_UNMATCHED) -- todo.lint-selection-folding
- CA042 -- todo `defers:` reference targets an Error or Warning finding (CAIRN_TODO_DEFERS_BLOCKING) -- todo.lint-selection-folding
- CA043 -- todo `defers:` frontmatter entry is malformed (CAIRN_TODO_DEFERS_INVALID) -- todo.lint-selection-folding
- CA044 -- source's `file:` resolves to the source artefact itself, under any verification value (CAIRN_SOURCE_SELF_REFERENCE) -- todo.source-self-reference-finding

## CC -- Changes

- CC001 -- verification blocked by upstream dependency -- phase-7.5c
- CC002 -- pending suggested-edges entries block --strict validate -- phase-7.6
- CC003 -- failed to enumerate changes directory -- phase-7.8 reforge cycle 3
- CC004 -- active change has all tasks complete (CAIRN_CHANGE_TASKS_COMPLETE) -- change tasks-complete finding
- CC005 -- failed to enumerate the changes directory (CAIRN_CHANGES_DISCOVERY_FAILED) -- pre-registry provenance unknown -- audited 2026-07-16
- CC006 -- two active changes both target the same artefact path (CAIRN_CHANGE_ARTEFACT_CONFLICT) -- pre-registry provenance unknown -- audited 2026-07-16
- CC007 -- two active changes both target the same blueprint path (CAIRN_CHANGE_BLUEPRINT_CONFLICT) -- pre-registry provenance unknown -- audited 2026-07-16
- CC008 -- requested change id was not found (CAIRN_CHANGE_NOT_FOUND) -- pre-registry provenance unknown -- audited 2026-07-16
- CC009 -- two active changes both rename the same path (CAIRN_CHANGE_RENAME_CONFLICT) -- pre-registry provenance unknown -- audited 2026-07-16
- CC010 -- failed to list change drafts (CAIRN_DRAFTS_LIST_FAILED) -- pre-registry provenance unknown -- audited 2026-07-16
- CC011 -- failed to accept a change draft (CAIRN_DRAFT_ACCEPT_FAILED) -- pre-registry provenance unknown -- audited 2026-07-16
- CC012 -- change draft transition is not valid from its current state (CAIRN_DRAFT_INVALID_TRANSITION) -- pre-registry provenance unknown -- audited 2026-07-16
- CC013 -- requested change draft was not found (CAIRN_DRAFT_NOT_FOUND) -- pre-registry provenance unknown -- audited 2026-07-16

## CH -- Hooks

- CH001 -- blueprint architectural mutation lacks paired decision artefact -- issue #68
- CH002 -- synced module lacks test coverage (CAIRN_TEST_COVERAGE_MISSING) -- change `cairn-test-coverage-gate`
- CH003 -- recorded interface hash differs from the interface's current state (CAIRN_INTERFACE_HASH_CHANGED) -- pre-registry provenance unknown -- audited 2026-07-16

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

- CM001 -- MCP request named an unknown method (CAIRN_MCP_METHOD_NOT_FOUND) -- pre-registry provenance unknown -- audited 2026-07-16
- CM002 -- MCP tool call is missing the required `params.name` (CAIRN_MCP_MISSING_TOOL) -- pre-registry provenance unknown -- audited 2026-07-16
- CM003 -- MCP request body failed to parse as JSON-RPC (CAIRN_MCP_PARSE_ERROR) -- pre-registry provenance unknown -- audited 2026-07-16
- CM004 -- fallback code for a findings-derived query error when the findings list is empty (CAIRN_QUERY_FINDINGS) -- pre-registry provenance unknown -- audited 2026-07-16
- CM005 -- hooks-architecture query received an unknown hook kind (CAIRN_QUERY_INVALID_HOOK_KIND) -- pre-registry provenance unknown -- audited 2026-07-16
- CM006 -- mutating query tool called without the mutating flag set (CAIRN_QUERY_MUTATION_NOT_ALLOWED) -- pre-registry provenance unknown -- audited 2026-07-16
- CM007 -- graph node lookup found no match, or found an ambiguous match (CAIRN_QUERY_NODE_NOT_FOUND) -- pre-registry provenance unknown -- audited 2026-07-16
- CM008 -- query tool is registered but has no implementation (CAIRN_QUERY_UNIMPLEMENTED_TOOL) -- pre-registry provenance unknown -- audited 2026-07-16
- CM009 -- query request named an unknown tool (CAIRN_QUERY_UNKNOWN_TOOL) -- pre-registry provenance unknown -- audited 2026-07-16
- CM010 -- required query node parameter is missing (CAIRN_QUERY_MISSING_NODE) -- pre-registry provenance unknown -- audited 2026-07-17
- CM011 -- required query change parameter is missing (CAIRN_QUERY_MISSING_CHANGE) -- pre-registry provenance unknown -- audited 2026-07-17
- CM012 -- required query old-id parameter is missing (CAIRN_QUERY_MISSING_OLD_ID) -- pre-registry provenance unknown -- audited 2026-07-17
- CM013 -- required query new-id parameter is missing (CAIRN_QUERY_MISSING_NEW_ID) -- pre-registry provenance unknown -- audited 2026-07-17
- CM014 -- required query symbol parameter is missing (CAIRN_QUERY_MISSING_SYMBOL) -- pre-registry provenance unknown -- audited 2026-07-17
- CM015 -- pending query found a proposed decision whose `date:` does not parse as YYYY-MM-DD (CAIRN_PENDING_INVALID_DATE) -- todo.maintainer-pending-queue -- added 2026-07-30

## CS -- Summariser

- CS001 -- summariser backend configuration is invalid (CAIRN_SUMMARISER_CONFIG_ERROR) -- pre-registry provenance unknown -- audited 2026-07-16
- CS002 -- summariser query called while the summariser is disabled (CAIRN_SUMMARISER_DISABLED) -- pre-registry provenance unknown -- audited 2026-07-16
- CS003 -- summariser backend failed to generate a response (CAIRN_SUMMARISER_GENERATION_FAILED) -- pre-registry provenance unknown -- audited 2026-07-16
- CS004 -- summariser prompt construction failed (CAIRN_SUMMARISER_PROMPT_ERROR) -- pre-registry provenance unknown -- audited 2026-07-16

## CB -- Brownfield

_No codes allocated yet._

## CD -- Distribution

_No codes allocated yet._

## CO -- CLI output / I/O

- CO001 -- failed to write CLI output to disk -- phase-7.8 reforge cycle 4
- CO002 -- CLI command invoked without a required change id argument (CAIRN_CLI_MISSING_CHANGE) -- pre-registry provenance unknown -- audited 2026-07-16
- CO003 -- CLI command invoked without a required node argument (CAIRN_CLI_MISSING_NODE) -- pre-registry provenance unknown -- audited 2026-07-16
- CO004 -- generic CLI command failure (invalid arguments, scan, workspace, or archive errors) (CAIRN_COMMAND_FAILED) -- pre-registry provenance unknown -- audited 2026-07-16
- CO005 -- scan ran with no `cairn.blueprint` present (CAIRN_NO_BLUEPRINT) -- pre-registry provenance unknown -- audited 2026-07-16
- CO006 -- local UI server failed to load the requested project (CAIRN_UI_PROJECT_LOAD_FAILED) -- pre-registry provenance unknown -- audited 2026-07-16
- CO007 -- CLI locate command was invoked without a symbol argument (CAIRN_CLI_MISSING_SYMBOL) -- pre-registry provenance unknown -- audited 2026-07-17

## CL -- LSP / Language Server

- CL001 -- LSP protocol or transport error -- cairn-d7s
