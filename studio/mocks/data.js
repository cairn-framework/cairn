window.CAIRN_STUDIO_DATA = {
  "generated_at": "2026-07-18",
  "source": "frozen-fixture-api",
  "status": {
    "schema_version": 1,
    "nodes": 24,
    "edges": 27,
    "findings": 0,
    "errors": 0,
    "warnings": 0,
    "infos": 0,
    "interface_hash": "2f99a9b48ed94b7b"
  },
  "graph": {
    "schema_version": 1,
    "nodes": [
      {
        "id": "cairn",
        "kind": "system",
        "name": "Cairn",
        "description": "Architecture map framework with pluggable reconcilers",
        "tags": [
          "framework"
        ],
        "parent": null,
        "children": [
          "cairn.root",
          "cairn.sse",
          "cairn.state",
          "cairn.watch",
          "cairn.kernel",
          "cairn.reconcile",
          "cairn.ui",
          "cairn.mcp",
          "cairn.lsp",
          "cairn.macros",
          "cairn.brownfield",
          "cairn.provenance",
          "cairn.suggested-edges",
          "cairn.summariser",
          "cairn.tests"
        ],
        "paths": [],
        "contracts": [],
        "state": "synced",
        "files": []
      },
      {
        "id": "cairn.brownfield",
        "kind": "module",
        "name": "Brownfield",
        "description": "Orphan grouping, candidate heuristics, and onboard analysis",
        "tags": [],
        "parent": "cairn",
        "children": [],
        "paths": [
          "./src/brownfield"
        ],
        "contracts": [],
        "state": "synced",
        "files": [
          "src/brownfield/discovery.rs",
          "src/brownfield/heuristics.rs",
          "src/brownfield/init.rs",
          "src/brownfield/interview.rs",
          "src/brownfield/mod.rs",
          "src/brownfield/onboard.rs",
          "src/brownfield/refine.rs",
          "src/brownfield/suggest.rs",
          "src/brownfield/summarise.rs",
          "src/brownfield/templates.rs"
        ]
      },
      {
        "id": "cairn.kernel",
        "kind": "container",
        "name": "Kernel",
        "description": "Domain-agnostic core",
        "tags": [
          "kernel"
        ],
        "parent": "cairn",
        "children": [
          "cairn.kernel.blueprint",
          "cairn.kernel.artefacts",
          "cairn.kernel.map",
          "cairn.kernel.scanner",
          "cairn.kernel.changes",
          "cairn.kernel.hooks",
          "cairn.kernel.query",
          "cairn.kernel.cli"
        ],
        "paths": [],
        "contracts": [],
        "state": "synced",
        "files": []
      },
      {
        "id": "cairn.kernel.artefacts",
        "kind": "module",
        "name": "Artefacts",
        "description": "Typed artefact registry and contract loader",
        "tags": [],
        "parent": "cairn.kernel",
        "children": [],
        "paths": [
          "./src/artefacts"
        ],
        "contracts": [],
        "state": "synced",
        "files": [
          "src/artefacts/contract.rs",
          "src/artefacts/frontmatter.rs",
          "src/artefacts/mod.rs",
          "src/artefacts/registry/io.rs",
          "src/artefacts/registry/mod.rs",
          "src/artefacts/registry/parse.rs",
          "src/artefacts/registry/sha256.rs",
          "src/artefacts/registry/types.rs",
          "src/artefacts/registry/validate/mod.rs",
          "src/artefacts/registry/validate/tests.rs"
        ]
      },
      {
        "id": "cairn.kernel.blueprint",
        "kind": "module",
        "name": "Blueprint",
        "description": "Parses .blueprint files into AST node graph",
        "tags": [],
        "parent": "cairn.kernel",
        "children": [],
        "paths": [
          "./src/blueprint"
        ],
        "contracts": [],
        "state": "synced",
        "files": [
          "src/blueprint/ast.rs",
          "src/blueprint/error.rs",
          "src/blueprint/lexer.rs",
          "src/blueprint/mod.rs",
          "src/blueprint/parser.rs"
        ]
      },
      {
        "id": "cairn.kernel.changes",
        "kind": "module",
        "name": "Changes",
        "description": "Change directories, delta semantics, and archive",
        "tags": [],
        "parent": "cairn.kernel",
        "children": [],
        "paths": [
          "./src/changes"
        ],
        "contracts": [],
        "state": "synced",
        "files": [
          "src/changes/apply/mod.rs",
          "src/changes/apply/preserve.rs",
          "src/changes/apply/tests.rs",
          "src/changes/artefact_ops.rs",
          "src/changes/delta.rs",
          "src/changes/delta/tests.rs",
          "src/changes/mod.rs",
          "src/changes/rename.rs",
          "src/changes/tests.rs",
          "src/changes/types.rs",
          "src/changes/validate/mod.rs",
          "src/changes/validate/tests.rs"
        ]
      },
      {
        "id": "cairn.kernel.cli",
        "kind": "module",
        "name": "CLI",
        "description": "Primary user surface and output formatting",
        "tags": [],
        "parent": "cairn.kernel",
        "children": [],
        "paths": [
          "./src/cli"
        ],
        "contracts": [],
        "state": "synced",
        "files": [
          "src/cli/accept.rs",
          "src/cli/commands/archive.rs",
          "src/cli/commands/change.rs",
          "src/cli/commands/feedback.rs",
          "src/cli/commands/hook.rs",
          "src/cli/commands/import.rs",
          "src/cli/commands/mod.rs",
          "src/cli/commands/onboard.rs",
          "src/cli/commands/project.rs",
          "src/cli/commands/watch.rs",
          "src/cli/copy.rs",
          "src/cli/export/builder.rs",
          "src/cli/export/json.rs",
          "src/cli/export/markdown.rs",
          "src/cli/export/mermaid.rs",
          "src/cli/export/mod.rs",
          "src/cli/export/runner.rs",
          "src/cli/format/json.rs",
          "src/cli/format/mod.rs",
          "src/cli/format/render.rs",
          "src/cli/format/util.rs",
          "src/cli/mod.rs",
          "src/cli/render/artefacts.rs",
          "src/cli/render/health.rs",
          "src/cli/render/mod.rs",
          "src/cli/render/node.rs",
          "src/cli/render/project.rs",
          "src/cli/render/remediate.rs"
        ]
      },
      {
        "id": "cairn.kernel.hooks",
        "kind": "module",
        "name": "Hooks",
        "description": "Commit and task-boundary enforcement gates",
        "tags": [],
        "parent": "cairn.kernel",
        "children": [],
        "paths": [
          "./src/hooks"
        ],
        "contracts": [],
        "state": "synced",
        "files": [
          "src/hooks/architecture.rs",
          "src/hooks/mod.rs",
          "src/hooks/render.rs",
          "src/hooks/tests.rs"
        ]
      },
      {
        "id": "cairn.kernel.map",
        "kind": "module",
        "name": "Map",
        "description": "Graph construction, integrity checks, and query traversal",
        "tags": [],
        "parent": "cairn.kernel",
        "children": [],
        "paths": [
          "./src/map"
        ],
        "contracts": [],
        "state": "synced",
        "files": [
          "src/map/build.rs",
          "src/map/graph.rs",
          "src/map/integrity.rs",
          "src/map/mod.rs",
          "src/map/query.rs",
          "src/map/test_coverage.rs"
        ]
      },
      {
        "id": "cairn.kernel.query",
        "kind": "module",
        "name": "QueryAPI",
        "description": "Structured query handlers and serialisation",
        "tags": [],
        "parent": "cairn.kernel",
        "children": [],
        "paths": [
          "./src/query_api"
        ],
        "contracts": [],
        "state": "synced",
        "files": [
          "src/query_api/change_queries.rs",
          "src/query_api/handlers/artefacts.rs",
          "src/query_api/handlers/graph.rs",
          "src/query_api/handlers/mod.rs",
          "src/query_api/handlers/node.rs",
          "src/query_api/handlers/project.rs",
          "src/query_api/handlers/remediate.rs",
          "src/query_api/mod.rs",
          "src/query_api/registry.rs",
          "src/query_api/serialise.rs",
          "src/query_api/tests.rs",
          "src/query_api/util.rs"
        ]
      },
      {
        "id": "cairn.kernel.scanner",
        "kind": "module",
        "name": "Scanner",
        "description": "Orchestrates parse, reconcile, and graph-build pipeline",
        "tags": [],
        "parent": "cairn.kernel",
        "children": [],
        "paths": [
          "./src/scanner"
        ],
        "contracts": [],
        "state": "synced",
        "files": [
          "src/scanner/cache.rs",
          "src/scanner/checks.rs",
          "src/scanner/config/mod.rs",
          "src/scanner/config/tests.rs",
          "src/scanner/mod.rs",
          "src/scanner/outputs.rs",
          "src/scanner/state.rs",
          "src/scanner/tests.rs"
        ]
      },
      {
        "id": "cairn.lsp",
        "kind": "module",
        "name": "LSP",
        "description": "LSP diagnostics server for OMP integration",
        "tags": [
          "integration"
        ],
        "parent": "cairn",
        "children": [],
        "paths": [
          "./src/lsp",
          "./src/bin/cairn-lsp.rs"
        ],
        "contracts": [],
        "state": "synced",
        "files": [
          "src/bin/cairn-lsp.rs",
          "src/lsp/diagnostics.rs",
          "src/lsp/mod.rs",
          "src/lsp/server.rs"
        ]
      },
      {
        "id": "cairn.macros",
        "kind": "module",
        "name": "Macros",
        "description": "Proc-macro crate for compile-time attributes",
        "tags": [
          "build",
          "no-test-coverage"
        ],
        "parent": "cairn",
        "children": [],
        "paths": [
          "./cairn-macros"
        ],
        "contracts": [],
        "state": "synced",
        "files": [
          "cairn-macros/src/lib.rs",
          "cairn-macros/tests/planned_attribute.rs"
        ]
      },
      {
        "id": "cairn.mcp",
        "kind": "module",
        "name": "MCP",
        "description": "MCP tool wrapper over query API",
        "tags": [
          "integration"
        ],
        "parent": "cairn",
        "children": [],
        "paths": [
          "./src/mcp",
          "./src/bin/cairn-mcp.rs"
        ],
        "contracts": [],
        "state": "synced",
        "files": [
          "src/bin/cairn-mcp.rs",
          "src/mcp/mod.rs",
          "src/mcp/tests.rs"
        ]
      },
      {
        "id": "cairn.provenance",
        "kind": "module",
        "name": "Provenance",
        "description": "Trace sidecar primitives for the provenance chain",
        "tags": [],
        "parent": "cairn",
        "children": [],
        "paths": [
          "./src/provenance"
        ],
        "contracts": [],
        "state": "synced",
        "files": [
          "src/provenance/mod.rs",
          "src/provenance/trace.rs"
        ]
      },
      {
        "id": "cairn.reconcile",
        "kind": "module",
        "name": "CodeReconciler",
        "description": "Tree-sitter reconciler for Rust, TypeScript, Python, Go",
        "tags": [
          "reconciler",
          "code"
        ],
        "parent": "cairn",
        "children": [],
        "paths": [
          "./src/reconcile"
        ],
        "contracts": [],
        "state": "synced",
        "files": [
          "src/reconcile/code.rs",
          "src/reconcile/fingerprint.rs",
          "src/reconcile/fixture.rs",
          "src/reconcile/go.rs",
          "src/reconcile/mod.rs",
          "src/reconcile/python.rs",
          "src/reconcile/target.rs",
          "src/reconcile/typescript.rs"
        ]
      },
      {
        "id": "cairn.root",
        "kind": "module",
        "name": "Root",
        "description": "Crate entry points, shared error types, and verification",
        "tags": [],
        "parent": "cairn",
        "children": [],
        "paths": [
          "./src/main.rs",
          "./src/lib.rs",
          "./src/error.rs",
          "./src/verification.rs",
          "./src/signal.rs"
        ],
        "contracts": [],
        "state": "synced",
        "files": [
          "src/error.rs",
          "src/lib.rs",
          "src/main.rs",
          "src/signal.rs",
          "src/verification.rs"
        ]
      },
      {
        "id": "cairn.sse",
        "kind": "module",
        "name": "SSE",
        "description": "Minimal SSE consumer for Gas City integration",
        "tags": [],
        "parent": "cairn",
        "children": [],
        "paths": [
          "./src/sse.rs"
        ],
        "contracts": [],
        "state": "synced",
        "files": [
          "src/sse.rs"
        ]
      },
      {
        "id": "cairn.state",
        "kind": "module",
        "name": "State",
        "description": "Pluggable state persistence backend",
        "tags": [],
        "parent": "cairn",
        "children": [],
        "paths": [
          "./src/state"
        ],
        "contracts": [],
        "state": "synced",
        "files": [
          "src/state/backlog.rs",
          "src/state/beads.rs",
          "src/state/mod.rs",
          "src/state/tests.rs"
        ]
      },
      {
        "id": "cairn.suggested-edges",
        "kind": "module",
        "name": "SuggestedEdges",
        "description": "Suggested-edges queue: mutable triage workflows for AI-suggested graph edges",
        "tags": [],
        "parent": "cairn",
        "children": [],
        "paths": [
          "./src/suggested_edges"
        ],
        "contracts": [],
        "state": "synced",
        "files": [
          "src/suggested_edges/mod.rs",
          "src/suggested_edges/types.rs"
        ]
      },
      {
        "id": "cairn.summariser",
        "kind": "module",
        "name": "Summariser",
        "description": "LLM-assisted summarisation backend and request queue",
        "tags": [],
        "parent": "cairn",
        "children": [],
        "paths": [
          "./src/summariser"
        ],
        "contracts": [],
        "state": "synced",
        "files": [
          "src/summariser/accept.rs",
          "src/summariser/backend/mod.rs",
          "src/summariser/backend/tests.rs",
          "src/summariser/config.rs",
          "src/summariser/generate.rs",
          "src/summariser/mod.rs",
          "src/summariser/prompt/mod.rs",
          "src/summariser/prompt/tests.rs",
          "src/summariser/request.rs",
          "src/summariser/store.rs"
        ]
      },
      {
        "id": "cairn.tests",
        "kind": "module",
        "name": "Tests",
        "description": "Integration and smoke tests",
        "tags": [
          "test",
          "no-test-coverage"
        ],
        "parent": "cairn",
        "children": [],
        "paths": [
          "./tests"
        ],
        "contracts": [],
        "state": "synced",
        "files": [
          "tests/artefacts_contract.rs",
          "tests/artefacts_frontmatter.rs",
          "tests/blueprint_lexer.rs",
          "tests/blueprint_parser.rs",
          "tests/check_a11y.rs",
          "tests/check_design_tokens.rs",
          "tests/check_file_sizes.rs",
          "tests/command_reference_consistency.rs",
          "tests/conventions.rs",
          "tests/decision_claims.rs",
          "tests/fixtures_smoke.rs",
          "tests/gitignore_lint.rs",
          "tests/graph_explorer.rs",
          "tests/hooks_architecture.rs",
          "tests/kernel.rs",
          "tests/landing_assets.rs",
          "tests/mcp.rs",
          "tests/phase_10_distribution.rs",
          "tests/phase_7_6_ai_provenance.rs",
          "tests/phase_7_7_ux_foundation.rs",
          "tests/phase_7_8_cairn_export.rs",
          "tests/phase_8_summariser.rs",
          "tests/phase_9_brownfield.rs",
          "tests/reconcile_go.rs",
          "tests/reconcile_python.rs",
          "tests/reconcile_rust.rs",
          "tests/reconcile_target_fingerprint.rs",
          "tests/reconcile_typescript.rs",
          "tests/scanner_interface_hash.rs",
          "tests/ui_mobile.rs",
          "tests/watch.rs",
          "tests/wire_format_snapshots.rs"
        ]
      },
      {
        "id": "cairn.ui",
        "kind": "module",
        "name": "UI",
        "description": "Read-only web graph explorer and API server",
        "tags": [
          "ui"
        ],
        "parent": "cairn",
        "children": [],
        "paths": [
          "./src/ui"
        ],
        "contracts": [],
        "state": "synced",
        "files": [
          "src/ui/api.rs",
          "src/ui/mod.rs",
          "src/ui/serialise.rs",
          "src/ui/server.rs"
        ]
      },
      {
        "id": "cairn.watch",
        "kind": "module",
        "name": "Watch",
        "description": "Watch mode: periodic scan with finding-change events",
        "tags": [],
        "parent": "cairn",
        "children": [],
        "paths": [
          "./src/watch.rs"
        ],
        "contracts": [],
        "state": "synced",
        "files": [
          "src/watch.rs"
        ]
      }
    ],
    "edges": [
      {
        "from": "cairn",
        "to": "cairn.brownfield",
        "kind": "ownership",
        "description": "owns"
      },
      {
        "from": "cairn",
        "to": "cairn.kernel",
        "kind": "ownership",
        "description": "owns"
      },
      {
        "from": "cairn",
        "to": "cairn.lsp",
        "kind": "ownership",
        "description": "owns"
      },
      {
        "from": "cairn",
        "to": "cairn.macros",
        "kind": "ownership",
        "description": "owns"
      },
      {
        "from": "cairn",
        "to": "cairn.mcp",
        "kind": "ownership",
        "description": "owns"
      },
      {
        "from": "cairn",
        "to": "cairn.provenance",
        "kind": "ownership",
        "description": "owns"
      },
      {
        "from": "cairn",
        "to": "cairn.reconcile",
        "kind": "ownership",
        "description": "owns"
      },
      {
        "from": "cairn",
        "to": "cairn.root",
        "kind": "ownership",
        "description": "owns"
      },
      {
        "from": "cairn",
        "to": "cairn.sse",
        "kind": "ownership",
        "description": "owns"
      },
      {
        "from": "cairn",
        "to": "cairn.state",
        "kind": "ownership",
        "description": "owns"
      },
      {
        "from": "cairn",
        "to": "cairn.suggested-edges",
        "kind": "ownership",
        "description": "owns"
      },
      {
        "from": "cairn",
        "to": "cairn.summariser",
        "kind": "ownership",
        "description": "owns"
      },
      {
        "from": "cairn",
        "to": "cairn.tests",
        "kind": "ownership",
        "description": "owns"
      },
      {
        "from": "cairn",
        "to": "cairn.ui",
        "kind": "ownership",
        "description": "owns"
      },
      {
        "from": "cairn",
        "to": "cairn.watch",
        "kind": "ownership",
        "description": "owns"
      },
      {
        "from": "cairn.brownfield",
        "to": "cairn.kernel.map",
        "kind": "dependency",
        "description": "Reads orphan findings"
      },
      {
        "from": "cairn.kernel",
        "to": "cairn.kernel.artefacts",
        "kind": "ownership",
        "description": "owns"
      },
      {
        "from": "cairn.kernel",
        "to": "cairn.kernel.blueprint",
        "kind": "ownership",
        "description": "owns"
      },
      {
        "from": "cairn.kernel",
        "to": "cairn.kernel.changes",
        "kind": "ownership",
        "description": "owns"
      },
      {
        "from": "cairn.kernel",
        "to": "cairn.kernel.cli",
        "kind": "ownership",
        "description": "owns"
      },
      {
        "from": "cairn.kernel",
        "to": "cairn.kernel.hooks",
        "kind": "ownership",
        "description": "owns"
      },
      {
        "from": "cairn.kernel",
        "to": "cairn.kernel.map",
        "kind": "ownership",
        "description": "owns"
      },
      {
        "from": "cairn.kernel",
        "to": "cairn.kernel.query",
        "kind": "ownership",
        "description": "owns"
      },
      {
        "from": "cairn.kernel",
        "to": "cairn.kernel.scanner",
        "kind": "ownership",
        "description": "owns"
      },
      {
        "from": "cairn.kernel.changes",
        "to": "cairn.kernel.blueprint",
        "kind": "dependency",
        "description": "Parses blueprint deltas"
      },
      {
        "from": "cairn.kernel.changes",
        "to": "cairn.kernel.map",
        "kind": "dependency",
        "description": "Reads graph state"
      },
      {
        "from": "cairn.kernel.changes",
        "to": "cairn.kernel.scanner",
        "kind": "dependency",
        "description": "Validates deltas before archive"
      },
      {
        "from": "cairn.kernel.cli",
        "to": "cairn.brownfield",
        "kind": "dependency",
        "description": "Dispatches onboard command"
      },
      {
        "from": "cairn.kernel.cli",
        "to": "cairn.kernel.hooks",
        "kind": "dependency",
        "description": "Runs hook checks"
      },
      {
        "from": "cairn.kernel.cli",
        "to": "cairn.kernel.query",
        "kind": "dependency",
        "description": "Exposes queries as CLI commands"
      },
      {
        "from": "cairn.kernel.cli",
        "to": "cairn.kernel.scanner",
        "kind": "dependency",
        "description": "Orchestrates scan runs"
      },
      {
        "from": "cairn.kernel.cli",
        "to": "cairn.ui",
        "kind": "dependency",
        "description": "Launches graph explorer"
      },
      {
        "from": "cairn.kernel.hooks",
        "to": "cairn.kernel.map",
        "kind": "dependency",
        "description": "Reads findings"
      },
      {
        "from": "cairn.kernel.hooks",
        "to": "cairn.kernel.scanner",
        "kind": "dependency",
        "description": "Gates commits on scan integrity"
      },
      {
        "from": "cairn.kernel.map",
        "to": "cairn.kernel.artefacts",
        "kind": "dependency",
        "description": "Validates contracts against code"
      },
      {
        "from": "cairn.kernel.map",
        "to": "cairn.kernel.blueprint",
        "kind": "dependency",
        "description": "Consumes parsed AST"
      },
      {
        "from": "cairn.kernel.query",
        "to": "cairn.kernel.changes",
        "kind": "dependency",
        "description": "Reads change state"
      },
      {
        "from": "cairn.kernel.query",
        "to": "cairn.kernel.hooks",
        "kind": "dependency",
        "description": "Reads hook findings"
      },
      {
        "from": "cairn.kernel.query",
        "to": "cairn.kernel.map",
        "kind": "dependency",
        "description": "Traverses graph"
      },
      {
        "from": "cairn.kernel.query",
        "to": "cairn.kernel.scanner",
        "kind": "dependency",
        "description": "Reads scan state"
      },
      {
        "from": "cairn.kernel.scanner",
        "to": "cairn.kernel.artefacts",
        "kind": "dependency",
        "description": "Loads artefact metadata"
      },
      {
        "from": "cairn.kernel.scanner",
        "to": "cairn.kernel.blueprint",
        "kind": "dependency",
        "description": "Parses blueprint files"
      },
      {
        "from": "cairn.kernel.scanner",
        "to": "cairn.kernel.map",
        "kind": "dependency",
        "description": "Builds graph from parsed output"
      },
      {
        "from": "cairn.kernel.scanner",
        "to": "cairn.reconcile",
        "kind": "dependency",
        "description": "Invokes registered reconcilers"
      },
      {
        "from": "cairn.kernel.scanner",
        "to": "cairn.state",
        "kind": "dependency",
        "description": "Reads beads to flag orphan node labels"
      },
      {
        "from": "cairn.mcp",
        "to": "cairn.kernel.query",
        "kind": "dependency",
        "description": "Wraps queries as MCP tools"
      },
      {
        "from": "cairn.reconcile",
        "to": "cairn.kernel.map",
        "kind": "dependency",
        "description": "Reports findings to graph"
      },
      {
        "from": "cairn.ui",
        "to": "cairn.kernel.map",
        "kind": "dependency",
        "description": "Serves graph data"
      },
      {
        "from": "cairn.ui",
        "to": "cairn.kernel.scanner",
        "kind": "dependency",
        "description": "Runs scans for API responses"
      },
      {
        "from": "cairn.ui",
        "to": "cairn.state",
        "kind": "dependency",
        "description": "Reads node-linked beads for the inspector"
      }
    ]
  },
  "lint": {
    "schema_version": 1,
    "findings": []
  },
  "meta": {
    "schema_version": 1,
    "available_commands": [
      {
        "name": "get",
        "request": "NodeRequest",
        "response": "NodeResponse",
        "safety": "ReadOnly"
      },
      {
        "name": "neighbourhood",
        "request": "NeighbourhoodRequest",
        "response": "NeighbourhoodResponse",
        "safety": "ReadOnly"
      },
      {
        "name": "contract",
        "request": "NodeRequest",
        "response": "ContractResponse",
        "safety": "ReadOnly"
      },
      {
        "name": "docstring",
        "request": "DocstringRequest",
        "response": "DocstringResponse",
        "safety": "ReadOnly"
      },
      {
        "name": "files",
        "request": "NodeRequest",
        "response": "FilesResponse",
        "safety": "ReadOnly"
      },
      {
        "name": "dependents",
        "request": "DependencyRequest",
        "response": "DependencyResponse",
        "safety": "ReadOnly"
      },
      {
        "name": "depends",
        "request": "DependencyRequest",
        "response": "DependencyResponse",
        "safety": "ReadOnly"
      },
      {
        "name": "order",
        "request": "OrderRequest",
        "response": "OrderResponse",
        "safety": "ReadOnly"
      },
      {
        "name": "islands",
        "request": "IslandsRequest",
        "response": "IslandsResponse",
        "safety": "ReadOnly"
      },
      {
        "name": "lint",
        "request": "LintRequest",
        "response": "LintResponse",
        "safety": "ReadOnly"
      },
      {
        "name": "status",
        "request": "StatusRequest",
        "response": "StatusResponse",
        "safety": "ReadOnly"
      },
      {
        "name": "rationale",
        "request": "NodeRequest",
        "response": "RationaleResponse",
        "safety": "ReadOnly"
      },
      {
        "name": "todos",
        "request": "ArtefactNodeRequest",
        "response": "TodosResponse",
        "safety": "ReadOnly"
      },
      {
        "name": "decisions",
        "request": "ArtefactNodeRequest",
        "response": "DecisionsResponse",
        "safety": "ReadOnly"
      },
      {
        "name": "research",
        "request": "NodeRequest",
        "response": "ResearchResponse",
        "safety": "ReadOnly"
      },
      {
        "name": "sources",
        "request": "NodeRequest",
        "response": "SourcesResponse",
        "safety": "ReadOnly"
      },
      {
        "name": "changes",
        "request": "ChangesRequest",
        "response": "ChangesResponse",
        "safety": "ReadOnly"
      },
      {
        "name": "show",
        "request": "ShowChangeRequest",
        "response": "ShowChangeResponse",
        "safety": "ReadOnly"
      },
      {
        "name": "hook",
        "request": "HookRequest",
        "response": "HookReport",
        "safety": "ReadOnly"
      },
      {
        "name": "health",
        "request": "HealthRequest",
        "response": "HealthResponse",
        "safety": "ReadOnly"
      },
      {
        "name": "remediate",
        "request": "RemediateRequest",
        "response": "RemediateResponse",
        "safety": "ReadOnly"
      },
      {
        "name": "ui",
        "request": "UiRequest",
        "response": "UiServerResponse",
        "safety": "ReadOnly"
      },
      {
        "name": "scan",
        "request": "ScanRequest",
        "response": "ScanResponse",
        "safety": "Mutating"
      },
      {
        "name": "archive",
        "request": "ArchiveRequest",
        "response": "ArchiveResponse",
        "safety": "Mutating"
      },
      {
        "name": "rename",
        "request": "RenameRequest",
        "response": "RenameResponse",
        "safety": "Mutating"
      },
      {
        "name": "init",
        "request": "InitRequest",
        "response": "InitResponse",
        "safety": "Mutating"
      },
      {
        "name": "context",
        "request": "ContextRequest",
        "response": "ContextResponse",
        "safety": "ReadOnly"
      },
      {
        "name": "init_from_code",
        "request": "InitFromCodeRequest",
        "response": "InitFromCodeResponse",
        "safety": "Mutating"
      },
      {
        "name": "refine",
        "request": "RefineRequest",
        "response": "RefineResponse",
        "safety": "Mutating"
      },
      {
        "name": "drafts",
        "request": "DraftsRequest",
        "response": "DraftsResponse",
        "safety": "ReadOnly"
      },
      {
        "name": "draft_show",
        "request": "DraftShowRequest",
        "response": "DraftShowResponse",
        "safety": "ReadOnly"
      },
      {
        "name": "draft_discard",
        "request": "DraftDiscardRequest",
        "response": "DraftDiscardResponse",
        "safety": "Mutating"
      },
      {
        "name": "draft_edit",
        "request": "DraftEditRequest",
        "response": "DraftEditResponse",
        "safety": "Mutating"
      },
      {
        "name": "draft_accept",
        "request": "DraftAcceptRequest",
        "response": "DraftAcceptResponse",
        "safety": "Mutating"
      },
      {
        "name": "summarise",
        "request": "SummariseRequest",
        "response": "SummariseResponse",
        "safety": "Mutating"
      },
      {
        "name": "watch",
        "request": "WatchRequest",
        "response": "WatchResponse",
        "safety": "ReadOnly"
      }
    ]
  },
  "blueprint": {
    "schema_version": 1,
    "path": "cairn.blueprint",
    "source": "# Cairn — the framework described as a Cairn project.\n#\n# Promoted from test/fixtures/cairn-bootstrap/ and updated to match\n# the real source tree as of 2026-05-11.\n\nSystem Cairn \"Architecture map framework with pluggable reconcilers\" id \"cairn\" @framework {\n    decisions \"./meta/decisions\"\n    research \"./meta/research\"\n    research \"./meta/research/gas-city-cairn-integration/analysis.md\"\n    sources \"./meta/sources\"\n\n    Module Root \"Crate entry points, shared error types, and verification\" id \"cairn.root\" {\n        path \"./src/main.rs\"\n        path \"./src/lib.rs\"\n        path \"./src/error.rs\"\n        path \"./src/verification.rs\"\n        path \"./src/signal.rs\"\n    }\n\n    Module SSE \"Minimal SSE consumer for Gas City integration\" id \"cairn.sse\" {\n        path \"./src/sse.rs\"\n    }\n\n    Module State \"Pluggable state persistence backend\" id \"cairn.state\" {\n        path \"./src/state\"\n    }\n\n    Module Watch \"Watch mode: periodic scan with finding-change events\" id \"cairn.watch\" {\n        path \"./src/watch.rs\"\n    }\n    Container Kernel \"Domain-agnostic core\" id \"cairn.kernel\" @kernel {\n        decisions \"./meta/decisions\"\n\n        Module Blueprint \"Parses .blueprint files into AST node graph\" id \"cairn.kernel.blueprint\" {\n            path \"./src/blueprint\"\n        }\n\n        Module Artefacts \"Typed artefact registry and contract loader\" id \"cairn.kernel.artefacts\" {\n            path \"./src/artefacts\"\n        }\n\n        Module Map \"Graph construction, integrity checks, and query traversal\" id \"cairn.kernel.map\" {\n            path \"./src/map\"\n        }\n\n        Module Scanner \"Orchestrates parse, reconcile, and graph-build pipeline\" id \"cairn.kernel.scanner\" {\n            path \"./src/scanner\"\n        }\n\n        Module Changes \"Change directories, delta semantics, and archive\" id \"cairn.kernel.changes\" {\n            path \"./src/changes\"\n        }\n\n        Module Hooks \"Commit and task-boundary enforcement gates\" id \"cairn.kernel.hooks\" {\n            path \"./src/hooks\"\n        }\n\n        Module QueryAPI \"Structured query handlers and serialisation\" id \"cairn.kernel.query\" {\n            path \"./src/query_api\"\n        }\n\n        Module CLI \"Primary user surface and output formatting\" id \"cairn.kernel.cli\" {\n            path \"./src/cli\"\n        }\n    }\n\n    Module CodeReconciler \"Tree-sitter reconciler for Rust, TypeScript, Python, Go\" id \"cairn.reconcile\" @reconciler @code {\n        path \"./src/reconcile\"\n    }\n\n    Module UI \"Read-only web graph explorer and API server\" id \"cairn.ui\" @ui {\n        path \"./src/ui\"\n    }\n\n    Module MCP \"MCP tool wrapper over query API\" id \"cairn.mcp\" @integration {\n        path \"./src/mcp\"\n        path \"./src/bin/cairn-mcp.rs\"\n    }\n\n    Module LSP \"LSP diagnostics server for OMP integration\" id \"cairn.lsp\" @integration {\n        path \"./src/lsp\"\n        path \"./src/bin/cairn-lsp.rs\"\n    }\n\n    Module Macros \"Proc-macro crate for compile-time attributes\" id \"cairn.macros\" @build @no-test-coverage {\n        path \"./cairn-macros\"\n    }\n\n    Module Brownfield \"Orphan grouping, candidate heuristics, and onboard analysis\" id \"cairn.brownfield\" {\n        path \"./src/brownfield\"\n    }\n\n    Module Provenance \"Trace sidecar primitives for the provenance chain\" id \"cairn.provenance\" {\n        path \"./src/provenance\"\n    }\n\n    Module SuggestedEdges \"Suggested-edges queue: mutable triage workflows for AI-suggested graph edges\" id \"cairn.suggested-edges\" {\n        path \"./src/suggested_edges\"\n    }\n\n    Module Summariser \"LLM-assisted summarisation backend and request queue\" id \"cairn.summariser\" {\n        path \"./src/summariser\"\n    }\n\n    Module Tests \"Integration and smoke tests\" id \"cairn.tests\" @test @no-test-coverage {\n        path \"./tests\"\n    }\n}\n\n# Core data flow: scanner is the orchestration hub\ncairn.kernel.scanner    -> cairn.kernel.blueprint   \"Parses blueprint files\"\ncairn.kernel.scanner    -> cairn.kernel.artefacts   \"Loads artefact metadata\"\ncairn.kernel.scanner    -> cairn.kernel.map         \"Builds graph from parsed output\"\ncairn.kernel.scanner    -> cairn.reconcile          \"Invokes registered reconcilers\"\ncairn.kernel.scanner    -> cairn.state            \"Reads beads to flag orphan node labels\"\n\n# Map consumes parser and artefact output\ncairn.kernel.map        -> cairn.kernel.blueprint   \"Consumes parsed AST\"\ncairn.kernel.map        -> cairn.kernel.artefacts   \"Validates contracts against code\"\n\n# Enforcement gates\ncairn.kernel.hooks      -> cairn.kernel.scanner     \"Gates commits on scan integrity\"\ncairn.kernel.hooks      -> cairn.kernel.map         \"Reads findings\"\ncairn.kernel.changes    -> cairn.kernel.scanner     \"Validates deltas before archive\"\ncairn.kernel.changes    -> cairn.kernel.blueprint   \"Parses blueprint deltas\"\ncairn.kernel.changes    -> cairn.kernel.map         \"Reads graph state\"\n\n# Query and presentation surfaces\ncairn.kernel.cli        -> cairn.kernel.query       \"Exposes queries as CLI commands\"\ncairn.kernel.cli        -> cairn.kernel.scanner     \"Orchestrates scan runs\"\ncairn.kernel.cli        -> cairn.kernel.hooks       \"Runs hook checks\"\ncairn.kernel.cli        -> cairn.ui                 \"Launches graph explorer\"\ncairn.kernel.query      -> cairn.kernel.scanner     \"Reads scan state\"\ncairn.kernel.query      -> cairn.kernel.map         \"Traverses graph\"\ncairn.kernel.query      -> cairn.kernel.changes     \"Reads change state\"\ncairn.kernel.query      -> cairn.kernel.hooks       \"Reads hook findings\"\ncairn.ui                -> cairn.kernel.scanner     \"Runs scans for API responses\"\ncairn.ui                -> cairn.kernel.map         \"Serves graph data\"\ncairn.ui                -> cairn.state            \"Reads node-linked beads for the inspector\"\ncairn.mcp               -> cairn.kernel.query       \"Wraps queries as MCP tools\"\n\n# Pluggable reconciler reports back to graph\ncairn.reconcile         -> cairn.kernel.map         \"Reports findings to graph\"\n\n# Brownfield onboard consumes scanner findings\ncairn.brownfield        -> cairn.kernel.map         \"Reads orphan findings\"\ncairn.kernel.cli        -> cairn.brownfield         \"Dispatches onboard command\"\n"
  },
  "nodes": [
    {
      "id": "cairn",
      "kind": "system",
      "name": "Cairn",
      "description": "Architecture map framework with pluggable reconcilers",
      "tags": [
        "framework"
      ],
      "parent": null,
      "children": [
        "cairn.root",
        "cairn.sse",
        "cairn.state",
        "cairn.watch",
        "cairn.kernel",
        "cairn.reconcile",
        "cairn.ui",
        "cairn.mcp",
        "cairn.lsp",
        "cairn.macros",
        "cairn.brownfield",
        "cairn.provenance",
        "cairn.suggested-edges",
        "cairn.summariser",
        "cairn.tests"
      ],
      "paths": [],
      "contracts": [],
      "state": "synced",
      "files": []
    },
    {
      "id": "cairn.brownfield",
      "kind": "module",
      "name": "Brownfield",
      "description": "Orphan grouping, candidate heuristics, and onboard analysis",
      "tags": [],
      "parent": "cairn",
      "children": [],
      "paths": [
        "./src/brownfield"
      ],
      "contracts": [],
      "state": "synced",
      "files": [
        "src/brownfield/discovery.rs",
        "src/brownfield/heuristics.rs",
        "src/brownfield/init.rs",
        "src/brownfield/interview.rs",
        "src/brownfield/mod.rs",
        "src/brownfield/onboard.rs",
        "src/brownfield/refine.rs",
        "src/brownfield/suggest.rs",
        "src/brownfield/summarise.rs",
        "src/brownfield/templates.rs"
      ]
    },
    {
      "id": "cairn.kernel",
      "kind": "container",
      "name": "Kernel",
      "description": "Domain-agnostic core",
      "tags": [
        "kernel"
      ],
      "parent": "cairn",
      "children": [
        "cairn.kernel.blueprint",
        "cairn.kernel.artefacts",
        "cairn.kernel.map",
        "cairn.kernel.scanner",
        "cairn.kernel.changes",
        "cairn.kernel.hooks",
        "cairn.kernel.query",
        "cairn.kernel.cli"
      ],
      "paths": [],
      "contracts": [],
      "state": "synced",
      "files": []
    },
    {
      "id": "cairn.kernel.artefacts",
      "kind": "module",
      "name": "Artefacts",
      "description": "Typed artefact registry and contract loader",
      "tags": [],
      "parent": "cairn.kernel",
      "children": [],
      "paths": [
        "./src/artefacts"
      ],
      "contracts": [],
      "state": "synced",
      "files": [
        "src/artefacts/contract.rs",
        "src/artefacts/frontmatter.rs",
        "src/artefacts/mod.rs",
        "src/artefacts/registry/io.rs",
        "src/artefacts/registry/mod.rs",
        "src/artefacts/registry/parse.rs",
        "src/artefacts/registry/sha256.rs",
        "src/artefacts/registry/types.rs",
        "src/artefacts/registry/validate/mod.rs",
        "src/artefacts/registry/validate/tests.rs"
      ]
    },
    {
      "id": "cairn.kernel.blueprint",
      "kind": "module",
      "name": "Blueprint",
      "description": "Parses .blueprint files into AST node graph",
      "tags": [],
      "parent": "cairn.kernel",
      "children": [],
      "paths": [
        "./src/blueprint"
      ],
      "contracts": [],
      "state": "synced",
      "files": [
        "src/blueprint/ast.rs",
        "src/blueprint/error.rs",
        "src/blueprint/lexer.rs",
        "src/blueprint/mod.rs",
        "src/blueprint/parser.rs"
      ]
    },
    {
      "id": "cairn.kernel.changes",
      "kind": "module",
      "name": "Changes",
      "description": "Change directories, delta semantics, and archive",
      "tags": [],
      "parent": "cairn.kernel",
      "children": [],
      "paths": [
        "./src/changes"
      ],
      "contracts": [],
      "state": "synced",
      "files": [
        "src/changes/apply/mod.rs",
        "src/changes/apply/preserve.rs",
        "src/changes/apply/tests.rs",
        "src/changes/artefact_ops.rs",
        "src/changes/delta.rs",
        "src/changes/delta/tests.rs",
        "src/changes/mod.rs",
        "src/changes/rename.rs",
        "src/changes/tests.rs",
        "src/changes/types.rs",
        "src/changes/validate/mod.rs",
        "src/changes/validate/tests.rs"
      ]
    },
    {
      "id": "cairn.kernel.cli",
      "kind": "module",
      "name": "CLI",
      "description": "Primary user surface and output formatting",
      "tags": [],
      "parent": "cairn.kernel",
      "children": [],
      "paths": [
        "./src/cli"
      ],
      "contracts": [],
      "state": "synced",
      "files": [
        "src/cli/accept.rs",
        "src/cli/commands/archive.rs",
        "src/cli/commands/change.rs",
        "src/cli/commands/feedback.rs",
        "src/cli/commands/hook.rs",
        "src/cli/commands/import.rs",
        "src/cli/commands/mod.rs",
        "src/cli/commands/onboard.rs",
        "src/cli/commands/project.rs",
        "src/cli/commands/watch.rs",
        "src/cli/copy.rs",
        "src/cli/export/builder.rs",
        "src/cli/export/json.rs",
        "src/cli/export/markdown.rs",
        "src/cli/export/mermaid.rs",
        "src/cli/export/mod.rs",
        "src/cli/export/runner.rs",
        "src/cli/format/json.rs",
        "src/cli/format/mod.rs",
        "src/cli/format/render.rs",
        "src/cli/format/util.rs",
        "src/cli/mod.rs",
        "src/cli/render/artefacts.rs",
        "src/cli/render/health.rs",
        "src/cli/render/mod.rs",
        "src/cli/render/node.rs",
        "src/cli/render/project.rs",
        "src/cli/render/remediate.rs"
      ]
    },
    {
      "id": "cairn.kernel.hooks",
      "kind": "module",
      "name": "Hooks",
      "description": "Commit and task-boundary enforcement gates",
      "tags": [],
      "parent": "cairn.kernel",
      "children": [],
      "paths": [
        "./src/hooks"
      ],
      "contracts": [],
      "state": "synced",
      "files": [
        "src/hooks/architecture.rs",
        "src/hooks/mod.rs",
        "src/hooks/render.rs",
        "src/hooks/tests.rs"
      ]
    },
    {
      "id": "cairn.kernel.map",
      "kind": "module",
      "name": "Map",
      "description": "Graph construction, integrity checks, and query traversal",
      "tags": [],
      "parent": "cairn.kernel",
      "children": [],
      "paths": [
        "./src/map"
      ],
      "contracts": [],
      "state": "synced",
      "files": [
        "src/map/build.rs",
        "src/map/graph.rs",
        "src/map/integrity.rs",
        "src/map/mod.rs",
        "src/map/query.rs",
        "src/map/test_coverage.rs"
      ]
    },
    {
      "id": "cairn.kernel.query",
      "kind": "module",
      "name": "QueryAPI",
      "description": "Structured query handlers and serialisation",
      "tags": [],
      "parent": "cairn.kernel",
      "children": [],
      "paths": [
        "./src/query_api"
      ],
      "contracts": [],
      "state": "synced",
      "files": [
        "src/query_api/change_queries.rs",
        "src/query_api/handlers/artefacts.rs",
        "src/query_api/handlers/graph.rs",
        "src/query_api/handlers/mod.rs",
        "src/query_api/handlers/node.rs",
        "src/query_api/handlers/project.rs",
        "src/query_api/handlers/remediate.rs",
        "src/query_api/mod.rs",
        "src/query_api/registry.rs",
        "src/query_api/serialise.rs",
        "src/query_api/tests.rs",
        "src/query_api/util.rs"
      ]
    },
    {
      "id": "cairn.kernel.scanner",
      "kind": "module",
      "name": "Scanner",
      "description": "Orchestrates parse, reconcile, and graph-build pipeline",
      "tags": [],
      "parent": "cairn.kernel",
      "children": [],
      "paths": [
        "./src/scanner"
      ],
      "contracts": [],
      "state": "synced",
      "files": [
        "src/scanner/cache.rs",
        "src/scanner/checks.rs",
        "src/scanner/config/mod.rs",
        "src/scanner/config/tests.rs",
        "src/scanner/mod.rs",
        "src/scanner/outputs.rs",
        "src/scanner/state.rs",
        "src/scanner/tests.rs"
      ]
    },
    {
      "id": "cairn.lsp",
      "kind": "module",
      "name": "LSP",
      "description": "LSP diagnostics server for OMP integration",
      "tags": [
        "integration"
      ],
      "parent": "cairn",
      "children": [],
      "paths": [
        "./src/lsp",
        "./src/bin/cairn-lsp.rs"
      ],
      "contracts": [],
      "state": "synced",
      "files": [
        "src/bin/cairn-lsp.rs",
        "src/lsp/diagnostics.rs",
        "src/lsp/mod.rs",
        "src/lsp/server.rs"
      ]
    },
    {
      "id": "cairn.macros",
      "kind": "module",
      "name": "Macros",
      "description": "Proc-macro crate for compile-time attributes",
      "tags": [
        "build",
        "no-test-coverage"
      ],
      "parent": "cairn",
      "children": [],
      "paths": [
        "./cairn-macros"
      ],
      "contracts": [],
      "state": "synced",
      "files": [
        "cairn-macros/src/lib.rs",
        "cairn-macros/tests/planned_attribute.rs"
      ]
    },
    {
      "id": "cairn.mcp",
      "kind": "module",
      "name": "MCP",
      "description": "MCP tool wrapper over query API",
      "tags": [
        "integration"
      ],
      "parent": "cairn",
      "children": [],
      "paths": [
        "./src/mcp",
        "./src/bin/cairn-mcp.rs"
      ],
      "contracts": [],
      "state": "synced",
      "files": [
        "src/bin/cairn-mcp.rs",
        "src/mcp/mod.rs",
        "src/mcp/tests.rs"
      ]
    },
    {
      "id": "cairn.provenance",
      "kind": "module",
      "name": "Provenance",
      "description": "Trace sidecar primitives for the provenance chain",
      "tags": [],
      "parent": "cairn",
      "children": [],
      "paths": [
        "./src/provenance"
      ],
      "contracts": [],
      "state": "synced",
      "files": [
        "src/provenance/mod.rs",
        "src/provenance/trace.rs"
      ]
    },
    {
      "id": "cairn.reconcile",
      "kind": "module",
      "name": "CodeReconciler",
      "description": "Tree-sitter reconciler for Rust, TypeScript, Python, Go",
      "tags": [
        "reconciler",
        "code"
      ],
      "parent": "cairn",
      "children": [],
      "paths": [
        "./src/reconcile"
      ],
      "contracts": [],
      "state": "synced",
      "files": [
        "src/reconcile/code.rs",
        "src/reconcile/fingerprint.rs",
        "src/reconcile/fixture.rs",
        "src/reconcile/go.rs",
        "src/reconcile/mod.rs",
        "src/reconcile/python.rs",
        "src/reconcile/target.rs",
        "src/reconcile/typescript.rs"
      ]
    },
    {
      "id": "cairn.root",
      "kind": "module",
      "name": "Root",
      "description": "Crate entry points, shared error types, and verification",
      "tags": [],
      "parent": "cairn",
      "children": [],
      "paths": [
        "./src/main.rs",
        "./src/lib.rs",
        "./src/error.rs",
        "./src/verification.rs",
        "./src/signal.rs"
      ],
      "contracts": [],
      "state": "synced",
      "files": [
        "src/error.rs",
        "src/lib.rs",
        "src/main.rs",
        "src/signal.rs",
        "src/verification.rs"
      ]
    },
    {
      "id": "cairn.sse",
      "kind": "module",
      "name": "SSE",
      "description": "Minimal SSE consumer for Gas City integration",
      "tags": [],
      "parent": "cairn",
      "children": [],
      "paths": [
        "./src/sse.rs"
      ],
      "contracts": [],
      "state": "synced",
      "files": [
        "src/sse.rs"
      ]
    },
    {
      "id": "cairn.state",
      "kind": "module",
      "name": "State",
      "description": "Pluggable state persistence backend",
      "tags": [],
      "parent": "cairn",
      "children": [],
      "paths": [
        "./src/state"
      ],
      "contracts": [],
      "state": "synced",
      "files": [
        "src/state/backlog.rs",
        "src/state/beads.rs",
        "src/state/mod.rs",
        "src/state/tests.rs"
      ]
    },
    {
      "id": "cairn.suggested-edges",
      "kind": "module",
      "name": "SuggestedEdges",
      "description": "Suggested-edges queue: mutable triage workflows for AI-suggested graph edges",
      "tags": [],
      "parent": "cairn",
      "children": [],
      "paths": [
        "./src/suggested_edges"
      ],
      "contracts": [],
      "state": "synced",
      "files": [
        "src/suggested_edges/mod.rs",
        "src/suggested_edges/types.rs"
      ]
    },
    {
      "id": "cairn.summariser",
      "kind": "module",
      "name": "Summariser",
      "description": "LLM-assisted summarisation backend and request queue",
      "tags": [],
      "parent": "cairn",
      "children": [],
      "paths": [
        "./src/summariser"
      ],
      "contracts": [],
      "state": "synced",
      "files": [
        "src/summariser/accept.rs",
        "src/summariser/backend/mod.rs",
        "src/summariser/backend/tests.rs",
        "src/summariser/config.rs",
        "src/summariser/generate.rs",
        "src/summariser/mod.rs",
        "src/summariser/prompt/mod.rs",
        "src/summariser/prompt/tests.rs",
        "src/summariser/request.rs",
        "src/summariser/store.rs"
      ]
    },
    {
      "id": "cairn.tests",
      "kind": "module",
      "name": "Tests",
      "description": "Integration and smoke tests",
      "tags": [
        "test",
        "no-test-coverage"
      ],
      "parent": "cairn",
      "children": [],
      "paths": [
        "./tests"
      ],
      "contracts": [],
      "state": "synced",
      "files": [
        "tests/artefacts_contract.rs",
        "tests/artefacts_frontmatter.rs",
        "tests/blueprint_lexer.rs",
        "tests/blueprint_parser.rs",
        "tests/check_a11y.rs",
        "tests/check_design_tokens.rs",
        "tests/check_file_sizes.rs",
        "tests/command_reference_consistency.rs",
        "tests/conventions.rs",
        "tests/decision_claims.rs",
        "tests/fixtures_smoke.rs",
        "tests/gitignore_lint.rs",
        "tests/graph_explorer.rs",
        "tests/hooks_architecture.rs",
        "tests/kernel.rs",
        "tests/landing_assets.rs",
        "tests/mcp.rs",
        "tests/phase_10_distribution.rs",
        "tests/phase_7_6_ai_provenance.rs",
        "tests/phase_7_7_ux_foundation.rs",
        "tests/phase_7_8_cairn_export.rs",
        "tests/phase_8_summariser.rs",
        "tests/phase_9_brownfield.rs",
        "tests/reconcile_go.rs",
        "tests/reconcile_python.rs",
        "tests/reconcile_rust.rs",
        "tests/reconcile_target_fingerprint.rs",
        "tests/reconcile_typescript.rs",
        "tests/scanner_interface_hash.rs",
        "tests/ui_mobile.rs",
        "tests/watch.rs",
        "tests/wire_format_snapshots.rs"
      ]
    },
    {
      "id": "cairn.ui",
      "kind": "module",
      "name": "UI",
      "description": "Read-only web graph explorer and API server",
      "tags": [
        "ui"
      ],
      "parent": "cairn",
      "children": [],
      "paths": [
        "./src/ui"
      ],
      "contracts": [],
      "state": "synced",
      "files": [
        "src/ui/api.rs",
        "src/ui/mod.rs",
        "src/ui/serialise.rs",
        "src/ui/server.rs"
      ]
    },
    {
      "id": "cairn.watch",
      "kind": "module",
      "name": "Watch",
      "description": "Watch mode: periodic scan with finding-change events",
      "tags": [],
      "parent": "cairn",
      "children": [],
      "paths": [
        "./src/watch.rs"
      ],
      "contracts": [],
      "state": "synced",
      "files": [
        "src/watch.rs"
      ]
    }
  ],
  "edges": [
    {
      "from": "cairn",
      "to": "cairn.brownfield",
      "kind": "ownership",
      "description": "owns"
    },
    {
      "from": "cairn",
      "to": "cairn.kernel",
      "kind": "ownership",
      "description": "owns"
    },
    {
      "from": "cairn",
      "to": "cairn.lsp",
      "kind": "ownership",
      "description": "owns"
    },
    {
      "from": "cairn",
      "to": "cairn.macros",
      "kind": "ownership",
      "description": "owns"
    },
    {
      "from": "cairn",
      "to": "cairn.mcp",
      "kind": "ownership",
      "description": "owns"
    },
    {
      "from": "cairn",
      "to": "cairn.provenance",
      "kind": "ownership",
      "description": "owns"
    },
    {
      "from": "cairn",
      "to": "cairn.reconcile",
      "kind": "ownership",
      "description": "owns"
    },
    {
      "from": "cairn",
      "to": "cairn.root",
      "kind": "ownership",
      "description": "owns"
    },
    {
      "from": "cairn",
      "to": "cairn.sse",
      "kind": "ownership",
      "description": "owns"
    },
    {
      "from": "cairn",
      "to": "cairn.state",
      "kind": "ownership",
      "description": "owns"
    },
    {
      "from": "cairn",
      "to": "cairn.suggested-edges",
      "kind": "ownership",
      "description": "owns"
    },
    {
      "from": "cairn",
      "to": "cairn.summariser",
      "kind": "ownership",
      "description": "owns"
    },
    {
      "from": "cairn",
      "to": "cairn.tests",
      "kind": "ownership",
      "description": "owns"
    },
    {
      "from": "cairn",
      "to": "cairn.ui",
      "kind": "ownership",
      "description": "owns"
    },
    {
      "from": "cairn",
      "to": "cairn.watch",
      "kind": "ownership",
      "description": "owns"
    },
    {
      "from": "cairn.brownfield",
      "to": "cairn.kernel.map",
      "kind": "dependency",
      "description": "Reads orphan findings"
    },
    {
      "from": "cairn.kernel",
      "to": "cairn.kernel.artefacts",
      "kind": "ownership",
      "description": "owns"
    },
    {
      "from": "cairn.kernel",
      "to": "cairn.kernel.blueprint",
      "kind": "ownership",
      "description": "owns"
    },
    {
      "from": "cairn.kernel",
      "to": "cairn.kernel.changes",
      "kind": "ownership",
      "description": "owns"
    },
    {
      "from": "cairn.kernel",
      "to": "cairn.kernel.cli",
      "kind": "ownership",
      "description": "owns"
    },
    {
      "from": "cairn.kernel",
      "to": "cairn.kernel.hooks",
      "kind": "ownership",
      "description": "owns"
    },
    {
      "from": "cairn.kernel",
      "to": "cairn.kernel.map",
      "kind": "ownership",
      "description": "owns"
    },
    {
      "from": "cairn.kernel",
      "to": "cairn.kernel.query",
      "kind": "ownership",
      "description": "owns"
    },
    {
      "from": "cairn.kernel",
      "to": "cairn.kernel.scanner",
      "kind": "ownership",
      "description": "owns"
    },
    {
      "from": "cairn.kernel.changes",
      "to": "cairn.kernel.blueprint",
      "kind": "dependency",
      "description": "Parses blueprint deltas"
    },
    {
      "from": "cairn.kernel.changes",
      "to": "cairn.kernel.map",
      "kind": "dependency",
      "description": "Reads graph state"
    },
    {
      "from": "cairn.kernel.changes",
      "to": "cairn.kernel.scanner",
      "kind": "dependency",
      "description": "Validates deltas before archive"
    },
    {
      "from": "cairn.kernel.cli",
      "to": "cairn.brownfield",
      "kind": "dependency",
      "description": "Dispatches onboard command"
    },
    {
      "from": "cairn.kernel.cli",
      "to": "cairn.kernel.hooks",
      "kind": "dependency",
      "description": "Runs hook checks"
    },
    {
      "from": "cairn.kernel.cli",
      "to": "cairn.kernel.query",
      "kind": "dependency",
      "description": "Exposes queries as CLI commands"
    },
    {
      "from": "cairn.kernel.cli",
      "to": "cairn.kernel.scanner",
      "kind": "dependency",
      "description": "Orchestrates scan runs"
    },
    {
      "from": "cairn.kernel.cli",
      "to": "cairn.ui",
      "kind": "dependency",
      "description": "Launches graph explorer"
    },
    {
      "from": "cairn.kernel.hooks",
      "to": "cairn.kernel.map",
      "kind": "dependency",
      "description": "Reads findings"
    },
    {
      "from": "cairn.kernel.hooks",
      "to": "cairn.kernel.scanner",
      "kind": "dependency",
      "description": "Gates commits on scan integrity"
    },
    {
      "from": "cairn.kernel.map",
      "to": "cairn.kernel.artefacts",
      "kind": "dependency",
      "description": "Validates contracts against code"
    },
    {
      "from": "cairn.kernel.map",
      "to": "cairn.kernel.blueprint",
      "kind": "dependency",
      "description": "Consumes parsed AST"
    },
    {
      "from": "cairn.kernel.query",
      "to": "cairn.kernel.changes",
      "kind": "dependency",
      "description": "Reads change state"
    },
    {
      "from": "cairn.kernel.query",
      "to": "cairn.kernel.hooks",
      "kind": "dependency",
      "description": "Reads hook findings"
    },
    {
      "from": "cairn.kernel.query",
      "to": "cairn.kernel.map",
      "kind": "dependency",
      "description": "Traverses graph"
    },
    {
      "from": "cairn.kernel.query",
      "to": "cairn.kernel.scanner",
      "kind": "dependency",
      "description": "Reads scan state"
    },
    {
      "from": "cairn.kernel.scanner",
      "to": "cairn.kernel.artefacts",
      "kind": "dependency",
      "description": "Loads artefact metadata"
    },
    {
      "from": "cairn.kernel.scanner",
      "to": "cairn.kernel.blueprint",
      "kind": "dependency",
      "description": "Parses blueprint files"
    },
    {
      "from": "cairn.kernel.scanner",
      "to": "cairn.kernel.map",
      "kind": "dependency",
      "description": "Builds graph from parsed output"
    },
    {
      "from": "cairn.kernel.scanner",
      "to": "cairn.reconcile",
      "kind": "dependency",
      "description": "Invokes registered reconcilers"
    },
    {
      "from": "cairn.kernel.scanner",
      "to": "cairn.state",
      "kind": "dependency",
      "description": "Reads beads to flag orphan node labels"
    },
    {
      "from": "cairn.mcp",
      "to": "cairn.kernel.query",
      "kind": "dependency",
      "description": "Wraps queries as MCP tools"
    },
    {
      "from": "cairn.reconcile",
      "to": "cairn.kernel.map",
      "kind": "dependency",
      "description": "Reports findings to graph"
    },
    {
      "from": "cairn.ui",
      "to": "cairn.kernel.map",
      "kind": "dependency",
      "description": "Serves graph data"
    },
    {
      "from": "cairn.ui",
      "to": "cairn.kernel.scanner",
      "kind": "dependency",
      "description": "Runs scans for API responses"
    },
    {
      "from": "cairn.ui",
      "to": "cairn.state",
      "kind": "dependency",
      "description": "Reads node-linked beads for the inspector"
    }
  ],
  "nodePayload": {
    "cairn": {
      "id": "cairn",
      "endpoints": {
        "beads": {
          "schema_version": 1,
          "node": "cairn",
          "beads": []
        },
        "contract": {
          "schema_version": 1,
          "node": "cairn",
          "artefacts": []
        },
        "decisions": {
          "schema_version": 1,
          "node": "cairn",
          "artefacts": []
        },
        "rationale": {
          "schema_version": 1,
          "node": "cairn",
          "artefacts": []
        },
        "research": {
          "schema_version": 1,
          "node": "cairn",
          "artefacts": []
        },
        "sources": {
          "schema_version": 1,
          "node": "cairn",
          "artefacts": []
        },
        "todos": {
          "schema_version": 1,
          "node": "cairn",
          "artefacts": []
        }
      }
    },
    "cairn.brownfield": {
      "id": "cairn.brownfield",
      "endpoints": {
        "beads": {
          "schema_version": 1,
          "node": "cairn.brownfield",
          "beads": []
        },
        "contract": {
          "schema_version": 1,
          "node": "cairn.brownfield",
          "artefacts": []
        },
        "decisions": {
          "schema_version": 1,
          "node": "cairn.brownfield",
          "artefacts": [
            {
              "type": "decisions",
              "path": "meta/decisions/build-and-extension.md",
              "title": "Build and extension modules",
              "frontmatter": {
                "date": "2026-06-16",
                "id": "dec.build-and-extension",
                "status": "accepted"
              },
              "body": "\n# Build and extension modules\n\n## Context\n\ncairn has several modules that are not part of the daily graph pipeline but are essential for adoption, provenance, and agent-assisted workflows.\n\n## Decision\n\nKeep these as first-class modules:\n\n- **Macros**: proc-macro crate for compile-time attributes (e.g., `#[cairn_planned]`).\n- **Brownfield**: orphan grouping, candidate heuristics, and onboard analysis for existing codebases.\n- **Provenance**: trace sidecar primitives and provenance-chain helpers.\n- **SuggestedEdges**: queue for AI-suggested graph edges with triage workflows.\n- **Summariser**: LLM-assisted contract summarisation backend.\n\n## Rationale\n\nThese are distinct enough to warrant separate modules. Brownfield and summariser are especially important for adoption: most users do not start greenfield.\n\n## Consequences\n\n- Brownfield heuristics affect `cairn init --from-code` and `cairn refine`.\n- Provenance types are consumed by the artefact registry and decision coverage gate.\n- SuggestedEdges and Summariser both touch LLM outputs and need careful prompt/version management."
            }
          ]
        },
        "rationale": {
          "schema_version": 1,
          "node": "cairn.brownfield",
          "artefacts": [
            {
              "type": "decisions",
              "path": "meta/decisions/build-and-extension.md",
              "title": "Build and extension modules",
              "frontmatter": {
                "date": "2026-06-16",
                "id": "dec.build-and-extension",
                "status": "accepted"
              },
              "body": "\n# Build and extension modules\n\n## Context\n\ncairn has several modules that are not part of the daily graph pipeline but are essential for adoption, provenance, and agent-assisted workflows.\n\n## Decision\n\nKeep these as first-class modules:\n\n- **Macros**: proc-macro crate for compile-time attributes (e.g., `#[cairn_planned]`).\n- **Brownfield**: orphan grouping, candidate heuristics, and onboard analysis for existing codebases.\n- **Provenance**: trace sidecar primitives and provenance-chain helpers.\n- **SuggestedEdges**: queue for AI-suggested graph edges with triage workflows.\n- **Summariser**: LLM-assisted contract summarisation backend.\n\n## Rationale\n\nThese are distinct enough to warrant separate modules. Brownfield and summariser are especially important for adoption: most users do not start greenfield.\n\n## Consequences\n\n- Brownfield heuristics affect `cairn init --from-code` and `cairn refine`.\n- Provenance types are consumed by the artefact registry and decision coverage gate.\n- SuggestedEdges and Summariser both touch LLM outputs and need careful prompt/version management."
            }
          ]
        },
        "research": {
          "schema_version": 1,
          "node": "cairn.brownfield",
          "artefacts": []
        },
        "sources": {
          "schema_version": 1,
          "node": "cairn.brownfield",
          "artefacts": []
        },
        "todos": {
          "schema_version": 1,
          "node": "cairn.brownfield",
          "artefacts": []
        }
      }
    },
    "cairn.kernel": {
      "id": "cairn.kernel",
      "endpoints": {
        "beads": {
          "schema_version": 1,
          "node": "cairn.kernel",
          "beads": []
        },
        "contract": {
          "schema_version": 1,
          "node": "cairn.kernel",
          "artefacts": []
        },
        "decisions": {
          "schema_version": 1,
          "node": "cairn.kernel",
          "artefacts": []
        },
        "rationale": {
          "schema_version": 1,
          "node": "cairn.kernel",
          "artefacts": []
        },
        "research": {
          "schema_version": 1,
          "node": "cairn.kernel",
          "artefacts": []
        },
        "sources": {
          "schema_version": 1,
          "node": "cairn.kernel",
          "artefacts": []
        },
        "todos": {
          "schema_version": 1,
          "node": "cairn.kernel",
          "artefacts": []
        }
      }
    },
    "cairn.kernel.artefacts": {
      "id": "cairn.kernel.artefacts",
      "endpoints": {
        "beads": {
          "schema_version": 1,
          "node": "cairn.kernel.artefacts",
          "beads": []
        },
        "contract": {
          "schema_version": 1,
          "node": "cairn.kernel.artefacts",
          "artefacts": []
        },
        "decisions": {
          "schema_version": 1,
          "node": "cairn.kernel.artefacts",
          "artefacts": [
            {
              "type": "decisions",
              "path": "meta/decisions/kernel-core.md",
              "title": "Kernel core: blueprint, artefacts, map, and scanner",
              "frontmatter": {
                "date": "2026-06-16",
                "id": "dec.kernel-core",
                "status": "accepted"
              },
              "body": "\n# Kernel core: blueprint, artefacts, map, and scanner\n\n## Context\n\nCAIRN's value is a typed architecture map that stays in sync with real code. This requires four tightly-coupled subsystems: parsing the declaration language, loading typed artefacts, building and validating a graph, and orchestrating reconcilers that compare the graph against source.\n\n## Decision\n\nKeep blueprint parsing, artefact loading, graph construction, and scan orchestration as the four core modules of `cairn.kernel`. They form a pipeline: blueprint and artefacts feed the graph, the scanner drives reconcilers and feeds findings back into the graph.\n\n## Rationale\n\nSeparating these concerns makes each unit testable and lets the CLI, query API, and web UI consume the same graph. The scanner is the orchestration hub because it is the only module that understands the full lifecycle: parse → reconcile → validate → emit.\n\n## Consequences\n\n- Changes to the blueprint grammar must update parser tests and scanner integration tests.\n- New artefact types extend `cairn.kernel.artefacts` and are consumed by `cairn.kernel.map` for validation.\n- The scanner owns caching and incremental-scan state."
            }
          ]
        },
        "rationale": {
          "schema_version": 1,
          "node": "cairn.kernel.artefacts",
          "artefacts": [
            {
              "type": "decisions",
              "path": "meta/decisions/kernel-core.md",
              "title": "Kernel core: blueprint, artefacts, map, and scanner",
              "frontmatter": {
                "date": "2026-06-16",
                "id": "dec.kernel-core",
                "status": "accepted"
              },
              "body": "\n# Kernel core: blueprint, artefacts, map, and scanner\n\n## Context\n\nCAIRN's value is a typed architecture map that stays in sync with real code. This requires four tightly-coupled subsystems: parsing the declaration language, loading typed artefacts, building and validating a graph, and orchestrating reconcilers that compare the graph against source.\n\n## Decision\n\nKeep blueprint parsing, artefact loading, graph construction, and scan orchestration as the four core modules of `cairn.kernel`. They form a pipeline: blueprint and artefacts feed the graph, the scanner drives reconcilers and feeds findings back into the graph.\n\n## Rationale\n\nSeparating these concerns makes each unit testable and lets the CLI, query API, and web UI consume the same graph. The scanner is the orchestration hub because it is the only module that understands the full lifecycle: parse → reconcile → validate → emit.\n\n## Consequences\n\n- Changes to the blueprint grammar must update parser tests and scanner integration tests.\n- New artefact types extend `cairn.kernel.artefacts` and are consumed by `cairn.kernel.map` for validation.\n- The scanner owns caching and incremental-scan state."
            }
          ]
        },
        "research": {
          "schema_version": 1,
          "node": "cairn.kernel.artefacts",
          "artefacts": []
        },
        "sources": {
          "schema_version": 1,
          "node": "cairn.kernel.artefacts",
          "artefacts": []
        },
        "todos": {
          "schema_version": 1,
          "node": "cairn.kernel.artefacts",
          "artefacts": []
        }
      }
    },
    "cairn.kernel.blueprint": {
      "id": "cairn.kernel.blueprint",
      "endpoints": {
        "beads": {
          "schema_version": 1,
          "node": "cairn.kernel.blueprint",
          "beads": []
        },
        "contract": {
          "schema_version": 1,
          "node": "cairn.kernel.blueprint",
          "artefacts": []
        },
        "decisions": {
          "schema_version": 1,
          "node": "cairn.kernel.blueprint",
          "artefacts": [
            {
              "type": "decisions",
              "path": "meta/decisions/kernel-core.md",
              "title": "Kernel core: blueprint, artefacts, map, and scanner",
              "frontmatter": {
                "date": "2026-06-16",
                "id": "dec.kernel-core",
                "status": "accepted"
              },
              "body": "\n# Kernel core: blueprint, artefacts, map, and scanner\n\n## Context\n\nCAIRN's value is a typed architecture map that stays in sync with real code. This requires four tightly-coupled subsystems: parsing the declaration language, loading typed artefacts, building and validating a graph, and orchestrating reconcilers that compare the graph against source.\n\n## Decision\n\nKeep blueprint parsing, artefact loading, graph construction, and scan orchestration as the four core modules of `cairn.kernel`. They form a pipeline: blueprint and artefacts feed the graph, the scanner drives reconcilers and feeds findings back into the graph.\n\n## Rationale\n\nSeparating these concerns makes each unit testable and lets the CLI, query API, and web UI consume the same graph. The scanner is the orchestration hub because it is the only module that understands the full lifecycle: parse → reconcile → validate → emit.\n\n## Consequences\n\n- Changes to the blueprint grammar must update parser tests and scanner integration tests.\n- New artefact types extend `cairn.kernel.artefacts` and are consumed by `cairn.kernel.map` for validation.\n- The scanner owns caching and incremental-scan state."
            }
          ]
        },
        "rationale": {
          "schema_version": 1,
          "node": "cairn.kernel.blueprint",
          "artefacts": [
            {
              "type": "decisions",
              "path": "meta/decisions/kernel-core.md",
              "title": "Kernel core: blueprint, artefacts, map, and scanner",
              "frontmatter": {
                "date": "2026-06-16",
                "id": "dec.kernel-core",
                "status": "accepted"
              },
              "body": "\n# Kernel core: blueprint, artefacts, map, and scanner\n\n## Context\n\nCAIRN's value is a typed architecture map that stays in sync with real code. This requires four tightly-coupled subsystems: parsing the declaration language, loading typed artefacts, building and validating a graph, and orchestrating reconcilers that compare the graph against source.\n\n## Decision\n\nKeep blueprint parsing, artefact loading, graph construction, and scan orchestration as the four core modules of `cairn.kernel`. They form a pipeline: blueprint and artefacts feed the graph, the scanner drives reconcilers and feeds findings back into the graph.\n\n## Rationale\n\nSeparating these concerns makes each unit testable and lets the CLI, query API, and web UI consume the same graph. The scanner is the orchestration hub because it is the only module that understands the full lifecycle: parse → reconcile → validate → emit.\n\n## Consequences\n\n- Changes to the blueprint grammar must update parser tests and scanner integration tests.\n- New artefact types extend `cairn.kernel.artefacts` and are consumed by `cairn.kernel.map` for validation.\n- The scanner owns caching and incremental-scan state."
            }
          ]
        },
        "research": {
          "schema_version": 1,
          "node": "cairn.kernel.blueprint",
          "artefacts": []
        },
        "sources": {
          "schema_version": 1,
          "node": "cairn.kernel.blueprint",
          "artefacts": []
        },
        "todos": {
          "schema_version": 1,
          "node": "cairn.kernel.blueprint",
          "artefacts": []
        }
      }
    },
    "cairn.kernel.changes": {
      "id": "cairn.kernel.changes",
      "endpoints": {
        "beads": {
          "schema_version": 1,
          "node": "cairn.kernel.changes",
          "beads": []
        },
        "contract": {
          "schema_version": 1,
          "node": "cairn.kernel.changes",
          "artefacts": []
        },
        "decisions": {
          "schema_version": 1,
          "node": "cairn.kernel.changes",
          "artefacts": [
            {
              "type": "decisions",
              "path": "meta/decisions/kernel-tooling.md",
              "title": "Kernel tooling: changes, hooks, query API, and CLI",
              "frontmatter": {
                "date": "2026-06-16",
                "id": "dec.kernel-tooling",
                "status": "accepted"
              },
              "body": "\n# Kernel tooling: changes, hooks, query API, and CLI\n\n## Context\n\nBeyond graph construction, cairn needs mechanisms to mutate and query the graph, gate commits, and expose everything to users.\n\n## Decision\n\nProvide four tooling modules under `cairn.kernel`:\n\n- **Changes**: change directories, delta parsing, and archive acceptance.\n- **Hooks**: commit and task-boundary gates (`structural`, `interface`, `tension`, `all`).\n- **Query API**: structured query handlers used by the CLI, web UI, and MCP wrapper.\n- **CLI**: primary user surface, command parsing, and output formatting.\n\n## Rationale\n\nKeeping these separate from the core graph pipeline lets the core stay focused on map integrity while tooling evolves independently. The query API is the stable boundary between cairn's internals and its surfaces.\n\n## Consequences\n\n- New commands start in `cairn.kernel.cli` and usually call into `cairn.kernel.query`.\n- Hook changes affect `scripts/` and CI, so they need extra test coverage.\n- The changes module is the only kernel submodule that writes to the working tree outside generated outputs."
            },
            {
              "type": "decisions",
              "path": "meta/decisions/preserve-blueprint-trivia.md",
              "title": "Archive preserves blueprint comments via a source-preserving delta splice",
              "frontmatter": {
                "date": "2026-06-23",
                "id": "dec.preserve-blueprint-trivia",
                "status": "accepted"
              },
              "body": "\n# Archive preserves blueprint comments via a source-preserving delta splice\n\n## Context\n\n`apply_blueprint_delta` parsed `cairn.blueprint` into an AST, mutated it, and\nre-emitted the whole tree with `serialize_ast`. That serializer reconstructs the\nfile from typed nodes alone, so every comment and blank line was discarded on\narchive. PR #157 (bead `cairn-giv`) only fixed the empty-delta case by skipping\nthe rewrite entirely. For a non-empty delta (a real node or edge change) the\nrewrite still ran and still stripped trivia (bead `cairn-2sh`).\n\nThe prior session deferred this pending a deliberate trivia-model decision. Two\ndesigns were on the table:\n\n1. **Full trivia model.** Capture comment and blank-line trivia in the lexer,\n   attach it to every AST node/field/edge, and re-emit it from the serializer.\n   Full round-trip fidelity, but it touches the whole parse/serialize pipeline,\n   adds trivia fields to `Node`/`Edge` (which derive `Eq`/`Hash`), and changes\n   the AST `Hash` that `scanner/cache.rs` uses as the reconciler cache key.\n2. **Source-preserving splice.** Keep the original source text and rewrite only\n   the declarations the delta actually changes.\n\nThe maintainer chose option 2: the conservative, contained option that fully\nsolves the stated problem (comments survive a structural-delta archive).\n\n## Decision\n\n`apply_blueprint_delta` delegates to `src/changes/apply/preserve.rs`, which\napplies the delta against the original source string:\n\n- Untouched lines (comments, blank lines, unchanged declarations) are copied\n  through byte-for-byte, at every nesting depth.\n- A node whose subtree is unchanged is copied verbatim; a node that is itself a\n  `modified` target is re-serialised wholesale; a node that is only renamed or\n  only has a changed descendant is recursed into, preserving its own trivia.\n- Top-level edges are kept verbatim unless an endpoint rename, removal, or\n  modification changes them; added/replaced edges and added nodes are appended.\n\nNode line extents come from the lexer token stream, not the AST: the k-th `{`\ntoken opens the k-th node in preorder, so a brace stack pairs each opener with\nits closer. This needs no change to the AST shape, the `Span` semantics, the\n`Node`/`Edge` `Eq`/`Hash` derives, the reconciler cache key, or the `query_api`\nspan JSON. `serialize_ast` (the whole-tree serializer) is removed; `serialize_node`\nremains for the canonical forms of changed and added nodes.\n\n## Rationale\n\nThe token-stream extent derivation is string-safe (a `{` inside a quoted\ndescription is a `String` token, never an `OpenBrace`), so brace matching is\nrobust. Confining the work to `src/changes/apply` keeps the blast radius to one\nmodule: no kernel parser, AST, or cache surface moves, so the change cannot\nregress reconciliation or the cache contract.\n\n## Consequences\n\n- A node that is a `modified` target is reproduced from the delta in canonical\n  form, so comments and blank lines *inside that one declaration* are not\n  retained. This is inherent: the declaration's body is replaced wholesale.\n- A removed node's own trivia is removed with it; a comment that sat above the\n  removed node is left in place and may read as orphaned.\n- Added nodes and added/replaced edges are appended in canonical serialised form\n  at end of file. The blueprint parser is order-independent (nodes and edges may\n  interleave), so this is purely cosmetic for the rare add-on-archive case.\n- Full round-trip fidelity (preserving comments inside a modified declaration)\n  is explicitly deferred. Reaching it requires the option-1 trivia model and a\n  reconciler-cache version bump; revisit only if a real need appears."
            }
          ]
        },
        "rationale": {
          "schema_version": 1,
          "node": "cairn.kernel.changes",
          "artefacts": [
            {
              "type": "decisions",
              "path": "meta/decisions/kernel-tooling.md",
              "title": "Kernel tooling: changes, hooks, query API, and CLI",
              "frontmatter": {
                "date": "2026-06-16",
                "id": "dec.kernel-tooling",
                "status": "accepted"
              },
              "body": "\n# Kernel tooling: changes, hooks, query API, and CLI\n\n## Context\n\nBeyond graph construction, cairn needs mechanisms to mutate and query the graph, gate commits, and expose everything to users.\n\n## Decision\n\nProvide four tooling modules under `cairn.kernel`:\n\n- **Changes**: change directories, delta parsing, and archive acceptance.\n- **Hooks**: commit and task-boundary gates (`structural`, `interface`, `tension`, `all`).\n- **Query API**: structured query handlers used by the CLI, web UI, and MCP wrapper.\n- **CLI**: primary user surface, command parsing, and output formatting.\n\n## Rationale\n\nKeeping these separate from the core graph pipeline lets the core stay focused on map integrity while tooling evolves independently. The query API is the stable boundary between cairn's internals and its surfaces.\n\n## Consequences\n\n- New commands start in `cairn.kernel.cli` and usually call into `cairn.kernel.query`.\n- Hook changes affect `scripts/` and CI, so they need extra test coverage.\n- The changes module is the only kernel submodule that writes to the working tree outside generated outputs."
            },
            {
              "type": "decisions",
              "path": "meta/decisions/preserve-blueprint-trivia.md",
              "title": "Archive preserves blueprint comments via a source-preserving delta splice",
              "frontmatter": {
                "date": "2026-06-23",
                "id": "dec.preserve-blueprint-trivia",
                "status": "accepted"
              },
              "body": "\n# Archive preserves blueprint comments via a source-preserving delta splice\n\n## Context\n\n`apply_blueprint_delta` parsed `cairn.blueprint` into an AST, mutated it, and\nre-emitted the whole tree with `serialize_ast`. That serializer reconstructs the\nfile from typed nodes alone, so every comment and blank line was discarded on\narchive. PR #157 (bead `cairn-giv`) only fixed the empty-delta case by skipping\nthe rewrite entirely. For a non-empty delta (a real node or edge change) the\nrewrite still ran and still stripped trivia (bead `cairn-2sh`).\n\nThe prior session deferred this pending a deliberate trivia-model decision. Two\ndesigns were on the table:\n\n1. **Full trivia model.** Capture comment and blank-line trivia in the lexer,\n   attach it to every AST node/field/edge, and re-emit it from the serializer.\n   Full round-trip fidelity, but it touches the whole parse/serialize pipeline,\n   adds trivia fields to `Node`/`Edge` (which derive `Eq`/`Hash`), and changes\n   the AST `Hash` that `scanner/cache.rs` uses as the reconciler cache key.\n2. **Source-preserving splice.** Keep the original source text and rewrite only\n   the declarations the delta actually changes.\n\nThe maintainer chose option 2: the conservative, contained option that fully\nsolves the stated problem (comments survive a structural-delta archive).\n\n## Decision\n\n`apply_blueprint_delta` delegates to `src/changes/apply/preserve.rs`, which\napplies the delta against the original source string:\n\n- Untouched lines (comments, blank lines, unchanged declarations) are copied\n  through byte-for-byte, at every nesting depth.\n- A node whose subtree is unchanged is copied verbatim; a node that is itself a\n  `modified` target is re-serialised wholesale; a node that is only renamed or\n  only has a changed descendant is recursed into, preserving its own trivia.\n- Top-level edges are kept verbatim unless an endpoint rename, removal, or\n  modification changes them; added/replaced edges and added nodes are appended.\n\nNode line extents come from the lexer token stream, not the AST: the k-th `{`\ntoken opens the k-th node in preorder, so a brace stack pairs each opener with\nits closer. This needs no change to the AST shape, the `Span` semantics, the\n`Node`/`Edge` `Eq`/`Hash` derives, the reconciler cache key, or the `query_api`\nspan JSON. `serialize_ast` (the whole-tree serializer) is removed; `serialize_node`\nremains for the canonical forms of changed and added nodes.\n\n## Rationale\n\nThe token-stream extent derivation is string-safe (a `{` inside a quoted\ndescription is a `String` token, never an `OpenBrace`), so brace matching is\nrobust. Confining the work to `src/changes/apply` keeps the blast radius to one\nmodule: no kernel parser, AST, or cache surface moves, so the change cannot\nregress reconciliation or the cache contract.\n\n## Consequences\n\n- A node that is a `modified` target is reproduced from the delta in canonical\n  form, so comments and blank lines *inside that one declaration* are not\n  retained. This is inherent: the declaration's body is replaced wholesale.\n- A removed node's own trivia is removed with it; a comment that sat above the\n  removed node is left in place and may read as orphaned.\n- Added nodes and added/replaced edges are appended in canonical serialised form\n  at end of file. The blueprint parser is order-independent (nodes and edges may\n  interleave), so this is purely cosmetic for the rare add-on-archive case.\n- Full round-trip fidelity (preserving comments inside a modified declaration)\n  is explicitly deferred. Reaching it requires the option-1 trivia model and a\n  reconciler-cache version bump; revisit only if a real need appears."
            }
          ]
        },
        "research": {
          "schema_version": 1,
          "node": "cairn.kernel.changes",
          "artefacts": []
        },
        "sources": {
          "schema_version": 1,
          "node": "cairn.kernel.changes",
          "artefacts": []
        },
        "todos": {
          "schema_version": 1,
          "node": "cairn.kernel.changes",
          "artefacts": []
        }
      }
    },
    "cairn.kernel.cli": {
      "id": "cairn.kernel.cli",
      "endpoints": {
        "beads": {
          "schema_version": 1,
          "node": "cairn.kernel.cli",
          "beads": []
        },
        "contract": {
          "schema_version": 1,
          "node": "cairn.kernel.cli",
          "artefacts": []
        },
        "decisions": {
          "schema_version": 1,
          "node": "cairn.kernel.cli",
          "artefacts": [
            {
              "type": "decisions",
              "path": "meta/decisions/adopt-cairn-dev-loop.md",
              "title": "Adopt the Cairn Dev Loop as the development workflow",
              "frontmatter": {
                "date": "2026-06-05",
                "id": "dec.adopt-cairn-dev-loop",
                "informed_by": "[]",
                "status": "accepted"
              },
              "body": "\n# Adopt the Cairn Dev Loop as the development workflow\n\n## Context\n\nThe repo dogfoods cairn but had no single, written iteration workflow that used\ncairn to drive its own development. The cairn-native skills (cairn-explore,\ncairn-propose, cairn-apply, cairn-archive) and the CLI gates existed\nindependently, with no canonical sequence tying orientation, scoping, proposal,\nverification, and provenance into one repeatable loop.\n\n## Decision\n\nAdopt a ten-phase coding loop, the Cairn Dev Loop, documented in\n`docs/agent/cairn-dev-workflow.md` and runnable as `/cairn-loop`. The phases are\norient, scope, propose, implement, test, verify, record, PR, merge, continue,\neach gated by cairn's own queries (`context`, `lint`, `neighbourhood`,\n`rationale`, `dependents`) and gates (`scan`, `hook all`), plus the language\ngates (`cargo test`, `clippy`) and the path to merge (CI green, review resolved).\nThe loop is continuous: phase ten selects the next unit and returns to phase one.\nA clean iteration is code merged, CI green, `cairn scan` clean, and the\nnext task identified.\n\n## Rationale\n\nThe framework should verify its own development. Using cairn to orient before\ncoding and to gate the result makes the dogfooding signal load-bearing rather\nthan aspirational: every iteration must leave `cairn scan` clean. The loop reuses\nthe existing skills and CLI surface instead of adding new machinery, so it stays\nthin and surgical.\n\nOne deliberate boundary: the loop does not wire a `decisions` pointer into\n`cairn.blueprint`. Provenance coverage in cairn is all-or-nothing (the first\ndeclared decision makes every uncovered leaf node raise\n`CAIRN_PROVENANCE_NO_DECISION`), so ingesting decisions is a repo-wide commitment\nleft for a dedicated iteration. Until then, decision records live in\n`meta/decisions/` as durable prose, and this file is the first one written under\nthe loop it describes.\n\n## Consequences\n\n- `docs/agent/cairn-dev-workflow.md` becomes the canonical loop documentation.\n- `CLAUDE.md` should reference it from the \"Using cairn in this repo\" section.\n- Iterations should leave `cairn scan`, `cairn lint`, and `cairn hook all` green.\n- Larger-than-atomic work must still go through a proposal/change directory, not\n  be forced through the loop."
            },
            {
              "type": "decisions",
              "path": "meta/decisions/feedback-loop.md",
              "title": "Close the dogfood loop from host projects via `cairn feedback`",
              "frontmatter": {
                "date": "2026-06-10",
                "id": "dec.feedback-loop",
                "status": "accepted"
              },
              "body": "\n# Close the dogfood loop from host projects via `cairn feedback`\n\n## Context\n\nCairn dogfoods itself inside this repository, but once it is installed on\nother projects there is no channel for friction discovered there to flow\nback. Coding agents working in a host repo hit confusing messages, wrong\nfindings, or missing capabilities, then route around them and the signal is\nlost. A blind two-arm agent experiment (see\n`meta/research/agent-experiment-linklint.md`) confirmed both the value of the\ngenerated guidance and the absence of any feedback path.\n\n## Decision\n\nAdd a `cairn feedback \"<message>\"` command that records friction locally and\npoints at the upstream tracker, and make `cairn init` generate agent-facing\nguidance that instructs agents to use it.\n\n- `cairn feedback` appends a timestamped entry (with cairn version) to\n  `.cairn/feedback.md` in the host project and prints a prefilled\n  `https://github.com/cairn-framework/cairn/issues/new` URL. No network\n  access, no GitHub credentials required; filing remains a human (or\n  authorised agent) action.\n- `cairn init` writes `.cairn/AGENTS.md`, a guide meant to be appended to the\n  host project's CLAUDE.md or AGENTS.md. It covers orientation commands, the\n  scan-before-commit loop, and the instruction to record cairn friction with\n  `cairn feedback` before working around it.\n- `cairn init` now prints next steps instead of a bare confirmation.\n\n## Consequences\n\n- Every project that adopts cairn becomes a dogfood site: friction\n  accumulates in a structured local log that maintainers can triage into\n  upstream issues, and the issue URL lowers the cost of filing directly.\n- The local-log-first design means feedback works offline and never blocks\n  an agent's task on network or auth.\n- The upstream repo URL is compiled in; if the canonical repo moves, the\n  constant in `src/cli/commands/feedback.rs` must move with it."
            },
            {
              "type": "decisions",
              "path": "meta/decisions/kernel-tooling.md",
              "title": "Kernel tooling: changes, hooks, query API, and CLI",
              "frontmatter": {
                "date": "2026-06-16",
                "id": "dec.kernel-tooling",
                "status": "accepted"
              },
              "body": "\n# Kernel tooling: changes, hooks, query API, and CLI\n\n## Context\n\nBeyond graph construction, cairn needs mechanisms to mutate and query the graph, gate commits, and expose everything to users.\n\n## Decision\n\nProvide four tooling modules under `cairn.kernel`:\n\n- **Changes**: change directories, delta parsing, and archive acceptance.\n- **Hooks**: commit and task-boundary gates (`structural`, `interface`, `tension`, `all`).\n- **Query API**: structured query handlers used by the CLI, web UI, and MCP wrapper.\n- **CLI**: primary user surface, command parsing, and output formatting.\n\n## Rationale\n\nKeeping these separate from the core graph pipeline lets the core stay focused on map integrity while tooling evolves independently. The query API is the stable boundary between cairn's internals and its surfaces.\n\n## Consequences\n\n- New commands start in `cairn.kernel.cli` and usually call into `cairn.kernel.query`.\n- Hook changes affect `scripts/` and CI, so they need extra test coverage.\n- The changes module is the only kernel submodule that writes to the working tree outside generated outputs."
            }
          ]
        },
        "rationale": {
          "schema_version": 1,
          "node": "cairn.kernel.cli",
          "artefacts": [
            {
              "type": "decisions",
              "path": "meta/decisions/adopt-cairn-dev-loop.md",
              "title": "Adopt the Cairn Dev Loop as the development workflow",
              "frontmatter": {
                "date": "2026-06-05",
                "id": "dec.adopt-cairn-dev-loop",
                "informed_by": "[]",
                "status": "accepted"
              },
              "body": "\n# Adopt the Cairn Dev Loop as the development workflow\n\n## Context\n\nThe repo dogfoods cairn but had no single, written iteration workflow that used\ncairn to drive its own development. The cairn-native skills (cairn-explore,\ncairn-propose, cairn-apply, cairn-archive) and the CLI gates existed\nindependently, with no canonical sequence tying orientation, scoping, proposal,\nverification, and provenance into one repeatable loop.\n\n## Decision\n\nAdopt a ten-phase coding loop, the Cairn Dev Loop, documented in\n`docs/agent/cairn-dev-workflow.md` and runnable as `/cairn-loop`. The phases are\norient, scope, propose, implement, test, verify, record, PR, merge, continue,\neach gated by cairn's own queries (`context`, `lint`, `neighbourhood`,\n`rationale`, `dependents`) and gates (`scan`, `hook all`), plus the language\ngates (`cargo test`, `clippy`) and the path to merge (CI green, review resolved).\nThe loop is continuous: phase ten selects the next unit and returns to phase one.\nA clean iteration is code merged, CI green, `cairn scan` clean, and the\nnext task identified.\n\n## Rationale\n\nThe framework should verify its own development. Using cairn to orient before\ncoding and to gate the result makes the dogfooding signal load-bearing rather\nthan aspirational: every iteration must leave `cairn scan` clean. The loop reuses\nthe existing skills and CLI surface instead of adding new machinery, so it stays\nthin and surgical.\n\nOne deliberate boundary: the loop does not wire a `decisions` pointer into\n`cairn.blueprint`. Provenance coverage in cairn is all-or-nothing (the first\ndeclared decision makes every uncovered leaf node raise\n`CAIRN_PROVENANCE_NO_DECISION`), so ingesting decisions is a repo-wide commitment\nleft for a dedicated iteration. Until then, decision records live in\n`meta/decisions/` as durable prose, and this file is the first one written under\nthe loop it describes.\n\n## Consequences\n\n- `docs/agent/cairn-dev-workflow.md` becomes the canonical loop documentation.\n- `CLAUDE.md` should reference it from the \"Using cairn in this repo\" section.\n- Iterations should leave `cairn scan`, `cairn lint`, and `cairn hook all` green.\n- Larger-than-atomic work must still go through a proposal/change directory, not\n  be forced through the loop."
            },
            {
              "type": "decisions",
              "path": "meta/decisions/feedback-loop.md",
              "title": "Close the dogfood loop from host projects via `cairn feedback`",
              "frontmatter": {
                "date": "2026-06-10",
                "id": "dec.feedback-loop",
                "status": "accepted"
              },
              "body": "\n# Close the dogfood loop from host projects via `cairn feedback`\n\n## Context\n\nCairn dogfoods itself inside this repository, but once it is installed on\nother projects there is no channel for friction discovered there to flow\nback. Coding agents working in a host repo hit confusing messages, wrong\nfindings, or missing capabilities, then route around them and the signal is\nlost. A blind two-arm agent experiment (see\n`meta/research/agent-experiment-linklint.md`) confirmed both the value of the\ngenerated guidance and the absence of any feedback path.\n\n## Decision\n\nAdd a `cairn feedback \"<message>\"` command that records friction locally and\npoints at the upstream tracker, and make `cairn init` generate agent-facing\nguidance that instructs agents to use it.\n\n- `cairn feedback` appends a timestamped entry (with cairn version) to\n  `.cairn/feedback.md` in the host project and prints a prefilled\n  `https://github.com/cairn-framework/cairn/issues/new` URL. No network\n  access, no GitHub credentials required; filing remains a human (or\n  authorised agent) action.\n- `cairn init` writes `.cairn/AGENTS.md`, a guide meant to be appended to the\n  host project's CLAUDE.md or AGENTS.md. It covers orientation commands, the\n  scan-before-commit loop, and the instruction to record cairn friction with\n  `cairn feedback` before working around it.\n- `cairn init` now prints next steps instead of a bare confirmation.\n\n## Consequences\n\n- Every project that adopts cairn becomes a dogfood site: friction\n  accumulates in a structured local log that maintainers can triage into\n  upstream issues, and the issue URL lowers the cost of filing directly.\n- The local-log-first design means feedback works offline and never blocks\n  an agent's task on network or auth.\n- The upstream repo URL is compiled in; if the canonical repo moves, the\n  constant in `src/cli/commands/feedback.rs` must move with it."
            },
            {
              "type": "decisions",
              "path": "meta/decisions/kernel-tooling.md",
              "title": "Kernel tooling: changes, hooks, query API, and CLI",
              "frontmatter": {
                "date": "2026-06-16",
                "id": "dec.kernel-tooling",
                "status": "accepted"
              },
              "body": "\n# Kernel tooling: changes, hooks, query API, and CLI\n\n## Context\n\nBeyond graph construction, cairn needs mechanisms to mutate and query the graph, gate commits, and expose everything to users.\n\n## Decision\n\nProvide four tooling modules under `cairn.kernel`:\n\n- **Changes**: change directories, delta parsing, and archive acceptance.\n- **Hooks**: commit and task-boundary gates (`structural`, `interface`, `tension`, `all`).\n- **Query API**: structured query handlers used by the CLI, web UI, and MCP wrapper.\n- **CLI**: primary user surface, command parsing, and output formatting.\n\n## Rationale\n\nKeeping these separate from the core graph pipeline lets the core stay focused on map integrity while tooling evolves independently. The query API is the stable boundary between cairn's internals and its surfaces.\n\n## Consequences\n\n- New commands start in `cairn.kernel.cli` and usually call into `cairn.kernel.query`.\n- Hook changes affect `scripts/` and CI, so they need extra test coverage.\n- The changes module is the only kernel submodule that writes to the working tree outside generated outputs."
            },
            {
              "type": "research",
              "path": "meta/research/agent-experiment-linklint.md",
              "title": "Blind two-arm agent experiment: building a small CLI with and without cairn",
              "frontmatter": {
                "date": "2026-06-10",
                "id": "res.agent-experiment-linklint",
                "method": "primary"
              },
              "body": "\n# Blind two-arm agent experiment: building a small CLI with and without cairn\n\n## Setup\n\nTwo identical fresh git repositories received the same product brief\n(`SPEC.md` for \"linklint\", a markdown broken-link checker with an explicit\nfour-module architecture intent) and the same base working notes. Arm A\nadditionally got what a real adopting repo would have: `cairn init` output, a\nhand-authored `cairn.blueprint` declaring the four intended modules as ghost\nnodes, and a CLAUDE.md section pointing agents at `cairn context`, `cairn\nscan`, and the keep-the-blueprint-in-sync rule. Arm B got no cairn at all.\n\nOne coding agent was launched per repo with an identical, neutral prompt.\nNeither agent was told it was part of a comparison. Both were asked, after\nfinishing, to report any friction caused by the repo's tooling or docs.\n\n## Results\n\nBoth arms shipped working tools meeting the full quality bar (fmt, clippy\n`-D warnings`, tests green) with near-identical module structure; the\narchitecture intent in SPEC.md was a strong enough signal on its own to\nproduce the four-module shape. Surface metrics were close: arm A 875 LOC and\n41 tests, arm B 794 LOC and 35 tests.\n\nThe behavioural difference showed up in correctness. On a shared fixture,\narm B reported two false positives (link syntax inside inline code spans and\nfenced code blocks was treated as a real link); arm A handled both correctly.\nArm A's agent caught this by smoke-testing against Markdown in its own repo,\nwhere the example link syntax in SPEC.md sat inside backticks. Attribution is\nsoft (single run per arm; could be agent variance), but arm A's workflow\nincluded more verification passes: its `.cairn/log.md` recorded four scans,\nand the ghost-to-synced transition was explicitly used as a to-do list\n(\"`cairn context` showed the four Ghost modules as a literal to-do list\").\n\n## Usability findings for cairn\n\n1. **Ghost modules work as scaffolding.** Declaring intended modules before\n   code exists gave the agent an orientation artefact it actively used. The\n   blueprint-as-skeleton pattern is worth documenting as a greenfield\n   workflow.\n2. **Starter guidance must mention test directories.** Arm A's only real\n   friction: SPEC.md required tests, the blueprint only declared `src/`\n   paths, and the agent had to make a judgment call before extending the\n   blueprint with a `./tests` path. Fixed: the init starter blueprint and the\n   generated `.cairn/AGENTS.md` now call out test directories explicitly.\n3. **No feedback channel existed.** Friction observed in a host project had\n   nowhere to go. Fixed: `cairn feedback` plus the generated agent guide\n   (see `meta/decisions/feedback-loop.md`).\n4. **CLI behaved as documented.** The arm A agent reported zero confusion\n   from cairn itself: \"the cairn CLI behaved exactly as documented.\"\n\n## Caveats\n\n- n=1 per arm; no statistical claim. The correctness delta is suggestive,\n  not conclusive.\n- Both agents inherited ambient context from the cairn repo's own CLAUDE.md\n  (a harness artefact, symmetric across arms). The arm B agent flagged this\n  as the main source of potential confusion, not the task repo itself.\n- The blueprint in arm A was hand-authored to match the spec; a sloppier\n  blueprint would presumably help less."
            }
          ]
        },
        "research": {
          "schema_version": 1,
          "node": "cairn.kernel.cli",
          "artefacts": [
            {
              "type": "research",
              "path": "meta/research/agent-experiment-linklint.md",
              "title": "Blind two-arm agent experiment: building a small CLI with and without cairn",
              "frontmatter": {
                "date": "2026-06-10",
                "id": "res.agent-experiment-linklint",
                "method": "primary"
              },
              "body": "\n# Blind two-arm agent experiment: building a small CLI with and without cairn\n\n## Setup\n\nTwo identical fresh git repositories received the same product brief\n(`SPEC.md` for \"linklint\", a markdown broken-link checker with an explicit\nfour-module architecture intent) and the same base working notes. Arm A\nadditionally got what a real adopting repo would have: `cairn init` output, a\nhand-authored `cairn.blueprint` declaring the four intended modules as ghost\nnodes, and a CLAUDE.md section pointing agents at `cairn context`, `cairn\nscan`, and the keep-the-blueprint-in-sync rule. Arm B got no cairn at all.\n\nOne coding agent was launched per repo with an identical, neutral prompt.\nNeither agent was told it was part of a comparison. Both were asked, after\nfinishing, to report any friction caused by the repo's tooling or docs.\n\n## Results\n\nBoth arms shipped working tools meeting the full quality bar (fmt, clippy\n`-D warnings`, tests green) with near-identical module structure; the\narchitecture intent in SPEC.md was a strong enough signal on its own to\nproduce the four-module shape. Surface metrics were close: arm A 875 LOC and\n41 tests, arm B 794 LOC and 35 tests.\n\nThe behavioural difference showed up in correctness. On a shared fixture,\narm B reported two false positives (link syntax inside inline code spans and\nfenced code blocks was treated as a real link); arm A handled both correctly.\nArm A's agent caught this by smoke-testing against Markdown in its own repo,\nwhere the example link syntax in SPEC.md sat inside backticks. Attribution is\nsoft (single run per arm; could be agent variance), but arm A's workflow\nincluded more verification passes: its `.cairn/log.md` recorded four scans,\nand the ghost-to-synced transition was explicitly used as a to-do list\n(\"`cairn context` showed the four Ghost modules as a literal to-do list\").\n\n## Usability findings for cairn\n\n1. **Ghost modules work as scaffolding.** Declaring intended modules before\n   code exists gave the agent an orientation artefact it actively used. The\n   blueprint-as-skeleton pattern is worth documenting as a greenfield\n   workflow.\n2. **Starter guidance must mention test directories.** Arm A's only real\n   friction: SPEC.md required tests, the blueprint only declared `src/`\n   paths, and the agent had to make a judgment call before extending the\n   blueprint with a `./tests` path. Fixed: the init starter blueprint and the\n   generated `.cairn/AGENTS.md` now call out test directories explicitly.\n3. **No feedback channel existed.** Friction observed in a host project had\n   nowhere to go. Fixed: `cairn feedback` plus the generated agent guide\n   (see `meta/decisions/feedback-loop.md`).\n4. **CLI behaved as documented.** The arm A agent reported zero confusion\n   from cairn itself: \"the cairn CLI behaved exactly as documented.\"\n\n## Caveats\n\n- n=1 per arm; no statistical claim. The correctness delta is suggestive,\n  not conclusive.\n- Both agents inherited ambient context from the cairn repo's own CLAUDE.md\n  (a harness artefact, symmetric across arms). The arm B agent flagged this\n  as the main source of potential confusion, not the task repo itself.\n- The blueprint in arm A was hand-authored to match the spec; a sloppier\n  blueprint would presumably help less."
            }
          ]
        },
        "sources": {
          "schema_version": 1,
          "node": "cairn.kernel.cli",
          "artefacts": []
        },
        "todos": {
          "schema_version": 1,
          "node": "cairn.kernel.cli",
          "artefacts": []
        }
      }
    },
    "cairn.kernel.hooks": {
      "id": "cairn.kernel.hooks",
      "endpoints": {
        "beads": {
          "schema_version": 1,
          "node": "cairn.kernel.hooks",
          "beads": []
        },
        "contract": {
          "schema_version": 1,
          "node": "cairn.kernel.hooks",
          "artefacts": []
        },
        "decisions": {
          "schema_version": 1,
          "node": "cairn.kernel.hooks",
          "artefacts": [
            {
              "type": "decisions",
              "path": "meta/decisions/kernel-tooling.md",
              "title": "Kernel tooling: changes, hooks, query API, and CLI",
              "frontmatter": {
                "date": "2026-06-16",
                "id": "dec.kernel-tooling",
                "status": "accepted"
              },
              "body": "\n# Kernel tooling: changes, hooks, query API, and CLI\n\n## Context\n\nBeyond graph construction, cairn needs mechanisms to mutate and query the graph, gate commits, and expose everything to users.\n\n## Decision\n\nProvide four tooling modules under `cairn.kernel`:\n\n- **Changes**: change directories, delta parsing, and archive acceptance.\n- **Hooks**: commit and task-boundary gates (`structural`, `interface`, `tension`, `all`).\n- **Query API**: structured query handlers used by the CLI, web UI, and MCP wrapper.\n- **CLI**: primary user surface, command parsing, and output formatting.\n\n## Rationale\n\nKeeping these separate from the core graph pipeline lets the core stay focused on map integrity while tooling evolves independently. The query API is the stable boundary between cairn's internals and its surfaces.\n\n## Consequences\n\n- New commands start in `cairn.kernel.cli` and usually call into `cairn.kernel.query`.\n- Hook changes affect `scripts/` and CI, so they need extra test coverage.\n- The changes module is the only kernel submodule that writes to the working tree outside generated outputs."
            },
            {
              "type": "decisions",
              "path": "meta/decisions/toolchain-lint-strictness.md",
              "title": "Adopt advisory toolchain-lint-strictness findings",
              "frontmatter": {
                "date": "2026-06-23",
                "id": "dec.toolchain-lint-strictness",
                "status": "accepted"
              },
              "body": "\n# Adopt advisory toolchain-lint-strictness findings\n\n## Context\n\nCairn reconciles the architecture map against reality. A recurring gap in host\nprojects is the absence of a strict lint configuration for a detected language:\na Rust crate may carry no clippy `-D warnings` configuration, or a JavaScript\nsurface may have no linter at all. This is one layer beyond pure architecture\n(it is project-health), but it is existence/linkage-shaped in the same way as\nexisting cairn checks such as contract presence and test-coverage linkage\n(cairn-a8z).\n\n## Decision\n\nAdopt a new advisory finding, `CAIRN_LINT_NOT_STRICT`, for tracked projects\nwhose detected primary language lacks a strict lint configuration.\n\n- **Finding code:** `CAIRN_LINT_NOT_STRICT`.\n- **Default severity:** `Warning` (non-blocking). Promote to blocking via the\n  existing `cairn lint --strict` flag.\n- **Scope:** config existence and strictness only. Cairn inspects configuration\n  files; it never invokes a linter or formatter.\n- **Per-language detection rules (initial):**\n  - **Rust:** strict clippy configuration is present. Acceptable signals include\n    a `.pre-commit-config.yaml` hook referencing `cargo clippy` with\n    `-D warnings` or equivalent, or `Cargo.toml` `[lints.clippy]` setting\n    warnings to `deny`. Presence of `cargo fmt` alone is not sufficient.\n  - **JavaScript / TypeScript:** an ESLint, Biome, or Oxlint configuration file\n    exists with at least one rule set to error (not warn-only) and the config is\n    referenced by CI or package scripts.\n  - **CSS:** a Stylelint or Biome CSS configuration exists with at least one\n    rule set to error.\n  - **Other languages:** defer. Do not emit the finding for languages without\n    defined detection rules.\n- **Node / project link:** the finding is attached to the node whose declared\n  paths contain the language evidence. If a node has no detectable primary\n  language, no finding is emitted.\n\n## Rationale\n\nThis follows the same pattern as cairn-a8z (test-coverage integrity): an\nadvisory-by-default, opt-in-strict existence check that respects cairn's\nboundary of observing reality rather than enforcing workflow. Keeping the check\nconfig-only means cairn stays out of the business of running external tools and\navoids version or platform coupling. The `Warning` default lets project teams\naddress the gap without blocking commits; `--strict` lets teams that want a hard\ngate opt in without adding a new config key.\n\n## Consequences\n\n- A future feature phase will implement the detector in the scanner/reconciler\n  path and add the finding to the registry.\n- Hook output may include `CAIRN_LINT_NOT_STRICT` under `cairn hook all` once\n  implemented, so the finding should be classified as advisory in hook reports.\n- Each new language added to the reconciler must either define a strict-lint\n  detection rule or explicitly opt out of this check for that language.\n- Cairn's own webui assets (`src/ui_assets/`) currently lack a strict CSS/JS\n  lint config. If those assets are claimed by a blueprint node, that node would\n  be flagged once the finding is implemented unless a strict config is added or\n  the node is excluded."
            }
          ]
        },
        "rationale": {
          "schema_version": 1,
          "node": "cairn.kernel.hooks",
          "artefacts": [
            {
              "type": "decisions",
              "path": "meta/decisions/kernel-tooling.md",
              "title": "Kernel tooling: changes, hooks, query API, and CLI",
              "frontmatter": {
                "date": "2026-06-16",
                "id": "dec.kernel-tooling",
                "status": "accepted"
              },
              "body": "\n# Kernel tooling: changes, hooks, query API, and CLI\n\n## Context\n\nBeyond graph construction, cairn needs mechanisms to mutate and query the graph, gate commits, and expose everything to users.\n\n## Decision\n\nProvide four tooling modules under `cairn.kernel`:\n\n- **Changes**: change directories, delta parsing, and archive acceptance.\n- **Hooks**: commit and task-boundary gates (`structural`, `interface`, `tension`, `all`).\n- **Query API**: structured query handlers used by the CLI, web UI, and MCP wrapper.\n- **CLI**: primary user surface, command parsing, and output formatting.\n\n## Rationale\n\nKeeping these separate from the core graph pipeline lets the core stay focused on map integrity while tooling evolves independently. The query API is the stable boundary between cairn's internals and its surfaces.\n\n## Consequences\n\n- New commands start in `cairn.kernel.cli` and usually call into `cairn.kernel.query`.\n- Hook changes affect `scripts/` and CI, so they need extra test coverage.\n- The changes module is the only kernel submodule that writes to the working tree outside generated outputs."
            },
            {
              "type": "decisions",
              "path": "meta/decisions/toolchain-lint-strictness.md",
              "title": "Adopt advisory toolchain-lint-strictness findings",
              "frontmatter": {
                "date": "2026-06-23",
                "id": "dec.toolchain-lint-strictness",
                "status": "accepted"
              },
              "body": "\n# Adopt advisory toolchain-lint-strictness findings\n\n## Context\n\nCairn reconciles the architecture map against reality. A recurring gap in host\nprojects is the absence of a strict lint configuration for a detected language:\na Rust crate may carry no clippy `-D warnings` configuration, or a JavaScript\nsurface may have no linter at all. This is one layer beyond pure architecture\n(it is project-health), but it is existence/linkage-shaped in the same way as\nexisting cairn checks such as contract presence and test-coverage linkage\n(cairn-a8z).\n\n## Decision\n\nAdopt a new advisory finding, `CAIRN_LINT_NOT_STRICT`, for tracked projects\nwhose detected primary language lacks a strict lint configuration.\n\n- **Finding code:** `CAIRN_LINT_NOT_STRICT`.\n- **Default severity:** `Warning` (non-blocking). Promote to blocking via the\n  existing `cairn lint --strict` flag.\n- **Scope:** config existence and strictness only. Cairn inspects configuration\n  files; it never invokes a linter or formatter.\n- **Per-language detection rules (initial):**\n  - **Rust:** strict clippy configuration is present. Acceptable signals include\n    a `.pre-commit-config.yaml` hook referencing `cargo clippy` with\n    `-D warnings` or equivalent, or `Cargo.toml` `[lints.clippy]` setting\n    warnings to `deny`. Presence of `cargo fmt` alone is not sufficient.\n  - **JavaScript / TypeScript:** an ESLint, Biome, or Oxlint configuration file\n    exists with at least one rule set to error (not warn-only) and the config is\n    referenced by CI or package scripts.\n  - **CSS:** a Stylelint or Biome CSS configuration exists with at least one\n    rule set to error.\n  - **Other languages:** defer. Do not emit the finding for languages without\n    defined detection rules.\n- **Node / project link:** the finding is attached to the node whose declared\n  paths contain the language evidence. If a node has no detectable primary\n  language, no finding is emitted.\n\n## Rationale\n\nThis follows the same pattern as cairn-a8z (test-coverage integrity): an\nadvisory-by-default, opt-in-strict existence check that respects cairn's\nboundary of observing reality rather than enforcing workflow. Keeping the check\nconfig-only means cairn stays out of the business of running external tools and\navoids version or platform coupling. The `Warning` default lets project teams\naddress the gap without blocking commits; `--strict` lets teams that want a hard\ngate opt in without adding a new config key.\n\n## Consequences\n\n- A future feature phase will implement the detector in the scanner/reconciler\n  path and add the finding to the registry.\n- Hook output may include `CAIRN_LINT_NOT_STRICT` under `cairn hook all` once\n  implemented, so the finding should be classified as advisory in hook reports.\n- Each new language added to the reconciler must either define a strict-lint\n  detection rule or explicitly opt out of this check for that language.\n- Cairn's own webui assets (`src/ui_assets/`) currently lack a strict CSS/JS\n  lint config. If those assets are claimed by a blueprint node, that node would\n  be flagged once the finding is implemented unless a strict config is added or\n  the node is excluded."
            }
          ]
        },
        "research": {
          "schema_version": 1,
          "node": "cairn.kernel.hooks",
          "artefacts": []
        },
        "sources": {
          "schema_version": 1,
          "node": "cairn.kernel.hooks",
          "artefacts": []
        },
        "todos": {
          "schema_version": 1,
          "node": "cairn.kernel.hooks",
          "artefacts": []
        }
      }
    },
    "cairn.kernel.map": {
      "id": "cairn.kernel.map",
      "endpoints": {
        "beads": {
          "schema_version": 1,
          "node": "cairn.kernel.map",
          "beads": []
        },
        "contract": {
          "schema_version": 1,
          "node": "cairn.kernel.map",
          "artefacts": []
        },
        "decisions": {
          "schema_version": 1,
          "node": "cairn.kernel.map",
          "artefacts": [
            {
              "type": "decisions",
              "path": "meta/decisions/kernel-core.md",
              "title": "Kernel core: blueprint, artefacts, map, and scanner",
              "frontmatter": {
                "date": "2026-06-16",
                "id": "dec.kernel-core",
                "status": "accepted"
              },
              "body": "\n# Kernel core: blueprint, artefacts, map, and scanner\n\n## Context\n\nCAIRN's value is a typed architecture map that stays in sync with real code. This requires four tightly-coupled subsystems: parsing the declaration language, loading typed artefacts, building and validating a graph, and orchestrating reconcilers that compare the graph against source.\n\n## Decision\n\nKeep blueprint parsing, artefact loading, graph construction, and scan orchestration as the four core modules of `cairn.kernel`. They form a pipeline: blueprint and artefacts feed the graph, the scanner drives reconcilers and feeds findings back into the graph.\n\n## Rationale\n\nSeparating these concerns makes each unit testable and lets the CLI, query API, and web UI consume the same graph. The scanner is the orchestration hub because it is the only module that understands the full lifecycle: parse → reconcile → validate → emit.\n\n## Consequences\n\n- Changes to the blueprint grammar must update parser tests and scanner integration tests.\n- New artefact types extend `cairn.kernel.artefacts` and are consumed by `cairn.kernel.map` for validation.\n- The scanner owns caching and incremental-scan state."
            }
          ]
        },
        "rationale": {
          "schema_version": 1,
          "node": "cairn.kernel.map",
          "artefacts": [
            {
              "type": "decisions",
              "path": "meta/decisions/kernel-core.md",
              "title": "Kernel core: blueprint, artefacts, map, and scanner",
              "frontmatter": {
                "date": "2026-06-16",
                "id": "dec.kernel-core",
                "status": "accepted"
              },
              "body": "\n# Kernel core: blueprint, artefacts, map, and scanner\n\n## Context\n\nCAIRN's value is a typed architecture map that stays in sync with real code. This requires four tightly-coupled subsystems: parsing the declaration language, loading typed artefacts, building and validating a graph, and orchestrating reconcilers that compare the graph against source.\n\n## Decision\n\nKeep blueprint parsing, artefact loading, graph construction, and scan orchestration as the four core modules of `cairn.kernel`. They form a pipeline: blueprint and artefacts feed the graph, the scanner drives reconcilers and feeds findings back into the graph.\n\n## Rationale\n\nSeparating these concerns makes each unit testable and lets the CLI, query API, and web UI consume the same graph. The scanner is the orchestration hub because it is the only module that understands the full lifecycle: parse → reconcile → validate → emit.\n\n## Consequences\n\n- Changes to the blueprint grammar must update parser tests and scanner integration tests.\n- New artefact types extend `cairn.kernel.artefacts` and are consumed by `cairn.kernel.map` for validation.\n- The scanner owns caching and incremental-scan state."
            }
          ]
        },
        "research": {
          "schema_version": 1,
          "node": "cairn.kernel.map",
          "artefacts": []
        },
        "sources": {
          "schema_version": 1,
          "node": "cairn.kernel.map",
          "artefacts": []
        },
        "todos": {
          "schema_version": 1,
          "node": "cairn.kernel.map",
          "artefacts": []
        }
      }
    },
    "cairn.kernel.query": {
      "id": "cairn.kernel.query",
      "endpoints": {
        "beads": {
          "schema_version": 1,
          "node": "cairn.kernel.query",
          "beads": []
        },
        "contract": {
          "schema_version": 1,
          "node": "cairn.kernel.query",
          "artefacts": []
        },
        "decisions": {
          "schema_version": 1,
          "node": "cairn.kernel.query",
          "artefacts": [
            {
              "type": "decisions",
              "path": "meta/decisions/kernel-tooling.md",
              "title": "Kernel tooling: changes, hooks, query API, and CLI",
              "frontmatter": {
                "date": "2026-06-16",
                "id": "dec.kernel-tooling",
                "status": "accepted"
              },
              "body": "\n# Kernel tooling: changes, hooks, query API, and CLI\n\n## Context\n\nBeyond graph construction, cairn needs mechanisms to mutate and query the graph, gate commits, and expose everything to users.\n\n## Decision\n\nProvide four tooling modules under `cairn.kernel`:\n\n- **Changes**: change directories, delta parsing, and archive acceptance.\n- **Hooks**: commit and task-boundary gates (`structural`, `interface`, `tension`, `all`).\n- **Query API**: structured query handlers used by the CLI, web UI, and MCP wrapper.\n- **CLI**: primary user surface, command parsing, and output formatting.\n\n## Rationale\n\nKeeping these separate from the core graph pipeline lets the core stay focused on map integrity while tooling evolves independently. The query API is the stable boundary between cairn's internals and its surfaces.\n\n## Consequences\n\n- New commands start in `cairn.kernel.cli` and usually call into `cairn.kernel.query`.\n- Hook changes affect `scripts/` and CI, so they need extra test coverage.\n- The changes module is the only kernel submodule that writes to the working tree outside generated outputs."
            },
            {
              "type": "decisions",
              "path": "meta/decisions/query-json-schema-version.md",
              "title": "Query-API JSON envelopes carry a uniform schema_version",
              "frontmatter": {
                "date": "2026-06-23",
                "id": "dec.query-json-schema-version",
                "status": "accepted"
              },
              "body": "\n# Query-API JSON envelopes carry a uniform schema_version\n\n## Context\n\nThe `query_api` command surface (consumed by the CLI `--json` flag and the MCP\nserver) emitted inconsistent JSON. `cairn islands --json` carried a top-level\n`schema_version` (sourced from the map-domain `IslandsResponse` and\n`ISLANDS_SCHEMA_VERSION`), while `order`, `contract`, `context`, `status`,\n`lint`, `dependents`, and every other command emitted a `data` payload with no\nversion at all. A JSON consumer could branch on the islands version but had no\nversion to branch on for any other command.\n\nThe prior session handoff flagged this as the next candidate and marked it a\nmaintainer decision, because standardizing it is a user-facing output-contract\nchange. The maintainer chose to add `schema_version` everywhere.\n\n## Decision\n\nEvery `query_api` command's JSON `data` payload carries a top-level\n`schema_version` field, currently `1`. The stamp is applied at a single choke\npoint in `query_api::execute`: after `execute_data` returns, the data object is\nstamped with `query_api::SCHEMA_VERSION`. Because the CLI prints `data`\ndirectly and the MCP envelope wraps `data`, both surfaces share one versioned\ncontract from one constant.\n\nThe redundant per-handler stamp in `islands_json` was removed so the universal\nstamp is the single source of truth. `ISLANDS_SCHEMA_VERSION` remains a\nmap-layer library concept (still constructed and tested on `IslandsResponse`)\nbut no longer drives the CLI islands envelope version. The live islands output\nis byte-identical to before (its value was already `1`).\n\n## Rationale\n\nVersioning the `data` payload at one choke point, with one constant, beats\nper-command stamps: it cannot drift, every command is covered automatically\n(including future ones and the change-directory tools), and there is exactly one\nplace to bump when the contract changes. Per-command divergent versions were\nexplicitly rejected: the goal is a uniform contract a consumer branches on, not\na matrix of per-command versions.\n\nOnly JSON objects are stamped; every current command returns an object, so the\nstamp is universal in practice.\n\n## Consequences\n\n- Every `query_api` `--json` command output now contains `\"schema_version\": 1`\n  (serialised in alphabetical key position). Bumping the wire contract means\n  bumping `query_api::SCHEMA_VERSION`.\n- The webui HTTP surface (`src/ui/api.rs`) is a separate JSON surface with its\n  own `SCHEMA_VERSION` and its own convention (only `/api/meta` and\n  `/api/status` carry a version today). Standardizing `schema_version` across\n  all webui `/api/*` endpoints is a distinct unit of work, not taken here.\n- The `cairn export` envelope and the summariser request/response wire schemas\n  keep their own independent version constants; they are not command envelopes."
            }
          ]
        },
        "rationale": {
          "schema_version": 1,
          "node": "cairn.kernel.query",
          "artefacts": [
            {
              "type": "decisions",
              "path": "meta/decisions/kernel-tooling.md",
              "title": "Kernel tooling: changes, hooks, query API, and CLI",
              "frontmatter": {
                "date": "2026-06-16",
                "id": "dec.kernel-tooling",
                "status": "accepted"
              },
              "body": "\n# Kernel tooling: changes, hooks, query API, and CLI\n\n## Context\n\nBeyond graph construction, cairn needs mechanisms to mutate and query the graph, gate commits, and expose everything to users.\n\n## Decision\n\nProvide four tooling modules under `cairn.kernel`:\n\n- **Changes**: change directories, delta parsing, and archive acceptance.\n- **Hooks**: commit and task-boundary gates (`structural`, `interface`, `tension`, `all`).\n- **Query API**: structured query handlers used by the CLI, web UI, and MCP wrapper.\n- **CLI**: primary user surface, command parsing, and output formatting.\n\n## Rationale\n\nKeeping these separate from the core graph pipeline lets the core stay focused on map integrity while tooling evolves independently. The query API is the stable boundary between cairn's internals and its surfaces.\n\n## Consequences\n\n- New commands start in `cairn.kernel.cli` and usually call into `cairn.kernel.query`.\n- Hook changes affect `scripts/` and CI, so they need extra test coverage.\n- The changes module is the only kernel submodule that writes to the working tree outside generated outputs."
            },
            {
              "type": "decisions",
              "path": "meta/decisions/query-json-schema-version.md",
              "title": "Query-API JSON envelopes carry a uniform schema_version",
              "frontmatter": {
                "date": "2026-06-23",
                "id": "dec.query-json-schema-version",
                "status": "accepted"
              },
              "body": "\n# Query-API JSON envelopes carry a uniform schema_version\n\n## Context\n\nThe `query_api` command surface (consumed by the CLI `--json` flag and the MCP\nserver) emitted inconsistent JSON. `cairn islands --json` carried a top-level\n`schema_version` (sourced from the map-domain `IslandsResponse` and\n`ISLANDS_SCHEMA_VERSION`), while `order`, `contract`, `context`, `status`,\n`lint`, `dependents`, and every other command emitted a `data` payload with no\nversion at all. A JSON consumer could branch on the islands version but had no\nversion to branch on for any other command.\n\nThe prior session handoff flagged this as the next candidate and marked it a\nmaintainer decision, because standardizing it is a user-facing output-contract\nchange. The maintainer chose to add `schema_version` everywhere.\n\n## Decision\n\nEvery `query_api` command's JSON `data` payload carries a top-level\n`schema_version` field, currently `1`. The stamp is applied at a single choke\npoint in `query_api::execute`: after `execute_data` returns, the data object is\nstamped with `query_api::SCHEMA_VERSION`. Because the CLI prints `data`\ndirectly and the MCP envelope wraps `data`, both surfaces share one versioned\ncontract from one constant.\n\nThe redundant per-handler stamp in `islands_json` was removed so the universal\nstamp is the single source of truth. `ISLANDS_SCHEMA_VERSION` remains a\nmap-layer library concept (still constructed and tested on `IslandsResponse`)\nbut no longer drives the CLI islands envelope version. The live islands output\nis byte-identical to before (its value was already `1`).\n\n## Rationale\n\nVersioning the `data` payload at one choke point, with one constant, beats\nper-command stamps: it cannot drift, every command is covered automatically\n(including future ones and the change-directory tools), and there is exactly one\nplace to bump when the contract changes. Per-command divergent versions were\nexplicitly rejected: the goal is a uniform contract a consumer branches on, not\na matrix of per-command versions.\n\nOnly JSON objects are stamped; every current command returns an object, so the\nstamp is universal in practice.\n\n## Consequences\n\n- Every `query_api` `--json` command output now contains `\"schema_version\": 1`\n  (serialised in alphabetical key position). Bumping the wire contract means\n  bumping `query_api::SCHEMA_VERSION`.\n- The webui HTTP surface (`src/ui/api.rs`) is a separate JSON surface with its\n  own `SCHEMA_VERSION` and its own convention (only `/api/meta` and\n  `/api/status` carry a version today). Standardizing `schema_version` across\n  all webui `/api/*` endpoints is a distinct unit of work, not taken here.\n- The `cairn export` envelope and the summariser request/response wire schemas\n  keep their own independent version constants; they are not command envelopes."
            }
          ]
        },
        "research": {
          "schema_version": 1,
          "node": "cairn.kernel.query",
          "artefacts": []
        },
        "sources": {
          "schema_version": 1,
          "node": "cairn.kernel.query",
          "artefacts": []
        },
        "todos": {
          "schema_version": 1,
          "node": "cairn.kernel.query",
          "artefacts": []
        }
      }
    },
    "cairn.kernel.scanner": {
      "id": "cairn.kernel.scanner",
      "endpoints": {
        "beads": {
          "schema_version": 1,
          "node": "cairn.kernel.scanner",
          "beads": []
        },
        "contract": {
          "schema_version": 1,
          "node": "cairn.kernel.scanner",
          "artefacts": []
        },
        "decisions": {
          "schema_version": 1,
          "node": "cairn.kernel.scanner",
          "artefacts": [
            {
              "type": "decisions",
              "path": "meta/decisions/graph-root-fingerprint.md",
              "title": "Graph-root fingerprint: reject the store, close edge drift, defer the aggregate root",
              "frontmatter": {
                "date": "2026-06-23",
                "id": "dec.graph-root-fingerprint",
                "status": "accepted"
              },
              "body": "\n# Graph-root fingerprint: reject the store, close edge drift, defer the aggregate root\n\n## Context\n\nA design question was raised: \"Dolt is git for SQL; should cairn be git for a\nknowledge graph?\" That frames two very different things, and conflating them\nwould push cairn toward the exact failure mode it exists to prevent.\n\nThe first reading is a **versioned graph STORE**: a Dolt-analogue that keeps the\nreconciled graph as the canonical, mutable-with-history artefact. The second is a\n**content-addressed fingerprint OF the reconciled graph**, an aggregate hash\nbound to a git commit. The first is a new source of truth; the second is a\nderived summary of the existing one.\n\nCairn already fingerprints reality, and already gates drift, at a fine\ngranularity:\n\n- **Per-target interface hashes.** `InterfaceFingerprint`\n  (`src/reconcile/fingerprint.rs`) is a deterministic hash of a node's sorted\n  public symbols, persisted as `TargetHashes` (`BTreeMap<String, String>`) in\n  `.cairn/state/interface-hashes.json`. The interface gate names the drifted\n  target (`CAIRN_INTERFACE_HASH_CHANGED`).\n- **Per-node structural fingerprints.** `NodeFingerprint` (kind, parent, sorted\n  paths) is collected into a versioned `BlueprintSnapshot`\n  (`BTreeMap<String, NodeFingerprint>`, schema version 1;\n  `src/scanner/state.rs:65-70`). The blueprint-change gate\n  (`check_blueprint_change_decisions`, `src/scanner/checks.rs:48-67`) names the\n  node whose shape changed and requires a covering decision\n  (`CAIRN_BLUEPRINT_CHANGE_NO_DECISION`).\n\nBoth maps use `BTreeMap`, so their iteration order is already deterministic.\n`.cairn/` is gitignored (`.gitignore:51`): the persisted state above is\nlocal-derived cache, never committed.\n\nTwo facts shape the decision below:\n\n- **There is a real gap: dependency-edge drift is not gated.** Declared\n  dependency edges are validated for endpoint existence only (`validate_edges`,\n  `src/map/build.rs:96-128`, `CAIRN_INTEGRITY_INVALID_EDGE_ENDPOINT`) and built\n  in-memory into `Graph.outbound`/`inbound` every scan for cycle, ordering, and\n  neighbourhood queries. They are never recorded in `BlueprintSnapshot` (which\n  carries `nodes` only) and never drift-compared. So adding, removing, or\n  retargeting a declared cross-module dependency is a structural change with no\n  covering-decision gate, unlike a node `kind`/`parent` change.\n- **An aggregate graph-root hash has no consumer.** Searching `src/` for\n  `GraphRoot`/`graph_root`/`Cairn-Graph-Root` returns nothing. Its one unique\n  capability, O(1) \"did the architecture change between commits A and B?\", is\n  hypothetical: nothing in cairn calls for it today.\n\n## Decision\n\n1. **Reject a versioned graph store** (the Dolt-analogue). The reconciled graph\n   is derived, not authored. Authored inputs (the blueprint and artefacts as\n   markdown, source files as code) already live in git's content-addressed object\n   store, which gives history, branch, merge, and diff for free. You version the\n   inputs and recompute the projection; you do not version the projection. A\n   second canonical store is the two-source-of-truth drift trap that\n   `dec.no-orchestrator` and `dec.bd-upgrade-plan` already reject in their\n   domains.\n\n2. **Close the real gap with the existing pattern: gate dependency-edge drift.**\n   Record each node's outbound edge set in `BlueprintSnapshot` (or a sibling\n   snapshot keyed by node id, deterministically ordered like the existing maps),\n   and extend `check_blueprint_change_decisions` to emit\n   `CAIRN_BLUEPRINT_CHANGE_NO_DECISION` for a node whose declared edge set\n   changed without a covering decision. This keeps the per-node, actionable\n   granularity cairn already commits to (`dec.code-reconciliation`: per-node\n   hashes \"let the gate identify which module drifted\") and reuses the snapshot\n   and finding machinery wholesale. No aggregate, no opaque \"something changed\".\n\n3. **Defer the aggregate graph-root fingerprint.** It is the correct shape *if* a\n   consumer ever appears (a derived Merkle-style fold over the deterministically\n   ordered node fingerprints, edges, and interface hashes, reusing\n   `InterfaceFingerprint` as the single hash primitive, always recomputed and\n   never read back as authority). But it has no consumer today and its value is\n   gated on the revisit triggers above. Adopting it now would be the same\n   premature abstraction this decision rejects the store for, and an opaque\n   aggregate would regress the per-node/per-target finding granularity rather than\n   add to enforcement. Recompute-and-compare at the existing granularity stays the\n   gate.\n\n4. **Bind nothing to the commit now.** The drift value is always recomputed from\n   the tree and is always available, so a committed copy is never load-bearing:\n   present-and-matching is redundant, present-and-stale is a worse signal than the\n   per-node gate, and absent is the routine state after any rebase, squash, or\n   amend. A committed value is also actively unsafe here: `InterfaceFingerprint`\n   uses `DefaultHasher` (`src/reconcile/fingerprint.rs:23`), a SipHash whose\n   output is not guaranteed stable across Rust versions, while CI runs a floating\n   stable toolchain. A cross-machine committed hash could therefore differ from a\n   local recompute with zero real drift, producing a false-positive finding. If a\n   binding is ever justified (a real cross-clone consumer plus a pinned,\n   cross-version-stable hash), a `Cairn-Graph-Root:` commit trailer is the\n   least-bad mechanism: it travels with git natively, adds no tree-diff noise, and\n   stays non-authoritative. A tracked `.cairn/state/root` would require un-ignoring\n   derived state and version a projection (contradicting point 1); a git note is\n   invisible to contributors and needs separate refspec plumbing.\n\n## Rationale\n\n**Why not a versioned graph DB.** Dolt exists because SQL had no native version\ncontrol: tables are mutable in place. Cairn has the opposite problem already\nsolved, because its authored inputs are git blobs. A versioned graph store would\nduplicate git's job for a projection that should be recomputed, and would\nreintroduce divergence between \"the graph the store remembers\" and \"the graph the\ncode reconciles to\" (the same lesson as jsonl-vs-Dolt in `dec.bd-upgrade-plan`).\n\n**Why the edge-drift gate is the real win.** The edge gap is genuine and\nnon-coverable by anything today, and it sits squarely in cairn's lane: gating\nstructural drift against covering decisions. Closing it via the snapshot pattern\ndelivers an actionable finding (\"node X's dependency edges changed without a\ndecision\") with no new abstraction, no new source of truth, and no new scan pass\n(edges are already recomputed in memory every scan).\n\n**Why defer the aggregate root.** An aggregate hash earns its keep only when a\nconsumer needs a single comparable value (cross-commit or cross-clone). Cairn has\nno such consumer, and the existing recompute-and-compare gate already answers\n\"did anything drift, and where?\" at finer granularity than a root ever could.\nBuilding the root now is cost (a fold, a CLI surface, a hash-stability obligation)\nfor a capability nothing uses.\n\n**Why no binding now.** Decision point 4 above: the recompute path always wins, so\nthe bound value is never load-bearing; and a committed value built on\n`DefaultHasher` is toolchain-dependent, so it is specifically wrong as a\ncross-machine artifact. The gitignore fact rules out the tracked-file option but\ndoes not by itself argue for any binding.\n\n## Consequences\n\n- **Adopt now (this spike's ruling):** reject the versioned store, and adopt the\n  *design* of the edge-drift gate (extend `BlueprintSnapshot` with edges; emit a\n  per-node change finding). **Defer** the aggregate graph-root fingerprint and any\n  commit-binding until a real consumer and a stable hash exist.\n- **Implementation is maintainer-gated and not started by this spike.** Per the\n  maintainer-directed posture in `meta/session-handoff.md`, scheduling the build\n  is George's call. One implementation bead (`cairn-9v1`) carries the actionable\n  unit: the dependency-edge-drift gate. The aggregate root and trailer remain a\n  recorded, deferred option, not filed for build.\n- **Non-goal, recorded explicitly:** cairn does not gain a versioned graph DB, a\n  Dolt-style mutable graph store, or any separate canonical graph store; and it\n  does not gain a committed graph-root artifact while the hash is toolchain-\n  dependent and no consumer exists. The graph stays derived and recomputed from\n  git-tracked inputs."
            },
            {
              "type": "decisions",
              "path": "meta/decisions/kernel-core.md",
              "title": "Kernel core: blueprint, artefacts, map, and scanner",
              "frontmatter": {
                "date": "2026-06-16",
                "id": "dec.kernel-core",
                "status": "accepted"
              },
              "body": "\n# Kernel core: blueprint, artefacts, map, and scanner\n\n## Context\n\nCAIRN's value is a typed architecture map that stays in sync with real code. This requires four tightly-coupled subsystems: parsing the declaration language, loading typed artefacts, building and validating a graph, and orchestrating reconcilers that compare the graph against source.\n\n## Decision\n\nKeep blueprint parsing, artefact loading, graph construction, and scan orchestration as the four core modules of `cairn.kernel`. They form a pipeline: blueprint and artefacts feed the graph, the scanner drives reconcilers and feeds findings back into the graph.\n\n## Rationale\n\nSeparating these concerns makes each unit testable and lets the CLI, query API, and web UI consume the same graph. The scanner is the orchestration hub because it is the only module that understands the full lifecycle: parse → reconcile → validate → emit.\n\n## Consequences\n\n- Changes to the blueprint grammar must update parser tests and scanner integration tests.\n- New artefact types extend `cairn.kernel.artefacts` and are consumed by `cairn.kernel.map` for validation.\n- The scanner owns caching and incremental-scan state."
            },
            {
              "type": "decisions",
              "path": "meta/decisions/toolchain-lint-strictness.md",
              "title": "Adopt advisory toolchain-lint-strictness findings",
              "frontmatter": {
                "date": "2026-06-23",
                "id": "dec.toolchain-lint-strictness",
                "status": "accepted"
              },
              "body": "\n# Adopt advisory toolchain-lint-strictness findings\n\n## Context\n\nCairn reconciles the architecture map against reality. A recurring gap in host\nprojects is the absence of a strict lint configuration for a detected language:\na Rust crate may carry no clippy `-D warnings` configuration, or a JavaScript\nsurface may have no linter at all. This is one layer beyond pure architecture\n(it is project-health), but it is existence/linkage-shaped in the same way as\nexisting cairn checks such as contract presence and test-coverage linkage\n(cairn-a8z).\n\n## Decision\n\nAdopt a new advisory finding, `CAIRN_LINT_NOT_STRICT`, for tracked projects\nwhose detected primary language lacks a strict lint configuration.\n\n- **Finding code:** `CAIRN_LINT_NOT_STRICT`.\n- **Default severity:** `Warning` (non-blocking). Promote to blocking via the\n  existing `cairn lint --strict` flag.\n- **Scope:** config existence and strictness only. Cairn inspects configuration\n  files; it never invokes a linter or formatter.\n- **Per-language detection rules (initial):**\n  - **Rust:** strict clippy configuration is present. Acceptable signals include\n    a `.pre-commit-config.yaml` hook referencing `cargo clippy` with\n    `-D warnings` or equivalent, or `Cargo.toml` `[lints.clippy]` setting\n    warnings to `deny`. Presence of `cargo fmt` alone is not sufficient.\n  - **JavaScript / TypeScript:** an ESLint, Biome, or Oxlint configuration file\n    exists with at least one rule set to error (not warn-only) and the config is\n    referenced by CI or package scripts.\n  - **CSS:** a Stylelint or Biome CSS configuration exists with at least one\n    rule set to error.\n  - **Other languages:** defer. Do not emit the finding for languages without\n    defined detection rules.\n- **Node / project link:** the finding is attached to the node whose declared\n  paths contain the language evidence. If a node has no detectable primary\n  language, no finding is emitted.\n\n## Rationale\n\nThis follows the same pattern as cairn-a8z (test-coverage integrity): an\nadvisory-by-default, opt-in-strict existence check that respects cairn's\nboundary of observing reality rather than enforcing workflow. Keeping the check\nconfig-only means cairn stays out of the business of running external tools and\navoids version or platform coupling. The `Warning` default lets project teams\naddress the gap without blocking commits; `--strict` lets teams that want a hard\ngate opt in without adding a new config key.\n\n## Consequences\n\n- A future feature phase will implement the detector in the scanner/reconciler\n  path and add the finding to the registry.\n- Hook output may include `CAIRN_LINT_NOT_STRICT` under `cairn hook all` once\n  implemented, so the finding should be classified as advisory in hook reports.\n- Each new language added to the reconciler must either define a strict-lint\n  detection rule or explicitly opt out of this check for that language.\n- Cairn's own webui assets (`src/ui_assets/`) currently lack a strict CSS/JS\n  lint config. If those assets are claimed by a blueprint node, that node would\n  be flagged once the finding is implemented unless a strict config is added or\n  the node is excluded."
            }
          ]
        },
        "rationale": {
          "schema_version": 1,
          "node": "cairn.kernel.scanner",
          "artefacts": [
            {
              "type": "decisions",
              "path": "meta/decisions/graph-root-fingerprint.md",
              "title": "Graph-root fingerprint: reject the store, close edge drift, defer the aggregate root",
              "frontmatter": {
                "date": "2026-06-23",
                "id": "dec.graph-root-fingerprint",
                "status": "accepted"
              },
              "body": "\n# Graph-root fingerprint: reject the store, close edge drift, defer the aggregate root\n\n## Context\n\nA design question was raised: \"Dolt is git for SQL; should cairn be git for a\nknowledge graph?\" That frames two very different things, and conflating them\nwould push cairn toward the exact failure mode it exists to prevent.\n\nThe first reading is a **versioned graph STORE**: a Dolt-analogue that keeps the\nreconciled graph as the canonical, mutable-with-history artefact. The second is a\n**content-addressed fingerprint OF the reconciled graph**, an aggregate hash\nbound to a git commit. The first is a new source of truth; the second is a\nderived summary of the existing one.\n\nCairn already fingerprints reality, and already gates drift, at a fine\ngranularity:\n\n- **Per-target interface hashes.** `InterfaceFingerprint`\n  (`src/reconcile/fingerprint.rs`) is a deterministic hash of a node's sorted\n  public symbols, persisted as `TargetHashes` (`BTreeMap<String, String>`) in\n  `.cairn/state/interface-hashes.json`. The interface gate names the drifted\n  target (`CAIRN_INTERFACE_HASH_CHANGED`).\n- **Per-node structural fingerprints.** `NodeFingerprint` (kind, parent, sorted\n  paths) is collected into a versioned `BlueprintSnapshot`\n  (`BTreeMap<String, NodeFingerprint>`, schema version 1;\n  `src/scanner/state.rs:65-70`). The blueprint-change gate\n  (`check_blueprint_change_decisions`, `src/scanner/checks.rs:48-67`) names the\n  node whose shape changed and requires a covering decision\n  (`CAIRN_BLUEPRINT_CHANGE_NO_DECISION`).\n\nBoth maps use `BTreeMap`, so their iteration order is already deterministic.\n`.cairn/` is gitignored (`.gitignore:51`): the persisted state above is\nlocal-derived cache, never committed.\n\nTwo facts shape the decision below:\n\n- **There is a real gap: dependency-edge drift is not gated.** Declared\n  dependency edges are validated for endpoint existence only (`validate_edges`,\n  `src/map/build.rs:96-128`, `CAIRN_INTEGRITY_INVALID_EDGE_ENDPOINT`) and built\n  in-memory into `Graph.outbound`/`inbound` every scan for cycle, ordering, and\n  neighbourhood queries. They are never recorded in `BlueprintSnapshot` (which\n  carries `nodes` only) and never drift-compared. So adding, removing, or\n  retargeting a declared cross-module dependency is a structural change with no\n  covering-decision gate, unlike a node `kind`/`parent` change.\n- **An aggregate graph-root hash has no consumer.** Searching `src/` for\n  `GraphRoot`/`graph_root`/`Cairn-Graph-Root` returns nothing. Its one unique\n  capability, O(1) \"did the architecture change between commits A and B?\", is\n  hypothetical: nothing in cairn calls for it today.\n\n## Decision\n\n1. **Reject a versioned graph store** (the Dolt-analogue). The reconciled graph\n   is derived, not authored. Authored inputs (the blueprint and artefacts as\n   markdown, source files as code) already live in git's content-addressed object\n   store, which gives history, branch, merge, and diff for free. You version the\n   inputs and recompute the projection; you do not version the projection. A\n   second canonical store is the two-source-of-truth drift trap that\n   `dec.no-orchestrator` and `dec.bd-upgrade-plan` already reject in their\n   domains.\n\n2. **Close the real gap with the existing pattern: gate dependency-edge drift.**\n   Record each node's outbound edge set in `BlueprintSnapshot` (or a sibling\n   snapshot keyed by node id, deterministically ordered like the existing maps),\n   and extend `check_blueprint_change_decisions` to emit\n   `CAIRN_BLUEPRINT_CHANGE_NO_DECISION` for a node whose declared edge set\n   changed without a covering decision. This keeps the per-node, actionable\n   granularity cairn already commits to (`dec.code-reconciliation`: per-node\n   hashes \"let the gate identify which module drifted\") and reuses the snapshot\n   and finding machinery wholesale. No aggregate, no opaque \"something changed\".\n\n3. **Defer the aggregate graph-root fingerprint.** It is the correct shape *if* a\n   consumer ever appears (a derived Merkle-style fold over the deterministically\n   ordered node fingerprints, edges, and interface hashes, reusing\n   `InterfaceFingerprint` as the single hash primitive, always recomputed and\n   never read back as authority). But it has no consumer today and its value is\n   gated on the revisit triggers above. Adopting it now would be the same\n   premature abstraction this decision rejects the store for, and an opaque\n   aggregate would regress the per-node/per-target finding granularity rather than\n   add to enforcement. Recompute-and-compare at the existing granularity stays the\n   gate.\n\n4. **Bind nothing to the commit now.** The drift value is always recomputed from\n   the tree and is always available, so a committed copy is never load-bearing:\n   present-and-matching is redundant, present-and-stale is a worse signal than the\n   per-node gate, and absent is the routine state after any rebase, squash, or\n   amend. A committed value is also actively unsafe here: `InterfaceFingerprint`\n   uses `DefaultHasher` (`src/reconcile/fingerprint.rs:23`), a SipHash whose\n   output is not guaranteed stable across Rust versions, while CI runs a floating\n   stable toolchain. A cross-machine committed hash could therefore differ from a\n   local recompute with zero real drift, producing a false-positive finding. If a\n   binding is ever justified (a real cross-clone consumer plus a pinned,\n   cross-version-stable hash), a `Cairn-Graph-Root:` commit trailer is the\n   least-bad mechanism: it travels with git natively, adds no tree-diff noise, and\n   stays non-authoritative. A tracked `.cairn/state/root` would require un-ignoring\n   derived state and version a projection (contradicting point 1); a git note is\n   invisible to contributors and needs separate refspec plumbing.\n\n## Rationale\n\n**Why not a versioned graph DB.** Dolt exists because SQL had no native version\ncontrol: tables are mutable in place. Cairn has the opposite problem already\nsolved, because its authored inputs are git blobs. A versioned graph store would\nduplicate git's job for a projection that should be recomputed, and would\nreintroduce divergence between \"the graph the store remembers\" and \"the graph the\ncode reconciles to\" (the same lesson as jsonl-vs-Dolt in `dec.bd-upgrade-plan`).\n\n**Why the edge-drift gate is the real win.** The edge gap is genuine and\nnon-coverable by anything today, and it sits squarely in cairn's lane: gating\nstructural drift against covering decisions. Closing it via the snapshot pattern\ndelivers an actionable finding (\"node X's dependency edges changed without a\ndecision\") with no new abstraction, no new source of truth, and no new scan pass\n(edges are already recomputed in memory every scan).\n\n**Why defer the aggregate root.** An aggregate hash earns its keep only when a\nconsumer needs a single comparable value (cross-commit or cross-clone). Cairn has\nno such consumer, and the existing recompute-and-compare gate already answers\n\"did anything drift, and where?\" at finer granularity than a root ever could.\nBuilding the root now is cost (a fold, a CLI surface, a hash-stability obligation)\nfor a capability nothing uses.\n\n**Why no binding now.** Decision point 4 above: the recompute path always wins, so\nthe bound value is never load-bearing; and a committed value built on\n`DefaultHasher` is toolchain-dependent, so it is specifically wrong as a\ncross-machine artifact. The gitignore fact rules out the tracked-file option but\ndoes not by itself argue for any binding.\n\n## Consequences\n\n- **Adopt now (this spike's ruling):** reject the versioned store, and adopt the\n  *design* of the edge-drift gate (extend `BlueprintSnapshot` with edges; emit a\n  per-node change finding). **Defer** the aggregate graph-root fingerprint and any\n  commit-binding until a real consumer and a stable hash exist.\n- **Implementation is maintainer-gated and not started by this spike.** Per the\n  maintainer-directed posture in `meta/session-handoff.md`, scheduling the build\n  is George's call. One implementation bead (`cairn-9v1`) carries the actionable\n  unit: the dependency-edge-drift gate. The aggregate root and trailer remain a\n  recorded, deferred option, not filed for build.\n- **Non-goal, recorded explicitly:** cairn does not gain a versioned graph DB, a\n  Dolt-style mutable graph store, or any separate canonical graph store; and it\n  does not gain a committed graph-root artifact while the hash is toolchain-\n  dependent and no consumer exists. The graph stays derived and recomputed from\n  git-tracked inputs."
            },
            {
              "type": "decisions",
              "path": "meta/decisions/kernel-core.md",
              "title": "Kernel core: blueprint, artefacts, map, and scanner",
              "frontmatter": {
                "date": "2026-06-16",
                "id": "dec.kernel-core",
                "status": "accepted"
              },
              "body": "\n# Kernel core: blueprint, artefacts, map, and scanner\n\n## Context\n\nCAIRN's value is a typed architecture map that stays in sync with real code. This requires four tightly-coupled subsystems: parsing the declaration language, loading typed artefacts, building and validating a graph, and orchestrating reconcilers that compare the graph against source.\n\n## Decision\n\nKeep blueprint parsing, artefact loading, graph construction, and scan orchestration as the four core modules of `cairn.kernel`. They form a pipeline: blueprint and artefacts feed the graph, the scanner drives reconcilers and feeds findings back into the graph.\n\n## Rationale\n\nSeparating these concerns makes each unit testable and lets the CLI, query API, and web UI consume the same graph. The scanner is the orchestration hub because it is the only module that understands the full lifecycle: parse → reconcile → validate → emit.\n\n## Consequences\n\n- Changes to the blueprint grammar must update parser tests and scanner integration tests.\n- New artefact types extend `cairn.kernel.artefacts` and are consumed by `cairn.kernel.map` for validation.\n- The scanner owns caching and incremental-scan state."
            },
            {
              "type": "decisions",
              "path": "meta/decisions/toolchain-lint-strictness.md",
              "title": "Adopt advisory toolchain-lint-strictness findings",
              "frontmatter": {
                "date": "2026-06-23",
                "id": "dec.toolchain-lint-strictness",
                "status": "accepted"
              },
              "body": "\n# Adopt advisory toolchain-lint-strictness findings\n\n## Context\n\nCairn reconciles the architecture map against reality. A recurring gap in host\nprojects is the absence of a strict lint configuration for a detected language:\na Rust crate may carry no clippy `-D warnings` configuration, or a JavaScript\nsurface may have no linter at all. This is one layer beyond pure architecture\n(it is project-health), but it is existence/linkage-shaped in the same way as\nexisting cairn checks such as contract presence and test-coverage linkage\n(cairn-a8z).\n\n## Decision\n\nAdopt a new advisory finding, `CAIRN_LINT_NOT_STRICT`, for tracked projects\nwhose detected primary language lacks a strict lint configuration.\n\n- **Finding code:** `CAIRN_LINT_NOT_STRICT`.\n- **Default severity:** `Warning` (non-blocking). Promote to blocking via the\n  existing `cairn lint --strict` flag.\n- **Scope:** config existence and strictness only. Cairn inspects configuration\n  files; it never invokes a linter or formatter.\n- **Per-language detection rules (initial):**\n  - **Rust:** strict clippy configuration is present. Acceptable signals include\n    a `.pre-commit-config.yaml` hook referencing `cargo clippy` with\n    `-D warnings` or equivalent, or `Cargo.toml` `[lints.clippy]` setting\n    warnings to `deny`. Presence of `cargo fmt` alone is not sufficient.\n  - **JavaScript / TypeScript:** an ESLint, Biome, or Oxlint configuration file\n    exists with at least one rule set to error (not warn-only) and the config is\n    referenced by CI or package scripts.\n  - **CSS:** a Stylelint or Biome CSS configuration exists with at least one\n    rule set to error.\n  - **Other languages:** defer. Do not emit the finding for languages without\n    defined detection rules.\n- **Node / project link:** the finding is attached to the node whose declared\n  paths contain the language evidence. If a node has no detectable primary\n  language, no finding is emitted.\n\n## Rationale\n\nThis follows the same pattern as cairn-a8z (test-coverage integrity): an\nadvisory-by-default, opt-in-strict existence check that respects cairn's\nboundary of observing reality rather than enforcing workflow. Keeping the check\nconfig-only means cairn stays out of the business of running external tools and\navoids version or platform coupling. The `Warning` default lets project teams\naddress the gap without blocking commits; `--strict` lets teams that want a hard\ngate opt in without adding a new config key.\n\n## Consequences\n\n- A future feature phase will implement the detector in the scanner/reconciler\n  path and add the finding to the registry.\n- Hook output may include `CAIRN_LINT_NOT_STRICT` under `cairn hook all` once\n  implemented, so the finding should be classified as advisory in hook reports.\n- Each new language added to the reconciler must either define a strict-lint\n  detection rule or explicitly opt out of this check for that language.\n- Cairn's own webui assets (`src/ui_assets/`) currently lack a strict CSS/JS\n  lint config. If those assets are claimed by a blueprint node, that node would\n  be flagged once the finding is implemented unless a strict config is added or\n  the node is excluded."
            }
          ]
        },
        "research": {
          "schema_version": 1,
          "node": "cairn.kernel.scanner",
          "artefacts": []
        },
        "sources": {
          "schema_version": 1,
          "node": "cairn.kernel.scanner",
          "artefacts": []
        },
        "todos": {
          "schema_version": 1,
          "node": "cairn.kernel.scanner",
          "artefacts": []
        }
      }
    },
    "cairn.lsp": {
      "id": "cairn.lsp",
      "endpoints": {
        "beads": {
          "schema_version": 1,
          "node": "cairn.lsp",
          "beads": []
        },
        "contract": {
          "schema_version": 1,
          "node": "cairn.lsp",
          "artefacts": []
        },
        "decisions": {
          "schema_version": 1,
          "node": "cairn.lsp",
          "artefacts": [
            {
              "type": "decisions",
              "path": "meta/decisions/lsp-diagnostics-server.md",
              "title": "LSP diagnostics server",
              "frontmatter": {
                "date": "2026-06-23",
                "id": "dec.lsp-diagnostics-server",
                "status": "accepted"
              },
              "body": "\n# LSP diagnostics server\n\n## Context\n\nOMP consumes language-server diagnostics for on-write feedback. cairn already produces findings via `cairn watch`; exposing them through a persistent LSP server lets orchestrators subscribe without parsing the CLI's JSON stream.\n\n## Decision\n\nAdd a `cairn.lsp` module that owns `src/lsp/` and `src/bin/cairn-lsp.rs`. The server uses `lsp-server` + `lsp-types` synchronously over stdio and publishes Cairn findings as `textDocument/publishDiagnostics` notifications. A background watch loop rescans the project and pushes diagnostic deltas.\n\n## Rationale\n\n- Synchronous implementation matches cairn's existing architecture; no async runtime is required.\n- `lsp-server` provides a blocking stdio transport and LSP handshake, while `lsp-types` gives portable diagnostic types.\n- Reusing `cairn watch`'s scan loop keeps the server deterministic and aligned with CLI behavior.\n\n## Consequences\n\n- The public crate API gains `cairn::lsp` with `LspOpts` and `run`.\n- New LSP-only diagnostics must be mapped from `Finding`; richer source-location mapping is future work.\n- The module needs its own decision and interface-hash tracking like `cairn.mcp`."
            }
          ]
        },
        "rationale": {
          "schema_version": 1,
          "node": "cairn.lsp",
          "artefacts": [
            {
              "type": "decisions",
              "path": "meta/decisions/lsp-diagnostics-server.md",
              "title": "LSP diagnostics server",
              "frontmatter": {
                "date": "2026-06-23",
                "id": "dec.lsp-diagnostics-server",
                "status": "accepted"
              },
              "body": "\n# LSP diagnostics server\n\n## Context\n\nOMP consumes language-server diagnostics for on-write feedback. cairn already produces findings via `cairn watch`; exposing them through a persistent LSP server lets orchestrators subscribe without parsing the CLI's JSON stream.\n\n## Decision\n\nAdd a `cairn.lsp` module that owns `src/lsp/` and `src/bin/cairn-lsp.rs`. The server uses `lsp-server` + `lsp-types` synchronously over stdio and publishes Cairn findings as `textDocument/publishDiagnostics` notifications. A background watch loop rescans the project and pushes diagnostic deltas.\n\n## Rationale\n\n- Synchronous implementation matches cairn's existing architecture; no async runtime is required.\n- `lsp-server` provides a blocking stdio transport and LSP handshake, while `lsp-types` gives portable diagnostic types.\n- Reusing `cairn watch`'s scan loop keeps the server deterministic and aligned with CLI behavior.\n\n## Consequences\n\n- The public crate API gains `cairn::lsp` with `LspOpts` and `run`.\n- New LSP-only diagnostics must be mapped from `Finding`; richer source-location mapping is future work.\n- The module needs its own decision and interface-hash tracking like `cairn.mcp`."
            }
          ]
        },
        "research": {
          "schema_version": 1,
          "node": "cairn.lsp",
          "artefacts": []
        },
        "sources": {
          "schema_version": 1,
          "node": "cairn.lsp",
          "artefacts": []
        },
        "todos": {
          "schema_version": 1,
          "node": "cairn.lsp",
          "artefacts": []
        }
      }
    },
    "cairn.macros": {
      "id": "cairn.macros",
      "endpoints": {
        "beads": {
          "schema_version": 1,
          "node": "cairn.macros",
          "beads": []
        },
        "contract": {
          "schema_version": 1,
          "node": "cairn.macros",
          "artefacts": []
        },
        "decisions": {
          "schema_version": 1,
          "node": "cairn.macros",
          "artefacts": [
            {
              "type": "decisions",
              "path": "meta/decisions/build-and-extension.md",
              "title": "Build and extension modules",
              "frontmatter": {
                "date": "2026-06-16",
                "id": "dec.build-and-extension",
                "status": "accepted"
              },
              "body": "\n# Build and extension modules\n\n## Context\n\ncairn has several modules that are not part of the daily graph pipeline but are essential for adoption, provenance, and agent-assisted workflows.\n\n## Decision\n\nKeep these as first-class modules:\n\n- **Macros**: proc-macro crate for compile-time attributes (e.g., `#[cairn_planned]`).\n- **Brownfield**: orphan grouping, candidate heuristics, and onboard analysis for existing codebases.\n- **Provenance**: trace sidecar primitives and provenance-chain helpers.\n- **SuggestedEdges**: queue for AI-suggested graph edges with triage workflows.\n- **Summariser**: LLM-assisted contract summarisation backend.\n\n## Rationale\n\nThese are distinct enough to warrant separate modules. Brownfield and summariser are especially important for adoption: most users do not start greenfield.\n\n## Consequences\n\n- Brownfield heuristics affect `cairn init --from-code` and `cairn refine`.\n- Provenance types are consumed by the artefact registry and decision coverage gate.\n- SuggestedEdges and Summariser both touch LLM outputs and need careful prompt/version management."
            }
          ]
        },
        "rationale": {
          "schema_version": 1,
          "node": "cairn.macros",
          "artefacts": [
            {
              "type": "decisions",
              "path": "meta/decisions/build-and-extension.md",
              "title": "Build and extension modules",
              "frontmatter": {
                "date": "2026-06-16",
                "id": "dec.build-and-extension",
                "status": "accepted"
              },
              "body": "\n# Build and extension modules\n\n## Context\n\ncairn has several modules that are not part of the daily graph pipeline but are essential for adoption, provenance, and agent-assisted workflows.\n\n## Decision\n\nKeep these as first-class modules:\n\n- **Macros**: proc-macro crate for compile-time attributes (e.g., `#[cairn_planned]`).\n- **Brownfield**: orphan grouping, candidate heuristics, and onboard analysis for existing codebases.\n- **Provenance**: trace sidecar primitives and provenance-chain helpers.\n- **SuggestedEdges**: queue for AI-suggested graph edges with triage workflows.\n- **Summariser**: LLM-assisted contract summarisation backend.\n\n## Rationale\n\nThese are distinct enough to warrant separate modules. Brownfield and summariser are especially important for adoption: most users do not start greenfield.\n\n## Consequences\n\n- Brownfield heuristics affect `cairn init --from-code` and `cairn refine`.\n- Provenance types are consumed by the artefact registry and decision coverage gate.\n- SuggestedEdges and Summariser both touch LLM outputs and need careful prompt/version management."
            }
          ]
        },
        "research": {
          "schema_version": 1,
          "node": "cairn.macros",
          "artefacts": []
        },
        "sources": {
          "schema_version": 1,
          "node": "cairn.macros",
          "artefacts": []
        },
        "todos": {
          "schema_version": 1,
          "node": "cairn.macros",
          "artefacts": []
        }
      }
    },
    "cairn.mcp": {
      "id": "cairn.mcp",
      "endpoints": {
        "beads": {
          "schema_version": 1,
          "node": "cairn.mcp",
          "beads": []
        },
        "contract": {
          "schema_version": 1,
          "node": "cairn.mcp",
          "artefacts": []
        },
        "decisions": {
          "schema_version": 1,
          "node": "cairn.mcp",
          "artefacts": [
            {
              "type": "decisions",
              "path": "meta/decisions/user-surfaces.md",
              "title": "User surfaces: web UI and MCP wrapper",
              "frontmatter": {
                "date": "2026-06-16",
                "id": "dec.user-surfaces",
                "status": "accepted"
              },
              "body": "\n# User surfaces: web UI and MCP wrapper\n\n## Context\n\nNot all consumers want a CLI. A read-only web graph explorer and an MCP tool wrapper make cairn accessible to agents and browser-based users.\n\n## Decision\n\nProvide two surfaces:\n\n- **UI (`cairn.ui`)**: an embedded HTTP server serving a read-only graph explorer.\n- **MCP (`cairn.mcp`)**: a Model Context Protocol server that exposes cairn queries as tools.\n\n## Rationale\n\nThe web UI is useful for human review of the architecture map. The MCP wrapper lets agent harnesses call cairn without shelling out, reducing latency and surface area.\n\n## Consequences\n\n- Both surfaces must consume the same query API as the CLI to avoid semantic drift.\n- UI assets live under `src/ui_assets/` and are served statically.\n- MCP schema changes require updating `src/mcp.rs` and any dependent clients."
            }
          ]
        },
        "rationale": {
          "schema_version": 1,
          "node": "cairn.mcp",
          "artefacts": [
            {
              "type": "decisions",
              "path": "meta/decisions/user-surfaces.md",
              "title": "User surfaces: web UI and MCP wrapper",
              "frontmatter": {
                "date": "2026-06-16",
                "id": "dec.user-surfaces",
                "status": "accepted"
              },
              "body": "\n# User surfaces: web UI and MCP wrapper\n\n## Context\n\nNot all consumers want a CLI. A read-only web graph explorer and an MCP tool wrapper make cairn accessible to agents and browser-based users.\n\n## Decision\n\nProvide two surfaces:\n\n- **UI (`cairn.ui`)**: an embedded HTTP server serving a read-only graph explorer.\n- **MCP (`cairn.mcp`)**: a Model Context Protocol server that exposes cairn queries as tools.\n\n## Rationale\n\nThe web UI is useful for human review of the architecture map. The MCP wrapper lets agent harnesses call cairn without shelling out, reducing latency and surface area.\n\n## Consequences\n\n- Both surfaces must consume the same query API as the CLI to avoid semantic drift.\n- UI assets live under `src/ui_assets/` and are served statically.\n- MCP schema changes require updating `src/mcp.rs` and any dependent clients."
            }
          ]
        },
        "research": {
          "schema_version": 1,
          "node": "cairn.mcp",
          "artefacts": []
        },
        "sources": {
          "schema_version": 1,
          "node": "cairn.mcp",
          "artefacts": []
        },
        "todos": {
          "schema_version": 1,
          "node": "cairn.mcp",
          "artefacts": []
        }
      }
    },
    "cairn.provenance": {
      "id": "cairn.provenance",
      "endpoints": {
        "beads": {
          "schema_version": 1,
          "node": "cairn.provenance",
          "beads": []
        },
        "contract": {
          "schema_version": 1,
          "node": "cairn.provenance",
          "artefacts": []
        },
        "decisions": {
          "schema_version": 1,
          "node": "cairn.provenance",
          "artefacts": [
            {
              "type": "decisions",
              "path": "meta/decisions/build-and-extension.md",
              "title": "Build and extension modules",
              "frontmatter": {
                "date": "2026-06-16",
                "id": "dec.build-and-extension",
                "status": "accepted"
              },
              "body": "\n# Build and extension modules\n\n## Context\n\ncairn has several modules that are not part of the daily graph pipeline but are essential for adoption, provenance, and agent-assisted workflows.\n\n## Decision\n\nKeep these as first-class modules:\n\n- **Macros**: proc-macro crate for compile-time attributes (e.g., `#[cairn_planned]`).\n- **Brownfield**: orphan grouping, candidate heuristics, and onboard analysis for existing codebases.\n- **Provenance**: trace sidecar primitives and provenance-chain helpers.\n- **SuggestedEdges**: queue for AI-suggested graph edges with triage workflows.\n- **Summariser**: LLM-assisted contract summarisation backend.\n\n## Rationale\n\nThese are distinct enough to warrant separate modules. Brownfield and summariser are especially important for adoption: most users do not start greenfield.\n\n## Consequences\n\n- Brownfield heuristics affect `cairn init --from-code` and `cairn refine`.\n- Provenance types are consumed by the artefact registry and decision coverage gate.\n- SuggestedEdges and Summariser both touch LLM outputs and need careful prompt/version management."
            }
          ]
        },
        "rationale": {
          "schema_version": 1,
          "node": "cairn.provenance",
          "artefacts": [
            {
              "type": "decisions",
              "path": "meta/decisions/build-and-extension.md",
              "title": "Build and extension modules",
              "frontmatter": {
                "date": "2026-06-16",
                "id": "dec.build-and-extension",
                "status": "accepted"
              },
              "body": "\n# Build and extension modules\n\n## Context\n\ncairn has several modules that are not part of the daily graph pipeline but are essential for adoption, provenance, and agent-assisted workflows.\n\n## Decision\n\nKeep these as first-class modules:\n\n- **Macros**: proc-macro crate for compile-time attributes (e.g., `#[cairn_planned]`).\n- **Brownfield**: orphan grouping, candidate heuristics, and onboard analysis for existing codebases.\n- **Provenance**: trace sidecar primitives and provenance-chain helpers.\n- **SuggestedEdges**: queue for AI-suggested graph edges with triage workflows.\n- **Summariser**: LLM-assisted contract summarisation backend.\n\n## Rationale\n\nThese are distinct enough to warrant separate modules. Brownfield and summariser are especially important for adoption: most users do not start greenfield.\n\n## Consequences\n\n- Brownfield heuristics affect `cairn init --from-code` and `cairn refine`.\n- Provenance types are consumed by the artefact registry and decision coverage gate.\n- SuggestedEdges and Summariser both touch LLM outputs and need careful prompt/version management."
            }
          ]
        },
        "research": {
          "schema_version": 1,
          "node": "cairn.provenance",
          "artefacts": []
        },
        "sources": {
          "schema_version": 1,
          "node": "cairn.provenance",
          "artefacts": []
        },
        "todos": {
          "schema_version": 1,
          "node": "cairn.provenance",
          "artefacts": []
        }
      }
    },
    "cairn.reconcile": {
      "id": "cairn.reconcile",
      "endpoints": {
        "beads": {
          "schema_version": 1,
          "node": "cairn.reconcile",
          "beads": []
        },
        "contract": {
          "schema_version": 1,
          "node": "cairn.reconcile",
          "artefacts": []
        },
        "decisions": {
          "schema_version": 1,
          "node": "cairn.reconcile",
          "artefacts": [
            {
              "type": "decisions",
              "path": "meta/decisions/code-reconciliation.md",
              "title": "Code reconciliation",
              "frontmatter": {
                "date": "2026-06-16",
                "id": "dec.code-reconciliation",
                "status": "accepted"
              },
              "body": "\n# Code reconciliation\n\n## Context\n\nA blueprint declaration is only useful if it reflects real code. cairn needs to extract public interfaces from source files and compare them against the declared graph.\n\n## Decision\n\nProvide a single `cairn.reconcile` module that dispatches to language-specific tree-sitter reconcilers (Rust, TypeScript, Python, Go). Each reconciler returns a `ReconcileReport` of claimed files, public symbols, and per-node interface fingerprints.\n\n## Rationale\n\nOne dispatch point keeps language parity manageable. Tree-sitter gives us parser reuse without shipping per-language compilers. Per-node interface hashes (not per-language global hashes) let the gate identify which module drifted.\n\n## Consequences\n\n- Adding a language means adding a new reconciler module and registering it in the scanner.\n- Interface fingerprint changes block the `interface` and `all` hooks.\n- The reconciler must attribute symbols to owner nodes so hashes are per-node."
            },
            {
              "type": "decisions",
              "path": "meta/decisions/graph-root-fingerprint.md",
              "title": "Graph-root fingerprint: reject the store, close edge drift, defer the aggregate root",
              "frontmatter": {
                "date": "2026-06-23",
                "id": "dec.graph-root-fingerprint",
                "status": "accepted"
              },
              "body": "\n# Graph-root fingerprint: reject the store, close edge drift, defer the aggregate root\n\n## Context\n\nA design question was raised: \"Dolt is git for SQL; should cairn be git for a\nknowledge graph?\" That frames two very different things, and conflating them\nwould push cairn toward the exact failure mode it exists to prevent.\n\nThe first reading is a **versioned graph STORE**: a Dolt-analogue that keeps the\nreconciled graph as the canonical, mutable-with-history artefact. The second is a\n**content-addressed fingerprint OF the reconciled graph**, an aggregate hash\nbound to a git commit. The first is a new source of truth; the second is a\nderived summary of the existing one.\n\nCairn already fingerprints reality, and already gates drift, at a fine\ngranularity:\n\n- **Per-target interface hashes.** `InterfaceFingerprint`\n  (`src/reconcile/fingerprint.rs`) is a deterministic hash of a node's sorted\n  public symbols, persisted as `TargetHashes` (`BTreeMap<String, String>`) in\n  `.cairn/state/interface-hashes.json`. The interface gate names the drifted\n  target (`CAIRN_INTERFACE_HASH_CHANGED`).\n- **Per-node structural fingerprints.** `NodeFingerprint` (kind, parent, sorted\n  paths) is collected into a versioned `BlueprintSnapshot`\n  (`BTreeMap<String, NodeFingerprint>`, schema version 1;\n  `src/scanner/state.rs:65-70`). The blueprint-change gate\n  (`check_blueprint_change_decisions`, `src/scanner/checks.rs:48-67`) names the\n  node whose shape changed and requires a covering decision\n  (`CAIRN_BLUEPRINT_CHANGE_NO_DECISION`).\n\nBoth maps use `BTreeMap`, so their iteration order is already deterministic.\n`.cairn/` is gitignored (`.gitignore:51`): the persisted state above is\nlocal-derived cache, never committed.\n\nTwo facts shape the decision below:\n\n- **There is a real gap: dependency-edge drift is not gated.** Declared\n  dependency edges are validated for endpoint existence only (`validate_edges`,\n  `src/map/build.rs:96-128`, `CAIRN_INTEGRITY_INVALID_EDGE_ENDPOINT`) and built\n  in-memory into `Graph.outbound`/`inbound` every scan for cycle, ordering, and\n  neighbourhood queries. They are never recorded in `BlueprintSnapshot` (which\n  carries `nodes` only) and never drift-compared. So adding, removing, or\n  retargeting a declared cross-module dependency is a structural change with no\n  covering-decision gate, unlike a node `kind`/`parent` change.\n- **An aggregate graph-root hash has no consumer.** Searching `src/` for\n  `GraphRoot`/`graph_root`/`Cairn-Graph-Root` returns nothing. Its one unique\n  capability, O(1) \"did the architecture change between commits A and B?\", is\n  hypothetical: nothing in cairn calls for it today.\n\n## Decision\n\n1. **Reject a versioned graph store** (the Dolt-analogue). The reconciled graph\n   is derived, not authored. Authored inputs (the blueprint and artefacts as\n   markdown, source files as code) already live in git's content-addressed object\n   store, which gives history, branch, merge, and diff for free. You version the\n   inputs and recompute the projection; you do not version the projection. A\n   second canonical store is the two-source-of-truth drift trap that\n   `dec.no-orchestrator` and `dec.bd-upgrade-plan` already reject in their\n   domains.\n\n2. **Close the real gap with the existing pattern: gate dependency-edge drift.**\n   Record each node's outbound edge set in `BlueprintSnapshot` (or a sibling\n   snapshot keyed by node id, deterministically ordered like the existing maps),\n   and extend `check_blueprint_change_decisions` to emit\n   `CAIRN_BLUEPRINT_CHANGE_NO_DECISION` for a node whose declared edge set\n   changed without a covering decision. This keeps the per-node, actionable\n   granularity cairn already commits to (`dec.code-reconciliation`: per-node\n   hashes \"let the gate identify which module drifted\") and reuses the snapshot\n   and finding machinery wholesale. No aggregate, no opaque \"something changed\".\n\n3. **Defer the aggregate graph-root fingerprint.** It is the correct shape *if* a\n   consumer ever appears (a derived Merkle-style fold over the deterministically\n   ordered node fingerprints, edges, and interface hashes, reusing\n   `InterfaceFingerprint` as the single hash primitive, always recomputed and\n   never read back as authority). But it has no consumer today and its value is\n   gated on the revisit triggers above. Adopting it now would be the same\n   premature abstraction this decision rejects the store for, and an opaque\n   aggregate would regress the per-node/per-target finding granularity rather than\n   add to enforcement. Recompute-and-compare at the existing granularity stays the\n   gate.\n\n4. **Bind nothing to the commit now.** The drift value is always recomputed from\n   the tree and is always available, so a committed copy is never load-bearing:\n   present-and-matching is redundant, present-and-stale is a worse signal than the\n   per-node gate, and absent is the routine state after any rebase, squash, or\n   amend. A committed value is also actively unsafe here: `InterfaceFingerprint`\n   uses `DefaultHasher` (`src/reconcile/fingerprint.rs:23`), a SipHash whose\n   output is not guaranteed stable across Rust versions, while CI runs a floating\n   stable toolchain. A cross-machine committed hash could therefore differ from a\n   local recompute with zero real drift, producing a false-positive finding. If a\n   binding is ever justified (a real cross-clone consumer plus a pinned,\n   cross-version-stable hash), a `Cairn-Graph-Root:` commit trailer is the\n   least-bad mechanism: it travels with git natively, adds no tree-diff noise, and\n   stays non-authoritative. A tracked `.cairn/state/root` would require un-ignoring\n   derived state and version a projection (contradicting point 1); a git note is\n   invisible to contributors and needs separate refspec plumbing.\n\n## Rationale\n\n**Why not a versioned graph DB.** Dolt exists because SQL had no native version\ncontrol: tables are mutable in place. Cairn has the opposite problem already\nsolved, because its authored inputs are git blobs. A versioned graph store would\nduplicate git's job for a projection that should be recomputed, and would\nreintroduce divergence between \"the graph the store remembers\" and \"the graph the\ncode reconciles to\" (the same lesson as jsonl-vs-Dolt in `dec.bd-upgrade-plan`).\n\n**Why the edge-drift gate is the real win.** The edge gap is genuine and\nnon-coverable by anything today, and it sits squarely in cairn's lane: gating\nstructural drift against covering decisions. Closing it via the snapshot pattern\ndelivers an actionable finding (\"node X's dependency edges changed without a\ndecision\") with no new abstraction, no new source of truth, and no new scan pass\n(edges are already recomputed in memory every scan).\n\n**Why defer the aggregate root.** An aggregate hash earns its keep only when a\nconsumer needs a single comparable value (cross-commit or cross-clone). Cairn has\nno such consumer, and the existing recompute-and-compare gate already answers\n\"did anything drift, and where?\" at finer granularity than a root ever could.\nBuilding the root now is cost (a fold, a CLI surface, a hash-stability obligation)\nfor a capability nothing uses.\n\n**Why no binding now.** Decision point 4 above: the recompute path always wins, so\nthe bound value is never load-bearing; and a committed value built on\n`DefaultHasher` is toolchain-dependent, so it is specifically wrong as a\ncross-machine artifact. The gitignore fact rules out the tracked-file option but\ndoes not by itself argue for any binding.\n\n## Consequences\n\n- **Adopt now (this spike's ruling):** reject the versioned store, and adopt the\n  *design* of the edge-drift gate (extend `BlueprintSnapshot` with edges; emit a\n  per-node change finding). **Defer** the aggregate graph-root fingerprint and any\n  commit-binding until a real consumer and a stable hash exist.\n- **Implementation is maintainer-gated and not started by this spike.** Per the\n  maintainer-directed posture in `meta/session-handoff.md`, scheduling the build\n  is George's call. One implementation bead (`cairn-9v1`) carries the actionable\n  unit: the dependency-edge-drift gate. The aggregate root and trailer remain a\n  recorded, deferred option, not filed for build.\n- **Non-goal, recorded explicitly:** cairn does not gain a versioned graph DB, a\n  Dolt-style mutable graph store, or any separate canonical graph store; and it\n  does not gain a committed graph-root artifact while the hash is toolchain-\n  dependent and no consumer exists. The graph stays derived and recomputed from\n  git-tracked inputs."
            }
          ]
        },
        "rationale": {
          "schema_version": 1,
          "node": "cairn.reconcile",
          "artefacts": [
            {
              "type": "decisions",
              "path": "meta/decisions/code-reconciliation.md",
              "title": "Code reconciliation",
              "frontmatter": {
                "date": "2026-06-16",
                "id": "dec.code-reconciliation",
                "status": "accepted"
              },
              "body": "\n# Code reconciliation\n\n## Context\n\nA blueprint declaration is only useful if it reflects real code. cairn needs to extract public interfaces from source files and compare them against the declared graph.\n\n## Decision\n\nProvide a single `cairn.reconcile` module that dispatches to language-specific tree-sitter reconcilers (Rust, TypeScript, Python, Go). Each reconciler returns a `ReconcileReport` of claimed files, public symbols, and per-node interface fingerprints.\n\n## Rationale\n\nOne dispatch point keeps language parity manageable. Tree-sitter gives us parser reuse without shipping per-language compilers. Per-node interface hashes (not per-language global hashes) let the gate identify which module drifted.\n\n## Consequences\n\n- Adding a language means adding a new reconciler module and registering it in the scanner.\n- Interface fingerprint changes block the `interface` and `all` hooks.\n- The reconciler must attribute symbols to owner nodes so hashes are per-node."
            },
            {
              "type": "decisions",
              "path": "meta/decisions/graph-root-fingerprint.md",
              "title": "Graph-root fingerprint: reject the store, close edge drift, defer the aggregate root",
              "frontmatter": {
                "date": "2026-06-23",
                "id": "dec.graph-root-fingerprint",
                "status": "accepted"
              },
              "body": "\n# Graph-root fingerprint: reject the store, close edge drift, defer the aggregate root\n\n## Context\n\nA design question was raised: \"Dolt is git for SQL; should cairn be git for a\nknowledge graph?\" That frames two very different things, and conflating them\nwould push cairn toward the exact failure mode it exists to prevent.\n\nThe first reading is a **versioned graph STORE**: a Dolt-analogue that keeps the\nreconciled graph as the canonical, mutable-with-history artefact. The second is a\n**content-addressed fingerprint OF the reconciled graph**, an aggregate hash\nbound to a git commit. The first is a new source of truth; the second is a\nderived summary of the existing one.\n\nCairn already fingerprints reality, and already gates drift, at a fine\ngranularity:\n\n- **Per-target interface hashes.** `InterfaceFingerprint`\n  (`src/reconcile/fingerprint.rs`) is a deterministic hash of a node's sorted\n  public symbols, persisted as `TargetHashes` (`BTreeMap<String, String>`) in\n  `.cairn/state/interface-hashes.json`. The interface gate names the drifted\n  target (`CAIRN_INTERFACE_HASH_CHANGED`).\n- **Per-node structural fingerprints.** `NodeFingerprint` (kind, parent, sorted\n  paths) is collected into a versioned `BlueprintSnapshot`\n  (`BTreeMap<String, NodeFingerprint>`, schema version 1;\n  `src/scanner/state.rs:65-70`). The blueprint-change gate\n  (`check_blueprint_change_decisions`, `src/scanner/checks.rs:48-67`) names the\n  node whose shape changed and requires a covering decision\n  (`CAIRN_BLUEPRINT_CHANGE_NO_DECISION`).\n\nBoth maps use `BTreeMap`, so their iteration order is already deterministic.\n`.cairn/` is gitignored (`.gitignore:51`): the persisted state above is\nlocal-derived cache, never committed.\n\nTwo facts shape the decision below:\n\n- **There is a real gap: dependency-edge drift is not gated.** Declared\n  dependency edges are validated for endpoint existence only (`validate_edges`,\n  `src/map/build.rs:96-128`, `CAIRN_INTEGRITY_INVALID_EDGE_ENDPOINT`) and built\n  in-memory into `Graph.outbound`/`inbound` every scan for cycle, ordering, and\n  neighbourhood queries. They are never recorded in `BlueprintSnapshot` (which\n  carries `nodes` only) and never drift-compared. So adding, removing, or\n  retargeting a declared cross-module dependency is a structural change with no\n  covering-decision gate, unlike a node `kind`/`parent` change.\n- **An aggregate graph-root hash has no consumer.** Searching `src/` for\n  `GraphRoot`/`graph_root`/`Cairn-Graph-Root` returns nothing. Its one unique\n  capability, O(1) \"did the architecture change between commits A and B?\", is\n  hypothetical: nothing in cairn calls for it today.\n\n## Decision\n\n1. **Reject a versioned graph store** (the Dolt-analogue). The reconciled graph\n   is derived, not authored. Authored inputs (the blueprint and artefacts as\n   markdown, source files as code) already live in git's content-addressed object\n   store, which gives history, branch, merge, and diff for free. You version the\n   inputs and recompute the projection; you do not version the projection. A\n   second canonical store is the two-source-of-truth drift trap that\n   `dec.no-orchestrator` and `dec.bd-upgrade-plan` already reject in their\n   domains.\n\n2. **Close the real gap with the existing pattern: gate dependency-edge drift.**\n   Record each node's outbound edge set in `BlueprintSnapshot` (or a sibling\n   snapshot keyed by node id, deterministically ordered like the existing maps),\n   and extend `check_blueprint_change_decisions` to emit\n   `CAIRN_BLUEPRINT_CHANGE_NO_DECISION` for a node whose declared edge set\n   changed without a covering decision. This keeps the per-node, actionable\n   granularity cairn already commits to (`dec.code-reconciliation`: per-node\n   hashes \"let the gate identify which module drifted\") and reuses the snapshot\n   and finding machinery wholesale. No aggregate, no opaque \"something changed\".\n\n3. **Defer the aggregate graph-root fingerprint.** It is the correct shape *if* a\n   consumer ever appears (a derived Merkle-style fold over the deterministically\n   ordered node fingerprints, edges, and interface hashes, reusing\n   `InterfaceFingerprint` as the single hash primitive, always recomputed and\n   never read back as authority). But it has no consumer today and its value is\n   gated on the revisit triggers above. Adopting it now would be the same\n   premature abstraction this decision rejects the store for, and an opaque\n   aggregate would regress the per-node/per-target finding granularity rather than\n   add to enforcement. Recompute-and-compare at the existing granularity stays the\n   gate.\n\n4. **Bind nothing to the commit now.** The drift value is always recomputed from\n   the tree and is always available, so a committed copy is never load-bearing:\n   present-and-matching is redundant, present-and-stale is a worse signal than the\n   per-node gate, and absent is the routine state after any rebase, squash, or\n   amend. A committed value is also actively unsafe here: `InterfaceFingerprint`\n   uses `DefaultHasher` (`src/reconcile/fingerprint.rs:23`), a SipHash whose\n   output is not guaranteed stable across Rust versions, while CI runs a floating\n   stable toolchain. A cross-machine committed hash could therefore differ from a\n   local recompute with zero real drift, producing a false-positive finding. If a\n   binding is ever justified (a real cross-clone consumer plus a pinned,\n   cross-version-stable hash), a `Cairn-Graph-Root:` commit trailer is the\n   least-bad mechanism: it travels with git natively, adds no tree-diff noise, and\n   stays non-authoritative. A tracked `.cairn/state/root` would require un-ignoring\n   derived state and version a projection (contradicting point 1); a git note is\n   invisible to contributors and needs separate refspec plumbing.\n\n## Rationale\n\n**Why not a versioned graph DB.** Dolt exists because SQL had no native version\ncontrol: tables are mutable in place. Cairn has the opposite problem already\nsolved, because its authored inputs are git blobs. A versioned graph store would\nduplicate git's job for a projection that should be recomputed, and would\nreintroduce divergence between \"the graph the store remembers\" and \"the graph the\ncode reconciles to\" (the same lesson as jsonl-vs-Dolt in `dec.bd-upgrade-plan`).\n\n**Why the edge-drift gate is the real win.** The edge gap is genuine and\nnon-coverable by anything today, and it sits squarely in cairn's lane: gating\nstructural drift against covering decisions. Closing it via the snapshot pattern\ndelivers an actionable finding (\"node X's dependency edges changed without a\ndecision\") with no new abstraction, no new source of truth, and no new scan pass\n(edges are already recomputed in memory every scan).\n\n**Why defer the aggregate root.** An aggregate hash earns its keep only when a\nconsumer needs a single comparable value (cross-commit or cross-clone). Cairn has\nno such consumer, and the existing recompute-and-compare gate already answers\n\"did anything drift, and where?\" at finer granularity than a root ever could.\nBuilding the root now is cost (a fold, a CLI surface, a hash-stability obligation)\nfor a capability nothing uses.\n\n**Why no binding now.** Decision point 4 above: the recompute path always wins, so\nthe bound value is never load-bearing; and a committed value built on\n`DefaultHasher` is toolchain-dependent, so it is specifically wrong as a\ncross-machine artifact. The gitignore fact rules out the tracked-file option but\ndoes not by itself argue for any binding.\n\n## Consequences\n\n- **Adopt now (this spike's ruling):** reject the versioned store, and adopt the\n  *design* of the edge-drift gate (extend `BlueprintSnapshot` with edges; emit a\n  per-node change finding). **Defer** the aggregate graph-root fingerprint and any\n  commit-binding until a real consumer and a stable hash exist.\n- **Implementation is maintainer-gated and not started by this spike.** Per the\n  maintainer-directed posture in `meta/session-handoff.md`, scheduling the build\n  is George's call. One implementation bead (`cairn-9v1`) carries the actionable\n  unit: the dependency-edge-drift gate. The aggregate root and trailer remain a\n  recorded, deferred option, not filed for build.\n- **Non-goal, recorded explicitly:** cairn does not gain a versioned graph DB, a\n  Dolt-style mutable graph store, or any separate canonical graph store; and it\n  does not gain a committed graph-root artifact while the hash is toolchain-\n  dependent and no consumer exists. The graph stays derived and recomputed from\n  git-tracked inputs."
            }
          ]
        },
        "research": {
          "schema_version": 1,
          "node": "cairn.reconcile",
          "artefacts": []
        },
        "sources": {
          "schema_version": 1,
          "node": "cairn.reconcile",
          "artefacts": []
        },
        "todos": {
          "schema_version": 1,
          "node": "cairn.reconcile",
          "artefacts": []
        }
      }
    },
    "cairn.root": {
      "id": "cairn.root",
      "endpoints": {
        "beads": {
          "schema_version": 1,
          "node": "cairn.root",
          "beads": []
        },
        "contract": {
          "schema_version": 1,
          "node": "cairn.root",
          "artefacts": []
        },
        "decisions": {
          "schema_version": 1,
          "node": "cairn.root",
          "artefacts": [
            {
              "type": "decisions",
              "path": "meta/decisions/bd-upgrade-plan.md",
              "title": "bd upgrade plan: keep jsonl-in-git, pin export config, defer the version bump",
              "frontmatter": {
                "date": "2026-06-23",
                "id": "dec.bd-upgrade-plan",
                "status": "accepted"
              },
              "body": "\n# bd upgrade plan: keep jsonl-in-git, pin export config, defer the version bump\n\n## Context\n\nThis repo tracks work in beads (`bd`). The installed tool is **1.0.4** (2026-05-07);\nthe latest stable is **1.0.5** (2026-05-28), and an unreleased 1.0.6 line adds\ncross-clone Dolt remote merge-safety work. Bead `cairn-dyc` asked for a deliberate\nupgrade plan rather than a blind `bd upgrade`, because crossing minor versions here\ntouches schema migrations and an opt-in default flip that can silently break this\nrepo's bead sync.\n\nTwo facts establish the blast radius:\n\n- **Sync model is jsonl-in-git, not Dolt remote.** `git ls-remote origin 'refs/dolt/*'`\n  returns nothing: there is no Dolt remote on `origin`. Cross-machine bead interchange\n  is the git-committed `.beads/issues.jsonl`. bd does run a local Dolt server as its\n  storage engine (`.beads/dolt` in a normal clone; in a git-worktree layout the store\n  lives in the main worktree's `.beads/` and bd resolves to it from any worktree), but\n  that store is local-only and is never pushed to `origin`.\n- **Auto-export is currently unpinned.** `bd config show` reports `export.auto` and\n  `export.git-add` as *defaults* (`true` on 1.0.4), not values set in\n  `.beads/config.yaml`. On 1.0.5+ the auto-export default flips to OPT-IN (false). If\n  we upgrade without pinning, the jsonl stops auto-refreshing and auto-staging, and\n  git-based bead sync breaks silently.\n\nRe-checking the claimed upgrade synergies against the *installed* 1.0.4 surface\nshrinks the case for upgrading now:\n\n| Claimed 1.0.5 synergy | Reality on 1.0.4 |\n|---|---|\n| `bd github` native sync (pull/push/sync) | **Already present in 1.0.4.** No upgrade needed. |\n| `bd create --defer <date>` -> deferred status | **Already present in 1.0.4** (`--defer` on create and update). |\n| Opt-in jsonl / Dolt-canonical formalization | Cosmetic here: we already treat Dolt as canonical and jsonl as a passive export. |\n| `types.custom` server-validated custom issue types | New in 1.0.5. We use a `spike` *label* today; `status.custom` is already available on 1.0.4. |\n| Ergonomics (per-id close reasons, `--skip-labels`, count-only JSON) | Minor quality-of-life. |\n\nThe two synergies most often cited (GitHub sync, defer) are already in hand, so the\nupgrade buys mainly `types.custom` plus ergonomics, against the cost of crossing\nmigrations 0040-0042 (FK/cascade) and the unreleased dependencies-PK reshape (0050).\n\n## Decision\n\n1. **Keep jsonl-in-git.** Dolt stays the local storage engine; `.beads/issues.jsonl`\n   stays the committed, human-diffable, upsert-only projection used for cross-machine\n   sync. Do not adopt a Dolt remote at this time.\n2. **Pin auto-export now**, independent of any upgrade, by writing to\n   `.beads/config.yaml`:\n\n   ```yaml\n   export.auto: true\n   export.git-add: true\n   ```\n\n   On 1.0.4 these match the defaults (a safe no-op functionally), but they pre-harden\n   the repo so a future 1.0.5+ upgrade cannot silently disable jsonl sync.\n3. **Defer the bd version bump.** Stay on 1.0.4 until a revisit trigger fires. The\n   high-value synergies (`bd github`, `--defer`) are already available; the remainder\n   does not justify crossing the migration boundary today.\n4. **Re-scope `cairn-y1m`** (bead<->GitHub sync spike) to evaluate the *existing*\n   `bd github` rather than building a custom ForgeDock-style label layer. The open\n   question is the maintainer's: whether to accept GitHub issues as a second source of\n   truth at all (divergence risk) and, if so, one-way (bead -> GH) vs bidirectional.\n5. **Do not adopt `types.custom` yet.** Continue using the `spike` label. Adopting\n   `--defer` to declutter `bd ready` of long-lived P3 spikes is recommended but\n   optional and is not mandated by this decision.\n\n## Deterministic upgrade procedure (when a trigger fires)\n\nRun this exact sequence; do not `bd upgrade` blindly.\n\n1. **Pre-flight.** Commit/push all bead work. Confirm `.beads/config.yaml` pins\n   `export.auto: true` and `export.git-add: true` (done by this decision).\n2. **Single-machine path (this repo today, no Dolt remote).**\n   - `bd upgrade` (or reinstall) to the target version.\n   - `bd doctor` and resolve any migration-content-skew warning.\n   - Verify auto-export: make a trivial bead edit, confirm `.beads/issues.jsonl`\n     refreshes and is git-staged.\n   - Commit the refreshed jsonl.\n3. **Dolt-remote path (NOT applicable here; documented for completeness).** If a\n   `refs/dolt/*` remote ever exists, all clones must `bd dolt push` + `pull` to the\n   same state BEFORE upgrading. One designated migrator runs the upgrade with\n   `BD_ALLOW_REMOTE_MIGRATE=1` and pushes; every other clone upgrades then pulls.\n   Independently-migrated clones that cross migrations 0040-0042 / 0050 un-synced\n   become permanently un-mergeable (Dolt hard-refuses), and 1.0.5's forward-schema\n   guard hard-fails older bd against a newer-migrated DB.\n4. **Re-evaluate `types.custom`** for promoting the `spike` label to a real type once\n   on 1.0.5+.\n\n## Rationale\n\n- The dominant near-term risk is the opt-in auto-export flip, and it is fully\n  mitigated by pinning config now, regardless of when the upgrade happens.\n- The cross-clone migration hazard is dormant because this repo has no Dolt remote;\n  pinning the procedure in writing keeps it dormant safely if a remote is ever added.\n- Deferring the bump avoids change for change's sake: the synergies that would have\n  justified it are already on 1.0.4.\n\n## Consequences\n\n- `.beads/config.yaml` now pins `export.auto`/`export.git-add`; the jsonl sync is\n  upgrade-safe.\n- `cairn-dyc` is satisfied and closed; `cairn-y1m` is re-scoped to \"evaluate\n  `bd github`\" with a maintainer do/don't decision still pending.\n- The repo stays on bd 1.0.4 until a revisit trigger fires; this document is the\n  runbook for that future upgrade."
            },
            {
              "type": "decisions",
              "path": "meta/decisions/bead-github-sync.md",
              "title": "Bead-GitHub sync: do not adopt GitHub issues as a second source of truth",
              "frontmatter": {
                "date": "2026-06-23",
                "id": "dec.bead-github-sync",
                "informed_by": "[res.gas-city-cairn-integration]",
                "status": "accepted"
              },
              "body": "\n# Bead-GitHub sync: do not adopt GitHub issues as a second source of truth\n\n## Context\n\n`cairn-y1m` asked whether and how cairn should sync bead issue state with GitHub\nissues, originally framed as a ForgeDock-style workflow-state label layer\n(`workflow:investigating`, `bead:open`, structured HTML-comment annotations,\n`gh` as the query interface). `dec.bd-upgrade-plan` (item 4) re-scoped the spike:\nevaluate the *existing* `bd github` surface first, and isolate the one open\nquestion that is genuinely the maintainer's, namely whether to accept GitHub\nissues as a second source of truth at all and, if so, one-way versus\nbidirectional.\n\nTwo facts shrink the question before any design work:\n\n- **`bd github` already ships in the installed bd 1.0.4.** `bd github\n  pull/push/sync/status/repos` exists today, configured via\n  `github.token`/`github.owner`/`github.repo` (or the `GITHUB_*` env vars). It\n  carries its own bead-id to issue-number identity mapping. No custom label\n  layer needs to be built to get bead-GitHub interchange.\n- **The canonical store is settled.** `dec.bd-upgrade-plan` fixed Dolt as the\n  local storage engine and `.beads/issues.jsonl` (jsonl-in-git, upsert-only) as\n  the committed cross-machine projection, with no Dolt remote on `origin`.\n\n## Decision\n\n**Recommendation: defer. Do not adopt GitHub issues as a second source of truth,\nand do not build any bead-GitHub sync surface inside cairn.** Three rulings\nfollow.\n\n1. **Do not build a custom label/annotation layer.** A ForgeDock-style\n   `workflow:*` / `bead:*` label scheme plus HTML-comment annotations would\n   duplicate `bd github`, which already exists. Custom code here is rejected.\n2. **Do not put bead-GitHub sync in cairn.** Per `dec.no-orchestrator`, cairn\n   owns the semantic layer (blueprint, typed artefacts, drift gate); work-item\n   coordination and process state live in the storage/orchestration layer\n   (beads, an external orchestrator). A GitHub-issue mirror is coordination, not\n   architecture truth, so it belongs to `bd github` or an orchestrator pack, not\n   to a cairn command, hook, or reconciler.\n3. **Keep a single source of truth: Dolt-local plus jsonl-in-git.** Do not\n   promote GitHub issues to canonical. If cross-platform visibility is ever\n   wanted, the only sanctioned shape is the one-way mirror in the design sketch\n   below, run opt-in, with GitHub treated as a read-only projection.\n\nThe maintainer's reserved call (whether to ever accept GitHub as a sanctioned\nprojection) stays open: adopting it would be a superseding decision, not a\nreversal of any guarantee this one makes.\n\n## Answers to the spike questions\n\n1. **Should we?** No, not as a second source of truth. The canonical store\n   (Dolt-local, jsonl-in-git) already gives cross-machine sync. A GitHub mirror\n   adds a divergence surface without replacing anything. The merge-conflict\n   class the team already hit on `issues.jsonl` is the concrete cost; visibility\n   is the only benefit, and it does not require canonical status.\n2. **Could we?** Yes, mechanically. `bd github sync` maps bead status and\n   priority to GitHub issue state and labels today. Bidirectional sync is\n   feasible but reintroduces the two-writer merge-conflict class. One-way\n   (bead to GitHub) is safe because GitHub never feeds back into the canonical\n   store.\n3. **Mechanism?** `bd github` (the bd CLI), not a cairn hook, bd plugin, or\n   GitHub Action that re-parses `issues.jsonl`. The mapping lives in bd, which\n   already owns it.\n4. **Identity?** Use `bd github`'s native bead-id to issue-number mapping. Do\n   not invent a `bead-id:cairn-xxx` label or store issue numbers in bead\n   metadata by hand.\n5. **Scope boundary vs OMP/cairn task-sync (cairn-d7s diagnostics server)?**\n   Distinct surfaces. cairn-d7s exposes cairn graph state (LSP/watch\n   diagnostics); this spike is about bead process state mirrored to GitHub. They\n   do not overlap, and neither pulls GitHub-issue sync into cairn.\n\n## Design sketch (only if a maintainer later opts in)\n\nA one-way, opt-in mirror. Never canonical, never bidirectional.\n\n- **Direction:** bead to GitHub only. GitHub issues are a read-only projection;\n  edits made on GitHub are not read back. The canonical store stays Dolt-local\n  plus jsonl-in-git.\n- **Mechanism:** `bd github push` (or `bd github sync` constrained to push),\n  invoked manually or from an orchestrator pack, never from a cairn command or\n  hook.\n- **Identity:** `bd github`'s native bead-id to issue-number mapping.\n- **Status and priority mapping:** bead `open`/`in_progress`/`closed` to GitHub\n  open/closed plus a small label set for the in-between and priority, owned by\n  bd's mapping, not a cairn convention.\n- **Cairn's role:** none. Cairn neither writes to GitHub nor reconciles GitHub\n  state into the graph.\n\n## Risks\n\n- **Two writers, one truth (rejected path).** Bidirectional sync lets a GitHub\n  edit and a `bd` edit diverge, recreating the `issues.jsonl` merge-conflict\n  class. The one-way mirror avoids this by construction.\n- **Scope creep into cairn (rejected by ruling 2).** A GitHub-sync command in\n  cairn would erode `dec.no-orchestrator` and blur the storage/semantic\n  boundary.\n- **Staleness of a one-way mirror.** A push-only mirror lags between pushes.\n  This is acceptable for a visibility projection and is the price of avoiding\n  bidirectional conflict.\n\n## Consequences\n\n- `cairn-y1m` is satisfied and can be closed: the spike's deliverable (a\n  recommendation with a design sketch and risks) is this document.\n- No code, command, hook, or blueprint edge is added for bead-GitHub sync.\n- The repo continues to track work in beads with Dolt-local plus jsonl-in-git as\n  the single source of truth.\n- If a maintainer later wants the mirror, the sanctioned shape is the one-way\n  `bd github push` design above, recorded by a superseding decision; cairn\n  remains uninvolved."
            },
            {
              "type": "decisions",
              "path": "meta/decisions/beads-task-layer.md",
              "title": "Beads as cairn's per-node task layer: a read-only derived view, not a Todo source",
              "frontmatter": {
                "date": "2026-06-23",
                "id": "dec.beads-task-layer",
                "informed_by": "[res.gas-city-cairn-integration]",
                "related": "[dec.no-orchestrator, dec.bead-github-sync]",
                "status": "accepted"
              },
              "body": "\n# Beads as cairn's per-node task layer: a read-only derived view, not a Todo source\n\n## Context\n\n`cairn-2z9` asked how to close the gap between cairn's two disconnected task\nworlds. Cairn HAS a first-class Todo artefact type (markdown under `meta/todos/`,\nsurfaced per node in the webui Todos panel), but this repo does not use it: task\ntracking lives in beads (`bd`), and beads are not in the cairn graph, so the\nper-node inspector and the webui Todos panel are empty. The proposal is to make\nnode-linked beads visible per node so a node's related tasks (with bd workflow\nstate) appear in the inspector.\n\nThree accepted decisions, one shipped feature, and the shipped storage boundary\nbound the question before any design work:\n\n- **`dec.bead-github-sync` (accepted).** Rejects a second source of truth and the\n  \"two writers, one truth\" divergence trap. Any task-layer design must keep a\n  single canonical store.\n- **The reader already exists.** `src/state/backlog.rs` (cairn.state, PR #140)\n  parses the passive `.beads/issues.jsonl` export into `BacklogItem`, is strictly\n  read-only (\"Beads remains the single source of truth\"), and already implements\n  the node link: `BacklogItem::linked_node()` strips a `cairn-node:<id>` label and\n  returns the bound node. Unlinked beads return `None`. PR #140 already surfaces\n  the bead-to-node link via CLI (`cairn get`, `cairn next`); the missing piece is\n  the inverse per-node grouping in the webui inspector.\n- **The shipped storage boundary keeps content in files.** `src/state/mod.rs:3-5`:\n  the `StateBackend` enum \"abstracts artefact *state* storage (status, claim,\n  ready-queries) from the filesystem default. Content (markdown bodies, blueprint\n  text) stays as files unconditionally.\" This decision's own informing research\n  records the refocus that produced that boundary: `#97` \"refocused from\n  `ArtefactStore` to `StateBackend` (state only; content stays as files)\" and\n  `#99` \"refocused from Beads as content store to Beads as state backend\"\n  (`meta/research/gas-city-cairn-integration/README.md:61-62`). There is no\n  shipped content-source pluggability, and `ArtefactStore` does not exist in the\n  source tree.\n- **`cairn-87n` (`CAIRN_TEST_COVERAGE_MISSING`, shipped; closed `cairn-a8z`).**\n  Cairn's coverage gate keys off node *reconciliation* state (ghost/synced), never\n  bead workflow state. Enforcement is reality-based and independent of any task\n  surface.\n\nThe Todo consuming chain is markdown-bound at exactly one stage. `load_todos`\n(`src/artefacts/registry/mod.rs:44`) reads markdown into `Vec<Todo>`; everything\ndownstream (query_api serialise -> artefacts handler -> UI server -> webui Todos\npanel) consumes that vec.\n\n## Decision\n\n**Surface node-linked beads as a read-only, derived navigational view, not as\ncairn Todo artefacts. Reject the export bridge.** Four rulings follow. This spike\nrules the design only; no implementation lands here (per the bead's acceptance\ncriteria).\n\n1. **Native read-only view over export bridge; build is a follow-up.** The\n   sanctioned shape is a thin read-only projection over the existing\n   `.beads/issues.jsonl` reader, filtered by the `cairn-node:<id>` label that\n   `backlog.rs::linked_node()` already parses, rendered as a per-node view in the\n   webui inspector. The export bridge (generating `meta/todos/<node>/*.md` from\n   beads) is rejected: it creates a second on-disk representation, a sync step,\n   and a staleness window, the exact divergence trap `dec.bead-github-sync`\n   rejects. Building the view is a follow-up unit; this ruling fixes only its\n   shape.\n\n2. **Do not make beads a Todo artefact *source*; keep the view separate from the\n   Todo type.** Sourcing a Todo's body, created date, or satisfies clause from a\n   bead would make beads a content store, which contradicts the shipped boundary\n   (\"content stays as files unconditionally\", `src/state/mod.rs:4-5`) and the\n   `#97`/`#99` refocus away from \"Beads as content store\". So the beads view is\n   **derived navigation, not a stored artefact**: it does not populate the `Todo`\n   struct, does not write to `meta/todos/`, and does not redefine the Todo type.\n   Crucially, **this keeps spec.md:11 (\"typed markdown files attached to nodes\")\n   and §8.2 Todo intact**: no spec invariant is bent and no spec amendment is\n   needed. Making node-linked beads a genuine Todo *source* would require a\n   maintainer-ratified spec.md:11 / §8.2 amendment first (recorded as a revisit\n   trigger); that path is explicitly out of scope here.\n\n3. **Tasks remain navigation, never enforcement (declared-not-verified\n   preserved).** A bead's status is a navigational claim cairn surfaces, not a\n   reconciled fact. Cairn must never gate on bead status. Enforcement stays\n   reality-based: `cairn-87n`'s coverage gate keys off reconciliation state, so\n   surfacing bead status changes nothing cairn enforces. This supersedes the\n   `cairn-2z9` framing that bead workflow-state would \"ride\" the TDD gate: the\n   shipped `cairn-87n` design makes the gate independent of bead state.\n\n4. **Single source of truth, no second projection.** Beads (local Dolt;\n   `.beads/issues.jsonl` as the git-tracked reality layer) stays canonical. This\n   ruling stays inside the `dec.bead-github-sync` boundary: no GitHub projection,\n   no markdown mirror, no bidirectional sync. The view reads; it never writes.\n\n## Answers to the spike questions\n\n1. **Export bridge vs native loader, and why.** Native read-only view. The reader\n   and the `cairn-node:<id>` link already exist in `backlog.rs`; the bridge adds a\n   second representation and a sync/staleness surface that the single-source-of-\n   truth invariant rules out.\n\n2. **Is a non-markdown artefact source acceptable (spec.md:11)?** No, not as a\n   Todo content source. spec.md:11 defines artefacts as \"typed markdown files\n   attached to nodes\", and the shipped `StateBackend` keeps artefact content in\n   files unconditionally (state only is pluggable). So beads are surfaced as a\n   derived read-only view that does not redefine the Todo type and touches no\n   invariant. spec.md:11 is unchanged. (A genuine beads-as-Todo-source would need\n   a maintainer-ratified spec amendment first; out of scope.)\n\n3. **Node-link convention, status display, field rendering.**\n   - Link label: `cairn-node:<id>` (already implemented in\n     `BacklogItem::linked_node`). A bead without the label is unlinked: not\n     surfaced per node, not an error.\n   - The view renders bd state directly (the bead's `status`, `priority`, `title`,\n     `id`); it does not map into `TodoStatus`, because it is not a Todo. For\n     reference, the bd-status to cairn-`TodoStatus`\n     (`src/artefacts/registry/types.rs:62`) correspondence, were a maintainer to\n     later promote beads to a Todo source, would be: `open`->`Open`,\n     `in_progress`->`InProgress`, `closed`->`Done`, `blocked`->`Blocked`,\n     `deferred`->`Blocked` (no `Deferred` variant). That mapping is recorded for\n     the future path only; the read-only view itself displays bd state verbatim.\n\n4. **Integrity rule for orphan task-beads.** Mirror spec.md:339: a\n   `cairn-node:<id>` label that resolves to a deleted or unknown node is an orphan\n   -> **warning** (informational, non-blocking), matching markdown-todo orphans.\n   A bead with no `cairn-node:` label is simply unlinked (not an error), matching\n   `backlog.rs` returning `None`.\n\n5. **Tasks remain navigation/context, not enforcement.** Confirmed (ruling 3).\n   Declared-not-verified is preserved; bead status is never reconciled fact.\n\n6. **Relationship to `cairn-y1m` and `cairn-a8z`.**\n   - `cairn-y1m` (beads<->GitHub label sync): CLOSED, resolved by\n     `dec.bead-github-sync` (defer; no second source of truth). This ruling stays\n     inside that boundary: jsonl-in-git remains the single source; no GitHub\n     projection.\n   - `cairn-a8z` (TDD/coverage gate): CLOSED, resolved by `cairn-87n`. Its gate\n     keys off reconciliation state, not bead status, so the task layer does not\n     carry or \"ride\" bead workflow-state for enforcement. The task layer is purely\n     navigational.\n\n## Implementation (follow-up bead, out of scope here)\n\nA single small unit: add a read-only per-node beads view derived from\n`backlog.rs` (group `BacklogItem`s by `linked_node()`, expose via `query_api`,\nrender in the webui per-node inspector), emit the orphan warning, and render bd\nstate verbatim. No change to `load_todos`, no `Todo` artefacts minted, no\n`meta/todos/` files, and no spec.md:11 / §8.2 edit. If a maintainer later wants\nbeads promoted to a genuine Todo source, that is a separate change gated on a\nratified spec amendment.\n\n## Risks\n\n- **jsonl staleness vs canonical Dolt.** The git-tracked export can lag the local\n  Dolt DB. Accepted: this is the same reality layer the dev loop already reads\n  (PR #140), and the read-only view never writes a competing copy. Reconciliation\n  is the existing `bd export -o` discipline, not a cairn concern.\n- **View vs Todo confusion.** A per-node beads view sitting beside an (unused)\n  Todo panel could read as two task surfaces. Mitigated by this repo declaring no\n  `todos` pointers (the Todo panel is empty) and by the view being labelled as a\n  beads view, not todos.\n\n## Consequences\n\n- `cairn-2z9` is satisfied and can be closed: the spike's deliverable (this\n  ruling) exists. Implementation moves to a follow-up bead.\n- The per-node inspector can gain a sanctioned, single-source, read-only beads\n  view without a generator, a second source of truth, or any spec amendment.\n- spec.md:11 and §8.2 are untouched; promoting beads to a genuine Todo source\n  remains a maintainer-ratified, separate decision."
            },
            {
              "type": "decisions",
              "path": "meta/decisions/no-orchestrator.md",
              "title": "No Orchestrator: CAIRN does not ship its own orchestrator",
              "frontmatter": {
                "date": "2026-05-13",
                "id": "dec.no-orchestrator",
                "informed_by": "[res.gas-city-cairn-integration]",
                "status": "accepted"
              },
              "body": "\n# No Orchestrator: CAIRN does not ship its own orchestrator\n\n## Context\n\nCairness was scoped as a lightweight orchestrator on top of CAIRN: graph-walking wave scheduling, YAML DAG flow engine, adapter registry, metrics dashboard, and self-improvement loop. The original estimate was 2,000 to 3,000 lines of Rust for a standalone harness.\n\nGas City (Steve Yegge's orchestrator, `gastownhall/gascity`) has since matured to production grade. Its controller provides declarative `city.toml` configuration, fsnotify-driven hot reload, pool evaluation in parallel, crash quarantine with `max_restarts`/`restart_window`, graceful two-pass shutdown, single-controller `flock` on `.gc/controller.lock`, and Unix-socket IPC. The extension model (packs, formulas, prompt templates, runtime providers) is designed for external contributors: `gastownhall/gascity-packs` exists as the community pack home.\n\nBeads (`gastownhall/beads`) provides distributed graph storage with hash-based merge-safe IDs, Dolt versioning, and federation via Wasteland. It is independently installable (`brew install beads` / `npm install -g @beads/bd`), orchestrator-independent.\n\nThe cairness issue inventory (#1, #2, #6, #7, #9, #10, #14) was evaluated against Gas City's actual codebase. Overlap analysis follows in the Rationale section.\n\n## Decision\n\nCAIRN does not own an orchestrator. Four consequences follow:\n\n1. **Integration via contract.** A documented integration contract (GH #96) defines the stable CLI surface, JSON schema per command, exit-code taxonomy, event envelope, and subscription primitive that any orchestrator needs to drive CAIRN.\n2. **Reference adapters under `adapters/`.** The first adapter (`adapters/gascity/`) packages CAIRN as a Gas City pack: formula definitions, prompt templates, and a thin shim that shells to `cairn`. Future runners get their own adapter directory.\n3. **Cairness as scoped is retired.** The standalone cairness project is superseded. The graph-walking scheduler (~400 LOC), the one novel piece with no Gas City analogue, survives as a Gas City formula in `adapters/gascity/`.\n4. **cflx is retired.** CAIRN's own `accept`/`archive` primitives run under any external orchestrator (or none). The cflx workflow runner is no longer maintained.\n\n## Rationale\n\nBuilding a CAIRN-owned orchestrator would duplicate approximately 70% of Gas City's mature surface while losing community, audit, and federation benefits. The unique CAIRN value (typed artefacts, two-chain topology, drift gate, blueprint reconciliation) has zero analogue in Gas City.\n\nThe structural argument resolves cleanly into three non-competing layers:\n\n- **Layer 3 (Orchestration, optional):** Gas City controller, sessions, packs, formulas. CAIRN consumed as formula steps.\n- **Layer 2 (Semantic, CAIRN's lane):** blueprint, typed artefacts, two-chain topology, reconciler, drift gate, interface hashes. No equivalent in Gas City.\n- **Layer 1 (Storage, pluggable):** Default filesystem. Optional Beads (Dolt-backed). CAIRN trait: `ArtefactStore`.\n\nThese compose. They do not compete.\n\n## Consequences\n\n1. The graph-walking scheduler from cairness (#7) survives as a Gas City formula in `adapters/gascity/`.\n2. CAIRN stays focused on architecture-truth.\n3. Spec section 4 requires amendment to reflect that workflow lives in external skills and optional formulas, not as a CAIRN non-goal.\n4. Three operational paths emerge: CAIRN queries drive Gas City formulas; Beads-mediated typed beads become work items; SSE reactive events on graph state changes.\n5. Contribution path is `gastownhall/gascity-packs` as `packs/cairn-governance/`."
            },
            {
              "type": "decisions",
              "path": "meta/decisions/root-module.md",
              "title": "Root module",
              "frontmatter": {
                "date": "2026-06-16",
                "id": "dec.root-module",
                "status": "accepted"
              },
              "body": "\n# Root module\n\n## Context\n\ncairn needs a single entry point that ties together the library, binary targets, shared error types, and verification helpers.\n\n## Decision\n\nKeep a `cairn.root` module that claims `src/main.rs`, `src/lib.rs`, `src/error.rs`, `src/verification.rs`, and `src/signal.rs`.\n\n## Rationale\n\nWithout this module, these files would be orphaned on every scan. Grouping them under one node reflects that they are the crate boundary, not domain logic.\n\n## Consequences\n\n- New top-level source files should either join `cairn.root` or spawn a new top-level module with its own decision.\n- Changes to `src/lib.rs` public API are high-impact and should be gated carefully."
            }
          ]
        },
        "rationale": {
          "schema_version": 1,
          "node": "cairn.root",
          "artefacts": [
            {
              "type": "decisions",
              "path": "meta/decisions/bd-upgrade-plan.md",
              "title": "bd upgrade plan: keep jsonl-in-git, pin export config, defer the version bump",
              "frontmatter": {
                "date": "2026-06-23",
                "id": "dec.bd-upgrade-plan",
                "status": "accepted"
              },
              "body": "\n# bd upgrade plan: keep jsonl-in-git, pin export config, defer the version bump\n\n## Context\n\nThis repo tracks work in beads (`bd`). The installed tool is **1.0.4** (2026-05-07);\nthe latest stable is **1.0.5** (2026-05-28), and an unreleased 1.0.6 line adds\ncross-clone Dolt remote merge-safety work. Bead `cairn-dyc` asked for a deliberate\nupgrade plan rather than a blind `bd upgrade`, because crossing minor versions here\ntouches schema migrations and an opt-in default flip that can silently break this\nrepo's bead sync.\n\nTwo facts establish the blast radius:\n\n- **Sync model is jsonl-in-git, not Dolt remote.** `git ls-remote origin 'refs/dolt/*'`\n  returns nothing: there is no Dolt remote on `origin`. Cross-machine bead interchange\n  is the git-committed `.beads/issues.jsonl`. bd does run a local Dolt server as its\n  storage engine (`.beads/dolt` in a normal clone; in a git-worktree layout the store\n  lives in the main worktree's `.beads/` and bd resolves to it from any worktree), but\n  that store is local-only and is never pushed to `origin`.\n- **Auto-export is currently unpinned.** `bd config show` reports `export.auto` and\n  `export.git-add` as *defaults* (`true` on 1.0.4), not values set in\n  `.beads/config.yaml`. On 1.0.5+ the auto-export default flips to OPT-IN (false). If\n  we upgrade without pinning, the jsonl stops auto-refreshing and auto-staging, and\n  git-based bead sync breaks silently.\n\nRe-checking the claimed upgrade synergies against the *installed* 1.0.4 surface\nshrinks the case for upgrading now:\n\n| Claimed 1.0.5 synergy | Reality on 1.0.4 |\n|---|---|\n| `bd github` native sync (pull/push/sync) | **Already present in 1.0.4.** No upgrade needed. |\n| `bd create --defer <date>` -> deferred status | **Already present in 1.0.4** (`--defer` on create and update). |\n| Opt-in jsonl / Dolt-canonical formalization | Cosmetic here: we already treat Dolt as canonical and jsonl as a passive export. |\n| `types.custom` server-validated custom issue types | New in 1.0.5. We use a `spike` *label* today; `status.custom` is already available on 1.0.4. |\n| Ergonomics (per-id close reasons, `--skip-labels`, count-only JSON) | Minor quality-of-life. |\n\nThe two synergies most often cited (GitHub sync, defer) are already in hand, so the\nupgrade buys mainly `types.custom` plus ergonomics, against the cost of crossing\nmigrations 0040-0042 (FK/cascade) and the unreleased dependencies-PK reshape (0050).\n\n## Decision\n\n1. **Keep jsonl-in-git.** Dolt stays the local storage engine; `.beads/issues.jsonl`\n   stays the committed, human-diffable, upsert-only projection used for cross-machine\n   sync. Do not adopt a Dolt remote at this time.\n2. **Pin auto-export now**, independent of any upgrade, by writing to\n   `.beads/config.yaml`:\n\n   ```yaml\n   export.auto: true\n   export.git-add: true\n   ```\n\n   On 1.0.4 these match the defaults (a safe no-op functionally), but they pre-harden\n   the repo so a future 1.0.5+ upgrade cannot silently disable jsonl sync.\n3. **Defer the bd version bump.** Stay on 1.0.4 until a revisit trigger fires. The\n   high-value synergies (`bd github`, `--defer`) are already available; the remainder\n   does not justify crossing the migration boundary today.\n4. **Re-scope `cairn-y1m`** (bead<->GitHub sync spike) to evaluate the *existing*\n   `bd github` rather than building a custom ForgeDock-style label layer. The open\n   question is the maintainer's: whether to accept GitHub issues as a second source of\n   truth at all (divergence risk) and, if so, one-way (bead -> GH) vs bidirectional.\n5. **Do not adopt `types.custom` yet.** Continue using the `spike` label. Adopting\n   `--defer` to declutter `bd ready` of long-lived P3 spikes is recommended but\n   optional and is not mandated by this decision.\n\n## Deterministic upgrade procedure (when a trigger fires)\n\nRun this exact sequence; do not `bd upgrade` blindly.\n\n1. **Pre-flight.** Commit/push all bead work. Confirm `.beads/config.yaml` pins\n   `export.auto: true` and `export.git-add: true` (done by this decision).\n2. **Single-machine path (this repo today, no Dolt remote).**\n   - `bd upgrade` (or reinstall) to the target version.\n   - `bd doctor` and resolve any migration-content-skew warning.\n   - Verify auto-export: make a trivial bead edit, confirm `.beads/issues.jsonl`\n     refreshes and is git-staged.\n   - Commit the refreshed jsonl.\n3. **Dolt-remote path (NOT applicable here; documented for completeness).** If a\n   `refs/dolt/*` remote ever exists, all clones must `bd dolt push` + `pull` to the\n   same state BEFORE upgrading. One designated migrator runs the upgrade with\n   `BD_ALLOW_REMOTE_MIGRATE=1` and pushes; every other clone upgrades then pulls.\n   Independently-migrated clones that cross migrations 0040-0042 / 0050 un-synced\n   become permanently un-mergeable (Dolt hard-refuses), and 1.0.5's forward-schema\n   guard hard-fails older bd against a newer-migrated DB.\n4. **Re-evaluate `types.custom`** for promoting the `spike` label to a real type once\n   on 1.0.5+.\n\n## Rationale\n\n- The dominant near-term risk is the opt-in auto-export flip, and it is fully\n  mitigated by pinning config now, regardless of when the upgrade happens.\n- The cross-clone migration hazard is dormant because this repo has no Dolt remote;\n  pinning the procedure in writing keeps it dormant safely if a remote is ever added.\n- Deferring the bump avoids change for change's sake: the synergies that would have\n  justified it are already on 1.0.4.\n\n## Consequences\n\n- `.beads/config.yaml` now pins `export.auto`/`export.git-add`; the jsonl sync is\n  upgrade-safe.\n- `cairn-dyc` is satisfied and closed; `cairn-y1m` is re-scoped to \"evaluate\n  `bd github`\" with a maintainer do/don't decision still pending.\n- The repo stays on bd 1.0.4 until a revisit trigger fires; this document is the\n  runbook for that future upgrade."
            },
            {
              "type": "decisions",
              "path": "meta/decisions/bead-github-sync.md",
              "title": "Bead-GitHub sync: do not adopt GitHub issues as a second source of truth",
              "frontmatter": {
                "date": "2026-06-23",
                "id": "dec.bead-github-sync",
                "informed_by": "[res.gas-city-cairn-integration]",
                "status": "accepted"
              },
              "body": "\n# Bead-GitHub sync: do not adopt GitHub issues as a second source of truth\n\n## Context\n\n`cairn-y1m` asked whether and how cairn should sync bead issue state with GitHub\nissues, originally framed as a ForgeDock-style workflow-state label layer\n(`workflow:investigating`, `bead:open`, structured HTML-comment annotations,\n`gh` as the query interface). `dec.bd-upgrade-plan` (item 4) re-scoped the spike:\nevaluate the *existing* `bd github` surface first, and isolate the one open\nquestion that is genuinely the maintainer's, namely whether to accept GitHub\nissues as a second source of truth at all and, if so, one-way versus\nbidirectional.\n\nTwo facts shrink the question before any design work:\n\n- **`bd github` already ships in the installed bd 1.0.4.** `bd github\n  pull/push/sync/status/repos` exists today, configured via\n  `github.token`/`github.owner`/`github.repo` (or the `GITHUB_*` env vars). It\n  carries its own bead-id to issue-number identity mapping. No custom label\n  layer needs to be built to get bead-GitHub interchange.\n- **The canonical store is settled.** `dec.bd-upgrade-plan` fixed Dolt as the\n  local storage engine and `.beads/issues.jsonl` (jsonl-in-git, upsert-only) as\n  the committed cross-machine projection, with no Dolt remote on `origin`.\n\n## Decision\n\n**Recommendation: defer. Do not adopt GitHub issues as a second source of truth,\nand do not build any bead-GitHub sync surface inside cairn.** Three rulings\nfollow.\n\n1. **Do not build a custom label/annotation layer.** A ForgeDock-style\n   `workflow:*` / `bead:*` label scheme plus HTML-comment annotations would\n   duplicate `bd github`, which already exists. Custom code here is rejected.\n2. **Do not put bead-GitHub sync in cairn.** Per `dec.no-orchestrator`, cairn\n   owns the semantic layer (blueprint, typed artefacts, drift gate); work-item\n   coordination and process state live in the storage/orchestration layer\n   (beads, an external orchestrator). A GitHub-issue mirror is coordination, not\n   architecture truth, so it belongs to `bd github` or an orchestrator pack, not\n   to a cairn command, hook, or reconciler.\n3. **Keep a single source of truth: Dolt-local plus jsonl-in-git.** Do not\n   promote GitHub issues to canonical. If cross-platform visibility is ever\n   wanted, the only sanctioned shape is the one-way mirror in the design sketch\n   below, run opt-in, with GitHub treated as a read-only projection.\n\nThe maintainer's reserved call (whether to ever accept GitHub as a sanctioned\nprojection) stays open: adopting it would be a superseding decision, not a\nreversal of any guarantee this one makes.\n\n## Answers to the spike questions\n\n1. **Should we?** No, not as a second source of truth. The canonical store\n   (Dolt-local, jsonl-in-git) already gives cross-machine sync. A GitHub mirror\n   adds a divergence surface without replacing anything. The merge-conflict\n   class the team already hit on `issues.jsonl` is the concrete cost; visibility\n   is the only benefit, and it does not require canonical status.\n2. **Could we?** Yes, mechanically. `bd github sync` maps bead status and\n   priority to GitHub issue state and labels today. Bidirectional sync is\n   feasible but reintroduces the two-writer merge-conflict class. One-way\n   (bead to GitHub) is safe because GitHub never feeds back into the canonical\n   store.\n3. **Mechanism?** `bd github` (the bd CLI), not a cairn hook, bd plugin, or\n   GitHub Action that re-parses `issues.jsonl`. The mapping lives in bd, which\n   already owns it.\n4. **Identity?** Use `bd github`'s native bead-id to issue-number mapping. Do\n   not invent a `bead-id:cairn-xxx` label or store issue numbers in bead\n   metadata by hand.\n5. **Scope boundary vs OMP/cairn task-sync (cairn-d7s diagnostics server)?**\n   Distinct surfaces. cairn-d7s exposes cairn graph state (LSP/watch\n   diagnostics); this spike is about bead process state mirrored to GitHub. They\n   do not overlap, and neither pulls GitHub-issue sync into cairn.\n\n## Design sketch (only if a maintainer later opts in)\n\nA one-way, opt-in mirror. Never canonical, never bidirectional.\n\n- **Direction:** bead to GitHub only. GitHub issues are a read-only projection;\n  edits made on GitHub are not read back. The canonical store stays Dolt-local\n  plus jsonl-in-git.\n- **Mechanism:** `bd github push` (or `bd github sync` constrained to push),\n  invoked manually or from an orchestrator pack, never from a cairn command or\n  hook.\n- **Identity:** `bd github`'s native bead-id to issue-number mapping.\n- **Status and priority mapping:** bead `open`/`in_progress`/`closed` to GitHub\n  open/closed plus a small label set for the in-between and priority, owned by\n  bd's mapping, not a cairn convention.\n- **Cairn's role:** none. Cairn neither writes to GitHub nor reconciles GitHub\n  state into the graph.\n\n## Risks\n\n- **Two writers, one truth (rejected path).** Bidirectional sync lets a GitHub\n  edit and a `bd` edit diverge, recreating the `issues.jsonl` merge-conflict\n  class. The one-way mirror avoids this by construction.\n- **Scope creep into cairn (rejected by ruling 2).** A GitHub-sync command in\n  cairn would erode `dec.no-orchestrator` and blur the storage/semantic\n  boundary.\n- **Staleness of a one-way mirror.** A push-only mirror lags between pushes.\n  This is acceptable for a visibility projection and is the price of avoiding\n  bidirectional conflict.\n\n## Consequences\n\n- `cairn-y1m` is satisfied and can be closed: the spike's deliverable (a\n  recommendation with a design sketch and risks) is this document.\n- No code, command, hook, or blueprint edge is added for bead-GitHub sync.\n- The repo continues to track work in beads with Dolt-local plus jsonl-in-git as\n  the single source of truth.\n- If a maintainer later wants the mirror, the sanctioned shape is the one-way\n  `bd github push` design above, recorded by a superseding decision; cairn\n  remains uninvolved."
            },
            {
              "type": "decisions",
              "path": "meta/decisions/beads-task-layer.md",
              "title": "Beads as cairn's per-node task layer: a read-only derived view, not a Todo source",
              "frontmatter": {
                "date": "2026-06-23",
                "id": "dec.beads-task-layer",
                "informed_by": "[res.gas-city-cairn-integration]",
                "related": "[dec.no-orchestrator, dec.bead-github-sync]",
                "status": "accepted"
              },
              "body": "\n# Beads as cairn's per-node task layer: a read-only derived view, not a Todo source\n\n## Context\n\n`cairn-2z9` asked how to close the gap between cairn's two disconnected task\nworlds. Cairn HAS a first-class Todo artefact type (markdown under `meta/todos/`,\nsurfaced per node in the webui Todos panel), but this repo does not use it: task\ntracking lives in beads (`bd`), and beads are not in the cairn graph, so the\nper-node inspector and the webui Todos panel are empty. The proposal is to make\nnode-linked beads visible per node so a node's related tasks (with bd workflow\nstate) appear in the inspector.\n\nThree accepted decisions, one shipped feature, and the shipped storage boundary\nbound the question before any design work:\n\n- **`dec.bead-github-sync` (accepted).** Rejects a second source of truth and the\n  \"two writers, one truth\" divergence trap. Any task-layer design must keep a\n  single canonical store.\n- **The reader already exists.** `src/state/backlog.rs` (cairn.state, PR #140)\n  parses the passive `.beads/issues.jsonl` export into `BacklogItem`, is strictly\n  read-only (\"Beads remains the single source of truth\"), and already implements\n  the node link: `BacklogItem::linked_node()` strips a `cairn-node:<id>` label and\n  returns the bound node. Unlinked beads return `None`. PR #140 already surfaces\n  the bead-to-node link via CLI (`cairn get`, `cairn next`); the missing piece is\n  the inverse per-node grouping in the webui inspector.\n- **The shipped storage boundary keeps content in files.** `src/state/mod.rs:3-5`:\n  the `StateBackend` enum \"abstracts artefact *state* storage (status, claim,\n  ready-queries) from the filesystem default. Content (markdown bodies, blueprint\n  text) stays as files unconditionally.\" This decision's own informing research\n  records the refocus that produced that boundary: `#97` \"refocused from\n  `ArtefactStore` to `StateBackend` (state only; content stays as files)\" and\n  `#99` \"refocused from Beads as content store to Beads as state backend\"\n  (`meta/research/gas-city-cairn-integration/README.md:61-62`). There is no\n  shipped content-source pluggability, and `ArtefactStore` does not exist in the\n  source tree.\n- **`cairn-87n` (`CAIRN_TEST_COVERAGE_MISSING`, shipped; closed `cairn-a8z`).**\n  Cairn's coverage gate keys off node *reconciliation* state (ghost/synced), never\n  bead workflow state. Enforcement is reality-based and independent of any task\n  surface.\n\nThe Todo consuming chain is markdown-bound at exactly one stage. `load_todos`\n(`src/artefacts/registry/mod.rs:44`) reads markdown into `Vec<Todo>`; everything\ndownstream (query_api serialise -> artefacts handler -> UI server -> webui Todos\npanel) consumes that vec.\n\n## Decision\n\n**Surface node-linked beads as a read-only, derived navigational view, not as\ncairn Todo artefacts. Reject the export bridge.** Four rulings follow. This spike\nrules the design only; no implementation lands here (per the bead's acceptance\ncriteria).\n\n1. **Native read-only view over export bridge; build is a follow-up.** The\n   sanctioned shape is a thin read-only projection over the existing\n   `.beads/issues.jsonl` reader, filtered by the `cairn-node:<id>` label that\n   `backlog.rs::linked_node()` already parses, rendered as a per-node view in the\n   webui inspector. The export bridge (generating `meta/todos/<node>/*.md` from\n   beads) is rejected: it creates a second on-disk representation, a sync step,\n   and a staleness window, the exact divergence trap `dec.bead-github-sync`\n   rejects. Building the view is a follow-up unit; this ruling fixes only its\n   shape.\n\n2. **Do not make beads a Todo artefact *source*; keep the view separate from the\n   Todo type.** Sourcing a Todo's body, created date, or satisfies clause from a\n   bead would make beads a content store, which contradicts the shipped boundary\n   (\"content stays as files unconditionally\", `src/state/mod.rs:4-5`) and the\n   `#97`/`#99` refocus away from \"Beads as content store\". So the beads view is\n   **derived navigation, not a stored artefact**: it does not populate the `Todo`\n   struct, does not write to `meta/todos/`, and does not redefine the Todo type.\n   Crucially, **this keeps spec.md:11 (\"typed markdown files attached to nodes\")\n   and §8.2 Todo intact**: no spec invariant is bent and no spec amendment is\n   needed. Making node-linked beads a genuine Todo *source* would require a\n   maintainer-ratified spec.md:11 / §8.2 amendment first (recorded as a revisit\n   trigger); that path is explicitly out of scope here.\n\n3. **Tasks remain navigation, never enforcement (declared-not-verified\n   preserved).** A bead's status is a navigational claim cairn surfaces, not a\n   reconciled fact. Cairn must never gate on bead status. Enforcement stays\n   reality-based: `cairn-87n`'s coverage gate keys off reconciliation state, so\n   surfacing bead status changes nothing cairn enforces. This supersedes the\n   `cairn-2z9` framing that bead workflow-state would \"ride\" the TDD gate: the\n   shipped `cairn-87n` design makes the gate independent of bead state.\n\n4. **Single source of truth, no second projection.** Beads (local Dolt;\n   `.beads/issues.jsonl` as the git-tracked reality layer) stays canonical. This\n   ruling stays inside the `dec.bead-github-sync` boundary: no GitHub projection,\n   no markdown mirror, no bidirectional sync. The view reads; it never writes.\n\n## Answers to the spike questions\n\n1. **Export bridge vs native loader, and why.** Native read-only view. The reader\n   and the `cairn-node:<id>` link already exist in `backlog.rs`; the bridge adds a\n   second representation and a sync/staleness surface that the single-source-of-\n   truth invariant rules out.\n\n2. **Is a non-markdown artefact source acceptable (spec.md:11)?** No, not as a\n   Todo content source. spec.md:11 defines artefacts as \"typed markdown files\n   attached to nodes\", and the shipped `StateBackend` keeps artefact content in\n   files unconditionally (state only is pluggable). So beads are surfaced as a\n   derived read-only view that does not redefine the Todo type and touches no\n   invariant. spec.md:11 is unchanged. (A genuine beads-as-Todo-source would need\n   a maintainer-ratified spec amendment first; out of scope.)\n\n3. **Node-link convention, status display, field rendering.**\n   - Link label: `cairn-node:<id>` (already implemented in\n     `BacklogItem::linked_node`). A bead without the label is unlinked: not\n     surfaced per node, not an error.\n   - The view renders bd state directly (the bead's `status`, `priority`, `title`,\n     `id`); it does not map into `TodoStatus`, because it is not a Todo. For\n     reference, the bd-status to cairn-`TodoStatus`\n     (`src/artefacts/registry/types.rs:62`) correspondence, were a maintainer to\n     later promote beads to a Todo source, would be: `open`->`Open`,\n     `in_progress`->`InProgress`, `closed`->`Done`, `blocked`->`Blocked`,\n     `deferred`->`Blocked` (no `Deferred` variant). That mapping is recorded for\n     the future path only; the read-only view itself displays bd state verbatim.\n\n4. **Integrity rule for orphan task-beads.** Mirror spec.md:339: a\n   `cairn-node:<id>` label that resolves to a deleted or unknown node is an orphan\n   -> **warning** (informational, non-blocking), matching markdown-todo orphans.\n   A bead with no `cairn-node:` label is simply unlinked (not an error), matching\n   `backlog.rs` returning `None`.\n\n5. **Tasks remain navigation/context, not enforcement.** Confirmed (ruling 3).\n   Declared-not-verified is preserved; bead status is never reconciled fact.\n\n6. **Relationship to `cairn-y1m` and `cairn-a8z`.**\n   - `cairn-y1m` (beads<->GitHub label sync): CLOSED, resolved by\n     `dec.bead-github-sync` (defer; no second source of truth). This ruling stays\n     inside that boundary: jsonl-in-git remains the single source; no GitHub\n     projection.\n   - `cairn-a8z` (TDD/coverage gate): CLOSED, resolved by `cairn-87n`. Its gate\n     keys off reconciliation state, not bead status, so the task layer does not\n     carry or \"ride\" bead workflow-state for enforcement. The task layer is purely\n     navigational.\n\n## Implementation (follow-up bead, out of scope here)\n\nA single small unit: add a read-only per-node beads view derived from\n`backlog.rs` (group `BacklogItem`s by `linked_node()`, expose via `query_api`,\nrender in the webui per-node inspector), emit the orphan warning, and render bd\nstate verbatim. No change to `load_todos`, no `Todo` artefacts minted, no\n`meta/todos/` files, and no spec.md:11 / §8.2 edit. If a maintainer later wants\nbeads promoted to a genuine Todo source, that is a separate change gated on a\nratified spec amendment.\n\n## Risks\n\n- **jsonl staleness vs canonical Dolt.** The git-tracked export can lag the local\n  Dolt DB. Accepted: this is the same reality layer the dev loop already reads\n  (PR #140), and the read-only view never writes a competing copy. Reconciliation\n  is the existing `bd export -o` discipline, not a cairn concern.\n- **View vs Todo confusion.** A per-node beads view sitting beside an (unused)\n  Todo panel could read as two task surfaces. Mitigated by this repo declaring no\n  `todos` pointers (the Todo panel is empty) and by the view being labelled as a\n  beads view, not todos.\n\n## Consequences\n\n- `cairn-2z9` is satisfied and can be closed: the spike's deliverable (this\n  ruling) exists. Implementation moves to a follow-up bead.\n- The per-node inspector can gain a sanctioned, single-source, read-only beads\n  view without a generator, a second source of truth, or any spec amendment.\n- spec.md:11 and §8.2 are untouched; promoting beads to a genuine Todo source\n  remains a maintainer-ratified, separate decision."
            },
            {
              "type": "decisions",
              "path": "meta/decisions/no-orchestrator.md",
              "title": "No Orchestrator: CAIRN does not ship its own orchestrator",
              "frontmatter": {
                "date": "2026-05-13",
                "id": "dec.no-orchestrator",
                "informed_by": "[res.gas-city-cairn-integration]",
                "status": "accepted"
              },
              "body": "\n# No Orchestrator: CAIRN does not ship its own orchestrator\n\n## Context\n\nCairness was scoped as a lightweight orchestrator on top of CAIRN: graph-walking wave scheduling, YAML DAG flow engine, adapter registry, metrics dashboard, and self-improvement loop. The original estimate was 2,000 to 3,000 lines of Rust for a standalone harness.\n\nGas City (Steve Yegge's orchestrator, `gastownhall/gascity`) has since matured to production grade. Its controller provides declarative `city.toml` configuration, fsnotify-driven hot reload, pool evaluation in parallel, crash quarantine with `max_restarts`/`restart_window`, graceful two-pass shutdown, single-controller `flock` on `.gc/controller.lock`, and Unix-socket IPC. The extension model (packs, formulas, prompt templates, runtime providers) is designed for external contributors: `gastownhall/gascity-packs` exists as the community pack home.\n\nBeads (`gastownhall/beads`) provides distributed graph storage with hash-based merge-safe IDs, Dolt versioning, and federation via Wasteland. It is independently installable (`brew install beads` / `npm install -g @beads/bd`), orchestrator-independent.\n\nThe cairness issue inventory (#1, #2, #6, #7, #9, #10, #14) was evaluated against Gas City's actual codebase. Overlap analysis follows in the Rationale section.\n\n## Decision\n\nCAIRN does not own an orchestrator. Four consequences follow:\n\n1. **Integration via contract.** A documented integration contract (GH #96) defines the stable CLI surface, JSON schema per command, exit-code taxonomy, event envelope, and subscription primitive that any orchestrator needs to drive CAIRN.\n2. **Reference adapters under `adapters/`.** The first adapter (`adapters/gascity/`) packages CAIRN as a Gas City pack: formula definitions, prompt templates, and a thin shim that shells to `cairn`. Future runners get their own adapter directory.\n3. **Cairness as scoped is retired.** The standalone cairness project is superseded. The graph-walking scheduler (~400 LOC), the one novel piece with no Gas City analogue, survives as a Gas City formula in `adapters/gascity/`.\n4. **cflx is retired.** CAIRN's own `accept`/`archive` primitives run under any external orchestrator (or none). The cflx workflow runner is no longer maintained.\n\n## Rationale\n\nBuilding a CAIRN-owned orchestrator would duplicate approximately 70% of Gas City's mature surface while losing community, audit, and federation benefits. The unique CAIRN value (typed artefacts, two-chain topology, drift gate, blueprint reconciliation) has zero analogue in Gas City.\n\nThe structural argument resolves cleanly into three non-competing layers:\n\n- **Layer 3 (Orchestration, optional):** Gas City controller, sessions, packs, formulas. CAIRN consumed as formula steps.\n- **Layer 2 (Semantic, CAIRN's lane):** blueprint, typed artefacts, two-chain topology, reconciler, drift gate, interface hashes. No equivalent in Gas City.\n- **Layer 1 (Storage, pluggable):** Default filesystem. Optional Beads (Dolt-backed). CAIRN trait: `ArtefactStore`.\n\nThese compose. They do not compete.\n\n## Consequences\n\n1. The graph-walking scheduler from cairness (#7) survives as a Gas City formula in `adapters/gascity/`.\n2. CAIRN stays focused on architecture-truth.\n3. Spec section 4 requires amendment to reflect that workflow lives in external skills and optional formulas, not as a CAIRN non-goal.\n4. Three operational paths emerge: CAIRN queries drive Gas City formulas; Beads-mediated typed beads become work items; SSE reactive events on graph state changes.\n5. Contribution path is `gastownhall/gascity-packs` as `packs/cairn-governance/`."
            },
            {
              "type": "decisions",
              "path": "meta/decisions/root-module.md",
              "title": "Root module",
              "frontmatter": {
                "date": "2026-06-16",
                "id": "dec.root-module",
                "status": "accepted"
              },
              "body": "\n# Root module\n\n## Context\n\ncairn needs a single entry point that ties together the library, binary targets, shared error types, and verification helpers.\n\n## Decision\n\nKeep a `cairn.root` module that claims `src/main.rs`, `src/lib.rs`, `src/error.rs`, `src/verification.rs`, and `src/signal.rs`.\n\n## Rationale\n\nWithout this module, these files would be orphaned on every scan. Grouping them under one node reflects that they are the crate boundary, not domain logic.\n\n## Consequences\n\n- New top-level source files should either join `cairn.root` or spawn a new top-level module with its own decision.\n- Changes to `src/lib.rs` public API are high-impact and should be gated carefully."
            },
            {
              "type": "research",
              "path": "meta/research/gas-city-cairn-integration/analysis.md",
              "title": "Gas City / Beads / cairness — Deep Analysis",
              "frontmatter": {
                "date": "2026-05-13",
                "id": "res.gas-city-cairn-integration"
              },
              "body": "\n# Gas City / Beads / cairness — Deep Analysis\n\n**Date:** 2026-05-13\n**Session:** Claude Code (Opus 4.7, 1M context) on branch `claude/gas-city-cairn-analysis-swwxw`\n**Method:** Live inspection of cloned repos (`gastownhall/gascity`, `gastownhall/beads`), reading of full architecture docs, Yegge's \"Welcome to Gas City\" blog (user-supplied verbatim transcript; original is paywalled), and supplied cairness issue inventory.\n\nAll citations below are repo-relative paths to files inspected in-session against `main` at session time. If this analysis is later promoted to a CAIRN Source artefact, pin the inspected commits explicitly.\n\n---\n\n## 1. Gas City: what it actually is\n\n### 1.1 The model\n\nPer `gascity/engdocs/architecture/nine-concepts.md`, Gas City is five primitives + four derived mechanisms:\n\n**Primitives (Layer 0-1):**\n1. **Session** — start/stop/prompt/observe sessions regardless of provider\n2. **Bead Store** — universal persistence substrate; everything is a bead\n3. **Event Bus** — append-only pub/sub log\n4. **Config** — TOML with progressive activation\n5. **Prompt Templates** — Go text/template in Markdown\n\n**Derived (Layer 2-4):**\n6. Messaging (mail + nudge)\n7. Formulas & Molecules (declarative workflows + runtime instances)\n8. Dispatch (Sling) — agent + formula + molecule composition\n9. Health Patrol — supervision, reconciliation, crash quarantine\n\n### 1.2 The controller loop\n\n`gascity/cmd/gc/controller.go:226` defines `controllerLoop()`. Each tick (default 30s):\n\n1. **Dirty check** — fsnotify-driven config reload via `tryReloadConfig()` at `gascity/cmd/gc/controller.go:137`\n2. **`buildAgents(cfg)`** — evaluates pool `check` commands in parallel, applies suspensions, resolves fixed agents\n3. **`reconcileSessionBeads()`** — declarative convergence between session beads and running sessions; see `gascity/cmd/gc/session_reconciler.go`\n4. **`wispGC.runGC()`** — purges expired molecules per TTL\n5. **`orderDispatcher.dispatch()`** — trigger-conditioned formula/exec dispatch\n\nConfiguration drives everything. From `gascity/engdocs/architecture/controller.md` §Invariants:\n> *\"No role names in Go code. The controller operates on resolved config, runtime session names, and provider state.\"*\n> *\"SDK self-sufficiency: All controller operations function with only the controller process running. No user-configured agent role is required for any infrastructure operation.\"*\n\n### 1.3 What \"drift detection\" means in Gas City\n\n`gascity/engdocs/architecture/controller.md` interactions table:\n> *\"`internal/runtime` | `Provider` interface for Start/Stop/IsRunning/ListRunning/Interrupt/Peek/SetMeta/GetMeta/ClearScrollback. `ConfigFingerprint()` drives drift detection.\"*\n\n`gascity/internal/runtime/fingerprint.go` is *\"`ConfigFingerprint()` (SHA-256 of command + env + extras for drift detection)\"* — drives agent restart when running instance's command/env diverges from declared config.\n\nOther drift usages in repo grep:\n- `gascity/release-gates/ga-9shf-gate.md` — `gc doctor` drift detector for Dolt port mismatches\n- `gascity/plans/archive/huma-openapi-migration*.md` — CI gate ensuring committed OpenAPI spec matches code\n\n**No drift concept between declared system architecture and actual code.** Verified by grep across `gascity/engdocs/`, `gascity/specs/`, and `gascity/internal/` for: `ontolog`, `blueprint`, `interface.hash`, `provenance`, `authority`. Only stray hits (e.g. `gascity/AGENTS.md`: *\"The architecture docs are a reference, not a blueprint\"*), never as an architectural primitive.\n\n### 1.4 Out of scope by Gas City's own declaration\n\n`gascity/specs/architecture.md` §7 explicitly excludes declarative schema specifications and framework positioning. Gas City is a control plane, not a framework.\n\n### 1.5 Runtime providers — leanness confirmed\n\nPer `gascity/engdocs/architecture/session.md`, providers include:\n- `tmux` — primary interactive\n- `subprocess` — local non-interactive\n- `exec` — script-backed\n- `k8s` — pod-backed\n- `acp/auto/hybrid` — routing layers\n\nAn \"agent\" is whatever you put behind a `runtime.Config` (command, env, cwd). Bare Python scripts, Go binaries, curl calls, MCP clients — all work. Nothing forces Claude Code or any heavy harness. Confirms that the leanness concern that motivated cairness's lightweight agent spec is already addressed in Gas City as a first-class case.\n\n---\n\n## 2. Beads (MEOW substrate): what it actually is\n\n### 2.1 Standalone, orchestrator-independent\n\nPer `gastownhall/beads/README.md`:\n> *\"Beads is a CLI tool you install once and use everywhere. You don't need to clone this repository into your project.\"*\n\nInstallation: `brew install beads` / `npm install -g @beads/bd` / curl script. `bd init` initializes in any project; no orchestrator required.\n\n### 2.2 The Bead schema\n\n`gascity/internal/beads/beads.go:Bead`:\n```go\ntype Bead struct {\n    ID           string\n    Title        string\n    Status       string   // \"open\", \"in_progress\", \"closed\"\n    Type         string   // \"task\" default; matches bd wire format\n    Priority     *int\n    CreatedAt    time.Time\n    Assignee     string\n    From         string\n    ParentID     string   // step → molecule\n    Ref          string   // formula step ID or formula name\n    Needs        []string // dependency step refs\n    Description  string\n    Labels       []string\n    Metadata     map[string]string\n    Dependencies []Dep\n}\n```\n\n`Type` is a free-form string. Beads persists; CAIRN would interpret.\n\n### 2.3 Hash IDs and Dolt backing\n\nBeads README §\"Zero Conflict\":\n> *\"Hash-based IDs (`bd-a1b2`) prevent merge collisions in multi-agent/multi-branch workflows.\"*\n\nBeads README §\"Features\":\n> *\"Dolt-Powered: Version-controlled SQL database with cell-level merge, native branching, and built-in sync via Dolt remotes.\"*\n\nFederation via Wasteland is built on Dolt-remote sync; orchestrator-independent.\n\n### 2.4 MEOW is not a library\n\n`gascity/AGENTS.md` verbatim:\n> *\"a thin layer atop the MEOW stack (beads → molecules → formulas).\"*\n\nMEOW = Beads (storage) + Molecules (formula instances, in gascity) + Formulas (TOML workflow definitions, in gascity). **Only Beads is independently installable.** \"MEOW stack\" describes the conceptual sandwich; not a downloadable package.\n\n---\n\n## 3. Gas City's API surface\n\nPer `gascity/engdocs/architecture/api-control-plane.md` §1:\n> *\"Two architectural themes run through everything below: 1. The object model is the center; the CLI and the HTTP + SSE API are projections over it. One canonical domain, two typed surfaces. 2. Typed data end-to-end. Go structs with annotations drive a generated OpenAPI 3.1 contract.\"*\n\n**Surfaces:**\n- CLI (`gascity/cmd/gc/`) — broad subcommand set\n- HTTP + SSE generated via Huma from typed Go structs\n- Generated Go client for cross-process calls\n- SSE event stream for long-running ops: 202 + `request_id` + `request.result` event\n\n**Extension points for external integrators:**\n- **Packs** — declarative agent topologies as TOML + prompts + formulas\n- **Formulas** — `*.formula.toml` workflow definitions\n- **Prompt templates** — Go text/template in Markdown\n- **Runtime providers** — tmux/subprocess/exec/k8s/acp\n- **`exec.Store`** — `provider = \"exec:<script>\"` delegates bead-store ops to user script\n\nThe canonical Gas Town topology itself ships as a pack (`gascity/examples/gastown/`). Per `gascity/examples/gastown/SDK-ROADMAP.md`: *\"~1,200 lines of Go to make Gas Town run as pure configuration.\"* Even Gas Town is just-a-pack.\n\n---\n\n## 4. Cairness scope vs Gas City overlap matrix\n\nBased on cairness issues #1, #2, #6, #7, #9, #10, #14 (supplied by user; repo `george-rd/cairness` is private). Coverage assessment against Gas City code and docs read in-session:\n\n| Cairness issue | Scope | Gas City equivalent | Verdict |\n|---|---|---|---|\n| **#1** Epic: Grapharness | Lightweight harness-agnostic agent orchestration on CAIRN graph; <5MB Rust; 2-3k LOC | Full control plane in Go | **Standalone form duplicative.** Salvage: graph-walking scheduler concept (~400 LOC) → Gas City formula |\n| **#2** Flow engine + YAML DAG, 500-700 LOC | YAML step DAG with conditions, retries, actions | Formulas + molecules (TOML + bead trees) | **Duplicated** |\n| **#6** Adapter registry, 200+150/adapter | YAML adapter contracts for jcode/CC/litellm/codex | Runtime providers + prompt templates | **Mostly duplicated.** Per-harness glue lives in packs |\n| **#7** Wave scheduler walking CAIRN graph, 400-500 LOC | Walk CAIRN graph, group into parallel waves, apply policy | Controller is config-driven, not graph-driven | **Not duplicated.** Real novel piece |\n| **#9** Stats + dashboard + self-improvement, 1150 LOC | SQLite metrics, TUI/web dashboard, analysis agents propose flow changes | Event bus + Dolt audit | **Data layer duplicated.** Self-improvement loop novel |\n| **#10** YAML flows vs CAIRN primitives | Architecture decision parked | — | Decision becomes: orchestrator-agnostic CAIRN with optional Beads backend |\n| **#14** SQLite cache + DB state (closed-source) | CAIRN open-source file-based, cairness closed-source DB-backed | Dolt via Beads | **Dolt strictly better than SQLite** for versioning/branching/federation |\n\n**Estimated overlap:** ~70%. Two novel pieces (#7, #9) survive but are formula-sized (hundreds of LOC), not standalone-orchestrator-sized (thousands).\n\n### 4.1 Where the surviving novel pieces actually live\n\n**Cairness #7 (graph-walking wave scheduler) splits across the two sides of the integration:**\n\n- **CAIRN side (~50-100 LOC).** The graph-walking primitive — *\"given the current change, what's ready right now?\"* — must live where the graph definition lives. Concretely: `cairn query --ready --change <id> --json` walks blueprint + active change, applies `needs:` edge resolution, groups results by topological depth, emits waves as JSON. Covered by existing slate issues #4 (JSON contract), #9 (tasks-as-beads gives `bd ready` for free when beads-backed), and #3 (`ArtefactStore.query_by_dependency`).\n\n- **Orchestrator side (~300-400 LOC, free re-use).** Wave dispatcher, concurrency limit, retry policy, role-based routing — these are operational, not architectural. Gas City already ships them via formula `needs:` edges, runtime pools, `max_restarts`/`restart_window`, label-based routing. In `adapters/gascity/` (issue #6) this becomes one formula (`cairn-wave-dispatch.formula.toml`) + a small worker prompt template.\n\nCairness was estimating 400-500 LOC because it was building the dispatcher from scratch. The dispatcher already exists in Gas City. We just need the right query feeding it. No new slate issue needed — the work is distributed across #3, #4, #6, #9.\n\n**Cairness #9 (self-improvement loop)** is similarly distributed: Gas City + Dolt gives the audit data; the analysis-agent-proposes-changes loop is one or two formulas on top, also in `adapters/gascity/` or as a future skill. Defer until the data is flowing.\n\n---\n\n## 5. What CAIRN already has\n\nVerified by source inspection in `~/cairn/`:\n\n- `src/changes/` — change primitive with `artefact_ops.rs`, `types.rs`, `validate.rs`. Hooks for `CAIRN_CHANGE_ARTEFACT_CONFLICT` (`src/hooks/mod.rs:144`).\n- `src/cli/accept.rs:run_accept_gate(change_id)` — apply/verify gate\n- `src/cli/commands.rs:run_archive_command` — archive command\n- `cairn.kernel.changes` module declared in `cairn.blueprint`\n- Spec §9 — change directories, delta semantics (ADDED/MODIFIED/REMOVED/RENAMED), archive operation\n- Spec line 178 — planned location `./meta/changes/`\n- Spec §4 verbatim:\n  > *\"Cairn and OpenSpec solve different problems (OpenSpec is a change-lifecycle workflow, Cairn is a structural reconciliation framework), but OpenSpec's change-isolation and delta-merging patterns are directly applicable and are adopted in sections 9 and 12. **Cairn deliberately does not adopt OpenSpec's workflow layer**; the two tools are complementary and could coexist in the same repo.\"*\n\nThat non-goal needs amendment if openspec is to be retired entirely. See issue-slate.md #8.\n\n---\n\n## 6. What CAIRN does NOT have (openspec retirement gaps)\n\n1. Conversational skills (`cairn-propose`, `cairn-explore`, `cairn-apply`, `cairn-archive`) — openspec's day-to-day value via `/openspec-propose` and friends.\n2. `cairn change new <name>` scaffold with proposal.md / design.md / tasks.md templates.\n3. In-change task tracking. OpenSpec has tasks.md; CAIRN doesn't yet. Beads with `parent=<change-id>` is the clean answer.\n4. `cairn import-openspec` migration helper.\n5. Registries as graph queries (currently `openspec/registries/*.md` as files).\n6. Conventions surface (currently `openspec/conventions.md`; should be per-module `rules` blocks in `cairn.blueprint` or a top-level Source on `cairn.root`).\n7. One-way switch: `openspec/changes/` → `meta/changes/`.\n\nNone of these are kernel-deep. Skills, scaffolds, a migration script, and small CLI commands. Reliable retirement is weeks of work, not months.\n\n---\n\n## 7. The structural argument\n\nThree layers, three concerns:\n\n```\nLayer 3: Orchestration (optional)\n   Gas City controller / sessions / packs / formulas\n   CAIRN consumed as formula steps\n   Future runners: adapters/<name>/\n\nLayer 2: Semantic (CAIRN's lane)\n   cairn.blueprint, typed artefacts, two-chain topology\n   Reconciler, drift gate, interface hashes\n   No equivalent in Gas City (verified by grep)\n\nLayer 1: Storage (pluggable)\n   Default: filesystem\n   Optional: Beads (bd CLI / Dolt-backed)\n   CAIRN trait: ArtefactStore\n```\n\n- Gas City: Layers 1 + 3, no Layer 2.\n- CAIRN: Layer 2, pluggable Layer 1, externalised Layer 3.\n- Beads: Layer 1 only.\n\nThese compose. They do not compete.\n\n---\n\n## 8. Yegge's framing (from supplied article transcript)\n\nDirect quotes from the \"Welcome to Gas City\" Medium article (user-supplied verbatim; original at https://steve-yegge.medium.com/welcome-to-gas-city-57f564bb3607 is paywalled):\n\n- *\"Gas City has deconstructed the entire Gas Town stack into composable, declarative building blocks called 'packs'.\"*\n- *\"MEOW, the Molecular Expression of Work, is a lightweight Beads-based framework that places Work front and center, as the first-class system primitive, creating a versioned knowledge graph of all your issues and tasks.\"*\n- *\"every agent action recorded in a git-versioned Dolt database. That's your SOC2 story, sitting right there in the database, already written.\"*\n- *\"any agent can go temporarily insane, at any time, and make a bad call. No matter how smart they are.\"*\n- *\"To replace SaaS, you need the unglamorous stuff: declarative deploys, audit trails, version history, identity, and a memory layer that survives the inevitable agent failures.\"*\n- *\"Gas City is a high-control system. It has high parallelism... but it uses structure to keep agent swarms organized.\"*\n\nThese quotes establish that:\n\n- Yegge's \"knowledge graph\" is the **work-as-graph** (beads with deps), not architecture-as-graph\n- The reliability story is **probabilistic** (more agents reviewing each other), not deterministic (gate at commit)\n- The pitch is **replace SaaS / business process automation**, not architectural governance\n\nCAIRN's deterministic-gate-at-commit + architectural-truth angle is complementary, not competing.\n\n---\n\n## 9. Decisions reached this session\n\n1. **Keep CAIRN.** Architecture-truth / typed-artefact / drift-gate / two-chain authority layer is genuine white space. Verified by grep of Gas City; no analogue.\n2. **Retire cairness as scoped.** ~70% overlap with Gas City's mature surface. Salvage the graph-walking scheduler (~400 LOC) as a Gas City formula in `adapters/gascity/`.\n3. **Retire cflx.** Was always experimental; CAIRN's `accept`/`archive` primitives plus an external runner replace it.\n4. **Adopt Beads as a pluggable storage backend.** Optional but worth it: hash-IDs, Dolt versioning, federation via Wasteland, no orchestrator coupling.\n5. **CAIRN does not ship its own orchestrator.** Integration with Gas City via a `cairn-gc` reference pack; future runners get their own adapter under `adapters/`.\n6. **Retire `openspec/changes/`.** Move active phases to `meta/changes/` (already planned per spec line 178). OpenSpec workflow replaced by CAIRN skills + (optionally) beads-backed tasks.\n7. **Amend spec §4** to reflect that workflow lives externally (skills + optional formulas), not as a CAIRN non-goal.\n\n---\n\n## 10. Honest limitations of this analysis\n\n- The Medium article was paywalled; analysis used user-supplied verbatim transcript. Quotes are traceable to that transcript.\n- `cairn` binary was not built in the session sandbox; analysis used grep/find/Read directly. A repeat with `cairn context` + `cairn neighbourhood` available would likely surface more.\n- `bd` was not installed in the session sandbox; Beads claims were verified via README + cloned source inspection only, not via runtime use.\n- cairness scope is from the issue inventory supplied by the user (`#1, #2, #6, #7, #9, #10, #14`). The repo `george-rd/cairness` is private; source not inspected.\n- Gas City and Beads repos were cloned shallow (`--depth 1`) to `/tmp/gc-review/gascity` and `/tmp/beads-repo`. Tag/commit not pinned. If this analysis is promoted to a Source artefact, re-clone with explicit refs and re-verify.\n\n---\n\n## 11. The \"graph IS orchestration\" framing\n\nSurfaced in conversation after the initial slate was drafted. Cairness #7 was reaching for this; the spec hints at it (line 71: *\"Decisions can declare the blueprint nodes they apply to; the framework can then flag when a change to those nodes appears to violate the decision (v2 capability, deferred)\"*).\n\nTwo distinct meanings:\n\n**(a) Reactive: graph state changes drive work.** New `Todo` appears → worker spawned. `Contract` interface hash changes → drift gate fires. `Decision` flips to `accepted` → implementation work materialises.\n\n**(b) Declarative: node types carry workflow semantics.** Each artefact type has an associated lifecycle and an associated kind-of-work. `Contract`: draft → reviewed → accepted. `Todo`: proposed → ready → claimed → done. The graph topology directly maps to dispatch decisions.\n\nBoth are CAIRN-side concerns. Neither requires CAIRN to own the dispatcher. The right division of labour:\n\n- **CAIRN owns the semantics:** which node states imply which work types, what the lifecycle transitions are, when the drift gate must fire\n- **The orchestrator owns the runtime:** parallelism, retries, pool scaling, crash recovery\n\nThis preserves the cairness vision in spirit (graph-native orchestration) while extracting the orchestrator into Gas City where it's more mature.\n\nThree operational paths for graph-state-driven work in the Gas City world:\n\n1. **CAIRN queries drive Gas City formulas.** `cairn query --ready --change <id>` returns ready wave; Gas City formula dispatches. Covered by #98 + #100.\n2. **Beads-mediated.** Typed beads (`type=contract`) become work items via existing `bd ready` detection. Covered by #99 + #103.\n3. **SSE reactive** (strongest form). CAIRN emits events on graph state changes; Gas City Orders react. Covered by #96 + #101.\n\n**Gap in the current slate:** explicit `node-type → workflow` association in `cairn.blueprint`. Example: `Module @api → on_drift: cairn-drift-gate`, `Contract → on_status_change(accepted): cairn-implement`. The orchestrator becomes a dumb pump that runs whatever formula the graph state says is implied. This is the missing piece that makes \"graph IS orchestration\" concrete on the CAIRN side. Candidate for a new slate issue; pending decision.\n\n---\n\n## 12. Gas City tech-debt assessment\n\nAsked late in the session because contributing back upstream became a strategic option. Concrete numbers from `/tmp/gc-review/gascity`:\n\n| Signal | Value | Read |\n|---|---|---|\n| TODO/FIXME/HACK in non-test Go | 21 across ~250k LOC | 0.0084% density — well below industry concern |\n| Test files | 796 | Heavy investment |\n| Active design RFCs (`engdocs/design/`) | 20 | Working RFC pipeline; debt is documented before it's debt |\n| Archived RFCs | 18 | Things actually ship and graduate |\n| CHANGELOG detail | Per-fix operator-impact notes | Mature release engineering |\n| Pre-commit hooks | Auto-regen OpenAPI + dashboard schema + lint + vet + test | CI-equivalent gates run locally |\n| Recent activity | PR #1169 in last commit message | High velocity, large contributor base |\n\nSample TODOs read as `// Wired: TODO — operation context plumbing pending` — deliberate incremental implementation, not rot. No \"broken and we don't know how to fix\" debt visible.\n\n`CONTRIBUTING.md` verbatim: *\"Gas City is experimental software, but the repo is now structured for external contributors.\"*\n\n**Verdict:** healthy. Contributing into Gas City would not be a rescue mission.\n\n---\n\n## 13. Contribution-path strategy\n\nKey finding: `gastownhall/gascity-packs` exists as the explicit community pack home. README verbatim: *\"A collection of opt-in Gas City packs... Packs compose through `pack.toml` imports, so a city can opt into any subset of the packs in this repo without forking.\"*\n\nSo the upstream contribution path is:\n\n1. Build `adapters/gascity/` in `cairn-framework/cairn` (issue #100)\n2. Dogfood locally for some weeks\n3. Polish: docs, README, pinned Gas City version\n4. Submit to `gastownhall/gascity-packs` as `packs/cairn-governance/` (or similar)\n5. Optionally: a small PR to `gascity` core if a genuine integration-contract gap surfaces (unlikely; their extension model is good)\n\nWe're not contributing into 250k LOC of Go. We're contributing a pack (TOML + Markdown + prompt templates + a thin shim that shells to `cairn`). Tractable from outside; minimal upstream maintainer load.\n\n**Community angle:** if `cairn-governance` lands in `gascity-packs`, CAIRN gets a discovery channel to ~15k-star Gas City community. The Gas City Discord audience (~2,000 active members per Yegge's article) is *exactly* the audience for architecture governance — people running multi-agent systems who've felt the hallucination pain and want deterministic gates. CAIRN repo stays the canonical home; the pack is the bridge.\n\nLow-risk strategic bet. Downside is zero — you'd build the pack anyway for your own use under issue #100.\n\n---\n\n## 14. Authoring workflows: same principle, applied to progressive disclosure\n\nThe \"workflow lives outside CAIRN; CAIRN provides atomic operations\" pattern from §11 also applies to **how a user builds out a spec one node at a time**. Surfaced in conversation when comparing to getcairn.dev's progressive-disclosure spec UX.\n\nA multi-step node creator — pick type → assign ID/name → fill required fields → validate → commit — is a workflow with `needs:` edges, conditional retries, and human-in-the-loop pauses. **That's what formulas are designed for.**\n\n### What CAIRN ships\n\nAtomic, composable, JSON-shaped CLI commands. Each independently testable.\n\n- `cairn node template --type=<artefact-type>` → emits a JSON schema with required/optional fields for the type\n- `cairn validate-node --file=<draft.toml> --strict` → exit 0/1/2 + JSON findings\n- `cairn change add-node --change=<id> --file=<draft.toml>` → idempotent commit\n\n### What CAIRN does NOT ship\n\n- Interactive prompt loops\n- Wizard state machines\n- Branching flow logic (\"if Contract, also ask for X\")\n- Retry/recovery on validation failure\n- Persistence of partial drafts\n\n### Where the wizard UX actually lives\n\nTwo surfaces, sharing the same underlying commands:\n\n**Formula version (Gas City users) — lives in `adapters/gascity/`:**\n\n```toml\nformula = \"cairn-propose-node\"\n\n[[steps]]\nid = \"pick-type\"\ndescription = \"Ask user: Module, Contract, Decision, Todo, Research, Review, Source\"\n\n[[steps]]\nid = \"id-and-name\"\nneeds = [\"pick-type\"]\ndescription = \"Run: cairn node template --type={{type}}; ask user for id + name\"\n\n[[steps]]\nid = \"fill-fields\"\nneeds = [\"id-and-name\"]\ndescription = \"Read template; prompt for each required field one at a time\"\n\n[[steps]]\nid = \"validate\"\nneeds = [\"fill-fields\"]\ndescription = \"cairn validate-node --file={{tmpfile}}; on exit 2, loop back to fill-fields with errors\"\n\n[[steps]]\nid = \"commit\"\nneeds = [\"validate\"]\ndescription = \"cairn change add-node --change={{change}} --file={{tmpfile}}\"\n```\n\n~30 lines of TOML. All flow state lives in the formula's molecule (bead tree). All semantic correctness lives in CAIRN's atomic commands.\n\n**Skill version (Claude Code / Codex / chat agents) — lives in `.claude/skills/`:**\n\nSame workflow, different surface. Markdown skill that drives the same atomic commands conversationally. Covered by issue #102.\n\n### Why this matters\n\nThe two surfaces — formula and skill — diverge only in *how they ask the user*. The CAIRN commands underneath are identical. This means:\n\n- Any future surface (web UI, TUI, getcairn.dev clone, IDE plugin) is a thin client over the same CLI\n- Each atomic command is unit-testable in isolation\n- The wizard's logic (which questions, what order, branching on type) is reviewable as a single TOML or Markdown file, not buried in Rust\n- CAIRN's binary stays small\n\n### Connection to slate issues\n\nThis **refines**, not adds:\n\n- **#98 (Stable JSON + exit codes)** hardens what \"atomic\" means: no command embeds multi-step state\n- **#100 (`adapters/gascity/` pack)** gains `cairn-propose-*.formula.toml` files as first-class content\n- **#102 (change-lifecycle skills)** gets the symmetric markdown skills\n\nNo new issue needed.\n\n### Risk\n\nIf every workflow lives outside CAIRN, *CAIRN-the-product* could feel skeletal to a new user. *\"I installed cairn but there's no `cairn wizard` command?\"*\n\n**Mitigation:** ship the skills + formulas in `.claude/skills/` and `adapters/gascity/` directories of the CAIRN repo itself. A fresh clone has the wizard UX available out of the box. The composition lives in the repo; only the *engine* runs externally.\n\n---\n\n## 15. Adversarial review\n\nRun at session-end when the plan/direction felt settled. Each item below is the strongest counter-argument against a decision in this analysis, answered honestly.\n\n### 1. \"CAIRN should just be a Gas City pack from day one. No separate Rust binary.\"\n\nCounter: drift detection needs to run **offline**, **in CI**, **on machines without Gas City**, as a **git pre-commit hook**. The reconciler must produce a content-addressable fingerprint per spec §3.5 — that's a deterministic-correctness claim, can't depend on an orchestrator. Standalone binary is essential. *Pressure created:* keep the Rust footprint tight enough to install in CI without pain.\n\n### 2. \"Three layers (semantic / storage / orchestrator) are too many. Skip the trait, just adopt Beads + skills.\"\n\nCounter: filesystem default is required for `brew install cairn` to work without `brew install beads`. Adoption friction matters. The trait is the seam between CAIRN-works-alone and CAIRN-better-with-Beads. Not speculative. Keep.\n\n### 3. \"Gas City might pivot or fade in 6 months.\"\n\nCounter: the adapter is small (formulas + prompts in `adapters/gascity/`). Core stack (#95–#98) is orchestrator-neutral. If Gas City fades, only `adapters/gascity/` needs replacing. *Pressure created:* don't let Gas-City-specific concepts leak into #96 (integration contract). It must stay generic.\n\n### 4. \"OpenSpec retirement is risky. Replacement skills are unbuilt.\"\n\nCounter: phasing is explicit. #102 + #103 must land and prove themselves before #104 fires. **Action item:** edit #104 body to add \"Blocked by: #102, #103.\"\n\n### 5. \"Authoring-workflows-as-external will fragment the user experience.\"\n\nCounter: skill and formula share the same `cairn` commands underneath. Divergence is bounded to question-asking surface. **Action item:** edit #102 acceptance to specify that required-field set + validation rules come from CAIRN (`cairn node template --type=X --json`), not duplicated in surfaces.\n\n### 6. \"11 issues is overscoped for solo work.\"\n\nCounter: roadmap, not sprint backlog. Phased dependencies are clear; agnostic core (#95–#98) is one-quarter scope. *Pressure created:* cross-refs between issues should be explicit. Currently only #99 references #91.\n\n### 7. \"Beads might fade too. Yegge-affiliated.\"\n\nCounter: Beads is more independent than cairness ever would have been (separate repo, brew/npm install, MIT). And the trait in #97 means we can swap backends. Lock-in bounded. *Pressure created:* the trait surface must be defined in terms of what CAIRN needs, not what Beads can offer.\n\n### 8. **Deepest risk.** \"The architecture-truth bet hasn't been validated externally. It might be wrong.\"\n\nCounter: acknowledged, not fully resolvable from inside. Yegge's probabilistic-reliability bet (more agents reviewing each other) might just be correct, and deterministic gates might be solving a problem nobody has. **Mitigation:** dogfood CAIRN aggressively *before* submitting `cairn-governance` to `gascity-packs`. Document concrete cases where the drift gate caught something a probabilistic agent review would have missed. Without case studies, the upstream submission is unsubstantiated. **Action item:** README open question — what counts as adequate validation evidence before upstream submission?\n\n### 9. \"Gas City community might reject a Rust-shim pack.\"\n\nCounter: subprocess/exec providers exist to run anything. Polyglot via subprocess is intentional. Examples in `gascity/examples/` already shell to bash. Low-medium risk.\n\n### 10. \"External workflows mean less out-of-the-box. openspec ships more.\"\n\nCounter: §14 mitigation — ship the skills + formulas in CAIRN's own repo. Fresh clone has everything. *Pressure created:* CAIRN's own README needs to lead with the wizard UX, not the kernel architecture. First-run experience matters.\n\n### Verdict\n\nThe plan survives the adversarial review. The deepest risk (#8 — validation of the architectural-truth bet) is unavoidable; you can't prove it from inside. Mitigation is dogfood + case studies before public submission.\n\nThree actionable sharpenings, captured as open questions / proposed issue edits:\n\n1. **Edit #104:** add \"Blocked by: #102, #103.\"\n2. **Edit #102 acceptance:** specify CAIRN owns the required-field set + validation rules; both surfaces consume `cairn node template --type=X --json`.\n3. **New README open question:** what counts as adequate validation evidence before upstream submission to `gascity-packs`?\n\n---\n\n## 16. Storage model refinement: content / state / map split\n\nSurfaced when the user asked directly: *\"Would we look at storing cairn's graphics and relations in Dolt/beads? Or should we look at it as an extension of beads?\"*\n\nPulling those apart led to a meaningful refinement of earlier issues #97 and #99.\n\n### Three distinct things, three distinct storage answers\n\n| Thing | What it is | Storage |\n|---|---|---|\n| **Content** | Authored text: `cairn.blueprint`, contract bodies, decision rationale, research notes, sources, todo descriptions. Reviewed in PRs. | **Files. Always.** Markdown + `cairn.blueprint`. No pluggable backend. |\n| **State** | Metadata about content: status, assignee, atomic claim, dependency edges between work items. Mutable. Two agents can race. | **Pluggable.** Filesystem default (status in frontmatter). Beads optional (atomic claim, hash IDs, Dolt versioning). |\n| **Map** | Typed node graph derived from parsing content + reconciling against the filesystem. | **Ephemeral by default; optional cache** (Dolt or SQLite, per cairness #14). Source of truth is files. |\n\n### Why this is cleaner than \"ArtefactStore for everything\"\n\n- **Atomic merge with code preserved.** A new contract or todo usually lands in the same PR that implements/adopts it. If content lives in Git, contract change + code change merge atomically as one unit, revert together, appear together in `git log`. If content lived in a separate Dolt store, you'd have a two-VCS coordination problem (no distributed transaction between Dolt and Git).\n- **Beads's strengths used where they matter.** Atomic claim, hash IDs, Dolt versioning — for state and work, where races and audit actually happen.\n- **No backend pluggability for content (today).** Files are the canonical format for commit-coupled content. The pluggable layer is the *state backend*, a much smaller surface.\n- **Reconciler simplicity.** Reads files, compares to filesystem, emits findings. No database round-trip per node.\n\n### What's *not* a reason for files-canonical\n\nThe earlier draft of this section claimed \"Dolt loses line-level diffs\" and \"content already git-versioned, so don't waste Beads on it.\" Both wrong:\n\n- **Dolt does have cell-level diffs.** Text content in a `text` column is fully diff-able across commits and branches. Beads proves this: every issue body lives in Dolt's `Description` column and is fully versioned, diffed, branched.\n- The \"already git-versioned\" argument was circular — it assumed git is the right versioner, which is the question, not the answer.\n\nThe actual argument is atomic-merge-with-code, above. That's the load-bearing constraint.\n\n### Per-artefact-type implications\n\n| Artefact type | Content storage | State storage |\n|---|---|---|\n| Contract | file | (none — derived from interface hash) |\n| Source | file | (none — immutable) |\n| Decision | file | bead (status: proposed/accepted/deprecated/superseded) |\n| Research | file | bead (status) |\n| Review | file | bead (status, who reviewed when) |\n| Todo | file | bead (status, assignee, claim) |\n\nHybrid artefacts (Decision, Research, Review, Todo) get the cleanest model: markdown owns *content*, bead owns *state*. The bead's `ref` field points at the markdown file path. `cairn get <id>` reads both. When the state backend is filesystem-only, state lives in markdown frontmatter — same fields, just no atomic-claim guarantee.\n\n### Two axes of pluggability (refined)\n\n| Axis | Default | Optional (today) | Optional (future) |\n|---|---|---|---|\n| **State** | filesystem (frontmatter) | Beads (#97 / #99) | remote `StateBackend` (Cairnhub) |\n| **Content** | filesystem (atomic merge with code) | — | Dolt-direct `ContentBackend` for non-commit-coupled artefacts |\n\nToday's slate covers **state**. Content stays filesystem-only by default because the artefacts CAIRN cares about (blueprint, contract, decision, todo bound to code) are commit-coupled. A future `ContentBackend` trait could mirror `StateBackend`, with filesystem as default and Dolt-direct as an option — for artefacts that *don't* travel with code (Cairnhub-style cross-project decisions, multi-project research, agent-action audit logs).\n\n### Slate impact\n\n- **#97 (now `StateBackend`)** — narrowed to state only for today. Forward-compatible with a future `ContentBackend` sibling.\n- **#99 (now Beads `StateBackend`)** — narrowed accordingly.\n- **No `ContentBackend` issue today.** Filesystem content is the right default while local-project workflows dominate. Add the trait only when Cairnhub-style multi-project workloads create real demand.\n\nThe \"extension of Beads\" framing remains rejected: CAIRN's commit-coupled content is not in Beads, today or ever, because it needs to merge atomically with code. The \"graph in Dolt\" framing is *partially* rejected: the graph stays derived locally; only state (today) and potentially non-commit-coupled content (future) go to Dolt.\n\n---\n\n## 17. Cairnhub: the long-horizon vision (not slate work)\n\nUser raised: *\"if dolt is VCS like git, we get cairn to be like a dolt powered system, which uses beads i guess for the task part, but it also just has all the code etc in one? So its like an agentic coding VCS. Cairnhub.\"*\n\nWorth capturing the shape, the rejections, and the forward-compatible parts.\n\n### Cairnhub's natural domain: non-commit-coupled artefacts\n\nRefined in light of §16's atomic-merge-with-code constraint: Cairnhub's clearest value is for artefacts that **don't** need to land atomically with specific code commits. Those are:\n\n- Cross-project decision archive (\"which projects adopted dec.use-shared-crypto?\")\n- Agent-action audit log (\"which agent did what in any project last week?\")\n- Cross-project contract dependencies (module A in project X importing contract from project Y)\n- Federated research across organisations\n- Hosted shared libraries of skills/model-definitions\n\nPer-project, commit-coupled content (blueprint, contract bodies, todos tied to specific code) stays in git repos under any architecture. Cairnhub indexes; it doesn't repatriate.\n\nThis sharpens what Cairnhub is *for* and what it's *not* for.\n\n### What's real in the vision\n\nDecomposed into evaluable pieces:\n\n| Piece | Worth pursuing? | When |\n|---|---|---|\n| Single-project single source of truth | ✓ Already in plan (§16) | Now |\n| Multi-project state aggregation | ✓ | Future server mode |\n| Standard agent skills + model definitions protocol | ✓ | Future |\n| Cross-orchestrator agent-action audit log | ✓ Yegge's SOC2 angle | Future |\n| Hosted \"Cairnhub\" SaaS | Possibly | Far future |\n| **Replace Git for code** | ✗ | Never |\n\n### Why \"replace Git\" is the wrong fight\n\nCode is unstructured text in files. Dolt wants structured rows in tables. Storing code as text blobs in Dolt costs: line-level diffs, blame, hunk operations, every IDE integration, GitHub network effects. Every previous \"replace Git\" attempt (Mercurial, Pijul, Fossil, Bazaar) is technically superior in some way and has tiny adoption. Network effects via GitHub are the strongest force in software tooling.\n\n### What the realistic Cairnhub looks like\n\nA *server tier* above today's local-file architecture:\n\n1. **Indexes multiple project repos.** Each project still has its own `cairn.blueprint`, content as files, Git as VCS for code.\n2. **Aggregates state in Dolt** — tables for projects, cross-project contracts, decisions-of-record, tasks-by-project, audit log of agent actions. Federation via Dolt remotes between teams/orgs.\n3. **Exposes a query API** — \"show me all contracts across all projects whose interface hash changed and have no review in 30 days\" becomes one SQL query.\n4. **Hosts standard protocol endpoints:**\n    - `GET /context/<project>` — current map + active change + ready tasks\n    - `POST /action` — agent publishes \"I did X\" (audit-log row)\n    - `POST /validate` — run drift gate against proposed change, return findings\n5. **Provides a plugin contract** — orchestrators (Gas City, Hermes, Claude Code, future) implement it. Plugins let agents read CAIRN context and publish actions; they don't replace Git.\n\n### Why today's architecture is forward-compatible\n\nThe `StateBackend` trait (#97) is already the right seam. Today's impls: filesystem, Beads (local Dolt). A future impl: `CairnhubBackend` (talks to a remote Dolt-backed CAIRN server). Trait surface unchanged.\n\nContent stays as files in repos, regardless of whether you run local-only or against Cairnhub. The server indexes; it doesn't replace.\n\n### What a Cairnhub Phase would actually add (someday)\n\n- A `cairn-server` binary or service\n- Protocol endpoint definitions (probably OpenAPI, learning from Gas City's Huma approach in `gascity/engdocs/architecture/api-control-plane.md`)\n- Cross-project schema in Dolt\n- Authentication/authorization layer\n- A plugin SDK for orchestrator integration\n\nNone of this is in the current slate. Adding it now would distract from getting #95 → #105 done. Recorded here so the vision isn't lost; promote to active scope only when local CAIRN has proven its value via the dogfood + case-study evidence the adversarial review §15 #8 demanded.\n\n### Decision\n\n- **Today:** local CAIRN, files + Beads + derived map. The plan we have.\n- **Forward-compatible:** all current trait surfaces and storage decisions accommodate a future server mode without breaking changes.\n- **Deferred:** Cairnhub server, protocol standardization, hosted service, cross-project state aggregation. Real opportunities, wrong time.\n- **Rejected:** code-in-Dolt as a replacement for Git. Wrong battle.\n\n---\n\n## 18. \"Everything is a bead\" — what Gas City actually claims (tutorial-verified)\n\nUser raised: *\"Gas City says everything is built on top of beads, but we were saying everything should be built on top of CAIRN.\"*\n\nRead the official tutorials at `gascity/docs/tutorials/` to check whether the claims actually conflict.\n\n### What the tutorials say verbatim\n\n`docs/tutorials/06-beads.md` line 471: *\"Beads are the ground truth of the **running state** of the city. Everything else in Gas City — sessions, mail, formulas, convoys — is built on top of them.\"*\n\nLine 170: *\"The bead store is effectively the **execution state** of the entire system.\"*\n\nLine 11: *\"Beads are the universal **work primitive** in Gas City.\"*\n\n`docs/tutorials/05-formulas.md`: *\"Beads — the universal **work primitive** underneath formulas, sessions, and everything else.\"*\n\nBead types per tutorial:\n\n| Type | What it is |\n|---|---|\n| task | A unit of work |\n| message | Inter-agent mail |\n| session | A running agent session |\n| molecule | Persistent formula instance |\n| wisp | Ephemeral formula instance |\n| convoy | Container grouping related beads |\n\n**Zero architectural concepts in the type list.** No module, contract, decision, drift finding, interface hash, blueprint node. Gas City's \"everything\" is honestly scoped: every *runtime / work / execution-state* thing is a bead.\n\n### Resolution\n\nThe two claims aren't competing. They cover different layers:\n\n| Layer | Gas City's claim | CAIRN's claim |\n|---|---|---|\n| Architectural truth (modules, contracts, decisions, blueprint, drift) | (out of scope) | CAIRN graph is ground truth |\n| Execution state (sessions, tasks, mail, formula runs, dispatched work) | Beads is ground truth | use Beads via #99 |\n\nThey compose vertically. CAIRN's structural ontology sits **above** Beads's execution ontology. The whole 7-tutorial set covers cities / rigs / agents / sessions / communication / formulas / beads / orders — all runtime concepts. Zero mention of architecture, modules, contracts, drift, blueprint, declared-vs-actual. **CAIRN's territory is unoccupied in Gas City's worldview.**\n\n### Why this matters for the slate\n\nThis strengthens, not weakens, the case for keeping CAIRN distinct:\n\n- The \"extension of Beads\" framing remains rejected: Beads doesn't reach into CAIRN's domain, so CAIRN isn't an extension.\n- The \"graph in Beads\" framing remains rejected: Beads's graph is the work-dependency graph, not the architectural graph.\n- The state-pluggable / content-files split (§16) holds: state is bead-shaped (execution state); architectural content isn't.\n- The Cairnhub vision (§17) is sharpened: it adds a *new* layer (cross-project structural aggregation) that Gas City doesn't claim to cover.\n\n### LSP/lint role: intact\n\nUser flagged: *\"havent forgotten i guess cairn supposed to be able to keep stuff on track, by its sort of linting or LSP, like highlighting stuff done not captured in cairn?\"*\n\nThat role is unaffected by every architectural decision in this session. The drift-gate pieces remain:\n\n- `cairn scan` — finds orphaned files (code that exists but isn't owned by any node)\n- `cairn lint --json` — runs the check battery\n- Interface-hash drift raises \"interface contradiction\" findings (spec §\"Freshness rule\")\n- Drift gate blocks commits when reality diverges from declaration\n- `cairn neighbourhood` — answers \"what does this code touch?\"\n\nThe session has been about what *not* to add to CAIRN. The drift gate hasn't moved.\n\n---\n\n## 19. Three positioning clarifications\n\nSurfaced in conversation late in the session. Each is a sharpening, not a change.\n\n### 19.1 CAIRN is a tool, not a pack\n\nEarlier wording in this analysis loosely said things like \"ship CAIRN as a Gas City pack.\" That muddles two things:\n\n- **CAIRN itself** is a CLI tool (`cairn scan`, `cairn lint`, `cairn neighbourhood`, ...) installed via brew/cargo/script. It runs on a project with or without Gas City. Runs in CI. Runs as a git pre-commit hook. **Not a pack.**\n- **`cairn-governance` (the Gas City adapter)** is a pack — `pack.toml` + formulas + prompts. The formulas shell out to `cairn` commands as steps. The pack is **integration glue**, not a wrapper of CAIRN.\n\nA pack is *\"a reusable agent configuration directory loaded from pack.toml\"* per `gascity/engdocs/architecture/glossary.md`. CAIRN doesn't fit that shape. The pack that uses CAIRN does. Issue #100 is correctly scoped on this — it's *\"`adapters/gascity/` reference pack: formulas, prompts, install steps\"* — the install steps are \"install cairn first.\"\n\n### 19.2 Autonomous generation = drift detection at a different timing\n\nUser raised: *\"i plan to use cairn for autonomous generation too, just thats where we were looking at pairing it (blitzy is hidden proprietary).\"*\n\nBoth use cases reduce to one primitive:\n\n| Use case | When CAIRN runs | What it does |\n|---|---|---|\n| Drift detection | Post-hoc / pre-commit | Verifies existing code matches declared blueprint + contracts |\n| Autonomous generation | During / after generation | Verifies just-generated code matches declared blueprint + contracts |\n\nThe drift gate doesn't know whether code was hand-written or AI-generated. It just enforces invariants. For autonomous generation the agent loop is:\n\n1. Agent reads `cairn.blueprint` + relevant contracts + neighbourhood\n2. Agent generates code\n3. Agent runs `cairn lint --json` to verify\n4. On exit 2 (blocking finding), agent iterates with findings as feedback\n5. On exit 0 (clean), commit\n\nCAIRN doesn't generate code. CAIRN doesn't know about the generating agent. It enforces the invariant either way. **Value of being a tool, not an agent.** Blitzy-style autonomous engineering, Gas City formula dispatch, Claude Code in-IDE — all consume the same primitive.\n\nThis means the slate (#95-#105) serves both use cases. No additional scope needed.\n\n### 19.3 \"Missing piece\" is the product positioning\n\nUser: *\"i don't necessarily want an all in one tool, but i want a very lean ability to achieve these goals. And i guess cairn in my mind is a missing piece, as loads of other things have different parts of the puzzle, with different overlaps in cairn.\"*\n\nRestating the position explicitly:\n\n| Existing tool category | What it does | What it doesn't do |\n|---|---|---|\n| Coding agents (Claude Code, Codex, jcode) | Generate / edit code | Know what's *supposed* to be true |\n| Orchestrators / harnesses (Gas City, Hermes, custom) | Run agents at scale | Know what's *supposed* to be true |\n| Memory systems (automem, mag, beads) | Remember what happened | Know what's *supposed* to be true |\n| Knowledge graphs (graphify, etc.) | Derive structure from code | Enforce what *should* be true |\n\nAcross all of these: nobody declares architectural truth and gates against drift from it. Existing tools *describe* (graphify, beads, automem) or *act* (Claude Code, Gas City, jcode). None *constrain*.\n\n**CAIRN's positioning: the declarative, deterministic constraint layer the ecosystem doesn't have.** Lean by design — a small CLI saying *\"this is supposed to be true, this is what's actually there, here's the diff.\"* Every other tool can use it as the conscience.\n\nThis is *why* CAIRN should not try to be a swiss army knife. Each thing it adds dilutes the constraint-layer identity. The integration value comes *from* being small and focused.\n\nAction implication: when introducing CAIRN to others (docs, README, pitch to gascity-packs reviewers), lead with \"the missing constraint layer.\" Not \"the AI coding framework.\" Not \"the agent orchestration system.\" The constraint layer."
            },
            {
              "type": "sources",
              "path": "meta/sources/beads-repo.md",
              "title": "Beads repository",
              "frontmatter": {
                "date": "2026-05-13",
                "file": "https://github.com/gastownhall/beads",
                "id": "src.beads-repo",
                "type": "repo",
                "verification": "external"
              },
              "body": "\n# Beads repository\n\nExternal source referenced by the no-orchestrator decision. Inspected during the 2026-05-13 integration analysis; not pinned to a specific commit."
            },
            {
              "type": "sources",
              "path": "meta/sources/gas-city-repo.md",
              "title": "Gas City repository",
              "frontmatter": {
                "date": "2026-05-13",
                "file": "https://github.com/gastownhall/gascity",
                "id": "src.gas-city-repo",
                "type": "repo",
                "verification": "external"
              },
              "body": "\n# Gas City repository\n\nExternal source referenced by the no-orchestrator decision. Inspected during the 2026-05-13 integration analysis; not pinned to a specific commit."
            }
          ]
        },
        "research": {
          "schema_version": 1,
          "node": "cairn.root",
          "artefacts": [
            {
              "type": "research",
              "path": "meta/research/gas-city-cairn-integration/analysis.md",
              "title": "Gas City / Beads / cairness — Deep Analysis",
              "frontmatter": {
                "date": "2026-05-13",
                "id": "res.gas-city-cairn-integration"
              },
              "body": "\n# Gas City / Beads / cairness — Deep Analysis\n\n**Date:** 2026-05-13\n**Session:** Claude Code (Opus 4.7, 1M context) on branch `claude/gas-city-cairn-analysis-swwxw`\n**Method:** Live inspection of cloned repos (`gastownhall/gascity`, `gastownhall/beads`), reading of full architecture docs, Yegge's \"Welcome to Gas City\" blog (user-supplied verbatim transcript; original is paywalled), and supplied cairness issue inventory.\n\nAll citations below are repo-relative paths to files inspected in-session against `main` at session time. If this analysis is later promoted to a CAIRN Source artefact, pin the inspected commits explicitly.\n\n---\n\n## 1. Gas City: what it actually is\n\n### 1.1 The model\n\nPer `gascity/engdocs/architecture/nine-concepts.md`, Gas City is five primitives + four derived mechanisms:\n\n**Primitives (Layer 0-1):**\n1. **Session** — start/stop/prompt/observe sessions regardless of provider\n2. **Bead Store** — universal persistence substrate; everything is a bead\n3. **Event Bus** — append-only pub/sub log\n4. **Config** — TOML with progressive activation\n5. **Prompt Templates** — Go text/template in Markdown\n\n**Derived (Layer 2-4):**\n6. Messaging (mail + nudge)\n7. Formulas & Molecules (declarative workflows + runtime instances)\n8. Dispatch (Sling) — agent + formula + molecule composition\n9. Health Patrol — supervision, reconciliation, crash quarantine\n\n### 1.2 The controller loop\n\n`gascity/cmd/gc/controller.go:226` defines `controllerLoop()`. Each tick (default 30s):\n\n1. **Dirty check** — fsnotify-driven config reload via `tryReloadConfig()` at `gascity/cmd/gc/controller.go:137`\n2. **`buildAgents(cfg)`** — evaluates pool `check` commands in parallel, applies suspensions, resolves fixed agents\n3. **`reconcileSessionBeads()`** — declarative convergence between session beads and running sessions; see `gascity/cmd/gc/session_reconciler.go`\n4. **`wispGC.runGC()`** — purges expired molecules per TTL\n5. **`orderDispatcher.dispatch()`** — trigger-conditioned formula/exec dispatch\n\nConfiguration drives everything. From `gascity/engdocs/architecture/controller.md` §Invariants:\n> *\"No role names in Go code. The controller operates on resolved config, runtime session names, and provider state.\"*\n> *\"SDK self-sufficiency: All controller operations function with only the controller process running. No user-configured agent role is required for any infrastructure operation.\"*\n\n### 1.3 What \"drift detection\" means in Gas City\n\n`gascity/engdocs/architecture/controller.md` interactions table:\n> *\"`internal/runtime` | `Provider` interface for Start/Stop/IsRunning/ListRunning/Interrupt/Peek/SetMeta/GetMeta/ClearScrollback. `ConfigFingerprint()` drives drift detection.\"*\n\n`gascity/internal/runtime/fingerprint.go` is *\"`ConfigFingerprint()` (SHA-256 of command + env + extras for drift detection)\"* — drives agent restart when running instance's command/env diverges from declared config.\n\nOther drift usages in repo grep:\n- `gascity/release-gates/ga-9shf-gate.md` — `gc doctor` drift detector for Dolt port mismatches\n- `gascity/plans/archive/huma-openapi-migration*.md` — CI gate ensuring committed OpenAPI spec matches code\n\n**No drift concept between declared system architecture and actual code.** Verified by grep across `gascity/engdocs/`, `gascity/specs/`, and `gascity/internal/` for: `ontolog`, `blueprint`, `interface.hash`, `provenance`, `authority`. Only stray hits (e.g. `gascity/AGENTS.md`: *\"The architecture docs are a reference, not a blueprint\"*), never as an architectural primitive.\n\n### 1.4 Out of scope by Gas City's own declaration\n\n`gascity/specs/architecture.md` §7 explicitly excludes declarative schema specifications and framework positioning. Gas City is a control plane, not a framework.\n\n### 1.5 Runtime providers — leanness confirmed\n\nPer `gascity/engdocs/architecture/session.md`, providers include:\n- `tmux` — primary interactive\n- `subprocess` — local non-interactive\n- `exec` — script-backed\n- `k8s` — pod-backed\n- `acp/auto/hybrid` — routing layers\n\nAn \"agent\" is whatever you put behind a `runtime.Config` (command, env, cwd). Bare Python scripts, Go binaries, curl calls, MCP clients — all work. Nothing forces Claude Code or any heavy harness. Confirms that the leanness concern that motivated cairness's lightweight agent spec is already addressed in Gas City as a first-class case.\n\n---\n\n## 2. Beads (MEOW substrate): what it actually is\n\n### 2.1 Standalone, orchestrator-independent\n\nPer `gastownhall/beads/README.md`:\n> *\"Beads is a CLI tool you install once and use everywhere. You don't need to clone this repository into your project.\"*\n\nInstallation: `brew install beads` / `npm install -g @beads/bd` / curl script. `bd init` initializes in any project; no orchestrator required.\n\n### 2.2 The Bead schema\n\n`gascity/internal/beads/beads.go:Bead`:\n```go\ntype Bead struct {\n    ID           string\n    Title        string\n    Status       string   // \"open\", \"in_progress\", \"closed\"\n    Type         string   // \"task\" default; matches bd wire format\n    Priority     *int\n    CreatedAt    time.Time\n    Assignee     string\n    From         string\n    ParentID     string   // step → molecule\n    Ref          string   // formula step ID or formula name\n    Needs        []string // dependency step refs\n    Description  string\n    Labels       []string\n    Metadata     map[string]string\n    Dependencies []Dep\n}\n```\n\n`Type` is a free-form string. Beads persists; CAIRN would interpret.\n\n### 2.3 Hash IDs and Dolt backing\n\nBeads README §\"Zero Conflict\":\n> *\"Hash-based IDs (`bd-a1b2`) prevent merge collisions in multi-agent/multi-branch workflows.\"*\n\nBeads README §\"Features\":\n> *\"Dolt-Powered: Version-controlled SQL database with cell-level merge, native branching, and built-in sync via Dolt remotes.\"*\n\nFederation via Wasteland is built on Dolt-remote sync; orchestrator-independent.\n\n### 2.4 MEOW is not a library\n\n`gascity/AGENTS.md` verbatim:\n> *\"a thin layer atop the MEOW stack (beads → molecules → formulas).\"*\n\nMEOW = Beads (storage) + Molecules (formula instances, in gascity) + Formulas (TOML workflow definitions, in gascity). **Only Beads is independently installable.** \"MEOW stack\" describes the conceptual sandwich; not a downloadable package.\n\n---\n\n## 3. Gas City's API surface\n\nPer `gascity/engdocs/architecture/api-control-plane.md` §1:\n> *\"Two architectural themes run through everything below: 1. The object model is the center; the CLI and the HTTP + SSE API are projections over it. One canonical domain, two typed surfaces. 2. Typed data end-to-end. Go structs with annotations drive a generated OpenAPI 3.1 contract.\"*\n\n**Surfaces:**\n- CLI (`gascity/cmd/gc/`) — broad subcommand set\n- HTTP + SSE generated via Huma from typed Go structs\n- Generated Go client for cross-process calls\n- SSE event stream for long-running ops: 202 + `request_id` + `request.result` event\n\n**Extension points for external integrators:**\n- **Packs** — declarative agent topologies as TOML + prompts + formulas\n- **Formulas** — `*.formula.toml` workflow definitions\n- **Prompt templates** — Go text/template in Markdown\n- **Runtime providers** — tmux/subprocess/exec/k8s/acp\n- **`exec.Store`** — `provider = \"exec:<script>\"` delegates bead-store ops to user script\n\nThe canonical Gas Town topology itself ships as a pack (`gascity/examples/gastown/`). Per `gascity/examples/gastown/SDK-ROADMAP.md`: *\"~1,200 lines of Go to make Gas Town run as pure configuration.\"* Even Gas Town is just-a-pack.\n\n---\n\n## 4. Cairness scope vs Gas City overlap matrix\n\nBased on cairness issues #1, #2, #6, #7, #9, #10, #14 (supplied by user; repo `george-rd/cairness` is private). Coverage assessment against Gas City code and docs read in-session:\n\n| Cairness issue | Scope | Gas City equivalent | Verdict |\n|---|---|---|---|\n| **#1** Epic: Grapharness | Lightweight harness-agnostic agent orchestration on CAIRN graph; <5MB Rust; 2-3k LOC | Full control plane in Go | **Standalone form duplicative.** Salvage: graph-walking scheduler concept (~400 LOC) → Gas City formula |\n| **#2** Flow engine + YAML DAG, 500-700 LOC | YAML step DAG with conditions, retries, actions | Formulas + molecules (TOML + bead trees) | **Duplicated** |\n| **#6** Adapter registry, 200+150/adapter | YAML adapter contracts for jcode/CC/litellm/codex | Runtime providers + prompt templates | **Mostly duplicated.** Per-harness glue lives in packs |\n| **#7** Wave scheduler walking CAIRN graph, 400-500 LOC | Walk CAIRN graph, group into parallel waves, apply policy | Controller is config-driven, not graph-driven | **Not duplicated.** Real novel piece |\n| **#9** Stats + dashboard + self-improvement, 1150 LOC | SQLite metrics, TUI/web dashboard, analysis agents propose flow changes | Event bus + Dolt audit | **Data layer duplicated.** Self-improvement loop novel |\n| **#10** YAML flows vs CAIRN primitives | Architecture decision parked | — | Decision becomes: orchestrator-agnostic CAIRN with optional Beads backend |\n| **#14** SQLite cache + DB state (closed-source) | CAIRN open-source file-based, cairness closed-source DB-backed | Dolt via Beads | **Dolt strictly better than SQLite** for versioning/branching/federation |\n\n**Estimated overlap:** ~70%. Two novel pieces (#7, #9) survive but are formula-sized (hundreds of LOC), not standalone-orchestrator-sized (thousands).\n\n### 4.1 Where the surviving novel pieces actually live\n\n**Cairness #7 (graph-walking wave scheduler) splits across the two sides of the integration:**\n\n- **CAIRN side (~50-100 LOC).** The graph-walking primitive — *\"given the current change, what's ready right now?\"* — must live where the graph definition lives. Concretely: `cairn query --ready --change <id> --json` walks blueprint + active change, applies `needs:` edge resolution, groups results by topological depth, emits waves as JSON. Covered by existing slate issues #4 (JSON contract), #9 (tasks-as-beads gives `bd ready` for free when beads-backed), and #3 (`ArtefactStore.query_by_dependency`).\n\n- **Orchestrator side (~300-400 LOC, free re-use).** Wave dispatcher, concurrency limit, retry policy, role-based routing — these are operational, not architectural. Gas City already ships them via formula `needs:` edges, runtime pools, `max_restarts`/`restart_window`, label-based routing. In `adapters/gascity/` (issue #6) this becomes one formula (`cairn-wave-dispatch.formula.toml`) + a small worker prompt template.\n\nCairness was estimating 400-500 LOC because it was building the dispatcher from scratch. The dispatcher already exists in Gas City. We just need the right query feeding it. No new slate issue needed — the work is distributed across #3, #4, #6, #9.\n\n**Cairness #9 (self-improvement loop)** is similarly distributed: Gas City + Dolt gives the audit data; the analysis-agent-proposes-changes loop is one or two formulas on top, also in `adapters/gascity/` or as a future skill. Defer until the data is flowing.\n\n---\n\n## 5. What CAIRN already has\n\nVerified by source inspection in `~/cairn/`:\n\n- `src/changes/` — change primitive with `artefact_ops.rs`, `types.rs`, `validate.rs`. Hooks for `CAIRN_CHANGE_ARTEFACT_CONFLICT` (`src/hooks/mod.rs:144`).\n- `src/cli/accept.rs:run_accept_gate(change_id)` — apply/verify gate\n- `src/cli/commands.rs:run_archive_command` — archive command\n- `cairn.kernel.changes` module declared in `cairn.blueprint`\n- Spec §9 — change directories, delta semantics (ADDED/MODIFIED/REMOVED/RENAMED), archive operation\n- Spec line 178 — planned location `./meta/changes/`\n- Spec §4 verbatim:\n  > *\"Cairn and OpenSpec solve different problems (OpenSpec is a change-lifecycle workflow, Cairn is a structural reconciliation framework), but OpenSpec's change-isolation and delta-merging patterns are directly applicable and are adopted in sections 9 and 12. **Cairn deliberately does not adopt OpenSpec's workflow layer**; the two tools are complementary and could coexist in the same repo.\"*\n\nThat non-goal needs amendment if openspec is to be retired entirely. See issue-slate.md #8.\n\n---\n\n## 6. What CAIRN does NOT have (openspec retirement gaps)\n\n1. Conversational skills (`cairn-propose`, `cairn-explore`, `cairn-apply`, `cairn-archive`) — openspec's day-to-day value via `/openspec-propose` and friends.\n2. `cairn change new <name>` scaffold with proposal.md / design.md / tasks.md templates.\n3. In-change task tracking. OpenSpec has tasks.md; CAIRN doesn't yet. Beads with `parent=<change-id>` is the clean answer.\n4. `cairn import-openspec` migration helper.\n5. Registries as graph queries (currently `openspec/registries/*.md` as files).\n6. Conventions surface (currently `openspec/conventions.md`; should be per-module `rules` blocks in `cairn.blueprint` or a top-level Source on `cairn.root`).\n7. One-way switch: `openspec/changes/` → `meta/changes/`.\n\nNone of these are kernel-deep. Skills, scaffolds, a migration script, and small CLI commands. Reliable retirement is weeks of work, not months.\n\n---\n\n## 7. The structural argument\n\nThree layers, three concerns:\n\n```\nLayer 3: Orchestration (optional)\n   Gas City controller / sessions / packs / formulas\n   CAIRN consumed as formula steps\n   Future runners: adapters/<name>/\n\nLayer 2: Semantic (CAIRN's lane)\n   cairn.blueprint, typed artefacts, two-chain topology\n   Reconciler, drift gate, interface hashes\n   No equivalent in Gas City (verified by grep)\n\nLayer 1: Storage (pluggable)\n   Default: filesystem\n   Optional: Beads (bd CLI / Dolt-backed)\n   CAIRN trait: ArtefactStore\n```\n\n- Gas City: Layers 1 + 3, no Layer 2.\n- CAIRN: Layer 2, pluggable Layer 1, externalised Layer 3.\n- Beads: Layer 1 only.\n\nThese compose. They do not compete.\n\n---\n\n## 8. Yegge's framing (from supplied article transcript)\n\nDirect quotes from the \"Welcome to Gas City\" Medium article (user-supplied verbatim; original at https://steve-yegge.medium.com/welcome-to-gas-city-57f564bb3607 is paywalled):\n\n- *\"Gas City has deconstructed the entire Gas Town stack into composable, declarative building blocks called 'packs'.\"*\n- *\"MEOW, the Molecular Expression of Work, is a lightweight Beads-based framework that places Work front and center, as the first-class system primitive, creating a versioned knowledge graph of all your issues and tasks.\"*\n- *\"every agent action recorded in a git-versioned Dolt database. That's your SOC2 story, sitting right there in the database, already written.\"*\n- *\"any agent can go temporarily insane, at any time, and make a bad call. No matter how smart they are.\"*\n- *\"To replace SaaS, you need the unglamorous stuff: declarative deploys, audit trails, version history, identity, and a memory layer that survives the inevitable agent failures.\"*\n- *\"Gas City is a high-control system. It has high parallelism... but it uses structure to keep agent swarms organized.\"*\n\nThese quotes establish that:\n\n- Yegge's \"knowledge graph\" is the **work-as-graph** (beads with deps), not architecture-as-graph\n- The reliability story is **probabilistic** (more agents reviewing each other), not deterministic (gate at commit)\n- The pitch is **replace SaaS / business process automation**, not architectural governance\n\nCAIRN's deterministic-gate-at-commit + architectural-truth angle is complementary, not competing.\n\n---\n\n## 9. Decisions reached this session\n\n1. **Keep CAIRN.** Architecture-truth / typed-artefact / drift-gate / two-chain authority layer is genuine white space. Verified by grep of Gas City; no analogue.\n2. **Retire cairness as scoped.** ~70% overlap with Gas City's mature surface. Salvage the graph-walking scheduler (~400 LOC) as a Gas City formula in `adapters/gascity/`.\n3. **Retire cflx.** Was always experimental; CAIRN's `accept`/`archive` primitives plus an external runner replace it.\n4. **Adopt Beads as a pluggable storage backend.** Optional but worth it: hash-IDs, Dolt versioning, federation via Wasteland, no orchestrator coupling.\n5. **CAIRN does not ship its own orchestrator.** Integration with Gas City via a `cairn-gc` reference pack; future runners get their own adapter under `adapters/`.\n6. **Retire `openspec/changes/`.** Move active phases to `meta/changes/` (already planned per spec line 178). OpenSpec workflow replaced by CAIRN skills + (optionally) beads-backed tasks.\n7. **Amend spec §4** to reflect that workflow lives externally (skills + optional formulas), not as a CAIRN non-goal.\n\n---\n\n## 10. Honest limitations of this analysis\n\n- The Medium article was paywalled; analysis used user-supplied verbatim transcript. Quotes are traceable to that transcript.\n- `cairn` binary was not built in the session sandbox; analysis used grep/find/Read directly. A repeat with `cairn context` + `cairn neighbourhood` available would likely surface more.\n- `bd` was not installed in the session sandbox; Beads claims were verified via README + cloned source inspection only, not via runtime use.\n- cairness scope is from the issue inventory supplied by the user (`#1, #2, #6, #7, #9, #10, #14`). The repo `george-rd/cairness` is private; source not inspected.\n- Gas City and Beads repos were cloned shallow (`--depth 1`) to `/tmp/gc-review/gascity` and `/tmp/beads-repo`. Tag/commit not pinned. If this analysis is promoted to a Source artefact, re-clone with explicit refs and re-verify.\n\n---\n\n## 11. The \"graph IS orchestration\" framing\n\nSurfaced in conversation after the initial slate was drafted. Cairness #7 was reaching for this; the spec hints at it (line 71: *\"Decisions can declare the blueprint nodes they apply to; the framework can then flag when a change to those nodes appears to violate the decision (v2 capability, deferred)\"*).\n\nTwo distinct meanings:\n\n**(a) Reactive: graph state changes drive work.** New `Todo` appears → worker spawned. `Contract` interface hash changes → drift gate fires. `Decision` flips to `accepted` → implementation work materialises.\n\n**(b) Declarative: node types carry workflow semantics.** Each artefact type has an associated lifecycle and an associated kind-of-work. `Contract`: draft → reviewed → accepted. `Todo`: proposed → ready → claimed → done. The graph topology directly maps to dispatch decisions.\n\nBoth are CAIRN-side concerns. Neither requires CAIRN to own the dispatcher. The right division of labour:\n\n- **CAIRN owns the semantics:** which node states imply which work types, what the lifecycle transitions are, when the drift gate must fire\n- **The orchestrator owns the runtime:** parallelism, retries, pool scaling, crash recovery\n\nThis preserves the cairness vision in spirit (graph-native orchestration) while extracting the orchestrator into Gas City where it's more mature.\n\nThree operational paths for graph-state-driven work in the Gas City world:\n\n1. **CAIRN queries drive Gas City formulas.** `cairn query --ready --change <id>` returns ready wave; Gas City formula dispatches. Covered by #98 + #100.\n2. **Beads-mediated.** Typed beads (`type=contract`) become work items via existing `bd ready` detection. Covered by #99 + #103.\n3. **SSE reactive** (strongest form). CAIRN emits events on graph state changes; Gas City Orders react. Covered by #96 + #101.\n\n**Gap in the current slate:** explicit `node-type → workflow` association in `cairn.blueprint`. Example: `Module @api → on_drift: cairn-drift-gate`, `Contract → on_status_change(accepted): cairn-implement`. The orchestrator becomes a dumb pump that runs whatever formula the graph state says is implied. This is the missing piece that makes \"graph IS orchestration\" concrete on the CAIRN side. Candidate for a new slate issue; pending decision.\n\n---\n\n## 12. Gas City tech-debt assessment\n\nAsked late in the session because contributing back upstream became a strategic option. Concrete numbers from `/tmp/gc-review/gascity`:\n\n| Signal | Value | Read |\n|---|---|---|\n| TODO/FIXME/HACK in non-test Go | 21 across ~250k LOC | 0.0084% density — well below industry concern |\n| Test files | 796 | Heavy investment |\n| Active design RFCs (`engdocs/design/`) | 20 | Working RFC pipeline; debt is documented before it's debt |\n| Archived RFCs | 18 | Things actually ship and graduate |\n| CHANGELOG detail | Per-fix operator-impact notes | Mature release engineering |\n| Pre-commit hooks | Auto-regen OpenAPI + dashboard schema + lint + vet + test | CI-equivalent gates run locally |\n| Recent activity | PR #1169 in last commit message | High velocity, large contributor base |\n\nSample TODOs read as `// Wired: TODO — operation context plumbing pending` — deliberate incremental implementation, not rot. No \"broken and we don't know how to fix\" debt visible.\n\n`CONTRIBUTING.md` verbatim: *\"Gas City is experimental software, but the repo is now structured for external contributors.\"*\n\n**Verdict:** healthy. Contributing into Gas City would not be a rescue mission.\n\n---\n\n## 13. Contribution-path strategy\n\nKey finding: `gastownhall/gascity-packs` exists as the explicit community pack home. README verbatim: *\"A collection of opt-in Gas City packs... Packs compose through `pack.toml` imports, so a city can opt into any subset of the packs in this repo without forking.\"*\n\nSo the upstream contribution path is:\n\n1. Build `adapters/gascity/` in `cairn-framework/cairn` (issue #100)\n2. Dogfood locally for some weeks\n3. Polish: docs, README, pinned Gas City version\n4. Submit to `gastownhall/gascity-packs` as `packs/cairn-governance/` (or similar)\n5. Optionally: a small PR to `gascity` core if a genuine integration-contract gap surfaces (unlikely; their extension model is good)\n\nWe're not contributing into 250k LOC of Go. We're contributing a pack (TOML + Markdown + prompt templates + a thin shim that shells to `cairn`). Tractable from outside; minimal upstream maintainer load.\n\n**Community angle:** if `cairn-governance` lands in `gascity-packs`, CAIRN gets a discovery channel to ~15k-star Gas City community. The Gas City Discord audience (~2,000 active members per Yegge's article) is *exactly* the audience for architecture governance — people running multi-agent systems who've felt the hallucination pain and want deterministic gates. CAIRN repo stays the canonical home; the pack is the bridge.\n\nLow-risk strategic bet. Downside is zero — you'd build the pack anyway for your own use under issue #100.\n\n---\n\n## 14. Authoring workflows: same principle, applied to progressive disclosure\n\nThe \"workflow lives outside CAIRN; CAIRN provides atomic operations\" pattern from §11 also applies to **how a user builds out a spec one node at a time**. Surfaced in conversation when comparing to getcairn.dev's progressive-disclosure spec UX.\n\nA multi-step node creator — pick type → assign ID/name → fill required fields → validate → commit — is a workflow with `needs:` edges, conditional retries, and human-in-the-loop pauses. **That's what formulas are designed for.**\n\n### What CAIRN ships\n\nAtomic, composable, JSON-shaped CLI commands. Each independently testable.\n\n- `cairn node template --type=<artefact-type>` → emits a JSON schema with required/optional fields for the type\n- `cairn validate-node --file=<draft.toml> --strict` → exit 0/1/2 + JSON findings\n- `cairn change add-node --change=<id> --file=<draft.toml>` → idempotent commit\n\n### What CAIRN does NOT ship\n\n- Interactive prompt loops\n- Wizard state machines\n- Branching flow logic (\"if Contract, also ask for X\")\n- Retry/recovery on validation failure\n- Persistence of partial drafts\n\n### Where the wizard UX actually lives\n\nTwo surfaces, sharing the same underlying commands:\n\n**Formula version (Gas City users) — lives in `adapters/gascity/`:**\n\n```toml\nformula = \"cairn-propose-node\"\n\n[[steps]]\nid = \"pick-type\"\ndescription = \"Ask user: Module, Contract, Decision, Todo, Research, Review, Source\"\n\n[[steps]]\nid = \"id-and-name\"\nneeds = [\"pick-type\"]\ndescription = \"Run: cairn node template --type={{type}}; ask user for id + name\"\n\n[[steps]]\nid = \"fill-fields\"\nneeds = [\"id-and-name\"]\ndescription = \"Read template; prompt for each required field one at a time\"\n\n[[steps]]\nid = \"validate\"\nneeds = [\"fill-fields\"]\ndescription = \"cairn validate-node --file={{tmpfile}}; on exit 2, loop back to fill-fields with errors\"\n\n[[steps]]\nid = \"commit\"\nneeds = [\"validate\"]\ndescription = \"cairn change add-node --change={{change}} --file={{tmpfile}}\"\n```\n\n~30 lines of TOML. All flow state lives in the formula's molecule (bead tree). All semantic correctness lives in CAIRN's atomic commands.\n\n**Skill version (Claude Code / Codex / chat agents) — lives in `.claude/skills/`:**\n\nSame workflow, different surface. Markdown skill that drives the same atomic commands conversationally. Covered by issue #102.\n\n### Why this matters\n\nThe two surfaces — formula and skill — diverge only in *how they ask the user*. The CAIRN commands underneath are identical. This means:\n\n- Any future surface (web UI, TUI, getcairn.dev clone, IDE plugin) is a thin client over the same CLI\n- Each atomic command is unit-testable in isolation\n- The wizard's logic (which questions, what order, branching on type) is reviewable as a single TOML or Markdown file, not buried in Rust\n- CAIRN's binary stays small\n\n### Connection to slate issues\n\nThis **refines**, not adds:\n\n- **#98 (Stable JSON + exit codes)** hardens what \"atomic\" means: no command embeds multi-step state\n- **#100 (`adapters/gascity/` pack)** gains `cairn-propose-*.formula.toml` files as first-class content\n- **#102 (change-lifecycle skills)** gets the symmetric markdown skills\n\nNo new issue needed.\n\n### Risk\n\nIf every workflow lives outside CAIRN, *CAIRN-the-product* could feel skeletal to a new user. *\"I installed cairn but there's no `cairn wizard` command?\"*\n\n**Mitigation:** ship the skills + formulas in `.claude/skills/` and `adapters/gascity/` directories of the CAIRN repo itself. A fresh clone has the wizard UX available out of the box. The composition lives in the repo; only the *engine* runs externally.\n\n---\n\n## 15. Adversarial review\n\nRun at session-end when the plan/direction felt settled. Each item below is the strongest counter-argument against a decision in this analysis, answered honestly.\n\n### 1. \"CAIRN should just be a Gas City pack from day one. No separate Rust binary.\"\n\nCounter: drift detection needs to run **offline**, **in CI**, **on machines without Gas City**, as a **git pre-commit hook**. The reconciler must produce a content-addressable fingerprint per spec §3.5 — that's a deterministic-correctness claim, can't depend on an orchestrator. Standalone binary is essential. *Pressure created:* keep the Rust footprint tight enough to install in CI without pain.\n\n### 2. \"Three layers (semantic / storage / orchestrator) are too many. Skip the trait, just adopt Beads + skills.\"\n\nCounter: filesystem default is required for `brew install cairn` to work without `brew install beads`. Adoption friction matters. The trait is the seam between CAIRN-works-alone and CAIRN-better-with-Beads. Not speculative. Keep.\n\n### 3. \"Gas City might pivot or fade in 6 months.\"\n\nCounter: the adapter is small (formulas + prompts in `adapters/gascity/`). Core stack (#95–#98) is orchestrator-neutral. If Gas City fades, only `adapters/gascity/` needs replacing. *Pressure created:* don't let Gas-City-specific concepts leak into #96 (integration contract). It must stay generic.\n\n### 4. \"OpenSpec retirement is risky. Replacement skills are unbuilt.\"\n\nCounter: phasing is explicit. #102 + #103 must land and prove themselves before #104 fires. **Action item:** edit #104 body to add \"Blocked by: #102, #103.\"\n\n### 5. \"Authoring-workflows-as-external will fragment the user experience.\"\n\nCounter: skill and formula share the same `cairn` commands underneath. Divergence is bounded to question-asking surface. **Action item:** edit #102 acceptance to specify that required-field set + validation rules come from CAIRN (`cairn node template --type=X --json`), not duplicated in surfaces.\n\n### 6. \"11 issues is overscoped for solo work.\"\n\nCounter: roadmap, not sprint backlog. Phased dependencies are clear; agnostic core (#95–#98) is one-quarter scope. *Pressure created:* cross-refs between issues should be explicit. Currently only #99 references #91.\n\n### 7. \"Beads might fade too. Yegge-affiliated.\"\n\nCounter: Beads is more independent than cairness ever would have been (separate repo, brew/npm install, MIT). And the trait in #97 means we can swap backends. Lock-in bounded. *Pressure created:* the trait surface must be defined in terms of what CAIRN needs, not what Beads can offer.\n\n### 8. **Deepest risk.** \"The architecture-truth bet hasn't been validated externally. It might be wrong.\"\n\nCounter: acknowledged, not fully resolvable from inside. Yegge's probabilistic-reliability bet (more agents reviewing each other) might just be correct, and deterministic gates might be solving a problem nobody has. **Mitigation:** dogfood CAIRN aggressively *before* submitting `cairn-governance` to `gascity-packs`. Document concrete cases where the drift gate caught something a probabilistic agent review would have missed. Without case studies, the upstream submission is unsubstantiated. **Action item:** README open question — what counts as adequate validation evidence before upstream submission?\n\n### 9. \"Gas City community might reject a Rust-shim pack.\"\n\nCounter: subprocess/exec providers exist to run anything. Polyglot via subprocess is intentional. Examples in `gascity/examples/` already shell to bash. Low-medium risk.\n\n### 10. \"External workflows mean less out-of-the-box. openspec ships more.\"\n\nCounter: §14 mitigation — ship the skills + formulas in CAIRN's own repo. Fresh clone has everything. *Pressure created:* CAIRN's own README needs to lead with the wizard UX, not the kernel architecture. First-run experience matters.\n\n### Verdict\n\nThe plan survives the adversarial review. The deepest risk (#8 — validation of the architectural-truth bet) is unavoidable; you can't prove it from inside. Mitigation is dogfood + case studies before public submission.\n\nThree actionable sharpenings, captured as open questions / proposed issue edits:\n\n1. **Edit #104:** add \"Blocked by: #102, #103.\"\n2. **Edit #102 acceptance:** specify CAIRN owns the required-field set + validation rules; both surfaces consume `cairn node template --type=X --json`.\n3. **New README open question:** what counts as adequate validation evidence before upstream submission to `gascity-packs`?\n\n---\n\n## 16. Storage model refinement: content / state / map split\n\nSurfaced when the user asked directly: *\"Would we look at storing cairn's graphics and relations in Dolt/beads? Or should we look at it as an extension of beads?\"*\n\nPulling those apart led to a meaningful refinement of earlier issues #97 and #99.\n\n### Three distinct things, three distinct storage answers\n\n| Thing | What it is | Storage |\n|---|---|---|\n| **Content** | Authored text: `cairn.blueprint`, contract bodies, decision rationale, research notes, sources, todo descriptions. Reviewed in PRs. | **Files. Always.** Markdown + `cairn.blueprint`. No pluggable backend. |\n| **State** | Metadata about content: status, assignee, atomic claim, dependency edges between work items. Mutable. Two agents can race. | **Pluggable.** Filesystem default (status in frontmatter). Beads optional (atomic claim, hash IDs, Dolt versioning). |\n| **Map** | Typed node graph derived from parsing content + reconciling against the filesystem. | **Ephemeral by default; optional cache** (Dolt or SQLite, per cairness #14). Source of truth is files. |\n\n### Why this is cleaner than \"ArtefactStore for everything\"\n\n- **Atomic merge with code preserved.** A new contract or todo usually lands in the same PR that implements/adopts it. If content lives in Git, contract change + code change merge atomically as one unit, revert together, appear together in `git log`. If content lived in a separate Dolt store, you'd have a two-VCS coordination problem (no distributed transaction between Dolt and Git).\n- **Beads's strengths used where they matter.** Atomic claim, hash IDs, Dolt versioning — for state and work, where races and audit actually happen.\n- **No backend pluggability for content (today).** Files are the canonical format for commit-coupled content. The pluggable layer is the *state backend*, a much smaller surface.\n- **Reconciler simplicity.** Reads files, compares to filesystem, emits findings. No database round-trip per node.\n\n### What's *not* a reason for files-canonical\n\nThe earlier draft of this section claimed \"Dolt loses line-level diffs\" and \"content already git-versioned, so don't waste Beads on it.\" Both wrong:\n\n- **Dolt does have cell-level diffs.** Text content in a `text` column is fully diff-able across commits and branches. Beads proves this: every issue body lives in Dolt's `Description` column and is fully versioned, diffed, branched.\n- The \"already git-versioned\" argument was circular — it assumed git is the right versioner, which is the question, not the answer.\n\nThe actual argument is atomic-merge-with-code, above. That's the load-bearing constraint.\n\n### Per-artefact-type implications\n\n| Artefact type | Content storage | State storage |\n|---|---|---|\n| Contract | file | (none — derived from interface hash) |\n| Source | file | (none — immutable) |\n| Decision | file | bead (status: proposed/accepted/deprecated/superseded) |\n| Research | file | bead (status) |\n| Review | file | bead (status, who reviewed when) |\n| Todo | file | bead (status, assignee, claim) |\n\nHybrid artefacts (Decision, Research, Review, Todo) get the cleanest model: markdown owns *content*, bead owns *state*. The bead's `ref` field points at the markdown file path. `cairn get <id>` reads both. When the state backend is filesystem-only, state lives in markdown frontmatter — same fields, just no atomic-claim guarantee.\n\n### Two axes of pluggability (refined)\n\n| Axis | Default | Optional (today) | Optional (future) |\n|---|---|---|---|\n| **State** | filesystem (frontmatter) | Beads (#97 / #99) | remote `StateBackend` (Cairnhub) |\n| **Content** | filesystem (atomic merge with code) | — | Dolt-direct `ContentBackend` for non-commit-coupled artefacts |\n\nToday's slate covers **state**. Content stays filesystem-only by default because the artefacts CAIRN cares about (blueprint, contract, decision, todo bound to code) are commit-coupled. A future `ContentBackend` trait could mirror `StateBackend`, with filesystem as default and Dolt-direct as an option — for artefacts that *don't* travel with code (Cairnhub-style cross-project decisions, multi-project research, agent-action audit logs).\n\n### Slate impact\n\n- **#97 (now `StateBackend`)** — narrowed to state only for today. Forward-compatible with a future `ContentBackend` sibling.\n- **#99 (now Beads `StateBackend`)** — narrowed accordingly.\n- **No `ContentBackend` issue today.** Filesystem content is the right default while local-project workflows dominate. Add the trait only when Cairnhub-style multi-project workloads create real demand.\n\nThe \"extension of Beads\" framing remains rejected: CAIRN's commit-coupled content is not in Beads, today or ever, because it needs to merge atomically with code. The \"graph in Dolt\" framing is *partially* rejected: the graph stays derived locally; only state (today) and potentially non-commit-coupled content (future) go to Dolt.\n\n---\n\n## 17. Cairnhub: the long-horizon vision (not slate work)\n\nUser raised: *\"if dolt is VCS like git, we get cairn to be like a dolt powered system, which uses beads i guess for the task part, but it also just has all the code etc in one? So its like an agentic coding VCS. Cairnhub.\"*\n\nWorth capturing the shape, the rejections, and the forward-compatible parts.\n\n### Cairnhub's natural domain: non-commit-coupled artefacts\n\nRefined in light of §16's atomic-merge-with-code constraint: Cairnhub's clearest value is for artefacts that **don't** need to land atomically with specific code commits. Those are:\n\n- Cross-project decision archive (\"which projects adopted dec.use-shared-crypto?\")\n- Agent-action audit log (\"which agent did what in any project last week?\")\n- Cross-project contract dependencies (module A in project X importing contract from project Y)\n- Federated research across organisations\n- Hosted shared libraries of skills/model-definitions\n\nPer-project, commit-coupled content (blueprint, contract bodies, todos tied to specific code) stays in git repos under any architecture. Cairnhub indexes; it doesn't repatriate.\n\nThis sharpens what Cairnhub is *for* and what it's *not* for.\n\n### What's real in the vision\n\nDecomposed into evaluable pieces:\n\n| Piece | Worth pursuing? | When |\n|---|---|---|\n| Single-project single source of truth | ✓ Already in plan (§16) | Now |\n| Multi-project state aggregation | ✓ | Future server mode |\n| Standard agent skills + model definitions protocol | ✓ | Future |\n| Cross-orchestrator agent-action audit log | ✓ Yegge's SOC2 angle | Future |\n| Hosted \"Cairnhub\" SaaS | Possibly | Far future |\n| **Replace Git for code** | ✗ | Never |\n\n### Why \"replace Git\" is the wrong fight\n\nCode is unstructured text in files. Dolt wants structured rows in tables. Storing code as text blobs in Dolt costs: line-level diffs, blame, hunk operations, every IDE integration, GitHub network effects. Every previous \"replace Git\" attempt (Mercurial, Pijul, Fossil, Bazaar) is technically superior in some way and has tiny adoption. Network effects via GitHub are the strongest force in software tooling.\n\n### What the realistic Cairnhub looks like\n\nA *server tier* above today's local-file architecture:\n\n1. **Indexes multiple project repos.** Each project still has its own `cairn.blueprint`, content as files, Git as VCS for code.\n2. **Aggregates state in Dolt** — tables for projects, cross-project contracts, decisions-of-record, tasks-by-project, audit log of agent actions. Federation via Dolt remotes between teams/orgs.\n3. **Exposes a query API** — \"show me all contracts across all projects whose interface hash changed and have no review in 30 days\" becomes one SQL query.\n4. **Hosts standard protocol endpoints:**\n    - `GET /context/<project>` — current map + active change + ready tasks\n    - `POST /action` — agent publishes \"I did X\" (audit-log row)\n    - `POST /validate` — run drift gate against proposed change, return findings\n5. **Provides a plugin contract** — orchestrators (Gas City, Hermes, Claude Code, future) implement it. Plugins let agents read CAIRN context and publish actions; they don't replace Git.\n\n### Why today's architecture is forward-compatible\n\nThe `StateBackend` trait (#97) is already the right seam. Today's impls: filesystem, Beads (local Dolt). A future impl: `CairnhubBackend` (talks to a remote Dolt-backed CAIRN server). Trait surface unchanged.\n\nContent stays as files in repos, regardless of whether you run local-only or against Cairnhub. The server indexes; it doesn't replace.\n\n### What a Cairnhub Phase would actually add (someday)\n\n- A `cairn-server` binary or service\n- Protocol endpoint definitions (probably OpenAPI, learning from Gas City's Huma approach in `gascity/engdocs/architecture/api-control-plane.md`)\n- Cross-project schema in Dolt\n- Authentication/authorization layer\n- A plugin SDK for orchestrator integration\n\nNone of this is in the current slate. Adding it now would distract from getting #95 → #105 done. Recorded here so the vision isn't lost; promote to active scope only when local CAIRN has proven its value via the dogfood + case-study evidence the adversarial review §15 #8 demanded.\n\n### Decision\n\n- **Today:** local CAIRN, files + Beads + derived map. The plan we have.\n- **Forward-compatible:** all current trait surfaces and storage decisions accommodate a future server mode without breaking changes.\n- **Deferred:** Cairnhub server, protocol standardization, hosted service, cross-project state aggregation. Real opportunities, wrong time.\n- **Rejected:** code-in-Dolt as a replacement for Git. Wrong battle.\n\n---\n\n## 18. \"Everything is a bead\" — what Gas City actually claims (tutorial-verified)\n\nUser raised: *\"Gas City says everything is built on top of beads, but we were saying everything should be built on top of CAIRN.\"*\n\nRead the official tutorials at `gascity/docs/tutorials/` to check whether the claims actually conflict.\n\n### What the tutorials say verbatim\n\n`docs/tutorials/06-beads.md` line 471: *\"Beads are the ground truth of the **running state** of the city. Everything else in Gas City — sessions, mail, formulas, convoys — is built on top of them.\"*\n\nLine 170: *\"The bead store is effectively the **execution state** of the entire system.\"*\n\nLine 11: *\"Beads are the universal **work primitive** in Gas City.\"*\n\n`docs/tutorials/05-formulas.md`: *\"Beads — the universal **work primitive** underneath formulas, sessions, and everything else.\"*\n\nBead types per tutorial:\n\n| Type | What it is |\n|---|---|\n| task | A unit of work |\n| message | Inter-agent mail |\n| session | A running agent session |\n| molecule | Persistent formula instance |\n| wisp | Ephemeral formula instance |\n| convoy | Container grouping related beads |\n\n**Zero architectural concepts in the type list.** No module, contract, decision, drift finding, interface hash, blueprint node. Gas City's \"everything\" is honestly scoped: every *runtime / work / execution-state* thing is a bead.\n\n### Resolution\n\nThe two claims aren't competing. They cover different layers:\n\n| Layer | Gas City's claim | CAIRN's claim |\n|---|---|---|\n| Architectural truth (modules, contracts, decisions, blueprint, drift) | (out of scope) | CAIRN graph is ground truth |\n| Execution state (sessions, tasks, mail, formula runs, dispatched work) | Beads is ground truth | use Beads via #99 |\n\nThey compose vertically. CAIRN's structural ontology sits **above** Beads's execution ontology. The whole 7-tutorial set covers cities / rigs / agents / sessions / communication / formulas / beads / orders — all runtime concepts. Zero mention of architecture, modules, contracts, drift, blueprint, declared-vs-actual. **CAIRN's territory is unoccupied in Gas City's worldview.**\n\n### Why this matters for the slate\n\nThis strengthens, not weakens, the case for keeping CAIRN distinct:\n\n- The \"extension of Beads\" framing remains rejected: Beads doesn't reach into CAIRN's domain, so CAIRN isn't an extension.\n- The \"graph in Beads\" framing remains rejected: Beads's graph is the work-dependency graph, not the architectural graph.\n- The state-pluggable / content-files split (§16) holds: state is bead-shaped (execution state); architectural content isn't.\n- The Cairnhub vision (§17) is sharpened: it adds a *new* layer (cross-project structural aggregation) that Gas City doesn't claim to cover.\n\n### LSP/lint role: intact\n\nUser flagged: *\"havent forgotten i guess cairn supposed to be able to keep stuff on track, by its sort of linting or LSP, like highlighting stuff done not captured in cairn?\"*\n\nThat role is unaffected by every architectural decision in this session. The drift-gate pieces remain:\n\n- `cairn scan` — finds orphaned files (code that exists but isn't owned by any node)\n- `cairn lint --json` — runs the check battery\n- Interface-hash drift raises \"interface contradiction\" findings (spec §\"Freshness rule\")\n- Drift gate blocks commits when reality diverges from declaration\n- `cairn neighbourhood` — answers \"what does this code touch?\"\n\nThe session has been about what *not* to add to CAIRN. The drift gate hasn't moved.\n\n---\n\n## 19. Three positioning clarifications\n\nSurfaced in conversation late in the session. Each is a sharpening, not a change.\n\n### 19.1 CAIRN is a tool, not a pack\n\nEarlier wording in this analysis loosely said things like \"ship CAIRN as a Gas City pack.\" That muddles two things:\n\n- **CAIRN itself** is a CLI tool (`cairn scan`, `cairn lint`, `cairn neighbourhood`, ...) installed via brew/cargo/script. It runs on a project with or without Gas City. Runs in CI. Runs as a git pre-commit hook. **Not a pack.**\n- **`cairn-governance` (the Gas City adapter)** is a pack — `pack.toml` + formulas + prompts. The formulas shell out to `cairn` commands as steps. The pack is **integration glue**, not a wrapper of CAIRN.\n\nA pack is *\"a reusable agent configuration directory loaded from pack.toml\"* per `gascity/engdocs/architecture/glossary.md`. CAIRN doesn't fit that shape. The pack that uses CAIRN does. Issue #100 is correctly scoped on this — it's *\"`adapters/gascity/` reference pack: formulas, prompts, install steps\"* — the install steps are \"install cairn first.\"\n\n### 19.2 Autonomous generation = drift detection at a different timing\n\nUser raised: *\"i plan to use cairn for autonomous generation too, just thats where we were looking at pairing it (blitzy is hidden proprietary).\"*\n\nBoth use cases reduce to one primitive:\n\n| Use case | When CAIRN runs | What it does |\n|---|---|---|\n| Drift detection | Post-hoc / pre-commit | Verifies existing code matches declared blueprint + contracts |\n| Autonomous generation | During / after generation | Verifies just-generated code matches declared blueprint + contracts |\n\nThe drift gate doesn't know whether code was hand-written or AI-generated. It just enforces invariants. For autonomous generation the agent loop is:\n\n1. Agent reads `cairn.blueprint` + relevant contracts + neighbourhood\n2. Agent generates code\n3. Agent runs `cairn lint --json` to verify\n4. On exit 2 (blocking finding), agent iterates with findings as feedback\n5. On exit 0 (clean), commit\n\nCAIRN doesn't generate code. CAIRN doesn't know about the generating agent. It enforces the invariant either way. **Value of being a tool, not an agent.** Blitzy-style autonomous engineering, Gas City formula dispatch, Claude Code in-IDE — all consume the same primitive.\n\nThis means the slate (#95-#105) serves both use cases. No additional scope needed.\n\n### 19.3 \"Missing piece\" is the product positioning\n\nUser: *\"i don't necessarily want an all in one tool, but i want a very lean ability to achieve these goals. And i guess cairn in my mind is a missing piece, as loads of other things have different parts of the puzzle, with different overlaps in cairn.\"*\n\nRestating the position explicitly:\n\n| Existing tool category | What it does | What it doesn't do |\n|---|---|---|\n| Coding agents (Claude Code, Codex, jcode) | Generate / edit code | Know what's *supposed* to be true |\n| Orchestrators / harnesses (Gas City, Hermes, custom) | Run agents at scale | Know what's *supposed* to be true |\n| Memory systems (automem, mag, beads) | Remember what happened | Know what's *supposed* to be true |\n| Knowledge graphs (graphify, etc.) | Derive structure from code | Enforce what *should* be true |\n\nAcross all of these: nobody declares architectural truth and gates against drift from it. Existing tools *describe* (graphify, beads, automem) or *act* (Claude Code, Gas City, jcode). None *constrain*.\n\n**CAIRN's positioning: the declarative, deterministic constraint layer the ecosystem doesn't have.** Lean by design — a small CLI saying *\"this is supposed to be true, this is what's actually there, here's the diff.\"* Every other tool can use it as the conscience.\n\nThis is *why* CAIRN should not try to be a swiss army knife. Each thing it adds dilutes the constraint-layer identity. The integration value comes *from* being small and focused.\n\nAction implication: when introducing CAIRN to others (docs, README, pitch to gascity-packs reviewers), lead with \"the missing constraint layer.\" Not \"the AI coding framework.\" Not \"the agent orchestration system.\" The constraint layer."
            }
          ]
        },
        "sources": {
          "schema_version": 1,
          "node": "cairn.root",
          "artefacts": [
            {
              "type": "sources",
              "path": "meta/sources/beads-repo.md",
              "title": "Beads repository",
              "frontmatter": {
                "date": "2026-05-13",
                "file": "https://github.com/gastownhall/beads",
                "id": "src.beads-repo",
                "type": "repo",
                "verification": "external"
              },
              "body": "\n# Beads repository\n\nExternal source referenced by the no-orchestrator decision. Inspected during the 2026-05-13 integration analysis; not pinned to a specific commit."
            },
            {
              "type": "sources",
              "path": "meta/sources/gas-city-repo.md",
              "title": "Gas City repository",
              "frontmatter": {
                "date": "2026-05-13",
                "file": "https://github.com/gastownhall/gascity",
                "id": "src.gas-city-repo",
                "type": "repo",
                "verification": "external"
              },
              "body": "\n# Gas City repository\n\nExternal source referenced by the no-orchestrator decision. Inspected during the 2026-05-13 integration analysis; not pinned to a specific commit."
            }
          ]
        },
        "todos": {
          "schema_version": 1,
          "node": "cairn.root",
          "artefacts": []
        }
      }
    },
    "cairn.sse": {
      "id": "cairn.sse",
      "endpoints": {
        "beads": {
          "schema_version": 1,
          "node": "cairn.sse",
          "beads": []
        },
        "contract": {
          "schema_version": 1,
          "node": "cairn.sse",
          "artefacts": []
        },
        "decisions": {
          "schema_version": 1,
          "node": "cairn.sse",
          "artefacts": [
            {
              "type": "decisions",
              "path": "meta/decisions/close-blueprint-drift.md",
              "title": "Close blueprint drift for orphaned modules",
              "frontmatter": {
                "date": "2026-06-03",
                "id": "dec.close-blueprint-drift",
                "status": "accepted"
              },
              "body": "\n# Close blueprint drift for orphaned modules\n\n## Context\n\nThree source modules existed in the codebase but were not declared in `cairn.blueprint`:\n\n- `src/sse.rs` (SSE event stream parser for Gas City integration spikes)\n- `src/state/mod.rs` (pluggable state persistence backend: filesystem + beads)\n- `src/watch.rs` (watch mode: periodic scan with finding-change events)\n\nThis drift produced `CAIRN_RECONCILE_ORPHANED_FILE` info findings on every scan and prevented `cairn lint` from exiting cleanly.\n\n## Decision\n\nDeclare all three as top-level modules under the `cairn` System node.\n\n## Rationale\n\nAll three modules are actively imported and used:\n\n- `sse` is exported from `lib.rs` and consumed by the Gas City adapter.\n- `state` is used by CLI commands for beads-backed persistence and by the scanner for snapshot state.\n- `watch` is used by the CLI `cairn watch` command and by the query API for finding-delta events.\n\nLeaving them orphaned weakens the dogfooding signal. The framework should model its own source tree completely."
            }
          ]
        },
        "rationale": {
          "schema_version": 1,
          "node": "cairn.sse",
          "artefacts": [
            {
              "type": "decisions",
              "path": "meta/decisions/close-blueprint-drift.md",
              "title": "Close blueprint drift for orphaned modules",
              "frontmatter": {
                "date": "2026-06-03",
                "id": "dec.close-blueprint-drift",
                "status": "accepted"
              },
              "body": "\n# Close blueprint drift for orphaned modules\n\n## Context\n\nThree source modules existed in the codebase but were not declared in `cairn.blueprint`:\n\n- `src/sse.rs` (SSE event stream parser for Gas City integration spikes)\n- `src/state/mod.rs` (pluggable state persistence backend: filesystem + beads)\n- `src/watch.rs` (watch mode: periodic scan with finding-change events)\n\nThis drift produced `CAIRN_RECONCILE_ORPHANED_FILE` info findings on every scan and prevented `cairn lint` from exiting cleanly.\n\n## Decision\n\nDeclare all three as top-level modules under the `cairn` System node.\n\n## Rationale\n\nAll three modules are actively imported and used:\n\n- `sse` is exported from `lib.rs` and consumed by the Gas City adapter.\n- `state` is used by CLI commands for beads-backed persistence and by the scanner for snapshot state.\n- `watch` is used by the CLI `cairn watch` command and by the query API for finding-delta events.\n\nLeaving them orphaned weakens the dogfooding signal. The framework should model its own source tree completely."
            }
          ]
        },
        "research": {
          "schema_version": 1,
          "node": "cairn.sse",
          "artefacts": []
        },
        "sources": {
          "schema_version": 1,
          "node": "cairn.sse",
          "artefacts": []
        },
        "todos": {
          "schema_version": 1,
          "node": "cairn.sse",
          "artefacts": []
        }
      }
    },
    "cairn.state": {
      "id": "cairn.state",
      "endpoints": {
        "beads": {
          "schema_version": 1,
          "node": "cairn.state",
          "beads": []
        },
        "contract": {
          "schema_version": 1,
          "node": "cairn.state",
          "artefacts": []
        },
        "decisions": {
          "schema_version": 1,
          "node": "cairn.state",
          "artefacts": [
            {
              "type": "decisions",
              "path": "meta/decisions/close-blueprint-drift.md",
              "title": "Close blueprint drift for orphaned modules",
              "frontmatter": {
                "date": "2026-06-03",
                "id": "dec.close-blueprint-drift",
                "status": "accepted"
              },
              "body": "\n# Close blueprint drift for orphaned modules\n\n## Context\n\nThree source modules existed in the codebase but were not declared in `cairn.blueprint`:\n\n- `src/sse.rs` (SSE event stream parser for Gas City integration spikes)\n- `src/state/mod.rs` (pluggable state persistence backend: filesystem + beads)\n- `src/watch.rs` (watch mode: periodic scan with finding-change events)\n\nThis drift produced `CAIRN_RECONCILE_ORPHANED_FILE` info findings on every scan and prevented `cairn lint` from exiting cleanly.\n\n## Decision\n\nDeclare all three as top-level modules under the `cairn` System node.\n\n## Rationale\n\nAll three modules are actively imported and used:\n\n- `sse` is exported from `lib.rs` and consumed by the Gas City adapter.\n- `state` is used by CLI commands for beads-backed persistence and by the scanner for snapshot state.\n- `watch` is used by the CLI `cairn watch` command and by the query API for finding-delta events.\n\nLeaving them orphaned weakens the dogfooding signal. The framework should model its own source tree completely."
            }
          ]
        },
        "rationale": {
          "schema_version": 1,
          "node": "cairn.state",
          "artefacts": [
            {
              "type": "decisions",
              "path": "meta/decisions/close-blueprint-drift.md",
              "title": "Close blueprint drift for orphaned modules",
              "frontmatter": {
                "date": "2026-06-03",
                "id": "dec.close-blueprint-drift",
                "status": "accepted"
              },
              "body": "\n# Close blueprint drift for orphaned modules\n\n## Context\n\nThree source modules existed in the codebase but were not declared in `cairn.blueprint`:\n\n- `src/sse.rs` (SSE event stream parser for Gas City integration spikes)\n- `src/state/mod.rs` (pluggable state persistence backend: filesystem + beads)\n- `src/watch.rs` (watch mode: periodic scan with finding-change events)\n\nThis drift produced `CAIRN_RECONCILE_ORPHANED_FILE` info findings on every scan and prevented `cairn lint` from exiting cleanly.\n\n## Decision\n\nDeclare all three as top-level modules under the `cairn` System node.\n\n## Rationale\n\nAll three modules are actively imported and used:\n\n- `sse` is exported from `lib.rs` and consumed by the Gas City adapter.\n- `state` is used by CLI commands for beads-backed persistence and by the scanner for snapshot state.\n- `watch` is used by the CLI `cairn watch` command and by the query API for finding-delta events.\n\nLeaving them orphaned weakens the dogfooding signal. The framework should model its own source tree completely."
            }
          ]
        },
        "research": {
          "schema_version": 1,
          "node": "cairn.state",
          "artefacts": []
        },
        "sources": {
          "schema_version": 1,
          "node": "cairn.state",
          "artefacts": []
        },
        "todos": {
          "schema_version": 1,
          "node": "cairn.state",
          "artefacts": []
        }
      }
    },
    "cairn.suggested-edges": {
      "id": "cairn.suggested-edges",
      "endpoints": {
        "beads": {
          "schema_version": 1,
          "node": "cairn.suggested-edges",
          "beads": []
        },
        "contract": {
          "schema_version": 1,
          "node": "cairn.suggested-edges",
          "artefacts": []
        },
        "decisions": {
          "schema_version": 1,
          "node": "cairn.suggested-edges",
          "artefacts": [
            {
              "type": "decisions",
              "path": "meta/decisions/build-and-extension.md",
              "title": "Build and extension modules",
              "frontmatter": {
                "date": "2026-06-16",
                "id": "dec.build-and-extension",
                "status": "accepted"
              },
              "body": "\n# Build and extension modules\n\n## Context\n\ncairn has several modules that are not part of the daily graph pipeline but are essential for adoption, provenance, and agent-assisted workflows.\n\n## Decision\n\nKeep these as first-class modules:\n\n- **Macros**: proc-macro crate for compile-time attributes (e.g., `#[cairn_planned]`).\n- **Brownfield**: orphan grouping, candidate heuristics, and onboard analysis for existing codebases.\n- **Provenance**: trace sidecar primitives and provenance-chain helpers.\n- **SuggestedEdges**: queue for AI-suggested graph edges with triage workflows.\n- **Summariser**: LLM-assisted contract summarisation backend.\n\n## Rationale\n\nThese are distinct enough to warrant separate modules. Brownfield and summariser are especially important for adoption: most users do not start greenfield.\n\n## Consequences\n\n- Brownfield heuristics affect `cairn init --from-code` and `cairn refine`.\n- Provenance types are consumed by the artefact registry and decision coverage gate.\n- SuggestedEdges and Summariser both touch LLM outputs and need careful prompt/version management."
            }
          ]
        },
        "rationale": {
          "schema_version": 1,
          "node": "cairn.suggested-edges",
          "artefacts": [
            {
              "type": "decisions",
              "path": "meta/decisions/build-and-extension.md",
              "title": "Build and extension modules",
              "frontmatter": {
                "date": "2026-06-16",
                "id": "dec.build-and-extension",
                "status": "accepted"
              },
              "body": "\n# Build and extension modules\n\n## Context\n\ncairn has several modules that are not part of the daily graph pipeline but are essential for adoption, provenance, and agent-assisted workflows.\n\n## Decision\n\nKeep these as first-class modules:\n\n- **Macros**: proc-macro crate for compile-time attributes (e.g., `#[cairn_planned]`).\n- **Brownfield**: orphan grouping, candidate heuristics, and onboard analysis for existing codebases.\n- **Provenance**: trace sidecar primitives and provenance-chain helpers.\n- **SuggestedEdges**: queue for AI-suggested graph edges with triage workflows.\n- **Summariser**: LLM-assisted contract summarisation backend.\n\n## Rationale\n\nThese are distinct enough to warrant separate modules. Brownfield and summariser are especially important for adoption: most users do not start greenfield.\n\n## Consequences\n\n- Brownfield heuristics affect `cairn init --from-code` and `cairn refine`.\n- Provenance types are consumed by the artefact registry and decision coverage gate.\n- SuggestedEdges and Summariser both touch LLM outputs and need careful prompt/version management."
            }
          ]
        },
        "research": {
          "schema_version": 1,
          "node": "cairn.suggested-edges",
          "artefacts": []
        },
        "sources": {
          "schema_version": 1,
          "node": "cairn.suggested-edges",
          "artefacts": []
        },
        "todos": {
          "schema_version": 1,
          "node": "cairn.suggested-edges",
          "artefacts": []
        }
      }
    },
    "cairn.summariser": {
      "id": "cairn.summariser",
      "endpoints": {
        "beads": {
          "schema_version": 1,
          "node": "cairn.summariser",
          "beads": []
        },
        "contract": {
          "schema_version": 1,
          "node": "cairn.summariser",
          "artefacts": []
        },
        "decisions": {
          "schema_version": 1,
          "node": "cairn.summariser",
          "artefacts": [
            {
              "type": "decisions",
              "path": "meta/decisions/build-and-extension.md",
              "title": "Build and extension modules",
              "frontmatter": {
                "date": "2026-06-16",
                "id": "dec.build-and-extension",
                "status": "accepted"
              },
              "body": "\n# Build and extension modules\n\n## Context\n\ncairn has several modules that are not part of the daily graph pipeline but are essential for adoption, provenance, and agent-assisted workflows.\n\n## Decision\n\nKeep these as first-class modules:\n\n- **Macros**: proc-macro crate for compile-time attributes (e.g., `#[cairn_planned]`).\n- **Brownfield**: orphan grouping, candidate heuristics, and onboard analysis for existing codebases.\n- **Provenance**: trace sidecar primitives and provenance-chain helpers.\n- **SuggestedEdges**: queue for AI-suggested graph edges with triage workflows.\n- **Summariser**: LLM-assisted contract summarisation backend.\n\n## Rationale\n\nThese are distinct enough to warrant separate modules. Brownfield and summariser are especially important for adoption: most users do not start greenfield.\n\n## Consequences\n\n- Brownfield heuristics affect `cairn init --from-code` and `cairn refine`.\n- Provenance types are consumed by the artefact registry and decision coverage gate.\n- SuggestedEdges and Summariser both touch LLM outputs and need careful prompt/version management."
            }
          ]
        },
        "rationale": {
          "schema_version": 1,
          "node": "cairn.summariser",
          "artefacts": [
            {
              "type": "decisions",
              "path": "meta/decisions/build-and-extension.md",
              "title": "Build and extension modules",
              "frontmatter": {
                "date": "2026-06-16",
                "id": "dec.build-and-extension",
                "status": "accepted"
              },
              "body": "\n# Build and extension modules\n\n## Context\n\ncairn has several modules that are not part of the daily graph pipeline but are essential for adoption, provenance, and agent-assisted workflows.\n\n## Decision\n\nKeep these as first-class modules:\n\n- **Macros**: proc-macro crate for compile-time attributes (e.g., `#[cairn_planned]`).\n- **Brownfield**: orphan grouping, candidate heuristics, and onboard analysis for existing codebases.\n- **Provenance**: trace sidecar primitives and provenance-chain helpers.\n- **SuggestedEdges**: queue for AI-suggested graph edges with triage workflows.\n- **Summariser**: LLM-assisted contract summarisation backend.\n\n## Rationale\n\nThese are distinct enough to warrant separate modules. Brownfield and summariser are especially important for adoption: most users do not start greenfield.\n\n## Consequences\n\n- Brownfield heuristics affect `cairn init --from-code` and `cairn refine`.\n- Provenance types are consumed by the artefact registry and decision coverage gate.\n- SuggestedEdges and Summariser both touch LLM outputs and need careful prompt/version management."
            }
          ]
        },
        "research": {
          "schema_version": 1,
          "node": "cairn.summariser",
          "artefacts": []
        },
        "sources": {
          "schema_version": 1,
          "node": "cairn.summariser",
          "artefacts": []
        },
        "todos": {
          "schema_version": 1,
          "node": "cairn.summariser",
          "artefacts": []
        }
      }
    },
    "cairn.tests": {
      "id": "cairn.tests",
      "endpoints": {
        "beads": {
          "schema_version": 1,
          "node": "cairn.tests",
          "beads": []
        },
        "contract": {
          "schema_version": 1,
          "node": "cairn.tests",
          "artefacts": []
        },
        "decisions": {
          "schema_version": 1,
          "node": "cairn.tests",
          "artefacts": [
            {
              "type": "decisions",
              "path": "meta/decisions/test-infrastructure.md",
              "title": "Test infrastructure",
              "frontmatter": {
                "date": "2026-06-16",
                "id": "dec.test-infrastructure",
                "status": "accepted"
              },
              "body": "\n# Test infrastructure\n\n## Context\n\nIntegration tests, phase tests, and smoke tests live outside `src/` so they exercise the public crate surface the way external callers would.\n\n## Decision\n\nMaintain a dedicated `cairn.tests` module that points at the `tests/` directory. This module is tagged `@test` and is not part of the production dependency graph.\n\n## Rationale\n\nSeparating tests from `src/` keeps the crate's internal modules focused on production code while still letting cairn model the test suite as a node. The `@test` tag lets gates and queries filter it out of build-order or dependency analysis when appropriate.\n\n## Consequences\n\n- New phase tests go into `tests/` and are covered by this decision.\n- The module should not claim source files under `src/`.\n- CI runs `cargo test --all-targets --all-features`."
            }
          ]
        },
        "rationale": {
          "schema_version": 1,
          "node": "cairn.tests",
          "artefacts": [
            {
              "type": "decisions",
              "path": "meta/decisions/test-infrastructure.md",
              "title": "Test infrastructure",
              "frontmatter": {
                "date": "2026-06-16",
                "id": "dec.test-infrastructure",
                "status": "accepted"
              },
              "body": "\n# Test infrastructure\n\n## Context\n\nIntegration tests, phase tests, and smoke tests live outside `src/` so they exercise the public crate surface the way external callers would.\n\n## Decision\n\nMaintain a dedicated `cairn.tests` module that points at the `tests/` directory. This module is tagged `@test` and is not part of the production dependency graph.\n\n## Rationale\n\nSeparating tests from `src/` keeps the crate's internal modules focused on production code while still letting cairn model the test suite as a node. The `@test` tag lets gates and queries filter it out of build-order or dependency analysis when appropriate.\n\n## Consequences\n\n- New phase tests go into `tests/` and are covered by this decision.\n- The module should not claim source files under `src/`.\n- CI runs `cargo test --all-targets --all-features`."
            }
          ]
        },
        "research": {
          "schema_version": 1,
          "node": "cairn.tests",
          "artefacts": []
        },
        "sources": {
          "schema_version": 1,
          "node": "cairn.tests",
          "artefacts": []
        },
        "todos": {
          "schema_version": 1,
          "node": "cairn.tests",
          "artefacts": []
        }
      }
    },
    "cairn.ui": {
      "id": "cairn.ui",
      "endpoints": {
        "beads": {
          "schema_version": 1,
          "node": "cairn.ui",
          "beads": []
        },
        "contract": {
          "schema_version": 1,
          "node": "cairn.ui",
          "artefacts": []
        },
        "decisions": {
          "schema_version": 1,
          "node": "cairn.ui",
          "artefacts": [
            {
              "type": "decisions",
              "path": "meta/decisions/landing-design-token-conformance.md",
              "title": "Landing page sources colour from the design system, not a fork",
              "frontmatter": {
                "date": "2026-06-23",
                "id": "dec.landing-design-token-conformance",
                "status": "accepted"
              },
              "body": "\n# Landing page sources colour from the design system, not a fork\n\n## Context\n\nThe marketing landing page (`docs/landing/index.html`) is bound to the same\ndesign-system tokens (`docs/design-system/tokens.css`) as the webui, and\n`docs/design-system/README.md` forbids forking the palette. The page instead\ndefined its own inline copy of every colour token across two theme blocks\n(`:root` dark, `[data-theme=\"light\"]`). That fork had drifted: the dark theme\nstill matched canonical byte-for-byte, but the light theme had diverged on 24 of\nthe 26 colour tokens the page uses (warmer paper stones, lighter amber and\nverdigris accents, lower-alpha washes). CLAUDE.md also asks marketing surfaces to\npull the design system in via `<link>`. The page already referenced every colour\nthrough `var(--token)`; only the definitions were forked.\n\n`dec.webui-design-token-gate` anticipated this: its consequences note that a\ntoken-conformance need for `docs/landing/` extends `scripts/check-design-tokens.sh`\nrather than adding a new mechanism.\n\n## Decision\n\nLink the canonical stylesheet from the landing `<head>`\n(`<link rel=\"stylesheet\" href=\"../design-system/tokens.css\">`, which resolves\nunder GitHub Pages because `pages.yml` deploys all of `docs/`) and delete the\ninlined colour token definitions from both theme blocks. Keep the page-specific\nnon-colour tokens inline (type scale, spacing, radii, motion, font stacks): the\nlanding intentionally runs a larger marketing type scale than the webui, so those\nare deliberate divergences, not palette drift.\n\nExtend `scripts/check-design-tokens.sh` to check the landing as a second default\ntarget, stripping HTML comments as well as CSS comments so a hex mentioned in\nprose does not trip the gate. Trigger the pre-commit hook on the landing path\ntoo. Pin the result with a real-file regression test in\n`tests/check_design_tokens.rs`.\n\n## Rationale\n\nDeleting the fork is load-bearing, not cosmetic: the inline `<style>` block\nfollows the linked stylesheet in source order, so while the page redefined the\ncolour tokens inline they overrode canonical and the fork persisted. Removing\nthem lets canonical win, while the kept inline non-colour `:root` tokens still\noverride canonical for type and motion (as intended).\n\nThe dark (default) view is byte-identical: all 26 colour tokens the page uses had\ndark values equal to canonical, so computed styles do not change. The light view\nreconciles to canonical, a deliberate, maintainer-approved change to the live\nmarketing site (its accents and paper tones shift toward the design system). The\nloop escalated this through AskUserQuestion before shipping, because `pages.yml`\nauto-deploys `docs/` on merge: it is a real outward change to a public page, the\nmaintainer's call, and the maintainer gave the GO.\n\n## Consequences\n\n- The landing can no longer drift from the design system on colour: a forked or\n  hardcoded hex now blocks commit, push, and CI, reported with file and line.\n- The light marketing theme now matches the canonical paper palette; future\n  palette edits propagate to the landing automatically through the link.\n- Page-specific type, spacing, and motion tokens remain a sanctioned divergence\n  and stay out of the gate's scope (it checks colour and rem only).\n- A pre-existing defect is now more visible but out of this unit's scope: the\n  hero image (`docs/images/webui-v2-empty.png`, referenced at\n  `docs/landing/index.html:877`) is missing, and the og/twitter images still\n  point at the old `dev` branch. Tracked as a separate follow-up."
            },
            {
              "type": "decisions",
              "path": "meta/decisions/user-surfaces.md",
              "title": "User surfaces: web UI and MCP wrapper",
              "frontmatter": {
                "date": "2026-06-16",
                "id": "dec.user-surfaces",
                "status": "accepted"
              },
              "body": "\n# User surfaces: web UI and MCP wrapper\n\n## Context\n\nNot all consumers want a CLI. A read-only web graph explorer and an MCP tool wrapper make cairn accessible to agents and browser-based users.\n\n## Decision\n\nProvide two surfaces:\n\n- **UI (`cairn.ui`)**: an embedded HTTP server serving a read-only graph explorer.\n- **MCP (`cairn.mcp`)**: a Model Context Protocol server that exposes cairn queries as tools.\n\n## Rationale\n\nThe web UI is useful for human review of the architecture map. The MCP wrapper lets agent harnesses call cairn without shelling out, reducing latency and surface area.\n\n## Consequences\n\n- Both surfaces must consume the same query API as the CLI to avoid semantic drift.\n- UI assets live under `src/ui_assets/` and are served statically.\n- MCP schema changes require updating `src/mcp.rs` and any dependent clients."
            },
            {
              "type": "decisions",
              "path": "meta/decisions/webui-a11y-static-audit-gate.md",
              "title": "Webui accessibility static-audit gate",
              "frontmatter": {
                "date": "2026-06-23",
                "id": "dec.webui-a11y-static-audit-gate",
                "status": "accepted"
              },
              "body": "\n# Webui accessibility static-audit gate\n\n## Context\n\nThe hand-authored web surfaces (the webui shell `src/ui_assets/index.html`, the\npreact app `src/ui_assets/app.js`, and the landing page\n`docs/landing/index.html`) already practise good accessibility hygiene: the app\nlabels its icon-only buttons with `aria-label`, marks the inspector\n`aria-live`, and groups the zoom controls with `role=\"group\"`. Both document\nshells declare `lang`, a `<title>`, and a zoom-friendly viewport, and the one\nlanding `<img>` carries an `alt`. Nothing enforced any of this. biome (added by\ncairn-xiw) lints `app.js` with its recommended rule set, which has no\naccessibility rule and cannot see markup inside `htm` template literals at all,\nand it does not lint the `.html` surfaces. A regression (an uncaptioned image,\na positive tabindex, a dropped `lang`, a zoom-disabling viewport) would pass\nevery gate.\n\nThis is the deterministic half of cairn-y7p. The bead's larger scope, a\nbrowser -> AI-vision-critique -> patch loop, stays blocked on two maintainer\nprerequisites (a Node/Playwright toolchain in this package.json-less Rust repo,\nand a paid AI vision provider that conflicts with the repo's deterministic-gates\nconvention). A static a11y audit needs neither and is the natural next unit.\n\n## Decision\n\nAdd a standalone repository gate, `scripts/check-a11y.sh`, that fails when a web\nsurface violates a statically decidable, WCAG-aligned accessibility invariant.\nElement-level checks run on every surface; document-level checks run only on full\nHTML documents (those with an `<html>` root), so JS/htm fragments are exempt from\nthem:\n\n- WCAG 1.1.1: every `<img>` carries an `alt` attribute (tag-aware, so a\n  multi-line `<img>` is judged as one tag).\n- WCAG 2.4.3: no positive `tabindex` (1+) overrides the natural focus order\n  (`0` and `-1` are allowed).\n- WCAG 3.1.1: a document's `<html>` declares a `lang`.\n- WCAG 2.4.2: a document has a `<title>`.\n- WCAG 1.4.4: a document's viewport meta does not disable pinch zoom\n  (`user-scalable=no` or `maximum-scale=1`).\n\nHTML and block comments are stripped first so markup mentioned in prose does not\ntrip the gate. Wire it into the same three places that already gate the webui:\nthe pre-commit config, the CI `webui` job, and the Makefile `check` target.\nCover its behaviour with `tests/check_a11y.rs`.\n\n## Rationale\n\nThis mirrors `dec.webui-design-token-gate` exactly: a project-health gate that\nlives alongside `scripts/check-design-tokens.sh` and `scripts/check-file-sizes.sh`,\nnot a cairn feature. It deliberately stays outside cairn's kernel: per\n`dec.toolchain-lint-strictness`, cairn inspects lint *configuration existence*\nand never invokes a linter. A repo script is the right home for a specific\naccessibility rule, and it is config-free and dependency-free (POSIX sh + awk +\ngrep), keeping the repo's low-dependency, single-binary ethos intact.\n\nThe checks are limited to invariants that are genuinely decidable from source\ntext without rendering. Colour contrast, dynamic ARIA state, and focus-trap\nbehaviour need a real DOM and a headless browser, which is precisely the blocked\nhalf of cairn-y7p; this gate does not attempt them.\n\n## Consequences\n\n- An uncaptioned image, a positive tabindex, a missing `lang` or `<title>`, or a\n  zoom-disabling viewport on a gated surface now blocks commit, push, and CI,\n  with the offending file and rule reported.\n- A future surface (a new `.html` page, a new asset) is added to the default\n  target list in the script, or scanned in isolation via `CAIRN_A11Y_TARGET`.\n- The remaining cairn-y7p scope is now exactly the AI-vision loop, still blocked\n  on the two maintainer prerequisites recorded on the bead."
            },
            {
              "type": "decisions",
              "path": "meta/decisions/webui-ai-vision-loop-declined.md",
              "title": "Webui AI-vision iteration loop declined",
              "frontmatter": {
                "date": "2026-06-23",
                "id": "dec.webui-ai-vision-loop-declined",
                "status": "accepted"
              },
              "body": "\n# Webui AI-vision iteration loop declined\n\n## Context\n\ncairn-y7p proposed an iterative testing loop for the cairn webui: drive a\nheadless browser to screenshot the rendered UI, feed the image to an AI vision\nmodel, apply its fix instructions to `src/ui_assets/`, reload, and repeat until\nthe UI meets acceptance criteria.\n\nThe bead's deterministic, decidable-from-source half shipped in two prior units\nand is now enforced on every commit, push, and CI run:\n\n- `dec.webui-design-token-gate` (PR #145): `scripts/check-design-tokens.sh`\n  blocks hardcoded hex/rem in `src/ui_assets/style.css`.\n- `dec.webui-a11y-static-audit-gate` (PR #148): `scripts/check-a11y.sh` blocks\n  WCAG img-alt, positive-tabindex, lang, title, and zoom-disable regressions on\n  the hand-authored web surfaces.\n\nWhat remained was only the browser -> AI-vision-critique -> patch -> reload loop.\nThat remainder cannot be built without two prerequisites that are the\nmaintainer's to grant, not the loop's to guess:\n\n1. Sanction to add a Node + Playwright/puppeteer toolchain to this deliberately\n   `package.json`-less, single-binary Rust repo (adds `node_modules` and CI\n   cost).\n2. An AI vision provider and API config. None exists, it is likely paid, and a\n   non-deterministic model inside a gate conflicts directly with the repo's\n   deterministic-gates convention (`dec.toolchain-lint-strictness`, and the\n   local-hooks-over-paid-CI posture).\n\n## Decision\n\nDecline the AI-vision loop and close cairn-y7p as resolved. The two deterministic\ngates are the durable, in-convention outcome of the bead; the vision loop is out\nof scope for a deterministic, low-dependency Rust repo and will not be built\nabsent an explicit maintainer reversal of both prerequisites above.\n\n## Rationale\n\nA non-deterministic, paid, network-dependent vision model gating UI changes is\nthe opposite of how every other cairn gate works (config-existence checks, static\nsource analysis, no linter invocation, no network). Adopting it would erode the\nsingle-binary, deterministic ethos the project has repeatedly chosen. The\ndeterministic gates already catch the regressions that matter and are decidable\nfrom source (design-token drift, the a11y invariants), so the bead's stated\nacceptance criteria are substantially met without the vision component.\n\nThis record exists so the dev loop does not re-derive the same blocked seed each\nsession: the question was raised, escalated to the maintainer, and answered.\n\n## Consequences\n\n- cairn-y7p is closed. The webui's enforced UI-quality surface is exactly the two\n  deterministic gates; there is no browser or AI tooling in the repo.\n- Reopening the vision loop requires a superseding decision that records the\n  maintainer's sanction for both a Node/Playwright toolchain and a vision\n  provider.\n- A future deterministic visual-regression approach (for example a pixel-diff\n  against a checked-in baseline rendered by an already-present tool) is not\n  precluded; it would be a new unit of work, not this one."
            },
            {
              "type": "decisions",
              "path": "meta/decisions/webui-design-token-gate.md",
              "title": "Webui design-token conformance gate",
              "frontmatter": {
                "date": "2026-06-23",
                "id": "dec.webui-design-token-gate",
                "status": "accepted"
              },
              "body": "\n# Webui design-token conformance gate\n\n## Context\n\nThe webui stylesheet (`src/ui_assets/style.css`) rides on the design-system\ntokens (`docs/design-system/tokens.css`) and must source every colour and\nrem-based size from a `var(--token)`. CLAUDE.md, AGENTS.md, and the stylesheet\nheader all state \"do not hardcode hex or rem values\", but nothing enforced it:\nbiome (added by cairn-xiw) formats and lints the asset with its recommended\nrule set, which has no rule for \"use a token instead of a literal colour or\nrem\". The invariant was documented and currently honoured, yet a regression\nwould pass every gate.\n\n## Decision\n\nAdd a standalone repository gate, `scripts/check-design-tokens.sh`, that fails\nwhen `style.css` contains a hardcoded hex colour or rem value (CSS comments are\nstripped first). Wire it into the same three places that already gate the webui\nasset: the pre-commit config, the CI `webui` job, and the Makefile `check`\ntarget. Cover its behaviour with `tests/check_design_tokens.rs`.\n\n## Rationale\n\nThis is a project-health gate that lives alongside `scripts/check-file-sizes.sh`,\nnot a cairn feature. It deliberately stays outside cairn's kernel: per\n`dec.toolchain-lint-strictness`, cairn inspects lint *configuration existence*\nand never invokes a linter or formatter. The `CAIRN_LINT_NOT_STRICT` finding\nchecks that a strict config exists; it does not (and should not) encode a\nproject's specific token-conformance rule. A repo script is the right home for\nthat rule, mirroring how the Rust file-size limit is enforced.\n\nScope is limited to `style.css`: it is the hand-written stylesheet bound to the\ntokens. `app.js` is behaviour, not styling, and the marketing surfaces under\n`docs/landing/` are out of this iteration's scope.\n\n## Consequences\n\n- A hardcoded hex colour or rem value in `style.css` now blocks commit, push,\n  and CI, with the offending file and line reported.\n- A future token-conformance need for `app.js` or `docs/landing/` extends this\n  script (parameterised via `CAIRN_DESIGN_TOKENS_TARGET`) rather than adding a\n  new mechanism.\n- The gate is config-free and dependency-free (POSIX sh + awk + grep), keeping\n  the repo's low-dependency, single-binary ethos intact."
            },
            {
              "type": "decisions",
              "path": "meta/decisions/webui-json-schema-version.md",
              "title": "Webui /api/* JSON responses carry a uniform schema_version",
              "frontmatter": {
                "date": "2026-06-23",
                "id": "dec.webui-json-schema-version",
                "status": "accepted"
              },
              "body": "\n# Webui /api/* JSON responses carry a uniform schema_version\n\n## Context\n\nThe webui HTTP surface (`src/ui`) emitted inconsistent JSON. Only `/api/meta`\nand `/api/status` carried a top-level `schema_version`; `/api/graph`,\n`/api/lint`, `/api/blueprint`, `/api/node/*` (and its `contract`, `decisions`,\n`todos`, `research`, `sources`, `beads`, `rationale` suffixes), `/api/depends/*`,\nand `/api/dependents/*` carried no version. A consumer could branch on the\nhandshake endpoints but had no version to branch on for the data endpoints.\n\n`dec.query-json-schema-version` standardized the sibling `query_api` surface and\nexplicitly scoped the webui out as \"a distinct unit of work, not taken here\",\nflagging it as a maintainer decision because handshake-only versioning could be\ndeliberate. The maintainer chose to standardize: stamp `schema_version` on every\n`/api/*` response. This decision completes that deferred unit; it does not\ncontradict `dec.query-json-schema-version`.\n\n## Decision\n\nEvery webui `/api/*` JSON response carries a top-level `schema_version` field,\ncurrently `1`, stamped at a single choke point: the `server::json` `Response`\nconstructor. `json` is the sole builder of `application/json` responses for the\nAPI surface, so a `versioned()` helper splices `schema_version` as the first key\nof the (always-object) body inside `json`. The redundant inline stamps in\n`meta_json` and `status_json` were removed so the choke point is the single\nsource of truth.\n\nThe webui keeps its own `ui::SCHEMA_VERSION` constant, independent of\n`query_api::SCHEMA_VERSION`: these are separate wire surfaces with separate\nversioning lifecycles, and coupling them would force a lockstep bump where none\nis warranted.\n\n## Rationale\n\nStamping at the `json` constructor, with one constant, beats per-handler stamps:\nit cannot drift, every endpoint is covered automatically (including future ones\nand error envelopes), and there is exactly one place to bump when the contract\nchanges. This mirrors the `query_api` choke-point philosophy on a separate\nsurface.\n\nOnly the top-level envelope is versioned. Nested objects (the node records\ninside `/api/graph`) are built by `node_json` as plain strings that never pass\nthrough `json`, so they stay unversioned: the version describes the response\ncontract, not every embedded record.\n\nError envelopes (404/500 `finding` bodies, the blueprint read-error body) flow\nthrough `json` too and so are versioned uniformly, which is desirable: a consumer\nparsing an error still gets a version to branch on. Plain-text fallbacks\n(`text(404, ...)`) are not JSON and carry no version.\n\n## Consequences\n\n- Every webui `/api/*` JSON response now contains `\"schema_version\": 1`\n  (serialised first, sorted alphabetically by the snapshot harness). Bumping the\n  wire contract means bumping `ui::SCHEMA_VERSION`.\n- `server::json` is now coupled to the API version contract. This is acceptable\n  because `json` is `pub(super)` and is used exclusively to build `/api/*`\n  responses; there is no non-API JSON consumer of it.\n- The `wire_format_snapshots` golden fixtures were regenerated to include the\n  stamp, and the test now asserts every endpoint carries a numeric\n  `schema_version`, so a future endpoint added without the stamp fails the gate.\n- `query_api::SCHEMA_VERSION` and `ui::SCHEMA_VERSION` remain independent\n  constants for independent surfaces; they are not required to move together."
            }
          ]
        },
        "rationale": {
          "schema_version": 1,
          "node": "cairn.ui",
          "artefacts": [
            {
              "type": "decisions",
              "path": "meta/decisions/landing-design-token-conformance.md",
              "title": "Landing page sources colour from the design system, not a fork",
              "frontmatter": {
                "date": "2026-06-23",
                "id": "dec.landing-design-token-conformance",
                "status": "accepted"
              },
              "body": "\n# Landing page sources colour from the design system, not a fork\n\n## Context\n\nThe marketing landing page (`docs/landing/index.html`) is bound to the same\ndesign-system tokens (`docs/design-system/tokens.css`) as the webui, and\n`docs/design-system/README.md` forbids forking the palette. The page instead\ndefined its own inline copy of every colour token across two theme blocks\n(`:root` dark, `[data-theme=\"light\"]`). That fork had drifted: the dark theme\nstill matched canonical byte-for-byte, but the light theme had diverged on 24 of\nthe 26 colour tokens the page uses (warmer paper stones, lighter amber and\nverdigris accents, lower-alpha washes). CLAUDE.md also asks marketing surfaces to\npull the design system in via `<link>`. The page already referenced every colour\nthrough `var(--token)`; only the definitions were forked.\n\n`dec.webui-design-token-gate` anticipated this: its consequences note that a\ntoken-conformance need for `docs/landing/` extends `scripts/check-design-tokens.sh`\nrather than adding a new mechanism.\n\n## Decision\n\nLink the canonical stylesheet from the landing `<head>`\n(`<link rel=\"stylesheet\" href=\"../design-system/tokens.css\">`, which resolves\nunder GitHub Pages because `pages.yml` deploys all of `docs/`) and delete the\ninlined colour token definitions from both theme blocks. Keep the page-specific\nnon-colour tokens inline (type scale, spacing, radii, motion, font stacks): the\nlanding intentionally runs a larger marketing type scale than the webui, so those\nare deliberate divergences, not palette drift.\n\nExtend `scripts/check-design-tokens.sh` to check the landing as a second default\ntarget, stripping HTML comments as well as CSS comments so a hex mentioned in\nprose does not trip the gate. Trigger the pre-commit hook on the landing path\ntoo. Pin the result with a real-file regression test in\n`tests/check_design_tokens.rs`.\n\n## Rationale\n\nDeleting the fork is load-bearing, not cosmetic: the inline `<style>` block\nfollows the linked stylesheet in source order, so while the page redefined the\ncolour tokens inline they overrode canonical and the fork persisted. Removing\nthem lets canonical win, while the kept inline non-colour `:root` tokens still\noverride canonical for type and motion (as intended).\n\nThe dark (default) view is byte-identical: all 26 colour tokens the page uses had\ndark values equal to canonical, so computed styles do not change. The light view\nreconciles to canonical, a deliberate, maintainer-approved change to the live\nmarketing site (its accents and paper tones shift toward the design system). The\nloop escalated this through AskUserQuestion before shipping, because `pages.yml`\nauto-deploys `docs/` on merge: it is a real outward change to a public page, the\nmaintainer's call, and the maintainer gave the GO.\n\n## Consequences\n\n- The landing can no longer drift from the design system on colour: a forked or\n  hardcoded hex now blocks commit, push, and CI, reported with file and line.\n- The light marketing theme now matches the canonical paper palette; future\n  palette edits propagate to the landing automatically through the link.\n- Page-specific type, spacing, and motion tokens remain a sanctioned divergence\n  and stay out of the gate's scope (it checks colour and rem only).\n- A pre-existing defect is now more visible but out of this unit's scope: the\n  hero image (`docs/images/webui-v2-empty.png`, referenced at\n  `docs/landing/index.html:877`) is missing, and the og/twitter images still\n  point at the old `dev` branch. Tracked as a separate follow-up."
            },
            {
              "type": "decisions",
              "path": "meta/decisions/user-surfaces.md",
              "title": "User surfaces: web UI and MCP wrapper",
              "frontmatter": {
                "date": "2026-06-16",
                "id": "dec.user-surfaces",
                "status": "accepted"
              },
              "body": "\n# User surfaces: web UI and MCP wrapper\n\n## Context\n\nNot all consumers want a CLI. A read-only web graph explorer and an MCP tool wrapper make cairn accessible to agents and browser-based users.\n\n## Decision\n\nProvide two surfaces:\n\n- **UI (`cairn.ui`)**: an embedded HTTP server serving a read-only graph explorer.\n- **MCP (`cairn.mcp`)**: a Model Context Protocol server that exposes cairn queries as tools.\n\n## Rationale\n\nThe web UI is useful for human review of the architecture map. The MCP wrapper lets agent harnesses call cairn without shelling out, reducing latency and surface area.\n\n## Consequences\n\n- Both surfaces must consume the same query API as the CLI to avoid semantic drift.\n- UI assets live under `src/ui_assets/` and are served statically.\n- MCP schema changes require updating `src/mcp.rs` and any dependent clients."
            },
            {
              "type": "decisions",
              "path": "meta/decisions/webui-a11y-static-audit-gate.md",
              "title": "Webui accessibility static-audit gate",
              "frontmatter": {
                "date": "2026-06-23",
                "id": "dec.webui-a11y-static-audit-gate",
                "status": "accepted"
              },
              "body": "\n# Webui accessibility static-audit gate\n\n## Context\n\nThe hand-authored web surfaces (the webui shell `src/ui_assets/index.html`, the\npreact app `src/ui_assets/app.js`, and the landing page\n`docs/landing/index.html`) already practise good accessibility hygiene: the app\nlabels its icon-only buttons with `aria-label`, marks the inspector\n`aria-live`, and groups the zoom controls with `role=\"group\"`. Both document\nshells declare `lang`, a `<title>`, and a zoom-friendly viewport, and the one\nlanding `<img>` carries an `alt`. Nothing enforced any of this. biome (added by\ncairn-xiw) lints `app.js` with its recommended rule set, which has no\naccessibility rule and cannot see markup inside `htm` template literals at all,\nand it does not lint the `.html` surfaces. A regression (an uncaptioned image,\na positive tabindex, a dropped `lang`, a zoom-disabling viewport) would pass\nevery gate.\n\nThis is the deterministic half of cairn-y7p. The bead's larger scope, a\nbrowser -> AI-vision-critique -> patch loop, stays blocked on two maintainer\nprerequisites (a Node/Playwright toolchain in this package.json-less Rust repo,\nand a paid AI vision provider that conflicts with the repo's deterministic-gates\nconvention). A static a11y audit needs neither and is the natural next unit.\n\n## Decision\n\nAdd a standalone repository gate, `scripts/check-a11y.sh`, that fails when a web\nsurface violates a statically decidable, WCAG-aligned accessibility invariant.\nElement-level checks run on every surface; document-level checks run only on full\nHTML documents (those with an `<html>` root), so JS/htm fragments are exempt from\nthem:\n\n- WCAG 1.1.1: every `<img>` carries an `alt` attribute (tag-aware, so a\n  multi-line `<img>` is judged as one tag).\n- WCAG 2.4.3: no positive `tabindex` (1+) overrides the natural focus order\n  (`0` and `-1` are allowed).\n- WCAG 3.1.1: a document's `<html>` declares a `lang`.\n- WCAG 2.4.2: a document has a `<title>`.\n- WCAG 1.4.4: a document's viewport meta does not disable pinch zoom\n  (`user-scalable=no` or `maximum-scale=1`).\n\nHTML and block comments are stripped first so markup mentioned in prose does not\ntrip the gate. Wire it into the same three places that already gate the webui:\nthe pre-commit config, the CI `webui` job, and the Makefile `check` target.\nCover its behaviour with `tests/check_a11y.rs`.\n\n## Rationale\n\nThis mirrors `dec.webui-design-token-gate` exactly: a project-health gate that\nlives alongside `scripts/check-design-tokens.sh` and `scripts/check-file-sizes.sh`,\nnot a cairn feature. It deliberately stays outside cairn's kernel: per\n`dec.toolchain-lint-strictness`, cairn inspects lint *configuration existence*\nand never invokes a linter. A repo script is the right home for a specific\naccessibility rule, and it is config-free and dependency-free (POSIX sh + awk +\ngrep), keeping the repo's low-dependency, single-binary ethos intact.\n\nThe checks are limited to invariants that are genuinely decidable from source\ntext without rendering. Colour contrast, dynamic ARIA state, and focus-trap\nbehaviour need a real DOM and a headless browser, which is precisely the blocked\nhalf of cairn-y7p; this gate does not attempt them.\n\n## Consequences\n\n- An uncaptioned image, a positive tabindex, a missing `lang` or `<title>`, or a\n  zoom-disabling viewport on a gated surface now blocks commit, push, and CI,\n  with the offending file and rule reported.\n- A future surface (a new `.html` page, a new asset) is added to the default\n  target list in the script, or scanned in isolation via `CAIRN_A11Y_TARGET`.\n- The remaining cairn-y7p scope is now exactly the AI-vision loop, still blocked\n  on the two maintainer prerequisites recorded on the bead."
            },
            {
              "type": "decisions",
              "path": "meta/decisions/webui-ai-vision-loop-declined.md",
              "title": "Webui AI-vision iteration loop declined",
              "frontmatter": {
                "date": "2026-06-23",
                "id": "dec.webui-ai-vision-loop-declined",
                "status": "accepted"
              },
              "body": "\n# Webui AI-vision iteration loop declined\n\n## Context\n\ncairn-y7p proposed an iterative testing loop for the cairn webui: drive a\nheadless browser to screenshot the rendered UI, feed the image to an AI vision\nmodel, apply its fix instructions to `src/ui_assets/`, reload, and repeat until\nthe UI meets acceptance criteria.\n\nThe bead's deterministic, decidable-from-source half shipped in two prior units\nand is now enforced on every commit, push, and CI run:\n\n- `dec.webui-design-token-gate` (PR #145): `scripts/check-design-tokens.sh`\n  blocks hardcoded hex/rem in `src/ui_assets/style.css`.\n- `dec.webui-a11y-static-audit-gate` (PR #148): `scripts/check-a11y.sh` blocks\n  WCAG img-alt, positive-tabindex, lang, title, and zoom-disable regressions on\n  the hand-authored web surfaces.\n\nWhat remained was only the browser -> AI-vision-critique -> patch -> reload loop.\nThat remainder cannot be built without two prerequisites that are the\nmaintainer's to grant, not the loop's to guess:\n\n1. Sanction to add a Node + Playwright/puppeteer toolchain to this deliberately\n   `package.json`-less, single-binary Rust repo (adds `node_modules` and CI\n   cost).\n2. An AI vision provider and API config. None exists, it is likely paid, and a\n   non-deterministic model inside a gate conflicts directly with the repo's\n   deterministic-gates convention (`dec.toolchain-lint-strictness`, and the\n   local-hooks-over-paid-CI posture).\n\n## Decision\n\nDecline the AI-vision loop and close cairn-y7p as resolved. The two deterministic\ngates are the durable, in-convention outcome of the bead; the vision loop is out\nof scope for a deterministic, low-dependency Rust repo and will not be built\nabsent an explicit maintainer reversal of both prerequisites above.\n\n## Rationale\n\nA non-deterministic, paid, network-dependent vision model gating UI changes is\nthe opposite of how every other cairn gate works (config-existence checks, static\nsource analysis, no linter invocation, no network). Adopting it would erode the\nsingle-binary, deterministic ethos the project has repeatedly chosen. The\ndeterministic gates already catch the regressions that matter and are decidable\nfrom source (design-token drift, the a11y invariants), so the bead's stated\nacceptance criteria are substantially met without the vision component.\n\nThis record exists so the dev loop does not re-derive the same blocked seed each\nsession: the question was raised, escalated to the maintainer, and answered.\n\n## Consequences\n\n- cairn-y7p is closed. The webui's enforced UI-quality surface is exactly the two\n  deterministic gates; there is no browser or AI tooling in the repo.\n- Reopening the vision loop requires a superseding decision that records the\n  maintainer's sanction for both a Node/Playwright toolchain and a vision\n  provider.\n- A future deterministic visual-regression approach (for example a pixel-diff\n  against a checked-in baseline rendered by an already-present tool) is not\n  precluded; it would be a new unit of work, not this one."
            },
            {
              "type": "decisions",
              "path": "meta/decisions/webui-design-token-gate.md",
              "title": "Webui design-token conformance gate",
              "frontmatter": {
                "date": "2026-06-23",
                "id": "dec.webui-design-token-gate",
                "status": "accepted"
              },
              "body": "\n# Webui design-token conformance gate\n\n## Context\n\nThe webui stylesheet (`src/ui_assets/style.css`) rides on the design-system\ntokens (`docs/design-system/tokens.css`) and must source every colour and\nrem-based size from a `var(--token)`. CLAUDE.md, AGENTS.md, and the stylesheet\nheader all state \"do not hardcode hex or rem values\", but nothing enforced it:\nbiome (added by cairn-xiw) formats and lints the asset with its recommended\nrule set, which has no rule for \"use a token instead of a literal colour or\nrem\". The invariant was documented and currently honoured, yet a regression\nwould pass every gate.\n\n## Decision\n\nAdd a standalone repository gate, `scripts/check-design-tokens.sh`, that fails\nwhen `style.css` contains a hardcoded hex colour or rem value (CSS comments are\nstripped first). Wire it into the same three places that already gate the webui\nasset: the pre-commit config, the CI `webui` job, and the Makefile `check`\ntarget. Cover its behaviour with `tests/check_design_tokens.rs`.\n\n## Rationale\n\nThis is a project-health gate that lives alongside `scripts/check-file-sizes.sh`,\nnot a cairn feature. It deliberately stays outside cairn's kernel: per\n`dec.toolchain-lint-strictness`, cairn inspects lint *configuration existence*\nand never invokes a linter or formatter. The `CAIRN_LINT_NOT_STRICT` finding\nchecks that a strict config exists; it does not (and should not) encode a\nproject's specific token-conformance rule. A repo script is the right home for\nthat rule, mirroring how the Rust file-size limit is enforced.\n\nScope is limited to `style.css`: it is the hand-written stylesheet bound to the\ntokens. `app.js` is behaviour, not styling, and the marketing surfaces under\n`docs/landing/` are out of this iteration's scope.\n\n## Consequences\n\n- A hardcoded hex colour or rem value in `style.css` now blocks commit, push,\n  and CI, with the offending file and line reported.\n- A future token-conformance need for `app.js` or `docs/landing/` extends this\n  script (parameterised via `CAIRN_DESIGN_TOKENS_TARGET`) rather than adding a\n  new mechanism.\n- The gate is config-free and dependency-free (POSIX sh + awk + grep), keeping\n  the repo's low-dependency, single-binary ethos intact."
            },
            {
              "type": "decisions",
              "path": "meta/decisions/webui-json-schema-version.md",
              "title": "Webui /api/* JSON responses carry a uniform schema_version",
              "frontmatter": {
                "date": "2026-06-23",
                "id": "dec.webui-json-schema-version",
                "status": "accepted"
              },
              "body": "\n# Webui /api/* JSON responses carry a uniform schema_version\n\n## Context\n\nThe webui HTTP surface (`src/ui`) emitted inconsistent JSON. Only `/api/meta`\nand `/api/status` carried a top-level `schema_version`; `/api/graph`,\n`/api/lint`, `/api/blueprint`, `/api/node/*` (and its `contract`, `decisions`,\n`todos`, `research`, `sources`, `beads`, `rationale` suffixes), `/api/depends/*`,\nand `/api/dependents/*` carried no version. A consumer could branch on the\nhandshake endpoints but had no version to branch on for the data endpoints.\n\n`dec.query-json-schema-version` standardized the sibling `query_api` surface and\nexplicitly scoped the webui out as \"a distinct unit of work, not taken here\",\nflagging it as a maintainer decision because handshake-only versioning could be\ndeliberate. The maintainer chose to standardize: stamp `schema_version` on every\n`/api/*` response. This decision completes that deferred unit; it does not\ncontradict `dec.query-json-schema-version`.\n\n## Decision\n\nEvery webui `/api/*` JSON response carries a top-level `schema_version` field,\ncurrently `1`, stamped at a single choke point: the `server::json` `Response`\nconstructor. `json` is the sole builder of `application/json` responses for the\nAPI surface, so a `versioned()` helper splices `schema_version` as the first key\nof the (always-object) body inside `json`. The redundant inline stamps in\n`meta_json` and `status_json` were removed so the choke point is the single\nsource of truth.\n\nThe webui keeps its own `ui::SCHEMA_VERSION` constant, independent of\n`query_api::SCHEMA_VERSION`: these are separate wire surfaces with separate\nversioning lifecycles, and coupling them would force a lockstep bump where none\nis warranted.\n\n## Rationale\n\nStamping at the `json` constructor, with one constant, beats per-handler stamps:\nit cannot drift, every endpoint is covered automatically (including future ones\nand error envelopes), and there is exactly one place to bump when the contract\nchanges. This mirrors the `query_api` choke-point philosophy on a separate\nsurface.\n\nOnly the top-level envelope is versioned. Nested objects (the node records\ninside `/api/graph`) are built by `node_json` as plain strings that never pass\nthrough `json`, so they stay unversioned: the version describes the response\ncontract, not every embedded record.\n\nError envelopes (404/500 `finding` bodies, the blueprint read-error body) flow\nthrough `json` too and so are versioned uniformly, which is desirable: a consumer\nparsing an error still gets a version to branch on. Plain-text fallbacks\n(`text(404, ...)`) are not JSON and carry no version.\n\n## Consequences\n\n- Every webui `/api/*` JSON response now contains `\"schema_version\": 1`\n  (serialised first, sorted alphabetically by the snapshot harness). Bumping the\n  wire contract means bumping `ui::SCHEMA_VERSION`.\n- `server::json` is now coupled to the API version contract. This is acceptable\n  because `json` is `pub(super)` and is used exclusively to build `/api/*`\n  responses; there is no non-API JSON consumer of it.\n- The `wire_format_snapshots` golden fixtures were regenerated to include the\n  stamp, and the test now asserts every endpoint carries a numeric\n  `schema_version`, so a future endpoint added without the stamp fails the gate.\n- `query_api::SCHEMA_VERSION` and `ui::SCHEMA_VERSION` remain independent\n  constants for independent surfaces; they are not required to move together."
            }
          ]
        },
        "research": {
          "schema_version": 1,
          "node": "cairn.ui",
          "artefacts": []
        },
        "sources": {
          "schema_version": 1,
          "node": "cairn.ui",
          "artefacts": []
        },
        "todos": {
          "schema_version": 1,
          "node": "cairn.ui",
          "artefacts": []
        }
      }
    },
    "cairn.watch": {
      "id": "cairn.watch",
      "endpoints": {
        "beads": {
          "schema_version": 1,
          "node": "cairn.watch",
          "beads": []
        },
        "contract": {
          "schema_version": 1,
          "node": "cairn.watch",
          "artefacts": []
        },
        "decisions": {
          "schema_version": 1,
          "node": "cairn.watch",
          "artefacts": [
            {
              "type": "decisions",
              "path": "meta/decisions/close-blueprint-drift.md",
              "title": "Close blueprint drift for orphaned modules",
              "frontmatter": {
                "date": "2026-06-03",
                "id": "dec.close-blueprint-drift",
                "status": "accepted"
              },
              "body": "\n# Close blueprint drift for orphaned modules\n\n## Context\n\nThree source modules existed in the codebase but were not declared in `cairn.blueprint`:\n\n- `src/sse.rs` (SSE event stream parser for Gas City integration spikes)\n- `src/state/mod.rs` (pluggable state persistence backend: filesystem + beads)\n- `src/watch.rs` (watch mode: periodic scan with finding-change events)\n\nThis drift produced `CAIRN_RECONCILE_ORPHANED_FILE` info findings on every scan and prevented `cairn lint` from exiting cleanly.\n\n## Decision\n\nDeclare all three as top-level modules under the `cairn` System node.\n\n## Rationale\n\nAll three modules are actively imported and used:\n\n- `sse` is exported from `lib.rs` and consumed by the Gas City adapter.\n- `state` is used by CLI commands for beads-backed persistence and by the scanner for snapshot state.\n- `watch` is used by the CLI `cairn watch` command and by the query API for finding-delta events.\n\nLeaving them orphaned weakens the dogfooding signal. The framework should model its own source tree completely."
            }
          ]
        },
        "rationale": {
          "schema_version": 1,
          "node": "cairn.watch",
          "artefacts": [
            {
              "type": "decisions",
              "path": "meta/decisions/close-blueprint-drift.md",
              "title": "Close blueprint drift for orphaned modules",
              "frontmatter": {
                "date": "2026-06-03",
                "id": "dec.close-blueprint-drift",
                "status": "accepted"
              },
              "body": "\n# Close blueprint drift for orphaned modules\n\n## Context\n\nThree source modules existed in the codebase but were not declared in `cairn.blueprint`:\n\n- `src/sse.rs` (SSE event stream parser for Gas City integration spikes)\n- `src/state/mod.rs` (pluggable state persistence backend: filesystem + beads)\n- `src/watch.rs` (watch mode: periodic scan with finding-change events)\n\nThis drift produced `CAIRN_RECONCILE_ORPHANED_FILE` info findings on every scan and prevented `cairn lint` from exiting cleanly.\n\n## Decision\n\nDeclare all three as top-level modules under the `cairn` System node.\n\n## Rationale\n\nAll three modules are actively imported and used:\n\n- `sse` is exported from `lib.rs` and consumed by the Gas City adapter.\n- `state` is used by CLI commands for beads-backed persistence and by the scanner for snapshot state.\n- `watch` is used by the CLI `cairn watch` command and by the query API for finding-delta events.\n\nLeaving them orphaned weakens the dogfooding signal. The framework should model its own source tree completely."
            }
          ]
        },
        "research": {
          "schema_version": 1,
          "node": "cairn.watch",
          "artefacts": []
        },
        "sources": {
          "schema_version": 1,
          "node": "cairn.watch",
          "artefacts": []
        },
        "todos": {
          "schema_version": 1,
          "node": "cairn.watch",
          "artefacts": []
        }
      }
    }
  },
  "defaultSelection": "cairn.kernel.scanner",
  "queryModes": [
    "overview",
    "depth",
    "lineage",
    "blueprint",
    "findings",
    "changes",
    "backlog"
  ]
};
