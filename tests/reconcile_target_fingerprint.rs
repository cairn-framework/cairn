//! Tests for Language detection, `TargetId` formatting, and `InterfaceFingerprint`.
//!
//! These drive the reconciler-routing and change-detection critical paths.

use std::path::Path;

use cairn::reconcile::{
    fingerprint::InterfaceFingerprint,
    target::{DEFAULT_CONTRACT_ROLE, Language, Target, TargetId},
};

// ── Language::from_extension ─────────────────────────────────────────────────

#[test]
fn test_lang_rs_extension() {
    assert_eq!(
        Language::from_extension(Path::new("src/lib.rs")),
        Some(Language::Rust)
    );
}

#[test]
fn test_lang_ts_extension() {
    assert_eq!(
        Language::from_extension(Path::new("src/api.ts")),
        Some(Language::TypeScript)
    );
}

#[test]
fn test_lang_tsx_extension() {
    assert_eq!(
        Language::from_extension(Path::new("src/App.tsx")),
        Some(Language::TypeScript),
        "TSX must map to TypeScript"
    );
}

#[test]
fn test_lang_py_extension() {
    assert_eq!(
        Language::from_extension(Path::new("api/views.py")),
        Some(Language::Python)
    );
}

#[test]
fn test_lang_go_extension() {
    assert_eq!(
        Language::from_extension(Path::new("handler/main.go")),
        Some(Language::Go)
    );
}

#[test]
fn test_lang_unknown_extension_is_none() {
    for path in &["README.md", "config.json", "Makefile", "build.sh"] {
        assert_eq!(
            Language::from_extension(Path::new(path)),
            None,
            "non-source file {path} must produce None"
        );
    }
}

#[test]
fn test_lang_no_extension_is_none() {
    assert_eq!(Language::from_extension(Path::new("Makefile")), None);
}

// ── Language::from_language_str ───────────────────────────────────────────────

#[test]
fn test_lang_from_str_rust() {
    assert_eq!(Language::from_language_str("rust"), Some(Language::Rust));
    assert_eq!(Language::from_language_str("Rust"), Some(Language::Rust));
    assert_eq!(Language::from_language_str("RUST"), Some(Language::Rust));
}

#[test]
fn test_lang_from_str_typescript_aliases() {
    assert_eq!(
        Language::from_language_str("typescript"),
        Some(Language::TypeScript)
    );
    assert_eq!(
        Language::from_language_str("TypeScript"),
        Some(Language::TypeScript)
    );
    assert_eq!(
        Language::from_language_str("ts"),
        Some(Language::TypeScript)
    );
    assert_eq!(
        Language::from_language_str("TS"),
        Some(Language::TypeScript)
    );
}

#[test]
fn test_lang_from_str_python_aliases() {
    assert_eq!(
        Language::from_language_str("python"),
        Some(Language::Python)
    );
    assert_eq!(
        Language::from_language_str("Python"),
        Some(Language::Python)
    );
    assert_eq!(Language::from_language_str("py"), Some(Language::Python));
    assert_eq!(Language::from_language_str("PY"), Some(Language::Python));
}

#[test]
fn test_lang_from_str_go() {
    assert_eq!(Language::from_language_str("go"), Some(Language::Go));
    assert_eq!(Language::from_language_str("Go"), Some(Language::Go));
    assert_eq!(Language::from_language_str("GO"), Some(Language::Go));
}

#[test]
fn test_lang_from_str_unknown_is_none() {
    for s in &["java", "ruby", "c++", "tsx", ""] {
        assert_eq!(
            Language::from_language_str(s),
            None,
            "'{s}' must produce None"
        );
    }
}

/// Round-trip: every Language variant must survive `as_str()` → `from_language_str()`.
#[test]
fn test_lang_as_str_round_trips() {
    for lang in [
        Language::Rust,
        Language::TypeScript,
        Language::Python,
        Language::Go,
    ] {
        assert_eq!(
            Language::from_language_str(lang.as_str()),
            Some(lang),
            "{lang:?}.as_str() round-trip failed"
        );
    }
}

// ── TargetId ─────────────────────────────────────────────────────────────────

#[test]
fn test_target_id_as_str_format() {
    let id = TargetId::new("app.api".to_owned(), "src/api".into());
    assert_eq!(id.as_str(), "app.api:src/api");
}

#[test]
fn test_target_id_ordering_by_node_then_path() {
    let a = TargetId::new("app.api".to_owned(), "src/a".into());
    let b = TargetId::new("app.api".to_owned(), "src/b".into());
    let c = TargetId::new("app.core".to_owned(), "src/a".into());
    assert!(a < b, "same node: earlier path sorts first");
    assert!(b < c, "earlier node ID sorts before later node ID");
}

// ── Target ───────────────────────────────────────────────────────────────────

#[test]
fn test_target_default_contract_role() {
    let t = Target::new("app.api".to_owned(), "src/api".into(), Language::Rust);
    assert_eq!(
        t.contract_role, DEFAULT_CONTRACT_ROLE,
        "default role must be DEFAULT_CONTRACT_ROLE"
    );
}

#[test]
fn test_target_with_contract_role_overrides() {
    let t = Target::new("app.api".to_owned(), "src/api".into(), Language::Rust)
        .with_contract_role("internal".to_owned());
    assert_eq!(t.contract_role, "internal");
}

#[test]
fn test_target_reconciler_id_matches_language() {
    assert_eq!(
        Target::new("n".to_owned(), "p".into(), Language::Rust)
            .reconciler_id
            .0,
        "rust-code"
    );
    assert_eq!(
        Target::new("n".to_owned(), "p".into(), Language::TypeScript)
            .reconciler_id
            .0,
        "typescript-code"
    );
    assert_eq!(
        Target::new("n".to_owned(), "p".into(), Language::Python)
            .reconciler_id
            .0,
        "python-code"
    );
    assert_eq!(
        Target::new("n".to_owned(), "p".into(), Language::Go)
            .reconciler_id
            .0,
        "go-code"
    );
}

// ── InterfaceFingerprint ──────────────────────────────────────────────────────

#[test]
fn test_fingerprint_is_deterministic() {
    let symbols = vec!["fn:alpha".to_owned(), "fn:beta".to_owned()];
    let fp1 = InterfaceFingerprint::from_symbols(&symbols);
    let fp2 = InterfaceFingerprint::from_symbols(&symbols);
    assert_eq!(fp1, fp2, "same symbols must produce same fingerprint");
}

#[test]
fn test_fingerprint_is_order_independent() {
    let a = InterfaceFingerprint::from_symbols(&["fn:alpha".to_owned(), "fn:beta".to_owned()]);
    let b = InterfaceFingerprint::from_symbols(&["fn:beta".to_owned(), "fn:alpha".to_owned()]);
    assert_eq!(a, b, "symbol order must not affect the fingerprint");
}

#[test]
fn test_fingerprint_changes_on_added_symbol() {
    let base = InterfaceFingerprint::from_symbols(&["fn:alpha".to_owned()]);
    let extended =
        InterfaceFingerprint::from_symbols(&["fn:alpha".to_owned(), "fn:beta".to_owned()]);
    assert_ne!(
        base, extended,
        "adding a symbol must change the fingerprint"
    );
}

#[test]
fn test_fingerprint_changes_on_removed_symbol() {
    let full = InterfaceFingerprint::from_symbols(&["fn:alpha".to_owned(), "fn:beta".to_owned()]);
    let partial = InterfaceFingerprint::from_symbols(&["fn:alpha".to_owned()]);
    assert_ne!(
        full, partial,
        "removing a symbol must change the fingerprint"
    );
}

#[test]
fn test_fingerprint_changes_on_renamed_symbol() {
    let a = InterfaceFingerprint::from_symbols(&["fn:alpha".to_owned()]);
    let b = InterfaceFingerprint::from_symbols(&["fn:Alpha".to_owned()]);
    assert_ne!(a, b, "renaming a symbol must change the fingerprint");
}

#[test]
fn test_fingerprint_empty_symbols_is_stable() {
    let fp1 = InterfaceFingerprint::from_symbols(&[]);
    let fp2 = InterfaceFingerprint::from_symbols(&[]);
    assert_eq!(
        fp1, fp2,
        "empty symbol list must produce a stable fingerprint"
    );
}

#[test]
fn test_fingerprint_hash_is_16_hex_chars() {
    for symbols in [
        vec![],
        vec!["fn:alpha".to_owned()],
        vec!["fn:alpha".to_owned(), "struct:Config".to_owned()],
    ] {
        let fp = InterfaceFingerprint::from_symbols(&symbols);
        assert_eq!(
            fp.hash.len(),
            16,
            "fingerprint hash must be exactly 16 hex characters; got {:?}",
            fp.hash
        );
        assert!(
            fp.hash.chars().all(|c| c.is_ascii_hexdigit()),
            "fingerprint hash must be lowercase hex; got {:?}",
            fp.hash
        );
    }
}

/// `DefaultHasher` is documented as stable within a process but not across
/// Rust versions. Pin the empty-list hash value so any version-induced
/// change becomes a deliberate, visible test failure rather than a silent
/// cache invalidation.
#[test]
fn test_fingerprint_empty_hash_is_pinned() {
    let fp = InterfaceFingerprint::from_symbols(&[]);
    // If this value changes, update the snapshot AND ensure the fingerprint
    // store is cleared on upgrade to avoid false-positive drift findings.
    assert_eq!(
        fp.hash, "bd60acb658c79e45",
        "empty-list fingerprint changed — update fingerprint store on deploy"
    );
}
