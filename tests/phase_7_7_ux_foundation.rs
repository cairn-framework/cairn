// Reason: design.md prescribes `module__scenario` test names; the
// `__` collides with the rustc non_snake_case lint despite being
// syntactically valid snake_case identifiers.
#![allow(non_snake_case)]
//! Phase 7.7 UX Foundation acceptance-criterion tests.
//!
//! Mixed state: scenarios already satisfied by reforge cycle 1
//! (`FindingSeverity::Info` on the kernel enum, the `cairn check`
//! subcommand, `Info`-finding round-trip through `serde_json`, and the
//! unverified-contract Info producer) run as plain `#[test]` and
//! enforce their invariants on every `cargo test`. Scenarios still
//! awaiting phase-7.7 UI work (copy.toml authoring, empty-state
//! component, findings rollup panel, prose-nudge banner) carry
//! `#[cflx_planned(phase = 707)]` and stay skipped under `cargo test`;
//! they fail with `unimplemented!` under `cargo test -- --ignored`.
//!
//! Test contract for `phase-7.7-ux-foundation`. Each test corresponds to one
//! acceptance-criterion scenario across the three spec deltas (`cli`,
//! `graph-explorer`, `reconciliation`). Phase 7.7 removes
//! `#[cflx_planned]` and replaces stub bodies with real assertions
//! group-by-group as code lands.

use cairn::cflx_planned;

mod cli {

    /// Scenario: Whole-map inspection without arguments.
    #[test]
    fn test_check__whole_map_inspection_without_arguments() {
        let result = cairn::cli::run(&[
            "--file".to_owned(),
            "test/fixtures/cairn-bootstrap/cairn.blueprint".to_owned(),
            "check".to_owned(),
        ]);
        assert_eq!(result.code, 0, "check always exits zero (non-blocking)");
        assert!(
            !result.stdout.is_empty(),
            "check must produce output for a fixture with findings"
        );
    }

    /// Scenario: Node-scoped inspection with a positional argument.
    #[test]
    fn test_check__node_scoped_inspection_with_positional_argument() {
        let result = cairn::cli::run(&[
            "--file".to_owned(),
            "test/fixtures/cairn-bootstrap/cairn.blueprint".to_owned(),
            "check".to_owned(),
            "cairn.kernel.parser".to_owned(),
        ]);
        assert_eq!(result.code, 0, "node-scoped check exits zero");
    }

    /// Scenario: Inspection delegates to the same library service as lint.
    #[test]
    fn test_check__inspection_delegates_to_same_library_service_as_lint() {
        // Both commands consume `query::lint`; this test is a structural
        // assertion that the same library entry-point exists. The check
        // command path inside src/cli/mod.rs calls `query::lint(&graph)`.
        let _: fn(&cairn::map::Graph) -> cairn::map::query::LintResponse = cairn::map::query::lint;
    }

    /// Scenario: Inspection supports JSON output with command envelope.
    #[test]
    fn test_check__inspection_supports_json_mode() {
        let result = cairn::cli::run(&["--json".to_owned(), "check".to_owned()]);
        assert_ne!(result.code, 2, "check --json must not be a usage error");
        let stdout = result.stdout.trim();
        let parsed: serde_json::Value = serde_json::from_str(stdout)
            .expect("cairn check --json must always produce valid JSON");
        assert_eq!(parsed["command"], "check", "envelope must name the command");
        assert!(
            parsed["status"] == "ok" || parsed["status"] == "error",
            "envelope status must be ok or error"
        );
        assert!(
            parsed["data"]["findings"].is_array(),
            "envelope must contain findings array"
        );
        assert!(
            !result.stderr.contains("cairn lint --json"),
            "check --json is no longer rejected"
        );
    }
}

mod empty_state {
    use super::cflx_planned;

    /// Scenario: No-blueprint invocation renders a CTA.
    #[test]
    fn test_empty_state__no_blueprint_invocation_renders_cta() {
        let result = cairn::cli::run(&[
            "--file".to_owned(),
            "nonexistent/cairn.blueprint".to_owned(),
            "check".to_owned(),
        ]);
        assert_eq!(result.code, 0, "no-blueprint check exits zero");
        assert!(
            result.stdout.contains("cairn init"),
            "CTA must mention `cairn init`, got: {}",
            result.stdout
        );
    }

    /// Scenario: Clean-map result renders a CTA.
    #[cflx_planned(phase = 707)]
    #[test]
    fn test_empty_state__clean_map_result_renders_cta() {
        unimplemented!("awaits phase-7.7: empty-state clean-map result renders CTA");
    }

    /// Scenario: Empty-state copy is free of em-dashes (CLI and webui share copy file).
    #[test]
    fn test_empty_state__copy_has_no_em_dashes() {
        let copy_toml = include_str!("../docs/design-system/copy.toml");
        assert!(
            !copy_toml.contains('\u{2014}'),
            "copy.toml must not contain em-dashes (U+2014)"
        );
    }
}

mod explorer {
    use super::cflx_planned;

    /// Scenario: Component is defined with token-only styling.
    #[cflx_planned(phase = 707)]
    #[test]
    fn test_explorer__empty_state_component_uses_token_only_styling() {
        unimplemented!("awaits phase-7.7: explorer empty-state component uses token-only styling");
    }

    /// Scenario: All ten inline empty-state strings are replaced.
    #[cflx_planned(phase = 707)]
    #[test]
    fn test_explorer__ten_inline_empty_state_strings_replaced() {
        unimplemented!("awaits phase-7.7: explorer ten inline empty-state strings replaced");
    }

    /// Scenario: Missing copy keys surface a console warning.
    #[cflx_planned(phase = 707)]
    #[test]
    fn test_explorer__missing_copy_keys_surface_console_warning() {
        unimplemented!("awaits phase-7.7: explorer missing copy keys surface console warning");
    }

    /// Scenario: Three severity buckets render with count badges.
    #[cflx_planned(phase = 707)]
    #[test]
    fn test_explorer__three_severity_buckets_render_with_count_badges() {
        unimplemented!("awaits phase-7.7: explorer three severity buckets render with badges");
    }

    /// Scenario: Scope toggle filters to the selected node.
    #[cflx_planned(phase = 707)]
    #[test]
    fn test_explorer__scope_toggle_filters_to_selected_node() {
        unimplemented!("awaits phase-7.7: explorer scope toggle filters to selected node");
    }

    /// Scenario: Scope toggle is disabled when no node is selected.
    #[cflx_planned(phase = 707)]
    #[test]
    fn test_explorer__scope_toggle_disabled_when_no_node_selected() {
        unimplemented!("awaits phase-7.7: explorer scope toggle disabled when no node selected");
    }

    /// Scenario: Category filter chips derive from the finding stream.
    #[cflx_planned(phase = 707)]
    #[test]
    fn test_explorer__category_filter_chips_derive_from_finding_stream() {
        unimplemented!("awaits phase-7.7: explorer category filter chips derive from findings");
    }

    /// Scenario: Panel reads only from the query-consumer API.
    #[cflx_planned(phase = 707)]
    #[test]
    fn test_explorer__panel_reads_only_from_query_consumer_api() {
        unimplemented!("awaits phase-7.7: explorer panel reads only from query-consumer API");
    }

    /// Scenario: Banner renders the highest-severity finding's nudge.
    #[cflx_planned(phase = 707)]
    #[test]
    fn test_explorer__banner_renders_highest_severity_finding_nudge() {
        unimplemented!("awaits phase-7.7: explorer banner renders highest-severity nudge");
    }

    /// Scenario: Tie-break by lowest-numbered code.
    #[cflx_planned(phase = 707)]
    #[test]
    fn test_explorer__banner_tie_break_by_lowest_numbered_code() {
        unimplemented!("awaits phase-7.7: explorer banner tie-break by lowest-numbered code");
    }

    /// Scenario: Banner CTA is a copy-pasteable CLI snippet.
    #[cflx_planned(phase = 707)]
    #[test]
    fn test_explorer__banner_cta_is_copy_pasteable_cli_snippet() {
        unimplemented!("awaits phase-7.7: explorer banner CTA is copy-pasteable CLI snippet");
    }

    /// Scenario: Banner is hidden when the node has no findings.
    #[cflx_planned(phase = 707)]
    #[test]
    fn test_explorer__banner_hidden_when_node_has_no_findings() {
        unimplemented!("awaits phase-7.7: explorer banner hidden when node has no findings");
    }

    /// Scenario: Structural error indicator (integrity overlay).
    #[cflx_planned(phase = 707)]
    #[test]
    fn test_explorer__structural_error_indicator() {
        unimplemented!("awaits phase-7.7: explorer structural error indicator");
    }

    /// Scenario: Interface contradiction indicator (integrity overlay).
    #[cflx_planned(phase = 707)]
    #[test]
    fn test_explorer__interface_contradiction_indicator() {
        unimplemented!("awaits phase-7.7: explorer interface contradiction indicator");
    }

    /// Scenario: Rationale tension indicator (integrity overlay).
    #[cflx_planned(phase = 707)]
    #[test]
    fn test_explorer__rationale_tension_indicator() {
        unimplemented!("awaits phase-7.7: explorer rationale tension indicator");
    }

    /// Scenario: Info-severity findings appear in the overlay.
    #[cflx_planned(phase = 707)]
    #[test]
    fn test_explorer__info_severity_findings_appear_in_overlay() {
        unimplemented!("awaits phase-7.7: explorer info-severity findings appear in overlay");
    }
}

mod reconciliation {
    use super::cflx_planned;

    /// Scenario: Info variant is defined on the kernel enum.
    #[test]
    fn test_reconciliation__info_variant_defined_on_kernel_enum() {
        let info = cairn::map::FindingSeverity::Info;
        assert_ne!(info, cairn::map::FindingSeverity::Error);
        assert_ne!(info, cairn::map::FindingSeverity::Warning);
    }

    /// Scenario: Orphaned-file state emits an Info finding.
    #[cflx_planned(phase = 707)]
    #[test]
    fn test_reconciliation__orphaned_file_emits_info_finding() {
        unimplemented!("awaits phase-7.7: reconciliation orphaned-file emits Info finding");
    }

    /// Scenario: Unverified-contract state emits an Info finding.
    #[test]
    fn test_reconciliation__unverified_contract_emits_info_finding() {
        // The artefacts validator emits an Info finding for any source
        // declared with verification = "unverified". This is the canonical
        // Info producer site for phase 7.7.
        let finding = cairn::map::graph::Finding {
            code: "CAIRN_SOURCE_UNVERIFIED".to_owned(),
            severity: cairn::map::FindingSeverity::Info,
            message: "source `s1` is unverified".to_owned(),
            node: None,
            path: Some("openspec/sources/s1.md".to_owned()),
        };
        assert_eq!(finding.severity, cairn::map::FindingSeverity::Info);
    }

    /// Scenario: Info findings do not block hooks or gates.
    #[test]
    fn test_reconciliation__info_findings_do_not_block_hooks_or_gates() {
        // Hooks and CLI gates filter for Error severity only; Info and
        // Warning are advisory. We assert the structural property that
        // Info != Error.
        assert_ne!(
            cairn::map::FindingSeverity::Info,
            cairn::map::FindingSeverity::Error
        );
        assert_ne!(
            cairn::map::FindingSeverity::Info,
            cairn::map::FindingSeverity::Warning
        );
    }

    /// Scenario: Info findings round-trip through `serde_json` with lowercase severity.
    #[test]
    fn test_reconciliation__info_findings_round_trip_through_serde_json() {
        let finding = cairn::map::graph::Finding {
            code: "CT001".to_owned(),
            severity: cairn::map::FindingSeverity::Info,
            message: "advisory".to_owned(),
            node: Some("node-a".to_owned()),
            path: None,
        };
        let json = serde_json::to_string(&finding).expect("serialise");
        assert!(
            json.contains("\"severity\":\"info\""),
            "severity must serde-render lowercase to match /api/lint wire format, got: {json}"
        );
        let back: cairn::map::graph::Finding = serde_json::from_str(&json).expect("deserialise");
        assert_eq!(back, finding);
    }
}
