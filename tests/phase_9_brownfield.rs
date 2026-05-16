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
//! `#[cflx_planned(phase = 900)]` and stay skipped under `cargo test`;
//! they fail with `unimplemented!` under `cargo test -- --ignored`.
//! Phase 9 removes `#[cflx_planned]` group-by-group as code lands.

use cairn::cflx_planned;

// Fixture helpers. Phase 9 supplies real implementations; the pre-phase
// keeps thin placeholders so test bodies referencing them compile.
#[allow(dead_code)]
fn fixture_repo_without_blueprint() {
    unimplemented!("awaits phase-9: fixture_repo_without_blueprint");
}

#[allow(dead_code)]
fn fixture_repo_with_blueprint() {
    unimplemented!("awaits phase-9: fixture_repo_with_blueprint");
}

#[allow(dead_code)]
fn fixture_repo_with_brownfield_change() {
    unimplemented!("awaits phase-9: fixture_repo_with_brownfield_change");
}

#[allow(dead_code)]
fn fixture_repo_with_pending_suggested_edges() {
    unimplemented!("awaits phase-9: fixture_repo_with_pending_suggested_edges");
}

#[allow(dead_code)]
fn fixture_change_with_partial_interview_state() {
    unimplemented!("awaits phase-9: fixture_change_with_partial_interview_state");
}

#[allow(dead_code)]
fn fixture_project_config_with_contract_template() {
    unimplemented!("awaits phase-9: fixture_project_config_with_contract_template");
}

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
        let change_dir = root.join("openspec/changes").join(&change_id);
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
        let change_dir = root.join("openspec/changes/brownfield-init");
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
        let result = write_change(&root, "test-change", &extraction);
        assert!(result.is_ok());
        let dir = root.join("openspec/changes/test-change");
        assert!(dir.join("proposal.md").exists());
        assert!(dir.join("blueprint.delta").exists());
        assert!(dir.join("contracts/src_core.md").exists());
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
        let change_dir = root.join("openspec/changes").join(&change_id);
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
        assert!(root.join("openspec/changes").join(&first).exists());
        assert!(root.join("openspec/changes").join(&second).exists());
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

mod review {
    use super::cflx_planned;

    /// Scenario: False-positive deletion respected.
    /// NOTE: needs sharpening in phase-9 with archive-mock.
    #[cflx_planned(phase = 900)]
    #[test]
    fn test_review__false_positive_deletion_respected() {
        unimplemented!("awaits phase-9: review false-positive deletion respected");
    }
}

mod mcp {
    use super::cflx_planned;

    /// Scenario: Brownfield tools absent in default (read-only) MCP mode.
    #[cflx_planned(phase = 900)]
    #[test]
    fn test_mcp__brownfield_tools_absent_in_default_mode() {
        unimplemented!("awaits phase-9: mcp brownfield tools absent in default mode");
    }

    /// Scenario: Brownfield tools present in mutating MCP mode.
    #[cflx_planned(phase = 900)]
    #[test]
    fn test_mcp__brownfield_tools_present_in_mutating_mode() {
        unimplemented!("awaits phase-9: mcp brownfield tools present in mutating mode");
    }
}

mod heuristics {
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
    #[cairn::cflx_planned(phase = 900)]
    #[test]
    fn test_heuristics__edge_threshold_two_import_observations() {
        unimplemented!(
            "awaits phase-9: import-observation engine honours EDGE_OBSERVATION_THRESHOLD"
        );
    }

    /// Scenario: Summariser disabled uses path-derived names.
    /// Cycle 3: reverted to `#[cflx_planned]` because the prior
    /// assertion only confirmed a hand-constructed Candidate had a
    /// non-empty id, not that the disabled-summariser path actually
    /// derives names from filesystem paths.
    #[cairn::cflx_planned(phase = 900)]
    #[test]
    fn test_heuristics__summariser_disabled_uses_path_derived_names() {
        unimplemented!(
            "awaits phase-9: disabled-summariser fallback constructor derives id from path"
        );
    }
}

mod suggest {
    use super::cflx_planned;

    /// Scenario: Suggest engine writes to queue file.
    #[cflx_planned(phase = 900)]
    #[test]
    fn test_suggest__engine_writes_to_queue_file() {
        unimplemented!("awaits phase-9: suggest engine writes to queue file");
    }

    /// Scenario: Entry triage state is pending.
    #[cflx_planned(phase = 900)]
    #[test]
    fn test_suggest__entry_triage_state_is_pending() {
        unimplemented!("awaits phase-9: suggest entry triage state is pending");
    }

    /// Scenario: Entry provenance carries `trace_phase`.
    #[cflx_planned(phase = 900)]
    #[test]
    fn test_suggest__entry_provenance_carries_trace_phase() {
        unimplemented!("awaits phase-9: suggest entry provenance carries trace_phase");
    }

    /// Scenario: Pending entries block archive with CC002.
    #[cflx_planned(phase = 900)]
    #[test]
    fn test_suggest__pending_entries_block_archive_with_cc002() {
        unimplemented!("awaits phase-9: suggest pending entries block archive with CC002");
    }

    /// Scenario: No auto-accept on high confidence.
    #[cflx_planned(phase = 900)]
    #[test]
    fn test_suggest__no_auto_accept_on_high_confidence() {
        unimplemented!("awaits phase-9: suggest no auto-accept on high confidence");
    }

    /// Scenario: Refine emits to queue file with propose stage.
    #[cflx_planned(phase = 900)]
    #[test]
    fn test_suggest__refine_emits_to_queue_file_with_propose_stage() {
        unimplemented!("awaits phase-9: suggest refine emits to queue file with propose stage");
    }

    /// Scenario: Force init aborts on pending entries.
    #[cflx_planned(phase = 900)]
    #[test]
    fn test_suggest__force_init_aborts_on_pending_entries() {
        unimplemented!("awaits phase-9: suggest force-init aborts on pending entries");
    }
}

mod interview {
    use super::cflx_planned;

    /// Scenario: Session persists across invocations.
    #[cflx_planned(phase = 900)]
    #[test]
    fn test_interview__session_persists_across_invocations() {
        unimplemented!("awaits phase-9: interview session persists across invocations");
    }

    /// Scenario: Final transcript lands at genesis path.
    #[cflx_planned(phase = 900)]
    #[test]
    fn test_interview__final_transcript_lands_at_genesis_path() {
        unimplemented!("awaits phase-9: interview final transcript lands at genesis path");
    }

    /// Scenario: Session state never leaks outside change directory.
    #[cflx_planned(phase = 900)]
    #[test]
    fn test_interview__session_state_never_leaks_outside_change_dir() {
        unimplemented!("awaits phase-9: interview session state never leaks outside change dir");
    }
}

mod templates {
    use super::cflx_planned;

    /// Scenario: Matching template guides stub authoring.
    #[cflx_planned(phase = 900)]
    #[test]
    fn test_templates__matching_template_guides_stub_authoring() {
        unimplemented!("awaits phase-9: templates matching template guides stub authoring");
    }

    /// Scenario: Non-matching candidates fall back to built-in stub.
    #[cflx_planned(phase = 900)]
    #[test]
    fn test_templates__non_matching_candidates_fall_back_to_builtin() {
        unimplemented!("awaits phase-9: templates non-matching candidates fall back to built-in");
    }

    /// Scenario: Ill-formed template does not block authoring.
    #[cflx_planned(phase = 900)]
    #[test]
    fn test_templates__ill_formed_template_does_not_block_authoring() {
        unimplemented!("awaits phase-9: templates ill-formed template does not block authoring");
    }
}

mod obligations {
    use super::cflx_planned;

    /// Scenario: Populated when obligations field exists on decision artefact.
    #[cflx_planned(phase = 900)]
    #[test]
    fn test_obligations__populated_when_field_exists() {
        unimplemented!("awaits phase-9: obligations populated when field exists");
    }

    /// Scenario: Obligations reviewable before archive.
    #[cflx_planned(phase = 900)]
    #[test]
    fn test_obligations__reviewable_before_archive() {
        unimplemented!("awaits phase-9: obligations reviewable before archive");
    }

    /// Scenario: No-op when obligations field absent.
    #[cflx_planned(phase = 900)]
    #[test]
    fn test_obligations__no_op_when_field_absent() {
        unimplemented!("awaits phase-9: obligations no-op when field absent");
    }
}
