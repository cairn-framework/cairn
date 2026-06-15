use super::*;

fn ignores(patterns: &[&str]) -> Vec<String> {
    patterns
        .iter()
        .map(|p| p.trim().trim_matches('/').to_owned())
        .collect()
}

// ── is_ignored ────────────────────────────────────────────────────────────

#[test]
fn test_exact_match() {
    assert!(is_ignored("foo.txt", &ignores(&["foo.txt"])));
}

#[test]
fn test_directory_prefix_match() {
    // "src" must match "src/main.rs" and all files under src/.
    assert!(is_ignored("src/main.rs", &ignores(&["src"])));
    assert!(is_ignored("src/deep/nested.rs", &ignores(&["src"])));
}

#[test]
fn test_suffix_match() {
    // "node_modules" must match "lib/node_modules" — useful for
    // mono-repos where the pattern appears at any depth.
    assert!(is_ignored("lib/node_modules", &ignores(&["node_modules"])));
    assert!(is_ignored(
        "packages/app/node_modules",
        &ignores(&["node_modules"])
    ));
}

#[test]
fn test_glob_extension_match() {
    assert!(is_ignored("dist/bundle.js", &ignores(&["*.js"])));
    assert!(is_ignored("src/image.png", &ignores(&["*.png"])));
}

#[test]
fn test_glob_does_not_match_wrong_extension() {
    // "*.js" must not match ".js.map" files — suffix must be exact.
    assert!(!is_ignored("file.js.map", &ignores(&["*.js"])));
}

#[test]
fn test_no_match_returns_false() {
    assert!(!is_ignored("src/main.rs", &ignores(&["dist"])));
    assert!(!is_ignored("src/main.rs", &ignores(&["*.js"])));
}

#[test]
fn test_partial_prefix_not_matched() {
    // "src" must not match "srcmap/main.rs" — prefix check requires trailing slash.
    assert!(!is_ignored("srcmap/main.rs", &ignores(&["src"])));
}

#[test]
fn test_empty_pattern_is_skipped() {
    assert!(!is_ignored("foo.txt", &ignores(&[""])));
    // slash-only pattern trims to empty and is also skipped
    assert!(!is_ignored("foo.txt", &ignores(&["/"])));
}

#[test]
fn test_trailing_slash_in_pattern_is_trimmed() {
    // "src/" must behave identically to "src".
    assert!(is_ignored("src/main.rs", &ignores(&["src/"])));
    assert!(is_ignored("src", &ignores(&["src/"])));
}

// ── is_protected (prevents ignoring sentinel files) ───────────────────────

#[test]
fn test_blueprint_file_is_protected() {
    assert!(!is_ignored(
        "cairn.blueprint",
        &ignores(&["cairn.blueprint"])
    ));
}

#[test]
fn test_config_file_is_protected() {
    assert!(!is_ignored(
        "cairn.config.yaml",
        &ignores(&["cairn.config.yaml"])
    ));
}

#[test]
fn test_meta_prefix_is_protected() {
    assert!(!is_ignored("meta/decisions/d1.md", &ignores(&["meta"])));
    assert!(!is_ignored("meta", &ignores(&["meta"])));
}

#[test]
fn test_cairn_hidden_dir_is_protected() {
    assert!(!is_ignored(".cairn/state.json", &ignores(&[".cairn"])));
    assert!(!is_ignored(".cairn", &ignores(&[".cairn"])));
}

#[test]
fn test_normal_paths_are_not_protected() {
    // Sanity: protection only applies to the sentinel paths.
    assert!(is_ignored("target", &ignores(&["target"])));
    assert!(is_ignored("dist/app.js", &ignores(&["*.js"])));
}

#[test]
fn test_meta_prefix_not_matched_for_metadata() {
    // "metadata/" starts with "meta" but not "meta/" — must not be protected.
    assert!(is_ignored("metadata/foo.txt", &ignores(&["metadata"])));
}

// ── parse_config ──────────────────────────────────────────────────────────

#[test]
fn test_parse_config_context_field() {
    let mut config = Config::default();
    parse_config("context: \"System-level AI agent\"\n", &mut config);
    assert_eq!(config.context, "System-level AI agent");
}

#[test]
fn test_parse_config_state_backend_field() {
    let mut config = Config::default();
    parse_config("state_backend: beads\n", &mut config);
    assert_eq!(config.state_backend, "beads");
}

#[test]
fn test_parse_config_ignore_list() {
    let mut config = Config::default();
    parse_config("ignore:\n  - dist\n  - \"*.lock\"\n", &mut config);
    assert!(config.ignores.contains(&"dist".to_owned()));
    assert!(config.ignores.contains(&"*.lock".to_owned()));
}

#[test]
fn test_parse_config_rules_map() {
    let mut config = Config::default();
    parse_config("rules:\n  tone: concise\n  format: json\n", &mut config);
    assert_eq!(
        config.rules.get("tone").map(String::as_str),
        Some("concise")
    );
    assert_eq!(config.rules.get("format").map(String::as_str), Some("json"));
}

#[test]
fn test_parse_config_targets() {
    let mut config = Config::default();
    parse_config(
        "targets:\n  - node: app.api\n    path: src/api\n    language: rust\n",
        &mut config,
    );
    assert_eq!(config.targets.len(), 1);
    assert_eq!(config.targets[0].node_id, "app.api");
    assert_eq!(config.targets[0].path, std::path::PathBuf::from("src/api"));
    assert_eq!(config.targets[0].language, "rust");
}

// ── load_ignore_file ──────────────────────────────────────────────────────

#[test]
fn test_load_ignore_file_strips_comments_and_blanks() {
    let dir = tempfile::tempdir().unwrap();
    std::fs::write(
        dir.path().join(".cairnignore"),
        "# comment\ndist\n\n*.log\n",
    )
    .unwrap();
    let patterns = load_ignore_file(&dir.path().join(".cairnignore")).unwrap();
    assert!(
        !patterns.iter().any(|p| p.starts_with('#')),
        "comments must be stripped"
    );
    assert!(patterns.contains(&"dist".to_owned()));
    assert!(patterns.contains(&"*.log".to_owned()));
    assert!(
        !patterns.contains(&String::new()),
        "blank lines must be stripped"
    );
}

#[test]
fn test_load_ignore_file_missing_returns_empty() {
    let dir = tempfile::tempdir().unwrap();
    let patterns = load_ignore_file(&dir.path().join(".cairnignore")).unwrap();
    assert!(patterns.is_empty());
}

// ── IntentionalAsymmetry::matches ─────────────────────────────────────────

#[test]
fn test_asymmetry_matches_exact_node_role_and_paths() {
    let p = std::path::PathBuf::from("src/api.ts");
    let asym = IntentionalAsymmetry {
        node: "app.api".to_owned(),
        contract_role: "public_api".to_owned(),
        targets: vec![p.clone()],
        reason: "generated".to_owned(),
    };
    assert!(asym.matches("app.api", "public_api", &[&p]));
}

#[test]
fn test_asymmetry_rejects_wrong_node() {
    let p = std::path::PathBuf::from("src/api.ts");
    let asym = IntentionalAsymmetry {
        node: "app.api".to_owned(),
        contract_role: "public_api".to_owned(),
        targets: vec![p.clone()],
        reason: "generated".to_owned(),
    };
    assert!(!asym.matches("app.db", "public_api", &[&p]));
}

#[test]
fn test_asymmetry_rejects_wrong_role() {
    let p = std::path::PathBuf::from("src/api.ts");
    let asym = IntentionalAsymmetry {
        node: "app.api".to_owned(),
        contract_role: "public_api".to_owned(),
        targets: vec![p.clone()],
        reason: "generated".to_owned(),
    };
    assert!(!asym.matches("app.api", "internal", &[&p]));
}

#[test]
fn test_asymmetry_rejects_extra_path() {
    let p1 = std::path::PathBuf::from("src/a.ts");
    let p2 = std::path::PathBuf::from("src/b.ts");
    let asym = IntentionalAsymmetry {
        node: "app.api".to_owned(),
        contract_role: "public_api".to_owned(),
        targets: vec![p1.clone()],
        reason: "generated".to_owned(),
    };
    // targets has p1 only; passing [p1, p2] is a different set.
    assert!(!asym.matches("app.api", "public_api", &[&p1, &p2]));
}
