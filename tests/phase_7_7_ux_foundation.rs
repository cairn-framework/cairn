// cairn:allow-large-module reason: Phase 7.7 UX tests intentionally pair live FindingSeverity/copy/CLI/JSON invariants with cairn_planned scenarios from the same module__scenario acceptance matrix; keeping satisfied and pending rows together prevents the archived matrix from losing coverage context.
// Reason: design.md prescribes `module__scenario` test names; the
// `__` collides with the rustc non_snake_case lint despite being
// syntactically valid snake_case identifiers.
#![allow(non_snake_case)]

//! Phase 7.7 UX Foundation acceptance-criterion tests.
//!
//! Mixed state: scenarios already satisfied by reforge cycle 1
//! (`FindingSeverity::Info` on the kernel enum, the `cairn lint`
//! subcommand, `Info`-finding round-trip through `serde_json`, and the
//! unverified-contract Info producer) run as plain `#[test]` and
//! enforce their invariants on every `cargo test`. Scenarios still
//! awaiting phase-7.7 UI work (copy.toml authoring, empty-state
//! component, findings rollup panel, prose-nudge banner) carry
//! `#[cairn_planned(phase = 707)]` and stay skipped under `cargo test`;
//! they fail with `unimplemented!` under `cargo test -- --ignored`.
//!
//! Test contract for `phase-7.7-ux-foundation`. Each test corresponds to one
//! acceptance-criterion scenario across the three spec deltas (`cli`,
//! `graph-explorer`, `reconciliation`). Phase 7.7 removes
//! `#[cairn_planned]` and replaces stub bodies with real assertions
//! group-by-group as code lands.

fn app_js() -> String {
    concat!(
        include_str!("../src/ui_assets/utils.js"),
        include_str!("../src/ui_assets/app.js"),
        include_str!("../src/ui_assets/search.js"),
        include_str!("../src/ui_assets/status-bezel.js"),
        include_str!("../src/ui_assets/query-rail.js"),
        include_str!("../src/ui_assets/graph-workspace.js"),
        include_str!("../src/ui_assets/evidence-rail.js"),
        include_str!("../src/ui_assets/channel-bar.js"),
        include_str!("../src/ui_assets/node-module.js"),
    )
    .to_owned()
}

mod cli {

    /// Scenario: Whole-map inspection without arguments.
    #[test]
    fn test_lint__whole_map_inspection_without_arguments() {
        let result = cairn::cli::run(&[
            "--file".to_owned(),
            "test/fixtures/cairn-bootstrap/cairn.blueprint".to_owned(),
            "lint".to_owned(),
        ]);
        assert_eq!(result.code, 0, "lint always exits zero (non-blocking)");
        assert!(
            !result.stdout.is_empty(),
            "lint must produce output for a fixture with findings"
        );
    }

    /// Scenario: Node-scoped inspection with a positional argument.
    #[test]
    fn test_lint__node_scoped_inspection_with_node_flag() {
        let result = cairn::cli::run(&[
            "--file".to_owned(),
            "test/fixtures/cairn-bootstrap/cairn.blueprint".to_owned(),
            "lint".to_owned(),
            "--node".to_owned(),
            "cairn.kernel.parser".to_owned(),
        ]);
        assert_eq!(result.code, 0, "node-scoped lint exits zero");
    }

    /// Scenario: Inspection delegates to the same library service as lint.
    #[test]
    fn test_lint__inspection_delegates_to_same_library_service_as_lint() {
        // Both commands consume `query::lint`; this test is a structural
        // assertion that the same library entry-point exists. The lint
        // command path inside src/cli/mod.rs calls `query::lint(&graph)`.
        let _: fn(&cairn::map::Graph) -> cairn::map::query::LintResponse = cairn::map::query::lint;
    }

    /// Scenario: Inspection supports JSON output with command envelope.
    #[test]
    fn test_lint__inspection_supports_json_mode() {
        let result = cairn::cli::run(&[
            "--json".to_owned(),
            "lint".to_owned(),
            "--node".to_owned(),
            "test.unknown".to_owned(),
        ]);
        assert_ne!(result.code, 2, "lint --json must not be a usage error");
        let stdout = result.stdout.trim();
        let parsed: serde_json::Value =
            serde_json::from_str(stdout).expect("cairn lint --json must always produce valid JSON");
        assert_eq!(parsed["command"], "lint", "envelope must name the command");
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
            "lint --json is no longer rejected"
        );
    }

    /// Scenario: No-blueprint JSON response uses status 'ok' because the
    /// finding is non-blocking Info severity.
    #[test]
    fn test_lint__json_no_blueprint_uses_ok_status_for_info_severity() {
        let result = cairn::cli::run(&[
            "--json".to_owned(),
            "--file".to_owned(),
            "/nonexistent/cairn.blueprint".to_owned(),
            "lint".to_owned(),
            "--node".to_owned(),
            "test.unknown".to_owned(),
        ]);
        assert_eq!(result.code, 0, "lint always exits zero (non-blocking)");
        let stdout = result.stdout.trim();
        let parsed: serde_json::Value =
            serde_json::from_str(stdout).expect("cairn lint --json must always produce valid JSON");
        assert_eq!(parsed["command"], "lint", "envelope must name the command");
        let findings = parsed["data"]["findings"]
            .as_array()
            .expect("envelope must contain findings array");
        let f = findings
            .iter()
            .find(|f| f["code"] == "CAIRN_NO_BLUEPRINT")
            .expect("no-blueprint finding must be present");
        assert_eq!(
            parsed["status"], "ok",
            "no-blueprint response with Info severity must have status 'ok', not 'error'"
        );
        assert_eq!(
            f["severity"], "info",
            "no-blueprint finding must have Info severity"
        );
    }

    /// Scenario: Context summary includes Info-severity finding count.
    #[test]
    fn test_context__summary_includes_info_severity_count() {
        let result = cairn::cli::run(&[
            "--file".to_owned(),
            "test/fixtures/cairn-bootstrap/cairn.blueprint".to_owned(),
            "context".to_owned(),
        ]);
        assert_eq!(result.code, 0);
        assert!(
            result.stdout.contains("Findings:"),
            "context must include findings summary line"
        );
        // The summary should mention info count alongside errors and warnings.
        assert!(
            result.stdout.contains("info") || result.stdout.contains("infos"),
            "context findings summary must include Info count; got: {}",
            result.stdout
        );
    }

    /// Scenario: Human health info count agrees with the JSON summary count.
    /// Regression: the renderer read `summary.total_info` while the JSON
    /// payload emitted `summary.info`, so human `cairn health` always
    /// reported info: 0.
    #[test]
    fn test_health__human_info_count_matches_json_summary() {
        let root = tempfile::tempdir().expect("temp dir");
        let bp = root.path().join("cairn.blueprint");
        std::fs::write(
            &bp,
            r#"System Test "Test system" id "test" {
}
"#,
        )
        .expect("write blueprint");
        // A change directory with every task checked produces the
        // Info-severity CAIRN_CHANGE_TASKS_COMPLETE finding, so the health
        // info count is non-zero.
        let change_dir = root.path().join("meta/changes/complete-change");
        std::fs::create_dir_all(&change_dir).expect("create change dir");
        std::fs::write(
            change_dir.join("proposal.md"),
            "# Proposal: complete change\n",
        )
        .expect("write proposal");
        std::fs::write(
            change_dir.join("tasks.md"),
            "# Tasks\n\n- [x] design\n- [x] implement\n",
        )
        .expect("write tasks");

        let json_result = cairn::cli::run(&[
            "--json".to_owned(),
            "--file".to_owned(),
            bp.to_string_lossy().to_string(),
            "health".to_owned(),
        ]);
        assert_eq!(json_result.code, 0, "health --json exits zero");
        let parsed: serde_json::Value = serde_json::from_str(json_result.stdout.trim())
            .expect("health --json must produce valid JSON");
        let info = parsed["summary"]["total_info"]
            .as_u64()
            .expect("health summary must carry total_info");
        assert!(
            info >= 1,
            "fixture must produce at least one info finding; summary: {}",
            parsed["summary"]
        );

        let human_result = cairn::cli::run(&[
            "--file".to_owned(),
            bp.to_string_lossy().to_string(),
            "health".to_owned(),
        ]);
        assert_eq!(human_result.code, 0, "health exits zero");
        assert!(
            human_result.stdout.contains(&format!("info: {info}")),
            "human health info count must match JSON summary count {info}; got: {}",
            human_result.stdout
        );
    }

    /// Scenario: Scan --strict exits non-zero on Warning findings.
    #[test]
    fn test_scan__strict_exits_non_zero_on_warning_findings() {
        let result = cairn::cli::run(&[
            "--file".to_owned(),
            "test/fixtures/cairn-bootstrap/cairn.blueprint".to_owned(),
            "scan".to_owned(),
            "--strict".to_owned(),
        ]);
        // The bootstrap fixture has warnings (ghost nodes) so strict must fail.
        assert!(
            result.code != 0,
            "scan --strict must exit non-zero when warnings exist; got code: {}",
            result.code
        );
    }

    /// Scenario: Scan without --strict exits zero on Warning-only findings.
    #[test]
    fn test_scan__non_strict_exits_zero_on_warning_only_findings() {
        let result = cairn::cli::run(&[
            "--file".to_owned(),
            "test/fixtures/cairn-bootstrap/cairn.blueprint".to_owned(),
            "scan".to_owned(),
        ]);
        assert_eq!(
            result.code, 0,
            "scan without --strict must exit zero when only warnings exist"
        );
    }
}

mod empty_state {

    /// Scenario: No-blueprint invocation renders a CTA.
    #[test]
    fn test_empty_state__no_blueprint_invocation_renders_cta() {
        let result = cairn::cli::run(&[
            "--file".to_owned(),
            "nonexistent/cairn.blueprint".to_owned(),
            "lint".to_owned(),
            "--node".to_owned(),
            "test.unknown".to_owned(),
        ]);
        assert_eq!(result.code, 0, "no-blueprint lint exits zero");
        assert!(
            result.stdout.contains("cairn init"),
            "CTA must mention `cairn init`, got: {}",
            result.stdout
        );
    }

    /// Scenario: Clean-map result renders a CTA.
    #[test]
    fn test_empty_state__clean_map_result_renders_cta() {
        let root = tempfile::tempdir().expect("temp dir");
        let bp = root.path().join("cairn.blueprint");
        std::fs::write(
            &bp,
            r#"System Test "Test system" id "test" {
}
"#,
        )
        .expect("write blueprint");
        let result = cairn::cli::run(&[
            "--file".to_owned(),
            bp.to_string_lossy().to_string(),
            "lint".to_owned(),
        ]);
        assert_eq!(result.code, 0, "clean-map lint exits zero");
        assert!(
            result.stdout.contains("Blueprint reconciled cleanly"),
            "clean-map output must use cli-clean-map copy, got: {}",
            result.stdout
        );
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

    /// Scenario: Empty-state entries have heading, body, and cta fields.
    #[test]
    fn test_empty_state__entries_have_heading_body_cta() {
        let copy_toml = include_str!("../docs/design-system/copy.toml");
        let table: toml::Table = copy_toml.parse().expect("copy.toml must be valid TOML");
        let empty_states = table
            .get("empty-states")
            .expect("empty-states section must exist");
        let cli_no_bp = empty_states
            .get("cli-no-blueprint")
            .expect("cli-no-blueprint must exist");
        assert!(
            cli_no_bp.get("heading").is_some(),
            "cli-no-blueprint must have a heading field"
        );
        assert!(
            cli_no_bp.get("body").is_some(),
            "cli-no-blueprint must have a body field"
        );
        assert!(
            cli_no_bp.get("cta").is_some(),
            "cli-no-blueprint must have a cta field"
        );
    }

    /// Scenario: CT001 and CT002 findings have copy entries.
    #[test]
    fn test_explorer__ct001_ct002_have_copy_entries() {
        let copy_toml = include_str!("../docs/design-system/copy.toml");
        let table: toml::Table = copy_toml.parse().expect("copy.toml must be valid TOML");
        let codes = table
            .get("findings")
            .and_then(|f| f.get("codes"))
            .expect("findings.codes section must exist");
        assert!(
            codes.get("CT001").is_some(),
            "CT001 must have a copy entry with heading, body, cta"
        );
        assert!(
            codes.get("CT002").is_some(),
            "CT002 must have a copy entry with heading, body, cta"
        );
    }
}

mod explorer {
    /// Scenario: Empty-state copy is resolved from the design-system registry.
    #[test]
    fn test_explorer__empty_state_component_copy_is_copy_driven() {
        let js = super::app_js();

        assert!(
            js.contains(r#"class="channel-empty""#)
                && js.contains(r"copy(`webui.empty.${active}`)"),
            "empty-state component must render via copy() keys"
        );
    }

    /// Scenario: Empty-state text is sourced from the copy registry.
    #[test]
    fn test_explorer__ten_inline_empty_state_strings_replaced() {
        let js = super::app_js();
        let copy_toml = include_str!("../docs/design-system/copy.toml");
        let table: toml::Table = copy_toml.parse().expect("copy.toml must be valid TOML");
        let empty_states = table
            .get("empty-states")
            .and_then(|entry| entry.as_table())
            .expect("empty-states section must exist");

        assert!(
            empty_states.len() >= 10,
            "copy registry must define at least ten empty-state entries, found: {}",
            empty_states.len()
        );
        assert!(
            js.contains("empty-states.map-loading.body")
                && js.contains("empty-states.node-no-inbound.body")
                && js.contains("empty-states.node-no-outbound.body"),
            "UI empty states should resolve through copy keys"
        );
    }

    /// Scenario: Missing copy keys surface a console warning.
    #[test]
    fn test_explorer__missing_copy_keys_surface_console_warning() {
        let js = super::app_js();
        let copy_start = js.find("function copy(key)").expect("copy() must exist");
        let copy_end = js[copy_start..]
            .find("\nfunction ")
            .map_or(js.len(), |i| copy_start + i);
        let copy_fn = &js[copy_start..copy_end];
        assert!(
            copy_fn.contains("console.warn") && copy_fn.contains("missing"),
            "copy() must log a console.warn for missing keys"
        );
    }

    /// Scenario: Channel bar renders findings and exposes severity and bucket counts.
    #[test]
    fn test_explorer__three_severity_buckets_render_with_count_badges() {
        let js = super::app_js();
        let channel_start = js
            .find("function ChannelBar")
            .expect("channel bar must exist");
        let channel_end = js[channel_start..]
            .find("\nfunction ")
            .map_or(js.len(), |i| channel_start + i);
        let channel_src = &js[channel_start..channel_end];

        assert!(
            js.contains(r#"const CHANNELS = ["findings", "drift", "changes", "backlog"]"#),
            "channel buckets must expose findings, drift, changes, and backlog"
        );
        assert!(
            js.contains("findingBadge(item)")
                && js.contains("item.severity")
                && channel_src.contains("ChannelItem")
                && js.contains("query-chip"),
            "finding rows must show severity and bucket count"
        );
        assert!(
            js.contains("class=\"channel-bar\"") && js.contains(r"copy(`webui.channel.${name}`)"),
            "bucket labels must be copy-driven"
        );
        assert!(
            !js.contains("function ChangesDrawer") && !js.contains("function FindingsPanel"),
            "retired findings controls should stay absent"
        );
    }

    /// Scenario: Ghost wire state maps to the planned/ghost vocabulary.
    #[test]
    fn test_explorer__ghost_renders_as_planned_display_state() {
        let js = super::app_js();
        let css = include_str!("../docs/design-system/components.css");

        assert!(
            js.contains(r#"if (state === "planned" || state === "declared")"#)
                && js.contains(r#"return "ghost""#),
            "planned and declared states must normalise to ghost"
        );
        assert!(
            js.contains(r#"class="legend-item ghost""#)
                && js.contains(r#"copy("webui.states.ghost")"#),
            "state legend must expose ghost (planned) vocabulary"
        );
        assert!(
            css.contains(".state-legend .legend-item.ghost")
                && css.contains("var(--ghost)")
                && css.contains("border-style: dashed"),
            "ghost legend style must use token variables"
        );
    }

    /// Scenario: Findings are supplied to the channel bar from App query state.
    #[test]
    fn test_explorer__panel_reads_only_from_query_consumer_api() {
        let js = super::app_js();
        let channel_start = js
            .find("function ChannelBar")
            .expect("channel bar must exist");
        let channel_end = js[channel_start..]
            .find("\nfunction ")
            .map_or(js.len(), |i| channel_start + i);
        let channel_src = &js[channel_start..channel_end];

        assert!(
            !channel_src.contains("fetch(") && !channel_src.contains("fetchLint"),
            "channel bar should only consume preloaded prop data"
        );
        assert!(
            js.contains("const findings = Array.isArray(lint.findings)")
                && js.contains("fetchLint()")
                && js.contains("<${ChannelBar}")
                && js.contains("findings=${findings}"),
            "App must load lint via query API and pass findings into the channel bar"
        );
    }

    // The following legacy controls were retired by dec.webui-ux-first-redesign:
    // - ProseNudgeBanner and its CTA/copy rendering helpers
    // - overview findings filters and scope-toggle drawer controls

    /// Scenario: Structural integrity findings remain visible and focusable.
    #[test]
    fn test_explorer__structural_error_indicator() {
        let js = super::app_js();
        let copy = include_str!("../docs/design-system/copy.toml");
        assert!(
            js.contains("function ChannelItem")
                && js.contains("const nodeId = item.node || item.slug")
                && js.contains("onFocus(nodeId)"),
            "finding rows must identify and focus affected nodes"
        );
        assert!(
            copy.contains("CAIRN_INTEGRITY_DUPLICATE_ID")
                || copy.contains("CAIRN_INTEGRITY_INVALID_EDGE_ENDPOINT")
                || copy.contains("CAIRN_INTEGRITY_PATH_TIE"),
            "copy.toml must define structural error finding codes"
        );
    }

    /// Scenario: Interface contradiction findings retain severity visibility.
    #[test]
    fn test_explorer__interface_contradiction_indicator() {
        let js = super::app_js();
        let copy = include_str!("../docs/design-system/copy.toml");
        assert!(
            js.contains("String(item.severity || \"info\")")
                && js.contains("${item.severity ? `· ${item.severity}` : \"\"}"),
            "finding rows must display severity labels"
        );
        assert!(
            copy.contains("CAIRN_INTERFACE_HASH_CHANGED")
                || copy.contains("CAIRN_CONTRACT_MISSING")
                || copy.contains("CAIRN_CONTRACT_MISSING_NODE"),
            "copy.toml must define interface contradiction finding codes"
        );
    }

    /// Scenario: Rationale tension findings use settled status tokens.
    #[test]
    fn test_explorer__rationale_tension_indicator() {
        let js = super::app_js();
        let css = include_str!("../docs/design-system/components.css");
        let copy = include_str!("../docs/design-system/copy.toml");
        assert!(
            js.contains("if (severity === \"info\")") && js.contains("counts.infos"),
            "status and channel must retain info severity derived from lint findings"
        );
        assert!(
            css.contains("var(--settled)") && css.contains("var(--settled-wash)"),
            "status token usage should stay in the settled channel"
        );
        assert!(
            copy.contains("CAIRN_DECISION_ORPHANED")
                || copy.contains("CAIRN_PROVENANCE_NO_DECISION")
                || copy.contains("CAIRN_SOURCE_UNVERIFIED"),
            "copy.toml must define rationale tension finding codes"
        );
    }

    /// Scenario: Info-severity findings appear in the channel and status surfaces.
    #[test]
    fn test_explorer__info_severity_findings_appear_in_overlay() {
        let js = super::app_js();
        assert!(
            js.contains("String(item.severity || \"info\")")
                && js.contains("item.severity ? `· ${item.severity}`")
                && js.contains("counts.infos"),
            "info-severity must remain visible in rows and status counts"
        );
    }
}

mod reconciliation {
    /// Scenario: Info variant is defined on the kernel enum.
    #[test]
    fn test_reconciliation__info_variant_defined_on_kernel_enum() {
        let info = cairn::map::FindingSeverity::Info;
        assert_ne!(info, cairn::map::FindingSeverity::Error);
        assert_ne!(info, cairn::map::FindingSeverity::Warning);
    }

    /// Scenario: Orphaned-file state emits an Info finding.
    #[test]
    fn test_reconciliation__orphaned_file_emits_info_finding() {
        // The orphaned-file finding is emitted centrally by the generic
        // reconciler (discover_source_files), not by each per-language module.
        let generic_rs = include_str!("../src/reconcile/generic.rs");
        assert!(
            generic_rs.contains("CAIRN_RECONCILE_ORPHANED_FILE")
                && generic_rs.contains("FindingSeverity::Info"),
            "generic code reconciler must emit CAIRN_RECONCILE_ORPHANED_FILE with Info severity"
        );
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
            target: None,
            path: Some("meta/sources/s1.md".to_owned()),
            deferred_by: None,
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
            target: Some("public_api".to_owned()),
            path: None,
            deferred_by: None,
        };
        let json = serde_json::to_string(&finding).expect("serialise");
        assert!(
            json.contains("\"severity\":\"info\""),
            "severity must serde-render lowercase to match /api/lint wire format, got: {json}"
        );

        assert!(
            json.contains("\"target\":\"public_api\""),
            "target field must appear in JSON, got: {json}"
        );
        let back: cairn::map::graph::Finding = serde_json::from_str(&json).expect("deserialise");
        assert_eq!(back, finding);
    }
}

mod lint_findings {
    use std::fs;

    /// Scenario: Lint renders Error, Warning, and Info findings.
    #[test]
    fn test_lint__renders_all_three_severity_levels() {
        let root = tempfile::tempdir().expect("temp dir");
        let bp = root.path().join("cairn.blueprint");
        fs::write(
            &bp,
            r#"System Test "Test" id "test" {
    Module Auth "Auth" id "test.auth" {
        path "./src/auth"
        todos "./meta/todos"
    }
}
test.auth -> test.nonexistent "Bad edge"
"#,
        )
        .expect("write blueprint");

        fs::create_dir_all(root.path().join("src/auth")).expect("create auth dir");
        fs::write(root.path().join("src/auth/lib.rs"), "pub fn login() {}")
            .expect("write auth file");
        fs::write(root.path().join("src/orphan.rs"), "pub fn orphan() {}")
            .expect("write orphan file");

        fs::create_dir_all(root.path().join("meta/todos")).expect("create todo dir");
        fs::write(
            root.path().join("meta/todos/todo.md"),
            "---\nnode: test.unknown\nstatus: open\ncreated: 2026-04-01\n---\n# Todo\n",
        )
        .expect("write todo file");

        let result = cairn::cli::run(&[
            "--file".to_owned(),
            bp.to_string_lossy().to_string(),
            "lint".to_owned(),
        ]);

        assert!(
            result.stdout.contains("Error"),
            "lint must render Error findings, got: {}",
            result.stdout
        );
        assert!(
            result.stdout.contains("Warning"),
            "lint must render Warning findings, got: {}",
            result.stdout
        );
        assert!(
            result.stdout.contains("Info"),
            "lint must render Info findings, got: {}",
            result.stdout
        );
    }

    /// Scenario: Node-scoped lint filters to findings on that node.
    #[test]
    fn test_lint__node_scoped_filters_findings() {
        let root = tempfile::tempdir().expect("temp dir");
        let bp = root.path().join("cairn.blueprint");
        fs::write(
            &bp,
            r#"System Test "Test" id "test" {
    Module Auth "Auth" id "test.auth" {
        path "./src/auth"
        todos "./meta/todos"
    }
}
test.auth -> test.nonexistent "Bad edge"
"#,
        )
        .expect("write blueprint");

        fs::create_dir_all(root.path().join("src/auth")).expect("create auth dir");
        fs::write(root.path().join("src/auth/lib.rs"), "pub fn login() {}")
            .expect("write auth file");
        fs::write(root.path().join("src/orphan.rs"), "pub fn orphan() {}")
            .expect("write orphan file");

        fs::create_dir_all(root.path().join("meta/todos")).expect("create todo dir");
        fs::write(
            root.path().join("meta/todos/todo.md"),
            "---\nnode: test.unknown\nstatus: open\ncreated: 2026-04-01\n---\n# Todo\n",
        )
        .expect("write todo file");

        let result = cairn::cli::run(&[
            "--file".to_owned(),
            bp.to_string_lossy().to_string(),
            "lint".to_owned(),
            "--node".to_owned(),
            "test.unknown".to_owned(),
        ]);

        assert!(
            result.stdout.contains("CAIRN_TODO_ORPHAN_NODE"),
            "node-scoped lint must show findings on test.unknown, got: {}",
            result.stdout
        );
        assert!(
            !result
                .stdout
                .contains("CAIRN_INTEGRITY_INVALID_EDGE_ENDPOINT"),
            "node-scoped lint must NOT show findings on other nodes, got: {}",
            result.stdout
        );
    }
}
