# Cairn bootstrap fixture for the kernel MVP.

System Cairn "Ontology framework with pluggable reconcilers" id "cairn" @framework {
    Container Kernel "Domain-agnostic core" id "cairn.kernel" @kernel {
        path "./src/kernel"

        Module Parser "Parses .dsl files into a node graph" id "cairn.kernel.parser" {
            path "./src/kernel/parser"
            contract "./meta/contracts/kernel/parser.md"
            decisions "./meta/decisions/kernel/parser/"
        }

        Module Artefacts "Pluggable artefact type registry" id "cairn.kernel.artefacts" {
            path "./src/kernel/artefacts"
            contract "./meta/contracts/kernel/artefacts.md"
        }

        Module ReconcilerInterface "Abstract contract for reconcilers" id "cairn.kernel.reconciler" {
            path "./src/kernel/reconciler"
            contract "./meta/contracts/kernel/reconciler.md"
        }

        Module Reconciliation "Joins DSL, artefacts, and reconciler output into ontology" id "cairn.kernel.reconciliation" {
            path "./src/kernel/reconciliation"
            contract "./meta/contracts/kernel/reconciliation.md"
        }

        Module Query "Typed queries over the ontology" id "cairn.kernel.query" {
            path "./src/kernel/query"
            contract "./meta/contracts/kernel/query.md"
        }

        Module Changes "Change directories, delta semantics, archive" id "cairn.kernel.changes" {
            path "./src/kernel/changes"
            contract "./meta/contracts/kernel/changes.md"
        }

        Module Hooks "Commit and task-boundary enforcement" id "cairn.kernel.hooks" {
            path "./src/kernel/hooks"
            contract "./meta/contracts/kernel/hooks.md"
        }

        Module CLI "Primary user surface" id "cairn.kernel.cli" {
            path "./src/kernel/cli"
            contract "./meta/contracts/kernel/cli.md"
        }
    }

    Module CodeReconciler "Tree-sitter-based reconciler for source code" id "cairn.reconcilers.code" @reconciler @code {
        path "./src/reconcilers/code"
        contract "./meta/contracts/reconcilers/code.md"
    }
}

cairn.kernel.cli -> cairn.kernel.query "Exposes queries as commands"
cairn.kernel.query -> cairn.kernel.reconciliation "Reads ontology"
cairn.kernel.reconciliation -> cairn.kernel.parser "Consumes parsed DSL"
cairn.kernel.reconciliation -> cairn.kernel.artefacts "Validates artefacts"
cairn.kernel.changes -> cairn.kernel.reconciliation "Validates before archive"
cairn.kernel.hooks -> cairn.kernel.reconciliation "Gates on integrity"
cairn.reconcilers.code -> cairn.kernel.reconciler "Implements"
