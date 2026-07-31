//! Tests for canonical decision subject manifests.

use std::fs;

use super::*;

fn decision(status: &str, receipts_block: &str, body: &str) -> String {
    // `receipts_block` is inserted verbatim: the candidate form omits the key
    // entirely (the protocol's real spelling before receipts exist), and the
    // accepted form carries the block list. Both reduce to identical governed
    // content because every RATIFICATION_KEYS field strips whole.
    format!(
        "---\nstatus: {status}\nratification: local\nratified_by: machine\n{receipts_block}title: Example\n---\n{body}"
    )
}

fn manifest_hash(entries: &[(&str, String)]) -> String {
    let manifest = entries
        .iter()
        .fold(String::new(), |mut output, (path, hash)| {
            output.push_str(hash);
            output.push_str("  ");
            output.push_str(path);
            output.push('\n');
            output
        });
    format!("sha256:{}", sha256_hex(manifest.as_bytes()))
}

#[test]
fn test_governed_content_strips_ratification_keys_only() {
    let raw = "---\ntitle: Example\nstatus: proposed\nratification: local\nratified_by: machine\nreceipts:\n  - rev.one\n  - rev.two\nnodes:\n  - app.core\n---\nBody\n";
    let expected = "---\ntitle: Example\nnodes:\n  - app.core\n---\nBody\n";

    assert_eq!(governed_content(raw), expected);
}

#[test]
fn test_governed_content_without_frontmatter_unchanged() {
    let raw = "# A decision\n\nstatus: proposed\n";

    assert_eq!(governed_content(raw), raw);
}

#[test]
fn test_subject_hash_stable_across_status_flip() {
    let root = tempfile::tempdir().expect("tempdir");
    let proposed = decision("proposed", "", "Rule body\n");
    let accepted = decision(
        "accepted",
        "receipts:\n  - rev.correctness\n  - rev.simplicity\n",
        "Rule body\n",
    );

    let before = compute_subject_hash(root.path(), "meta/decisions/example.md", &proposed, &[])
        .expect("proposed hash");
    let after = compute_subject_hash(root.path(), "meta/decisions/example.md", &accepted, &[])
        .expect("accepted hash");

    assert_eq!(before, after);
}

#[test]
fn test_subject_hash_candidate_equals_acceptance() {
    let root = tempfile::tempdir().expect("tempdir");
    let affects = vec![
        "meta/reviews/rev.example-correctness.md".to_owned(),
        "meta/reviews/rev.example-simplicity.md".to_owned(),
    ];
    let proposed = "---\nstatus: proposed\nratification: local\naffects:\n  - meta/reviews/rev.example-correctness.md\n  - meta/reviews/rev.example-simplicity.md\ntitle: Example\n---\nRule body\n";
    let accepted = "---\nstatus: accepted\nratification: local\nreceipts:\n  - rev.example-correctness\n  - rev.example-simplicity\naffects:\n  - meta/reviews/rev.example-correctness.md\n  - meta/reviews/rev.example-simplicity.md\ntitle: Example\n---\nRule body\n";

    let candidate =
        compute_subject_hash(root.path(), "meta/decisions/example.md", proposed, &affects)
            .expect("candidate hash");
    fs::create_dir_all(root.path().join("meta/reviews")).expect("review directory");
    fs::write(
        root.path().join("meta/reviews/rev.example-correctness.md"),
        b"correctness receipt",
    )
    .expect("correctness receipt");
    fs::write(
        root.path().join("meta/reviews/rev.example-simplicity.md"),
        b"simplicity receipt",
    )
    .expect("simplicity receipt");
    let acceptance =
        compute_subject_hash(root.path(), "meta/decisions/example.md", accepted, &affects)
            .expect("acceptance hash");

    assert_eq!(candidate, acceptance);
}

#[test]
fn test_subject_hash_changes_on_body_byte_edit_and_affects_file_byte_edit() {
    let root = tempfile::tempdir().expect("tempdir");
    fs::create_dir_all(root.path().join("src")).expect("source directory");
    fs::write(root.path().join("src/example.rs"), b"let x = 1;\n").expect("source file");
    let raw = decision("proposed", "", "Rule body\n");
    let affects = vec!["src/example.rs".to_owned()];

    let original = compute_subject_hash(root.path(), "meta/decisions/example.md", &raw, &affects)
        .expect("original hash");
    let body_changed = compute_subject_hash(
        root.path(),
        "meta/decisions/example.md",
        &decision("proposed", "", "Rule body!\n"),
        &affects,
    )
    .expect("body hash");
    fs::write(root.path().join("src/example.rs"), b"let x = 2;\n").expect("edited source");
    let affects_changed =
        compute_subject_hash(root.path(), "meta/decisions/example.md", &raw, &affects)
            .expect("affects hash");

    assert_ne!(original, body_changed);
    assert_ne!(original, affects_changed);
}

#[test]
fn test_manifest_entries_sorted_and_receipts_excluded() {
    let root = tempfile::tempdir().expect("tempdir");
    fs::create_dir_all(root.path().join("meta/reviews")).expect("review directory");
    fs::write(root.path().join("a.txt"), b"a").expect("a file");
    fs::write(root.path().join("z.txt"), b"z").expect("z file");
    fs::write(root.path().join("meta/reviews/rev.one.md"), b"receipt").expect("receipt");
    let raw = decision("proposed", "", "Body\n");
    let actual = compute_subject_hash(
        root.path(),
        "meta/decisions/example.md",
        &raw,
        &[
            "z.txt".to_owned(),
            "meta/reviews/rev.one.md".to_owned(),
            "a.txt".to_owned(),
        ],
    )
    .expect("manifest hash");
    let expected = manifest_hash(&[
        ("a.txt", sha256_hex(b"a")),
        (
            "meta/decisions/example.md",
            sha256_hex(governed_content(&raw).as_bytes()),
        ),
        ("z.txt", sha256_hex(b"z")),
    ]);

    assert_eq!(actual, expected);
}

#[test]
fn test_normalise_repo_path_rejections() {
    for path in [
        "",
        "/absolute",
        "C:/outside",
        "../escape",
        "src/./file",
        "src\\file",
    ] {
        assert_eq!(normalise_repo_path(path), None, "{path} must be rejected");
    }
    assert_eq!(
        normalise_repo_path("src/file///"),
        Some("src/file".to_owned())
    );
}

#[test]
fn test_compute_subject_hash_missing_file_errors() {
    let root = tempfile::tempdir().expect("tempdir");
    let error = compute_subject_hash(
        root.path(),
        "meta/decisions/example.md",
        &decision("proposed", "", "Body\n"),
        &["missing.txt".to_owned()],
    )
    .expect_err("missing file must fail");

    assert!(error.message.contains("missing.txt"));
}

#[test]
fn test_compute_subject_hash_dir_entry_expands_sorted_recursive() {
    let root = tempfile::tempdir().expect("tempdir");
    fs::create_dir_all(root.path().join("src/nested")).expect("source directories");
    fs::write(root.path().join("src/z.txt"), b"z").expect("z file");
    fs::write(root.path().join("src/nested/a.txt"), b"a").expect("a file");
    let raw = decision("proposed", "", "Body\n");

    let actual = compute_subject_hash(
        root.path(),
        "meta/decisions/example.md",
        &raw,
        &["src/".to_owned()],
    )
    .expect("directory hash");
    let expected = manifest_hash(&[
        (
            "meta/decisions/example.md",
            sha256_hex(governed_content(&raw).as_bytes()),
        ),
        ("src/nested/a.txt", sha256_hex(b"a")),
        ("src/z.txt", sha256_hex(b"z")),
    ]);

    assert_eq!(actual, expected);
}

#[test]
fn test_normalise_repo_entry_preserves_dir_marker() {
    assert_eq!(
        normalise_repo_entry("src/nested/"),
        Some(RepoPathRule::Dir("src/nested".to_owned()))
    );
    assert_eq!(
        normalise_repo_entry("src/nested"),
        Some(RepoPathRule::File("src/nested".to_owned()))
    );
}

#[test]
fn test_rule_matches_file_and_dir_prefix() {
    let file = RepoPathRule::File("src/main.rs".to_owned());
    let directory = RepoPathRule::Dir("src".to_owned());

    assert!(rule_matches(&file, "src/main.rs"));
    assert!(!rule_matches(&file, "src/lib.rs"));
    assert!(rule_matches(&directory, "src/lib.rs"));
    assert!(!rule_matches(&directory, "src"));
    assert!(!rule_matches(&directory, "source/lib.rs"));
}

#[cfg(unix)]
#[test]
fn test_compute_subject_hash_symlink_escape_errors() {
    use std::os::unix::fs::symlink;

    let root = tempfile::tempdir().expect("tempdir");
    let outside = tempfile::tempdir().expect("outside tempdir");
    fs::write(outside.path().join("secret.txt"), b"secret").expect("outside file");
    symlink(
        outside.path().join("secret.txt"),
        root.path().join("escape.txt"),
    )
    .expect("escape symlink");

    let error = compute_subject_hash(
        root.path(),
        "meta/decisions/example.md",
        &decision("proposed", "", "Body\n"),
        &["escape.txt".to_owned()],
    )
    .expect_err("escaping symlink must fail");

    assert!(error.message.contains("escape.txt"));
}

#[cfg(unix)]
#[test]
fn test_compute_subject_hash_symlink_inside_root_hashes_target() {
    use std::os::unix::fs::symlink;

    let root = tempfile::tempdir().expect("tempdir");
    fs::write(root.path().join("target.txt"), b"target bytes").expect("target file");
    symlink(
        root.path().join("target.txt"),
        root.path().join("alias.txt"),
    )
    .expect("inside symlink");
    let raw = decision("proposed", "", "Body\n");

    let actual = compute_subject_hash(
        root.path(),
        "meta/decisions/example.md",
        &raw,
        &["alias.txt".to_owned()],
    )
    .expect("inside symlink hash");
    let expected = manifest_hash(&[
        ("alias.txt", sha256_hex(b"target bytes")),
        (
            "meta/decisions/example.md",
            sha256_hex(governed_content(&raw).as_bytes()),
        ),
    ]);

    assert_eq!(actual, expected);
}

#[test]
fn test_dir_expansion_excludes_receipt_stems() {
    let root = tempfile::tempdir().expect("tempdir");
    fs::create_dir_all(root.path().join("meta/reviews")).expect("review directory");
    fs::write(root.path().join("meta/reviews/rev.one.md"), b"receipt").expect("receipt");
    let raw = decision("proposed", "", "Body\n");

    let actual = compute_subject_hash(
        root.path(),
        "meta/decisions/example.md",
        &raw,
        &["meta/reviews/".to_owned()],
    )
    .expect("directory hash");
    let expected = manifest_hash(&[(
        "meta/decisions/example.md",
        sha256_hex(governed_content(&raw).as_bytes()),
    )]);

    assert_eq!(actual, expected);
}

#[test]
fn test_governed_content_keeps_indented_governed_key_after_receipts() {
    // The frontmatter parser trims lines, so an indented `nodes:` after
    // `receipts:` is a TOP-LEVEL governed field and must stay in the hash.
    let raw = "---\nreceipts:\n  - rev.one\n  nodes: [app.core]\n---\nBody\n";
    let governed = governed_content(raw);
    assert!(
        governed.contains("nodes: [app.core]"),
        "indented governed key escaped the hash: {governed:?}"
    );
    assert!(!governed.contains("rev.one"));
}

#[test]
fn test_governed_content_strips_unindented_list_items() {
    // The parser accepts column-zero `- ` rows as items of the open list, so
    // the acceptance flip written unindented must stay hash-invariant.
    let raw = "---\nreceipts:\n- rev.one\n- rev.two\ntitle: T\n---\nBody\n";
    let governed = governed_content(raw);
    assert!(!governed.contains("rev.one") && !governed.contains("rev.two"));
    assert!(governed.contains("title: T"));
}

#[test]
fn test_governed_content_strips_indented_ratification_key() {
    // An indented `status:` still parses as the top-level status field, so it
    // is a ratification key and must never enter the hash.
    let raw = "---\ntitle: T\n  status: accepted\n---\nBody\n";
    let governed = governed_content(raw);
    assert!(!governed.contains("status: accepted"), "{governed:?}");
    assert!(governed.contains("title: T"));
}

#[test]
fn test_governed_content_blank_lines_between_list_items_stripped() {
    let raw = "---\nreceipts:\n- rev.one\n\n- rev.two\nnodes: [a]\n---\nB\n";
    let governed = governed_content(raw);
    assert!(!governed.contains("rev.two"));
    assert!(governed.contains("nodes: [a]"));
}

#[test]
fn test_normalise_repo_path_rejects_empty_segment() {
    assert_eq!(normalise_repo_path("docs//registries"), None);
    assert_eq!(normalise_repo_entry("docs//registries/"), None);
}

#[test]
fn test_governed_content_comment_row_keeps_list_open() {
    // The parser ignores colon-free rows without closing the active list, so
    // a receipt after a comment is still a receipt and must strip.
    let raw = "---\nreceipts:\n- rev.one\n# second lens\n- rev.two\ntitle: T\n---\nB\n";
    let governed = governed_content(raw);
    assert!(!governed.contains("rev.two"), "{governed:?}");
    assert!(!governed.contains("# second lens"));
    assert!(governed.contains("title: T"));
}

#[test]
fn test_governed_content_indented_closer_ends_frontmatter() {
    // The parser closes frontmatter on a TRIMMED `---`, so stripping must see
    // the same span and still remove the ratification keys before it.
    let raw = "---\nstatus: proposed\ntitle: T\n  ---\nBody status: text\n";
    let governed = governed_content(raw);
    assert!(!governed.contains("status: proposed"), "{governed:?}");
    assert!(governed.contains("Body status: text"));
}

#[cfg(unix)]
#[test]
fn test_compute_subject_hash_symlink_alias_into_reviews_excluded() {
    use std::os::unix::fs::symlink;

    let root = tempfile::tempdir().expect("tempdir");
    fs::create_dir_all(root.path().join("meta/reviews")).expect("reviews dir");
    fs::write(root.path().join("meta/reviews/rev.one.md"), "receipt").expect("receipt");
    fs::create_dir_all(root.path().join("meta/decisions")).expect("decisions dir");
    let decision_raw = "---\ntitle: T\n---\nBody\n";
    fs::write(root.path().join("meta/decisions/d.md"), decision_raw).expect("decision");
    symlink(
        root.path().join("meta/reviews"),
        root.path().join("evidence"),
    )
    .expect("alias");

    let with_alias = compute_subject_hash(
        root.path(),
        "meta/decisions/d.md",
        decision_raw,
        &["evidence/".to_owned()],
    )
    .expect("alias hash");
    let without_alias = compute_subject_hash(root.path(), "meta/decisions/d.md", decision_raw, &[])
        .expect("bare hash");
    assert_eq!(
        with_alias, without_alias,
        "a symlink alias into meta/reviews must never contribute receipt bytes"
    );
}
