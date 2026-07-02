//! CLI-only `cairn gap <node> --question "<text>"` command.
//!
//! Writes a `gap: true`, `status: proposed` decision artefact logging a
//! genuine underspecification an agent hit while implementing `node`.
//! `CAIRN_GAP_UNRESOLVED` lints every open (still-`proposed`) gap; resolving
//! the question and flipping the artefact to `status: accepted` (or deleting
//! it) clears the finding.
// Reason: child module imports re-exported public surface from parent via use super::*
#![allow(clippy::wildcard_imports)]
use super::super::*;
use super::decision::today_utc;

pub(crate) fn run_gap_command(
    parsed: &ParsedArgs,
    root: &Path,
    scan_result: &scanner::ScanResult,
) -> CliResult {
    let Some(node_arg) = parsed.command_args.get(1) else {
        return err(2, "usage: cairn gap <node> --question \"<text>\"");
    };
    let node = match scan_result.graph.resolve(node_arg) {
        Ok(node) => node,
        Err(finding) => return findings_output(parsed.json, std::slice::from_ref(&finding)),
    };
    let question = format::flag_value(&parsed.command_args, "--question")
        .unwrap_or_default()
        .trim();
    if question.is_empty() {
        return err(2, "usage: cairn gap <node> --question \"<text>\"");
    }

    let node_slug = slugify(&node.id);
    let question_slug = slugify(&first_words(question, 6));
    let decisions_dir = root.join("meta/decisions");
    if let Err(error) = fs::create_dir_all(&decisions_dir) {
        return err(1, &format!("failed to create meta/decisions: {error}"));
    }
    let base = format!("gap-{node_slug}-{question_slug}");
    let mut file_stem = base.clone();
    let mut target = decisions_dir.join(format!("{file_stem}.md"));
    let mut suffix = 2;
    while target.exists() {
        file_stem = format!("{base}-{suffix}");
        target = decisions_dir.join(format!("{file_stem}.md"));
        suffix += 1;
    }
    let id = format!("dec.{file_stem}");

    let content = gap_stub(
        &id,
        &node.id,
        &format!("{:?}", node.state),
        question,
        &today_utc(),
    );
    if let Err(error) = fs::write(&target, content) {
        return err(1, &format!("failed to write {file_stem}.md: {error}"));
    }

    let path = format!("meta/decisions/{file_stem}.md");
    if parsed.json {
        return ok(format!("{{\"created\":\"{path}\",\"id\":\"{id}\"}}\n"));
    }
    ok(format!(
        "created {path} (id: {id}); resolve the question and accept (or delete) the artefact to clear CAIRN_GAP_UNRESOLVED\n"
    ))
}

/// The first `n` whitespace-separated words of `text`, rejoined with spaces.
fn first_words(text: &str, n: usize) -> String {
    text.split_whitespace()
        .take(n)
        .collect::<Vec<_>>()
        .join(" ")
}

/// Lowercases and hyphen-joins `text`: strips anything that isn't an
/// alphanumeric or space/hyphen, collapses runs of separators to one hyphen,
/// and trims leading/trailing hyphens. Empty input falls back to `"q"` so the
/// filename stays valid.
fn slugify(text: &str) -> String {
    let mut slug = String::with_capacity(text.len());
    let mut last_was_sep = true;
    for ch in text.chars() {
        if ch.is_ascii_alphanumeric() {
            slug.push(ch.to_ascii_lowercase());
            last_was_sep = false;
        } else if !last_was_sep {
            slug.push('-');
            last_was_sep = true;
        }
    }
    while slug.ends_with('-') {
        slug.pop();
    }
    if slug.is_empty() {
        "q".to_owned()
    } else {
        slug
    }
}

/// Builds the gap decision artefact body.
fn gap_stub(id: &str, node: &str, node_state: &str, question: &str, date: &str) -> String {
    format!(
        "---\nid: {id}\nnodes: [{node}]\nstatus: proposed\ndate: {date}\ngap: true\ninformed_by: []\n---\n\n# Gap: {question}\n\n## Question\n\n{question}\n\n## Context\n\nNode: `{node}` (state: {node_state})\n\nOpened by `cairn gap {node} --question \"{question}\"`.\n\n## Resolution\n\n(Answer the question here, then flip `status` to `accepted` or delete this file.)\n"
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fixture_scan(dir: &Path) -> scanner::ScanResult {
        fs::write(
            dir.join("cairn.blueprint"),
            "System App \"d\" id \"app\" {\n    Module Api \"d\" id \"app.api\" {\n        path \"./src\"\n    }\n}\n",
        )
        .unwrap();
        fs::create_dir_all(dir.join("src")).unwrap();
        fs::write(dir.join("src/lib.rs"), "pub fn f() {}\n").unwrap();
        scanner::load_project(dir, &dir.join("cairn.blueprint")).unwrap()
    }

    fn gap_args(args: &[&str]) -> ParsedArgs {
        ParsedArgs {
            json: false,
            strict: false,
            file: PathBuf::from("cairn.blueprint"),
            changes_dir: PathBuf::from("meta/changes"),
            command: "gap".to_owned(),
            command_args: args.iter().map(|s| (*s).to_owned()).collect(),
        }
    }

    #[test]
    fn run_gap_command_writes_exact_frontmatter_and_suffixes_on_collision() {
        let dir = tempfile::tempdir().unwrap();
        let scan = fixture_scan(dir.path());
        let parsed = gap_args(&[
            "gap",
            "app.api",
            "--question",
            "What auth model should we use for remote queries?",
        ]);

        let first = run_gap_command(&parsed, dir.path(), &scan);
        assert_eq!(first.code, 0, "stdout: {}", first.stdout);
        let path = dir
            .path()
            .join("meta/decisions/gap-app-api-what-auth-model-should-we-use.md");
        assert!(path.exists(), "expected {}", path.display());
        let content = fs::read_to_string(&path).unwrap();
        assert!(content.starts_with(
            "---\nid: dec.gap-app-api-what-auth-model-should-we-use\nnodes: [app.api]\nstatus: proposed\n"
        ));
        assert!(content.contains("gap: true\n"));
        assert!(
            content.contains("## Question\n\nWhat auth model should we use for remote queries?\n")
        );
        assert!(content.contains("## Resolution\n"));

        let second = run_gap_command(&parsed, dir.path(), &scan);
        assert_eq!(second.code, 0, "stdout: {}", second.stdout);
        let suffixed = dir
            .path()
            .join("meta/decisions/gap-app-api-what-auth-model-should-we-use-2.md");
        assert!(
            suffixed.exists(),
            "second identical call must create a -2 suffixed file"
        );
        assert!(
            path.exists(),
            "the first file must be left untouched, not overwritten"
        );
    }

    #[test]
    fn run_gap_command_refuses_empty_question() {
        let dir = tempfile::tempdir().unwrap();
        let scan = fixture_scan(dir.path());
        let parsed = gap_args(&["gap", "app.api", "--question", ""]);
        let result = run_gap_command(&parsed, dir.path(), &scan);
        assert_eq!(result.code, 2);
        assert!(
            !dir.path().join("meta/decisions").exists() || {
                fs::read_dir(dir.path().join("meta/decisions"))
                    .map_or(true, |mut entries| entries.next().is_none())
            }
        );
    }

    #[test]
    fn run_gap_command_unknown_node_returns_finding_error() {
        let dir = tempfile::tempdir().unwrap();
        let scan = fixture_scan(dir.path());
        let parsed = gap_args(&["gap", "app.bogus", "--question", "does this exist?"]);
        let result = run_gap_command(&parsed, dir.path(), &scan);
        assert_eq!(result.code, 1);
        assert!(result.stdout.contains("CAIRN_QUERY_NODE_NOT_FOUND"));
    }

    #[test]
    fn slugify_lowercases_and_hyphenates() {
        assert_eq!(slugify("cairn.kernel.map"), "cairn-kernel-map");
        assert_eq!(slugify("What auth model?"), "what-auth-model");
        assert_eq!(slugify("  leading and trailing  "), "leading-and-trailing");
    }

    #[test]
    fn slugify_empty_input_falls_back() {
        assert_eq!(slugify("???"), "q");
    }

    #[test]
    fn first_words_truncates_to_n() {
        assert_eq!(
            first_words("one two three four five six seven eight", 6),
            "one two three four five six"
        );
        assert_eq!(first_words("only two", 6), "only two");
    }
}
