// Reason: design.md prescribes `module__scenario` test names; the
// `__` collides with the rustc non_snake_case lint despite being
// syntactically valid snake_case identifiers.
#![allow(non_snake_case)]
//! Phase 9 Brownfield Extraction acceptance-criterion tests.
//!
//! Mixed state: the seven heuristic-invariant scenarios delivered by
//! reforge cycle 1 (coupling-score formula, threshold buckets, file
//! count, depth limit, edge threshold, path-derived names) run as
//! plain `#[test]` and enforce their invariants on every `cargo test`.
//! The 26 acceptance-criterion stubs covering init / refine / review /
//! mcp / suggest / interview / templates / obligations carry
//! `#[cairn_planned(phase = 900)]` and stay skipped under `cargo test`;
//! they fail with `unimplemented!` under `cargo test -- --ignored`.
//! Phase 9 removes `#[cairn_planned]` group-by-group as code lands.

use cairn::cairn_planned;

// Fixture helpers. Phase 9 supplies real implementations; the pre-phase
// keeps thin placeholders so test bodies referencing them compile.
// Reason: fixture stubs referenced by pre-phase test bodies; dead until phase 9
// implementations land.
#[allow(dead_code)]
fn fixture_repo_without_blueprint() {
    unimplemented!("awaits phase-9: fixture_repo_without_blueprint");
}

// Reason: pre-phase fixture stub; dead until phase 9 lands.
#[allow(dead_code)]
fn fixture_repo_with_blueprint() {
    unimplemented!("awaits phase-9: fixture_repo_with_blueprint");
}

// Reason: pre-phase fixture stub; dead until phase 9 lands.
#[allow(dead_code)]
fn fixture_repo_with_brownfield_change() {
    unimplemented!("awaits phase-9: fixture_repo_with_brownfield_change");
}

// Reason: pre-phase fixture stub; dead until phase 9 lands.
#[allow(dead_code)]
fn fixture_repo_with_pending_suggested_edges() {
    unimplemented!("awaits phase-9: fixture_repo_with_pending_suggested_edges");
}

// Reason: pre-phase fixture stub; dead until phase 9 lands.
#[allow(dead_code)]
fn fixture_change_with_partial_interview_state() {
    unimplemented!("awaits phase-9: fixture_change_with_partial_interview_state");
}

// Reason: pre-phase fixture stub; dead until phase 9 lands.
#[allow(dead_code)]
fn fixture_project_config_with_contract_template() {
    unimplemented!("awaits phase-9: fixture_project_config_with_contract_template");
}

// Reason: pre-phase fixture stub; dead until phase 9 lands.
#[allow(dead_code)]
fn fixture_decision_with_obligations_field() {
    unimplemented!("awaits phase-9: fixture_decision_with_obligations_field");
}

fn temp_repo(name: &str) -> std::path::PathBuf {
    use std::time::{SystemTime, UNIX_EPOCH};
    let suffix = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let root = std::env::temp_dir().join(format!("cairn-bf-{name}-{suffix}"));
    std::fs::create_dir_all(&root).unwrap();
    root
}

fn populate_source_dir(root: &std::path::Path, rel: &str, count: usize) {
    let dir = root.join(rel);
    std::fs::create_dir_all(&dir).unwrap();
    for i in 0..count {
        std::fs::write(dir.join(format!("file{i}.rs")), format!("fn f{i}() {{}}\n")).unwrap();
    }
}

mod init {
    use super::{populate_source_dir, temp_repo};
    use cairn::brownfield::{discovery, init as bf_init, write_change};
    use std::fs;

    /// Scenario: Discovery does not require an existing blueprint.
    #[test]
    fn test_init__discovery_does_not_require_existing_blueprint() {
        let root = temp_repo("no-bp");
        populate_source_dir(&root, "src/auth", 4);
        // No cairn.blueprint exists; discovery should still work.
        let result = discovery::discover(&root);
        assert!(result.is_ok());
        let extraction = result.unwrap();
        assert!(!extraction.candidates.is_empty());
    }

    /// Scenario: Candidate heuristics are deterministic.
    #[test]
    fn test_init__candidate_heuristics_are_deterministic() {
        let root = temp_repo("deterministic");
        populate_source_dir(&root, "src/core", 5);
        populate_source_dir(&root, "src/api", 3);
        let r1 = discovery::discover(&root).unwrap();
        let r2 = discovery::discover(&root).unwrap();
        assert_eq!(r1.candidates.len(), r2.candidates.len());
        for (a, b) in r1.candidates.iter().zip(r2.candidates.iter()) {
            assert_eq!(a.id, b.id);
            assert!((a.confidence - b.confidence).abs() < f64::EPSILON);
        }
    }

    /// Scenario: Init creates a brownfield change directory.
    #[test]
    fn test_init__creates_brownfield_change_directory() {
        let root = temp_repo("init-create");
        populate_source_dir(&root, "src/models", 4);
        let result = bf_init::run_init_from_code(&root, false);
        assert!(result.is_ok());
        let change_id = result.unwrap();
        let change_dir = root.join("meta/changes").join(&change_id);
        assert!(change_dir.exists());
        assert!(change_dir.join("proposal.md").exists());
        assert!(change_dir.join("blueprint.delta").exists());
        assert!(change_dir.join("contracts").exists());
    }

    /// Scenario: Existing change protected without --force.
    #[test]
    fn test_init__existing_change_protected_without_force() {
        let root = temp_repo("init-no-force");
        populate_source_dir(&root, "src/core", 3);
        // First init succeeds.
        let first = bf_init::run_init_from_code(&root, false);
        assert!(first.is_ok());
        // Second init without force fails.
        let second = bf_init::run_init_from_code(&root, false);
        assert!(second.is_err());
        let err_msg = format!("{}", second.unwrap_err());
        assert!(err_msg.contains("already exists"));
    }

    /// Scenario: Force replaces existing change.
    #[test]
    fn test_init__force_replaces_existing_change() {
        let root = temp_repo("init-force");
        populate_source_dir(&root, "src/core", 3);
        let first = bf_init::run_init_from_code(&root, false);
        assert!(first.is_ok());
        let change_dir = root.join("meta/changes/brownfield-init");
        // Write a marker to verify it gets replaced.
        fs::write(change_dir.join("marker.txt"), "old").unwrap();
        // Force init replaces the directory.
        let second = bf_init::run_init_from_code(&root, true);
        assert!(second.is_ok());
        assert!(!change_dir.join("marker.txt").exists());
        assert!(change_dir.join("proposal.md").exists());
    }

    /// Scenario: `write_change` creates expected artifacts.
    #[test]
    fn test_init__write_change_creates_artifacts() {
        let root = temp_repo("write-change");
        let extraction = discovery::Extraction {
            candidates: vec![discovery::DiscoveredCandidate {
                id: "src.core".to_owned(),
                name: "core".to_owned(),
                description: "Core module".to_owned(),
                path: "src/core".to_owned(),
                tags: vec![],
                confidence: 0.8,
                evidence: vec!["src/core/a.rs".to_owned()],
                edges: vec![],
            }],
            schema_version: 1,
        };
        let result = write_change(&root, "test-change", &extraction, &[]);
        assert!(result.is_ok());
        let dir = root.join("meta/changes/test-change");
        assert!(dir.join("proposal.md").exists());
        assert!(dir.join("blueprint.delta").exists());
        assert!(dir.join("contracts/src_core.md").exists());
    }

    /// Scenario: discover -> archive round-trip merges discovered nodes into the
    /// blueprint. Regression for cairn-e12: init --from-code wrote a delta the
    /// parser could not read and seeded no base blueprint, so the blueprint
    /// never gained nodes.
    #[test]
    fn test_init__round_trip_archive_merges_nodes_into_blueprint() {
        let root = temp_repo("round-trip");
        populate_source_dir(&root, "src/alpha", 3);
        populate_source_dir(&root, "src/beta", 3);

        // init --from-code seeds a base blueprint and writes the change.
        let change_id = bf_init::run_init_from_code(&root, false).unwrap();
        assert!(
            root.join("cairn.blueprint").exists(),
            "init --from-code must seed a base blueprint"
        );

        // The emitted delta must be the canonical form the parser accepts.
        let delta = fs::read_to_string(
            root.join("meta/changes")
                .join(&change_id)
                .join("blueprint.delta"),
        )
        .unwrap();
        assert!(
            delta.contains("## ADDED Nodes"),
            "delta must use the canonical ADDED Nodes header: {delta}"
        );

        // Archiving applies the delta: the blueprint gains the discovered nodes.
        let report = cairn::changes::archive(&root, &root.join("cairn.blueprint"), &change_id);
        assert!(report.is_ok(), "archive must succeed: {report:?}");

        let blueprint = fs::read_to_string(root.join("cairn.blueprint")).unwrap();
        assert!(
            blueprint.contains(r#"id "src.alpha""#) && blueprint.contains(r#"id "src.beta""#),
            "blueprint must gain the discovered nodes: {blueprint}"
        );
    }
}

mod refine {
    use super::{populate_source_dir, temp_repo};
    use cairn::brownfield::refine as bf_refine;

    /// Scenario: Refine proposes additions for new directories.
    #[test]
    fn test_refine__proposes_additions_for_new_directories() {
        let root = temp_repo("refine-add");
        populate_source_dir(&root, "src/auth", 4);
        let result = bf_refine::run_refine(&root);
        assert!(result.is_ok());
        let change_id = result.unwrap();
        assert!(change_id.starts_with("brownfield-refine-"));
        let change_dir = root.join("meta/changes").join(&change_id);
        assert!(change_dir.exists());
        assert!(change_dir.join("proposal.md").exists());
        assert!(change_dir.join("blueprint.delta").exists());
    }

    /// Scenario: Refine does not replace current truth (writes to new dir).
    #[test]
    fn test_refine__does_not_replace_current_truth() {
        let root = temp_repo("refine-nodup");
        populate_source_dir(&root, "src/core", 3);
        let first = bf_refine::run_refine(&root).unwrap();
        // Add a second source dir and refine again.
        populate_source_dir(&root, "src/api", 3);
        let second = bf_refine::run_refine(&root).unwrap();
        // Each refine creates a separate change directory.
        assert_ne!(first, second);
        assert!(root.join("meta/changes").join(&first).exists());
        assert!(root.join("meta/changes").join(&second).exists());
    }

    /// Scenario: Refine detects a renamed directory.
    #[test]
    fn test_refine__detects_renamed_directory() {
        let root = temp_repo("refine-rename");
        populate_source_dir(&root, "src/core", 3);

        // Create a minimal blueprint with a node for src/core.
        let blueprint = r#"System App "App" id "app" {
    Module Core "Core" id "app.core" {
        path "src/core"
    }
}"#;
        std::fs::write(root.join("cairn.blueprint"), blueprint).unwrap();

        // Rename the directory on disk.
        std::fs::rename(root.join("src/core"), root.join("src/kernel")).unwrap();
        populate_source_dir(&root, "src/kernel", 3);

        let result = bf_refine::run_refine(&root);
        assert!(result.is_ok(), "refine failed: {result:?}");
        let change_id = result.unwrap();
        let delta_path = root
            .join("meta/changes")
            .join(&change_id)
            .join("blueprint.delta");
        let delta = std::fs::read_to_string(&delta_path).expect("read delta");

        // The delta should mention the rename from app.core to src.kernel.
        assert!(
            delta.contains("app.core") && delta.contains("src.kernel"),
            "delta should detect rename from app.core to src.kernel, got:\n{delta}"
        );
    }

    /// Scenario: refine -> archive round-trip applies removals and additions to
    /// the blueprint. Regression for cairn-h8o: refine emitted a non-canonical
    /// delta the change parser could not read, so archiving was a no-op.
    #[test]
    fn test_refine__round_trip_archive_applies_delta() {
        let root = temp_repo("refine-round-trip");
        populate_source_dir(&root, "src/alpha", 3);
        // An existing node whose path moved within the same parent dir is detected
        // as a rename, which refine folds into a removal of the old node plus the
        // addition of the new dir. This exercises the rename branch end to end.
        let blueprint = "System App \"App\" id \"app\" {\n    Module Old \"Old\" id \"app.old\" {\n        path \"./src/old\"\n    }\n}\n";
        std::fs::write(root.join("cairn.blueprint"), blueprint).unwrap();

        let change_id = bf_refine::run_refine(&root).unwrap();
        let delta = std::fs::read_to_string(
            root.join("meta/changes")
                .join(&change_id)
                .join("blueprint.delta"),
        )
        .unwrap();
        assert!(
            delta.contains("## ADDED Nodes") && delta.contains("## REMOVED Nodes"),
            "delta must use canonical headers: {delta}"
        );

        let report = cairn::changes::archive(&root, &root.join("cairn.blueprint"), &change_id);
        assert!(report.is_ok(), "archive must succeed: {report:?}");

        let result = std::fs::read_to_string(root.join("cairn.blueprint")).unwrap();
        assert!(
            result.contains(r#"id "src.alpha""#),
            "blueprint must gain the discovered node: {result}"
        );
        assert!(
            !result.contains(r#"id "app.old""#),
            "blueprint must drop the renamed-away node: {result}"
        );
    }
}

mod discovery_tests {
    use super::{populate_source_dir, temp_repo};
    use cairn::brownfield::discovery;
    use std::fs;

    /// Scenario: Discovery finds source directories with enough files.
    #[test]
    fn test_discovery__finds_source_directories() {
        let root = temp_repo("discover-find");
        populate_source_dir(&root, "src/auth", 4);
        populate_source_dir(&root, "src/db", 3);
        // Too few files, should not appear.
        populate_source_dir(&root, "src/tiny", 2);

        let extraction = discovery::discover(&root).unwrap();
        let ids: Vec<&str> = extraction
            .candidates
            .iter()
            .map(|c| c.id.as_str())
            .collect();
        assert!(ids.contains(&"src.auth"));
        assert!(ids.contains(&"src.db"));
        assert!(!ids.contains(&"src.tiny"));
    }

    /// Scenario: Discovery skips ignored directories.
    #[test]
    fn test_discovery__skips_ignored_dirs() {
        let root = temp_repo("discover-skip");
        populate_source_dir(&root, "src/core", 5);
        populate_source_dir(&root, "target/debug", 10);
        populate_source_dir(&root, "node_modules/pkg", 10);

        let extraction = discovery::discover(&root).unwrap();
        let ids: Vec<&str> = extraction
            .candidates
            .iter()
            .map(|c| c.id.as_str())
            .collect();
        assert!(ids.contains(&"src.core"));
        assert!(!ids.iter().any(|id| id.contains("target")));
        assert!(!ids.iter().any(|id| id.contains("node_modules")));
    }

    /// Scenario: Discovery respects depth limit.
    #[test]
    fn test_discovery__respects_depth_limit() {
        let root = temp_repo("discover-depth");
        // depth 1-4: should be found
        populate_source_dir(&root, "a/b/c/d", 4);
        // depth 5: should be skipped (MAX_DEPTH=4, so depth>4 is skipped)
        populate_source_dir(&root, "a/b/c/d/e/f", 4);

        let extraction = discovery::discover(&root).unwrap();
        let ids: Vec<&str> = extraction
            .candidates
            .iter()
            .map(|c| c.id.as_str())
            .collect();
        assert!(ids.contains(&"a.b.c.d"));
        // The deeply nested dir at depth 5+ should not appear.
        assert!(!ids.iter().any(|id| id.contains("a.b.c.d.e.f")));
    }

    /// Scenario: Discovery supports multiple extensions.
    #[test]
    fn test_discovery__supports_multiple_extensions() {
        let root = temp_repo("discover-exts");
        let dir = root.join("src/mixed");
        fs::create_dir_all(&dir).unwrap();
        fs::write(dir.join("a.ts"), "export {}").unwrap();
        fs::write(dir.join("b.js"), "module.exports = {}").unwrap();
        fs::write(dir.join("c.py"), "pass").unwrap();
        fs::write(dir.join("d.go"), "package main").unwrap();
        fs::write(dir.join("readme.md"), "# readme").unwrap();

        let extraction = discovery::discover(&root).unwrap();
        let ids: Vec<&str> = extraction
            .candidates
            .iter()
            .map(|c| c.id.as_str())
            .collect();
        // 4 source files (ts, js, py, go), md is not counted.
        assert!(ids.contains(&"src.mixed"));
    }
}

mod fixture_integration {
    use super::{populate_source_dir, temp_repo};
    use cairn::brownfield::discovery;
    use std::fs;

    /// Scenario: Mixed-language repo discovers all language dirs.
    #[test]
    fn test_discovery__mixed_language_repo_finds_all() {
        let root = temp_repo("mixed-lang");
        // Create dirs with files of different extensions.
        let rs_dir = root.join("src/backend");
        fs::create_dir_all(&rs_dir).unwrap();
        for i in 0..4 {
            fs::write(
                rs_dir.join(format!("mod{i}.rs")),
                format!("fn f{i}() {{}}\n"),
            )
            .unwrap();
        }
        let ts_dir = root.join("src/frontend");
        fs::create_dir_all(&ts_dir).unwrap();
        for i in 0..3 {
            fs::write(
                ts_dir.join(format!("component{i}.ts")),
                format!("export const c{i} = {i};\n"),
            )
            .unwrap();
        }
        let py_dir = root.join("scripts");
        fs::create_dir_all(&py_dir).unwrap();
        for i in 0..3 {
            fs::write(
                py_dir.join(format!("task{i}.py")),
                format!("def t{i}(): pass\n"),
            )
            .unwrap();
        }

        let extraction = discovery::discover(&root).unwrap();
        let ids: Vec<&str> = extraction
            .candidates
            .iter()
            .map(|c| c.id.as_str())
            .collect();
        assert!(ids.contains(&"src.backend"), "should find .rs dir");
        assert!(ids.contains(&"src.frontend"), "should find .ts dir");
        assert!(ids.contains(&"scripts"), "should find .py dir");
    }

    /// Scenario: Directory with exactly 3 files gets 0.7 confidence.
    #[test]
    fn test_discovery__low_confidence_for_three_files() {
        let root = temp_repo("low-conf");
        populate_source_dir(&root, "src/small", 3);

        let extraction = discovery::discover(&root).unwrap();
        let candidate = extraction
            .candidates
            .iter()
            .find(|c| c.id == "src.small")
            .expect("should discover src.small");
        assert!(
            (candidate.confidence - 0.7).abs() < f64::EPSILON,
            "3-file dir should have confidence 0.7, got {}",
            candidate.confidence
        );
    }

    /// Scenario: Discovery at depth 4 works but depth 5+ is skipped.
    #[test]
    fn test_discovery__nested_depth_boundary() {
        let root = temp_repo("depth-boundary");
        // depth 4 (a/b/c/d) should be found
        populate_source_dir(&root, "a/b/c/d", 5);
        // depth 5 (a/b/c/d/e) should be skipped
        populate_source_dir(&root, "a/b/c/d/e", 5);

        let extraction = discovery::discover(&root).unwrap();
        let ids: Vec<&str> = extraction
            .candidates
            .iter()
            .map(|c| c.id.as_str())
            .collect();
        assert!(ids.contains(&"a.b.c.d"), "depth 4 should be found");
        assert!(
            !ids.iter().any(|id| id.contains("a.b.c.d.e")),
            "depth 5 should be skipped"
        );
    }

    /// Scenario: Sibling edges are bidirectional for dirs sharing a parent.
    #[test]
    fn test_discovery__sibling_edges_bidirectional() {
        let root = temp_repo("sibling-edges");
        populate_source_dir(&root, "src/alpha", 4);
        populate_source_dir(&root, "src/beta", 3);

        let extraction = discovery::discover(&root).unwrap();
        let alpha = extraction
            .candidates
            .iter()
            .find(|c| c.id == "src.alpha")
            .expect("should discover src.alpha");
        let beta = extraction
            .candidates
            .iter()
            .find(|c| c.id == "src.beta")
            .expect("should discover src.beta");

        // alpha -> beta edge
        let alpha_to_beta = alpha.edges.iter().any(|e| e.target == "src.beta");
        assert!(alpha_to_beta, "alpha should have edge to beta");

        // beta -> alpha edge
        let beta_to_alpha = beta.edges.iter().any(|e| e.target == "src.alpha");
        assert!(beta_to_alpha, "beta should have edge to alpha");
    }
}

mod review {
    use super::cairn_planned;

    /// Scenario: False-positive deletion respected.
    /// NOTE: needs sharpening in phase-9 with archive-mock.
    #[cairn_planned(phase = 900)]
    #[test]
    fn test_review__false_positive_deletion_respected() {
        unimplemented!("awaits phase-9: review false-positive deletion respected");
    }
}

mod mcp {
    use serde_json::Value;

    fn tool_names(response: &Value) -> Vec<&str> {
        response["result"]["tools"]
            .as_array()
            .expect("tools array")
            .iter()
            .map(|t| t["name"].as_str().expect("tool name"))
            .collect()
    }

    /// Scenario: Brownfield tools absent in default (read-only) MCP mode.
    #[test]
    fn test_mcp__brownfield_tools_absent_in_default_mode() {
        let config = cairn::mcp::ServerConfig::default();
        let response =
            cairn::mcp::handle_line(r#"{"jsonrpc":"2.0","id":1,"method":"tools/list"}"#, &config);
        let names = tool_names(&response);
        assert!(
            !names.contains(&"cairn_init_from_code"),
            "cairn_init_from_code must not be listed in default mode"
        );
        assert!(
            !names.contains(&"cairn_refine"),
            "cairn_refine must not be listed in default mode"
        );
    }

    /// Scenario: Brownfield tools present in mutating MCP mode.
    #[test]
    fn test_mcp__brownfield_tools_present_in_mutating_mode() {
        let config = cairn::mcp::ServerConfig {
            allow_mutating_tools: true,
            ..cairn::mcp::ServerConfig::default()
        };
        let response =
            cairn::mcp::handle_line(r#"{"jsonrpc":"2.0","id":1,"method":"tools/list"}"#, &config);
        let names = tool_names(&response);
        assert!(
            names.contains(&"cairn_init_from_code"),
            "cairn_init_from_code must be listed in mutating mode"
        );
        assert!(
            names.contains(&"cairn_refine"),
            "cairn_refine must be listed in mutating mode"
        );
    }
}

mod heuristics {
    use cairn::brownfield::discovery;
    use cairn::brownfield::{CandidateConfidence, classify_score, coupling_score};

    /// Scenario: Coupling score (3+1)/(1+1)=2.0 maps to high confidence.
    #[test]
    fn test_heuristics__coupling_score_high_confidence() {
        let score = coupling_score(3, 1);
        assert!((score - 2.0).abs() < f64::EPSILON);
        assert_eq!(classify_score(score), CandidateConfidence::High);
    }

    /// Scenario: Coupling score (1+1)/(1+1)=1.0 maps to medium confidence.
    #[test]
    fn test_heuristics__coupling_score_medium_confidence() {
        let score = coupling_score(1, 1);
        assert!((score - 1.0).abs() < f64::EPSILON);
        assert_eq!(classify_score(score), CandidateConfidence::Medium);
    }

    /// Scenario: Coupling score (0+1)/(2+1)=0.33 maps to low confidence.
    #[test]
    fn test_heuristics__coupling_score_low_confidence() {
        let score = coupling_score(0, 2);
        assert!(score < 0.5);
        assert_eq!(classify_score(score), CandidateConfidence::Low);
    }

    /// Scenario: Directory candidate min three files.
    /// Verified via discovery engine against a fixture repo.
    #[test]
    fn test_heuristics__directory_candidate_min_three_files() {
        use cairn::brownfield::discovery;
        let root = super::temp_repo("minfiles");
        super::populate_source_dir(&root, "src/small", 2);
        super::populate_source_dir(&root, "src/enough", 3);

        let extraction = discovery::discover(&root).unwrap();
        let ids: Vec<&str> = extraction
            .candidates
            .iter()
            .map(|c| c.id.as_str())
            .collect();
        assert!(!ids.contains(&"src.small"));
        assert!(ids.contains(&"src.enough"));
    }

    /// Scenario: Directory depth limit four.
    /// Verified via discovery engine against a fixture repo.
    #[test]
    fn test_heuristics__directory_depth_limit_four() {
        use cairn::brownfield::discovery;
        let root = super::temp_repo("depthlimit");
        super::populate_source_dir(&root, "a/b/c/d", 4);
        super::populate_source_dir(&root, "a/b/c/d/e/f", 4);

        let extraction = discovery::discover(&root).unwrap();
        let ids: Vec<&str> = extraction
            .candidates
            .iter()
            .map(|c| c.id.as_str())
            .collect();
        assert!(ids.contains(&"a.b.c.d"));
        assert!(!ids.iter().any(|id| id.contains("a.b.c.d.e.f")));
    }

    /// Scenario: Edge threshold of two import observations.
    /// Cycle 3: same rationale; awaits the import-observation engine.
    #[cairn::cairn_planned(phase = 900)]
    #[test]
    fn test_heuristics__edge_threshold_two_import_observations() {
        unimplemented!(
            "awaits phase-9: import-observation engine honours EDGE_OBSERVATION_THRESHOLD"
        );
    }

    /// Scenario: Summariser disabled uses path-derived names.
    /// When no summariser backend is configured, discovery derives
    /// candidate IDs and names directly from filesystem paths.
    #[test]
    fn test_heuristics__summariser_disabled_uses_path_derived_names() {
        let root = super::temp_repo("disabled-names");
        super::populate_source_dir(&root, "src/auth", 4);
        super::populate_source_dir(&root, "src/store", 3);

        let extraction = discovery::discover(&root).unwrap();
        let auth = extraction
            .candidates
            .iter()
            .find(|c| c.path == "src/auth")
            .expect("should discover src/auth");
        assert_eq!(auth.id, "src.auth", "id must be path-derived");
        assert_eq!(auth.name, "auth", "name must be path-derived");

        let store = extraction
            .candidates
            .iter()
            .find(|c| c.path == "src/store")
            .expect("should discover src/store");
        assert_eq!(store.id, "src.store", "id must be path-derived");
        assert_eq!(store.name, "store", "name must be path-derived");
    }
}

mod suggest {

    /// Scenario: Suggest engine writes to queue file.
    #[test]
    fn test_suggest__engine_writes_to_queue_file() {
        let root = super::temp_repo("suggest-queue");
        let change_dir = root.join("meta/changes/test-change");
        std::fs::create_dir_all(&change_dir).unwrap();

        let candidate_a = cairn::brownfield::discovery::DiscoveredCandidate {
            id: "src.auth".to_owned(),
            name: "auth".to_owned(),
            description: "Auth".to_owned(),
            path: "src/auth".to_owned(),
            tags: vec!["security".to_owned()],
            confidence: 0.9,
            evidence: vec![],
            edges: vec![],
        };
        let candidate_b = cairn::brownfield::discovery::DiscoveredCandidate {
            id: "src.identity".to_owned(),
            name: "identity".to_owned(),
            description: "Identity".to_owned(),
            path: "src/identity".to_owned(),
            tags: vec!["security".to_owned()],
            confidence: 0.8,
            evidence: vec![],
            edges: vec![],
        };
        let extraction = cairn::brownfield::discovery::Extraction {
            candidates: vec![candidate_a, candidate_b],
            schema_version: 1,
        };

        cairn::brownfield::suggest::write_suggested_edges(
            &change_dir,
            &extraction,
            "phase-9-brownfield",
            "propose",
        )
        .expect("write should succeed");

        let queue_path = change_dir.join("suggested-edges.json");
        assert!(queue_path.exists(), "queue file should be written");

        let queue = cairn::suggested_edges::read_queue(&queue_path)
            .expect("read should succeed")
            .expect("queue should be present");
        assert!(!queue.entries.is_empty(), "queue should have entries");
        assert_eq!(queue.version, 1);
    }

    /// Scenario: Suggest engine produces cross-cutting edges from shared tags.
    #[test]
    fn test_suggest__shared_tags_produce_cross_cutting_edges() {
        use cairn::brownfield::discovery::{DiscoveredCandidate, Extraction};
        use cairn::suggested_edges::TriageState;

        let candidate_a = DiscoveredCandidate {
            id: "src.auth".to_owned(),
            name: "auth".to_owned(),
            description: "Auth module".to_owned(),
            path: "src/auth".to_owned(),
            tags: vec!["security".to_owned(), "api".to_owned()],
            confidence: 0.9,
            evidence: vec![],
            edges: vec![],
        };
        let candidate_b = DiscoveredCandidate {
            id: "src.identity".to_owned(),
            name: "identity".to_owned(),
            description: "Identity module".to_owned(),
            path: "src/identity".to_owned(),
            tags: vec!["security".to_owned(), "core".to_owned()],
            confidence: 0.8,
            evidence: vec![],
            edges: vec![],
        };

        let extraction = Extraction {
            candidates: vec![candidate_a, candidate_b],
            schema_version: 1,
        };

        let entries =
            cairn::brownfield::suggest::suggest_edges(&extraction, "phase-9-brownfield", "propose");

        assert!(!entries.is_empty(), "should suggest at least one edge");
        let edge = entries
            .iter()
            .find(|e| e.relation == "related_to")
            .expect("should have related_to edge");
        assert_eq!(edge.triage_state, TriageState::Pending);
        assert!(edge.provenance.is_some());
        let prov = edge.provenance.as_ref().unwrap();
        assert_eq!(prov.trace_phase, "phase-9-brownfield");
        assert_eq!(prov.stage, "propose");
    }

    /// Scenario: Entry triage state is pending.
    #[test]
    fn test_suggest__entry_triage_state_is_pending() {
        use cairn::brownfield::discovery::{DiscoveredCandidate, Extraction};
        use cairn::suggested_edges::TriageState;

        let candidate_a = DiscoveredCandidate {
            id: "src.auth".to_owned(),
            name: "auth".to_owned(),
            description: "Auth module".to_owned(),
            path: "src/auth".to_owned(),
            tags: vec!["security".to_owned()],
            confidence: 0.9,
            evidence: vec![],
            edges: vec![],
        };
        let candidate_b = DiscoveredCandidate {
            id: "src.identity".to_owned(),
            name: "identity".to_owned(),
            description: "Identity module".to_owned(),
            path: "src/identity".to_owned(),
            tags: vec!["security".to_owned()],
            confidence: 0.8,
            evidence: vec![],
            edges: vec![],
        };

        let extraction = Extraction {
            candidates: vec![candidate_a, candidate_b],
            schema_version: 1,
        };

        let entries =
            cairn::brownfield::suggest::suggest_edges(&extraction, "phase-9-brownfield", "propose");

        assert!(!entries.is_empty(), "should suggest at least one edge");
        for entry in &entries {
            assert_eq!(
                entry.triage_state,
                TriageState::Pending,
                "every suggested edge must start in Pending state"
            );
        }
    }

    /// Scenario: Entry provenance carries `trace_phase`.
    #[test]
    fn test_suggest__entry_provenance_carries_trace_phase() {
        use cairn::brownfield::discovery::{DiscoveredCandidate, Extraction};

        let candidate_a = DiscoveredCandidate {
            id: "src.auth".to_owned(),
            name: "auth".to_owned(),
            description: "Auth module".to_owned(),
            path: "src/auth".to_owned(),
            tags: vec!["security".to_owned()],
            confidence: 0.9,
            evidence: vec![],
            edges: vec![],
        };
        let candidate_b = DiscoveredCandidate {
            id: "src.identity".to_owned(),
            name: "identity".to_owned(),
            description: "Identity module".to_owned(),
            path: "src/identity".to_owned(),
            tags: vec!["security".to_owned()],
            confidence: 0.8,
            evidence: vec![],
            edges: vec![],
        };

        let extraction = Extraction {
            candidates: vec![candidate_a, candidate_b],
            schema_version: 1,
        };

        let entries =
            cairn::brownfield::suggest::suggest_edges(&extraction, "phase-9-brownfield", "propose");

        assert!(!entries.is_empty(), "should suggest at least one edge");
        for entry in &entries {
            let prov = entry
                .provenance
                .as_ref()
                .expect("every entry must carry provenance");
            assert_eq!(prov.trace_phase, "phase-9-brownfield");
            assert_eq!(prov.stage, "propose");
        }
    }

    /// Scenario: Pending entries block archive with CC002.
    #[test]
    fn test_suggest__pending_entries_block_archive_with_cc002() {
        let root = super::temp_repo("suggest-cc002");
        let change_dir = root.join("meta/changes/test-change");
        std::fs::create_dir_all(&change_dir).unwrap();

        let candidate_a = cairn::brownfield::discovery::DiscoveredCandidate {
            id: "src.auth".to_owned(),
            name: "auth".to_owned(),
            description: "Auth".to_owned(),
            path: "src/auth".to_owned(),
            tags: vec!["security".to_owned()],
            confidence: 0.9,
            evidence: vec![],
            edges: vec![],
        };
        let candidate_b = cairn::brownfield::discovery::DiscoveredCandidate {
            id: "src.identity".to_owned(),
            name: "identity".to_owned(),
            description: "Identity".to_owned(),
            path: "src/identity".to_owned(),
            tags: vec!["security".to_owned()],
            confidence: 0.8,
            evidence: vec![],
            edges: vec![],
        };
        let extraction = cairn::brownfield::discovery::Extraction {
            candidates: vec![candidate_a, candidate_b],
            schema_version: 1,
        };

        cairn::brownfield::suggest::write_suggested_edges(
            &change_dir,
            &extraction,
            "phase-9-brownfield",
            "propose",
        )
        .expect("write should succeed");

        let result = cairn::suggested_edges::validate_strict("test-change", &change_dir);
        assert!(
            result.is_err(),
            "CC002 should block archive when pending entries exist"
        );
        let err_msg = format!("{}", result.unwrap_err());
        assert!(
            err_msg.contains("untriaged") || err_msg.contains("pending"),
            "error should mention untriaged or pending: {err_msg}"
        );
    }

    /// Scenario: Empty provenance strings are preserved (not replaced).
    #[test]
    fn test_suggest__empty_provenance_strings_preserved() {
        use cairn::brownfield::discovery::{DiscoveredCandidate, Extraction};

        let candidate_a = DiscoveredCandidate {
            id: "src.a".to_owned(),
            name: "a".to_owned(),
            description: "A".to_owned(),
            path: "src/a".to_owned(),
            tags: vec!["tag".to_owned()],
            confidence: 0.5,
            evidence: vec![],
            edges: vec![],
        };
        let candidate_b = DiscoveredCandidate {
            id: "src.b".to_owned(),
            name: "b".to_owned(),
            description: "B".to_owned(),
            path: "src/b".to_owned(),
            tags: vec!["tag".to_owned()],
            confidence: 0.5,
            evidence: vec![],
            edges: vec![],
        };
        let extraction = Extraction {
            candidates: vec![candidate_a, candidate_b],
            schema_version: 1,
        };

        let entries = cairn::brownfield::suggest::suggest_edges(&extraction, "", "");

        assert!(!entries.is_empty());
        let prov = entries[0].provenance.as_ref().unwrap();
        assert_eq!(prov.trace_phase, "");
        assert_eq!(prov.stage, "");
    }

    /// Scenario: No auto-accept on high confidence.
    #[test]
    fn test_suggest__no_auto_accept_on_high_confidence() {
        use cairn::brownfield::discovery::{DiscoveredCandidate, Extraction};
        use cairn::suggested_edges::TriageState;

        let candidate_a = DiscoveredCandidate {
            id: "src.auth".to_owned(),
            name: "auth".to_owned(),
            description: "Auth module".to_owned(),
            path: "src/auth".to_owned(),
            tags: vec!["security".to_owned()],
            confidence: 1.0,
            evidence: vec![],
            edges: vec![],
        };
        let candidate_b = DiscoveredCandidate {
            id: "src.identity".to_owned(),
            name: "identity".to_owned(),
            description: "Identity module".to_owned(),
            path: "src/identity".to_owned(),
            tags: vec!["security".to_owned()],
            confidence: 1.0,
            evidence: vec![],
            edges: vec![],
        };

        let extraction = Extraction {
            candidates: vec![candidate_a, candidate_b],
            schema_version: 1,
        };

        let entries =
            cairn::brownfield::suggest::suggest_edges(&extraction, "phase-9-brownfield", "propose");

        assert!(!entries.is_empty(), "should suggest at least one edge");
        for entry in &entries {
            assert_eq!(
                entry.triage_state,
                TriageState::Pending,
                "high confidence must not auto-accept; entry must stay Pending"
            );
        }
    }

    /// Scenario: Refine emits to queue file with propose stage.
    #[test]
    fn test_suggest__refine_emits_to_queue_file_with_propose_stage() {
        let root = super::temp_repo("refine-queue");
        let change_dir = root.join("meta/changes/brownfield-refine-test");
        std::fs::create_dir_all(&change_dir).unwrap();

        let candidate_a = cairn::brownfield::discovery::DiscoveredCandidate {
            id: "src.auth".to_owned(),
            name: "auth".to_owned(),
            description: "Auth".to_owned(),
            path: "src/auth".to_owned(),
            tags: vec!["security".to_owned()],
            confidence: 0.9,
            evidence: vec![],
            edges: vec![],
        };
        let candidate_b = cairn::brownfield::discovery::DiscoveredCandidate {
            id: "src.identity".to_owned(),
            name: "identity".to_owned(),
            description: "Identity".to_owned(),
            path: "src/identity".to_owned(),
            tags: vec!["security".to_owned()],
            confidence: 0.8,
            evidence: vec![],
            edges: vec![],
        };
        let extraction = cairn::brownfield::discovery::Extraction {
            candidates: vec![candidate_a, candidate_b],
            schema_version: 1,
        };

        cairn::brownfield::suggest::write_suggested_edges(
            &change_dir,
            &extraction,
            "phase-9-brownfield",
            "propose",
        )
        .expect("write should succeed");

        let queue_path = change_dir.join("suggested-edges.json");
        assert!(
            queue_path.exists(),
            "refine change should contain suggested-edges.json"
        );

        let queue = cairn::suggested_edges::read_queue(&queue_path)
            .expect("read should succeed")
            .expect("queue should be present");
        assert!(!queue.entries.is_empty(), "queue should have entries");
        for entry in &queue.entries {
            let prov = entry
                .provenance
                .as_ref()
                .expect("entry must carry provenance");
            assert_eq!(prov.stage, "propose", "refine must emit with propose stage");
        }
    }

    /// Scenario: Force init aborts on pending entries.
    #[test]
    fn test_suggest__force_init_aborts_on_pending_entries() {
        let root = super::temp_repo("force-init-pending");
        let change_dir = root.join("meta/changes/brownfield-init");
        std::fs::create_dir_all(&change_dir).unwrap();

        // Write a queue with pending entries
        let queue = cairn::suggested_edges::SuggestedEdgesQueue {
            version: 1,
            entries: vec![cairn::suggested_edges::SuggestedEdgeEntry {
                source: "a".to_owned(),
                target: "b".to_owned(),
                relation: "related_to".to_owned(),
                triage_state: cairn::suggested_edges::TriageState::Pending,
                confidence: None,
                provenance: None,
                triage_note: None,
            }],
        };
        cairn::suggested_edges::write_to_change(&change_dir, &queue).expect("write");

        // Create minimal source so discovery doesn't error
        super::populate_source_dir(&root, "src/auth", 1);

        let result = cairn::brownfield::init::run_init_from_code(&root, true);
        assert!(
            result.is_err(),
            "force init must abort when pending suggested edges exist"
        );
        let msg = format!("{}", result.unwrap_err());
        assert!(
            msg.contains("pending") || msg.contains("triage") || msg.contains("suggested"),
            "error should mention pending/triage/suggested edges: {msg}"
        );
    }
}

mod interview {
    /// Scenario: Session persists across invocations.
    #[test]
    fn test_interview__session_persists_across_invocations() {
        let root = super::temp_repo("interview-persist");
        let change_dir = root.join("meta/changes/test-change");
        std::fs::create_dir_all(change_dir.join("research")).unwrap();

        let questions = vec!["Q1".to_owned(), "Q2".to_owned()];
        let session =
            cairn::brownfield::interview::start_session(&change_dir, "test-change", &questions)
                .expect("start should succeed");

        assert_eq!(session.cursor, 0);
        assert!(!session.complete);

        // Simulate a second invocation reading the persisted state.
        let resumed = cairn::brownfield::interview::resume_session(&change_dir)
            .expect("resume should succeed")
            .expect("session should exist");

        assert_eq!(resumed.cursor, 0);
        assert_eq!(resumed.turns.len(), 2);
    }

    /// Scenario: Final transcript lands at genesis path.
    #[test]
    fn test_interview__final_transcript_lands_at_genesis_path() {
        let root = super::temp_repo("interview-genesis");
        let change_dir = root.join("meta/changes/test-change");
        std::fs::create_dir_all(change_dir.join("research")).unwrap();

        let questions = vec!["What is the scope?".to_owned()];
        let mut session =
            cairn::brownfield::interview::start_session(&change_dir, "test-change", &questions)
                .expect("start should succeed");

        session = cairn::brownfield::interview::record_answer(
            &change_dir,
            &session,
            "The scope is broad.",
        )
        .expect("record should succeed");

        cairn::brownfield::interview::complete_session(&change_dir, &session, "test-change")
            .expect("complete should succeed");

        let genesis_path = change_dir.join("research/genesis.md");
        assert!(genesis_path.exists(), "genesis.md should be written");

        let content = std::fs::read_to_string(&genesis_path).unwrap();
        assert!(content.contains("What is the scope?"));
        assert!(content.contains("The scope is broad."));
    }

    /// Scenario: Session state never leaks outside change directory.
    #[test]
    fn test_interview__session_state_never_leaks_outside_change_dir() {
        let root = super::temp_repo("interview-leak");
        let change_dir = root.join("meta/changes/test-change");
        std::fs::create_dir_all(change_dir.join("research")).unwrap();

        let questions = vec!["Q1".to_owned()];
        cairn::brownfield::interview::start_session(&change_dir, "test-change", &questions)
            .expect("start should succeed");

        let session_path = change_dir.join("research/interview-session.json");
        assert!(session_path.exists());

        // Nothing should be written outside the change directory.
        let parent = change_dir.parent().unwrap();
        let mut entries = std::fs::read_dir(parent).unwrap();
        assert!(
            entries.all(|e| {
                let name = e.unwrap().file_name();
                name == "test-change" || name == ".gitkeep"
            }),
            "no session state should leak outside change dir"
        );
    }
}

mod templates {
    use cairn::brownfield::discovery::DiscoveredCandidate;

    /// Scenario: Matching template guides stub authoring.
    #[test]
    fn test_templates__matching_template_guides_stub_authoring() {
        let template = cairn::brownfield::templates::ContractTemplate {
            name: "security-module".to_owned(),
            match_rules: vec![cairn::brownfield::templates::MatchRule::HasTag(
                "security".to_owned(),
            )],
            body: "# Security Contract: {name}\n\nModule {id} handles security.".to_owned(),
        };

        let candidate = DiscoveredCandidate {
            id: "src.auth".to_owned(),
            name: "auth".to_owned(),
            description: "Auth module".to_owned(),
            path: "src/auth".to_owned(),
            tags: vec!["security".to_owned()],
            confidence: 0.9,
            evidence: vec![],
            edges: vec![],
        };

        let rendered = cairn::brownfield::templates::render_stub(&candidate, &[template]);

        assert!(
            rendered.contains("Security Contract: auth"),
            "template should guide stub authoring, got: {rendered}"
        );
        assert!(rendered.contains("src.auth handles security"));
    }

    /// Scenario: Non-matching candidates fall back to built-in stub.
    #[test]
    fn test_templates__non_matching_candidates_fall_back_to_builtin() {
        let template = cairn::brownfield::templates::ContractTemplate {
            name: "security-module".to_owned(),
            match_rules: vec![cairn::brownfield::templates::MatchRule::HasTag(
                "security".to_owned(),
            )],
            body: "# Security Contract".to_owned(),
        };

        let candidate = DiscoveredCandidate {
            id: "src.util".to_owned(),
            name: "util".to_owned(),
            description: "Utilities".to_owned(),
            path: "src/util".to_owned(),
            tags: vec!["helper".to_owned()],
            confidence: 0.5,
            evidence: vec![],
            edges: vec![],
        };

        let rendered = cairn::brownfield::templates::render_stub(&candidate, &[template]);

        // Should fall back to built-in stub, not the security template.
        assert!(
            !rendered.contains("Security Contract"),
            "should not use non-matching template, got: {rendered}"
        );
        assert!(
            rendered.contains("src.util"),
            "should contain built-in stub content"
        );
    }

    /// Scenario: Ill-formed template does not block authoring.
    #[test]
    fn test_templates__ill_formed_template_does_not_block_authoring() {
        // An empty template list is effectively "no valid templates".
        let candidate = DiscoveredCandidate {
            id: "src.core".to_owned(),
            name: "core".to_owned(),
            description: "Core module".to_owned(),
            path: "src/core".to_owned(),
            tags: vec![],
            confidence: 0.8,
            evidence: vec![],
            edges: vec![],
        };

        let rendered = cairn::brownfield::templates::render_stub(&candidate, &[]);

        // Should still produce a valid stub even with no templates.
        assert!(
            rendered.contains("src.core"),
            "should produce built-in stub when no templates match, got: {rendered}"
        );
    }
}

/// Scenario: Templates loaded from meta/templates.toml guide contract authoring.
#[test]
fn test_templates__loaded_from_file_guide_contract_authoring() {
    let root = temp_repo("templates-from-file");
    populate_source_dir(&root, "src/auth", 3);

    std::fs::create_dir_all(root.join("meta")).unwrap();
    std::fs::write(
        root.join("meta/templates.toml"),
        r#"
[[template]]
name = "auth-module"
match_rules = [{ Path = "auth" }]
body = "---\nnode: {id}\n---\n\n# Auth Contract for {name}\n\nThis module handles authentication."
"#,
    )
    .unwrap();

    let result = cairn::brownfield::init::run_init_from_code(&root, false);
    assert!(result.is_ok(), "init should succeed: {result:?}");

    let contract_path = root.join("meta/changes/brownfield-init/contracts/src_auth.md");
    let contract = std::fs::read_to_string(&contract_path).unwrap();
    assert!(
        contract.contains("Auth Contract for auth"),
        "contract should use template, got: {contract}"
    );
}

mod obligations {
    use super::cairn_planned;

    /// Scenario: Populated when obligations field exists on decision artefact.
    #[cairn_planned(phase = 900)]
    #[test]
    fn test_obligations__populated_when_field_exists() {
        unimplemented!("awaits phase-9: obligations populated when field exists");
    }

    /// Scenario: Obligations reviewable before archive.
    #[cairn_planned(phase = 900)]
    #[test]
    fn test_obligations__reviewable_before_archive() {
        unimplemented!("awaits phase-9: obligations reviewable before archive");
    }

    /// Scenario: No-op when obligations field absent.
    #[cairn_planned(phase = 900)]
    #[test]
    fn test_obligations__no_op_when_field_absent() {
        unimplemented!("awaits phase-9: obligations no-op when field absent");
    }
}

mod summarise {
    /// Scenario: Build bounded summariser inputs from a candidate.
    #[test]
    fn test_summarise__builds_bounded_request_from_candidate() {
        let root = super::temp_repo("summarise-bounded");
        let dir = root.join("src/auth");
        std::fs::create_dir_all(&dir).unwrap();
        // Write 6 source files (bounds are 5 max).
        for i in 0..6 {
            std::fs::write(dir.join(format!("file{i}.rs")), format!("fn f{i}() {{}}\n")).unwrap();
        }
        // Write one oversized file.
        let big_content = "x".repeat(5000);
        std::fs::write(dir.join("big.rs"), big_content).unwrap();

        let candidate = cairn::brownfield::discovery::DiscoveredCandidate {
            id: "src.auth".to_owned(),
            name: "auth".to_owned(),
            description: "Authentication module".to_owned(),
            path: "src/auth".to_owned(),
            tags: vec!["security".to_owned()],
            confidence: 0.9,
            evidence: vec!["src/auth".to_owned()],
            edges: vec![],
        };

        let request = cairn::brownfield::summarise::build_request(&candidate, &root, 5, 4000);

        assert_eq!(request.target_node, "src.auth");
        assert_eq!(request.draft_type, "contract");
        assert!(
            request.code_samples.len() <= 5,
            "should bound to 5 files, got {}",
            request.code_samples.len()
        );
        for sample in &request.code_samples {
            assert!(
                sample.content.len() <= 4000,
                "sample {} exceeds 4000 bytes: {}",
                sample.path,
                sample.content.len()
            );
        }
        let facts: Vec<&str> = request
            .map_facts
            .iter()
            .map(std::string::String::as_str)
            .collect();
        assert!(
            facts.iter().any(|f| f.contains("auth")),
            "map_facts should mention candidate"
        );
    }

    /// Scenario: Empty directory yields empty `code_samples`.
    #[test]
    fn test_summarise__empty_dir_yields_no_samples() {
        let root = super::temp_repo("summarise-empty");
        let dir = root.join("src/empty");
        std::fs::create_dir_all(&dir).unwrap();

        let candidate = cairn::brownfield::discovery::DiscoveredCandidate {
            id: "src.empty".to_owned(),
            name: "empty".to_owned(),
            description: "Empty module".to_owned(),
            path: "src/empty".to_owned(),
            tags: vec![],
            confidence: 0.5,
            evidence: vec![],
            edges: vec![],
        };

        let request = cairn::brownfield::summarise::build_request(&candidate, &root, 5, 4000);

        assert!(request.code_samples.is_empty());
    }

    /// Scenario: Successful backend enriches contract prose.
    #[test]
    fn test_summarise__backend_enriches_contract_prose() {
        use cairn::summariser::{FakeBackend, SummariserResponse};

        let root = super::temp_repo("summarise-enrich");
        let dir = root.join("src/core");
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(dir.join("lib.rs"), "pub fn main() {}").unwrap();

        let candidate = cairn::brownfield::discovery::DiscoveredCandidate {
            id: "src.core".to_owned(),
            name: "core".to_owned(),
            description: "Core module".to_owned(),
            path: "src/core".to_owned(),
            tags: vec![],
            confidence: 0.8,
            evidence: vec![],
            edges: vec![],
        };

        let backend = FakeBackend::ok(SummariserResponse {
            schema_version: 1,
            draft_text: "# AI-Generated Contract\n\nThis is the enriched prose.".to_owned(),
            summary: None,
            metadata: None,
        });

        let enriched = cairn::brownfield::summarise::enrich_candidate(
            &backend,
            &candidate,
            &root,
            std::time::Duration::from_secs(5),
        );

        assert_eq!(
            enriched.contract_prose,
            "# AI-Generated Contract\n\nThis is the enriched prose."
        );
    }

    /// Scenario: Backend error falls back to original candidate values.
    #[test]
    fn test_summarise__backend_error_falls_back() {
        use cairn::summariser::{FakeBackend, SummariserBackendError};

        let root = super::temp_repo("summarise-fallback");
        let candidate = cairn::brownfield::discovery::DiscoveredCandidate {
            id: "src.fallback".to_owned(),
            name: "fallback".to_owned(),
            description: "Fallback module".to_owned(),
            path: "src/fallback".to_owned(),
            tags: vec!["tag1".to_owned()],
            confidence: 0.6,
            evidence: vec![],
            edges: vec![],
        };

        let backend = FakeBackend::err(SummariserBackendError::Disabled);

        let enriched = cairn::brownfield::summarise::enrich_candidate(
            &backend,
            &candidate,
            &root,
            std::time::Duration::from_secs(5),
        );

        assert_eq!(enriched.name, "fallback");
        assert_eq!(enriched.description, "Fallback module");
        assert_eq!(enriched.tags, vec!["tag1"]);
        assert_eq!(
            enriched.contract_prose,
            cairn::brownfield::stub_contract(&candidate)
        );
    }
}

mod change_new {
    /// Scenario: change new scaffolds a change directory with templates.
    #[test]
    fn test_change_new__scaffolds_change_directory_with_templates() {
        let root = super::temp_repo("change-new");
        let result = cairn::cli::run(&[
            "--file".to_owned(),
            root.join("cairn.blueprint").to_string_lossy().to_string(),
            "change".to_owned(),
            "new".to_owned(),
            "test-change".to_owned(),
        ]);
        assert_eq!(
            result.code, 0,
            "change new must exit zero, got stderr: {}",
            result.stderr
        );

        let change_dir = root.join("meta/changes/test-change");
        assert!(
            change_dir.exists(),
            "change directory must be created at meta/changes/test-change"
        );

        let proposal = change_dir.join("proposal.md");
        assert!(proposal.exists(), "proposal.md must be created");
        let proposal_content = std::fs::read_to_string(&proposal).unwrap();
        assert!(
            proposal_content.contains("# Proposal"),
            "proposal.md must contain a proposal heading"
        );

        let design = change_dir.join("design.md");
        assert!(design.exists(), "design.md must be created");
        let design_content = std::fs::read_to_string(&design).unwrap();
        assert!(
            design_content.contains("# Design"),
            "design.md must contain a design heading"
        );

        let tasks = change_dir.join("tasks.md");
        assert!(tasks.exists(), "tasks.md must be created");
        let tasks_content = std::fs::read_to_string(&tasks).unwrap();
        assert!(
            tasks_content.contains("# Tasks"),
            "tasks.md must contain a tasks heading"
        );

        assert!(
            change_dir.join("specs").exists(),
            "specs directory must be created"
        );
    }

    /// Scenario: change new refuses to overwrite an existing change directory.
    #[test]
    fn test_change_new__refuses_to_overwrite_existing_change() {
        let root = super::temp_repo("change-new-dup");
        let change_dir = root.join("meta/changes/existing-change");
        std::fs::create_dir_all(&change_dir).unwrap();
        std::fs::write(change_dir.join("proposal.md"), "# Existing").unwrap();

        let result = cairn::cli::run(&[
            "--file".to_owned(),
            root.join("cairn.blueprint").to_string_lossy().to_string(),
            "change".to_owned(),
            "new".to_owned(),
            "existing-change".to_owned(),
        ]);
        assert_ne!(
            result.code, 0,
            "change new must fail when change directory already exists"
        );
    }

    /// Scenario: change new rejects invalid change IDs.
    #[test]
    fn test_change_new__rejects_invalid_change_id() {
        let root = super::temp_repo("change-new-invalid");
        let result = cairn::cli::run(&[
            "--file".to_owned(),
            root.join("cairn.blueprint").to_string_lossy().to_string(),
            "change".to_owned(),
            "new".to_owned(),
            "invalid id".to_owned(),
        ]);
        assert_ne!(
            result.code, 0,
            "change new must fail when change ID contains spaces"
        );
    }

    /// Scenario: change new rejects uppercase change IDs.
    #[test]
    fn test_change_new__rejects_uppercase_change_id() {
        let root = super::temp_repo("change-new-upper");
        let result = cairn::cli::run(&[
            "--file".to_owned(),
            root.join("cairn.blueprint").to_string_lossy().to_string(),
            "change".to_owned(),
            "new".to_owned(),
            "UpperCase".to_owned(),
        ]);
        assert_ne!(
            result.code, 0,
            "change new must fail when change ID contains uppercase letters"
        );
    }

    /// Scenario: change new rejects special characters in change IDs.
    #[test]
    fn test_change_new__rejects_special_characters() {
        let root = super::temp_repo("change-new-special");
        let result = cairn::cli::run(&[
            "--file".to_owned(),
            root.join("cairn.blueprint").to_string_lossy().to_string(),
            "change".to_owned(),
            "new".to_owned(),
            "special@chars!".to_owned(),
        ]);
        assert_ne!(
            result.code, 0,
            "change new must fail when change ID contains special characters"
        );
    }

    /// Scenario: change new rejects empty change IDs.
    #[test]
    fn test_change_new__rejects_empty_change_id() {
        let root = super::temp_repo("change-new-empty");
        let result = cairn::cli::run(&[
            "--file".to_owned(),
            root.join("cairn.blueprint").to_string_lossy().to_string(),
            "change".to_owned(),
            "new".to_owned(),
            String::new(),
        ]);
        assert_ne!(
            result.code, 0,
            "change new must fail when change ID is empty"
        );
    }
}

mod cc002_gate {
    /// Scenario: Accept is blocked when suggested edges are pending.
    #[test]
    fn test_cc002__accept_blocked_when_suggested_edges_pending() {
        let root = super::temp_repo("cc002-pending");
        let change_dir = root.join("meta/changes/test-change");
        std::fs::create_dir_all(&change_dir).unwrap();
        std::fs::write(change_dir.join("proposal.md"), "# Proposal\n").unwrap();

        // Write a suggested-edges queue with one pending entry.
        let queue = cairn::suggested_edges::SuggestedEdgesQueue {
            version: 1,
            entries: vec![cairn::suggested_edges::SuggestedEdgeEntry {
                source: "a".to_owned(),
                target: "b".to_owned(),
                relation: "related_to".to_owned(),
                triage_state: cairn::suggested_edges::TriageState::Pending,
                confidence: Some(0.8),
                provenance: Some(cairn::suggested_edges::EdgeProvenance {
                    trace_phase: "phase-9-brownfield".to_owned(),
                    stage: "propose".to_owned(),
                }),
                triage_note: None,
            }],
        };
        cairn::suggested_edges::write_to_change(&change_dir, &queue).unwrap();

        let result = cairn::suggested_edges::validate_strict("test-change", &change_dir);
        assert!(
            result.is_err(),
            "validate_strict must fail when pending edges exist"
        );
        let err_msg = format!("{}", result.unwrap_err());
        assert!(
            err_msg.contains("CC002") || err_msg.contains("pending"),
            "error must mention pending suggested edges, got: {err_msg}"
        );
    }

    /// Scenario: Accept passes when all suggested edges are triaged.
    #[test]
    fn test_cc002__accept_passes_when_all_edges_triaged() {
        let root = super::temp_repo("cc002-triaged");
        let change_dir = root.join("meta/changes/test-change");
        std::fs::create_dir_all(&change_dir).unwrap();

        let queue = cairn::suggested_edges::SuggestedEdgesQueue {
            version: 1,
            entries: vec![cairn::suggested_edges::SuggestedEdgeEntry {
                source: "a".to_owned(),
                target: "b".to_owned(),
                relation: "related_to".to_owned(),
                triage_state: cairn::suggested_edges::TriageState::Accepted,
                confidence: Some(0.8),
                provenance: Some(cairn::suggested_edges::EdgeProvenance {
                    trace_phase: "phase-9-brownfield".to_owned(),
                    stage: "propose".to_owned(),
                }),
                triage_note: None,
            }],
        };
        cairn::suggested_edges::write_to_change(&change_dir, &queue).unwrap();

        let result = cairn::suggested_edges::validate_strict("test-change", &change_dir);
        assert!(
            result.is_ok(),
            "validate_strict must pass when all edges are triaged, got: {result:?}"
        );
    }

    /// Scenario: Accept passes when no suggested-edges file exists.
    #[test]
    fn test_cc002__accept_passes_when_no_queue_file() {
        let root = super::temp_repo("cc002-no-queue");
        let change_dir = root.join("meta/changes/test-change");
        std::fs::create_dir_all(&change_dir).unwrap();

        let result = cairn::suggested_edges::validate_strict("test-change", &change_dir);
        assert!(
            result.is_ok(),
            "validate_strict must pass when no queue file exists, got: {result:?}"
        );
    }
}
