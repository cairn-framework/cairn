//! CLI decision-artefact scaffolding command.
// Reason: child module imports re-exported public surface from parent via use super::*
#![allow(clippy::wildcard_imports)]
use std::fmt::Write as _;

use super::super::*;

/// Dispatches `cairn decision <subcommand>`.
pub(crate) fn run_decision_command(parsed: &ParsedArgs, root: &Path) -> CliResult {
    match parsed.command_args.get(1).map(String::as_str) {
        Some("new") => {
            let Some(slug) = parsed.command_args.get(2) else {
                return err(1, copy::lookup("decision.usage"));
            };
            let nodes = flag_values(&parsed.command_args, "--node");
            let informed_by = flag_values(&parsed.command_args, "--informed-by");
            run_decision_new(root, slug, &nodes, &informed_by)
        }
        _ => err(1, copy::lookup("decision.usage")),
    }
}

/// Scaffolds `meta/decisions/dec.<slug>.md` with deterministic frontmatter and
/// the standard Context/Decision/Rationale/Consequences sections.
fn run_decision_new(
    root: &Path,
    slug: &str,
    nodes: &[String],
    informed_by: &[String],
) -> CliResult {
    if !is_kebab_slug(slug) {
        return err(1, copy::lookup("decision.invalid-slug"));
    }
    let decisions_dir = root.join("meta/decisions");
    let file_name = format!("dec.{slug}.md");
    let target = decisions_dir.join(&file_name);
    if target.exists() {
        return err(1, &copy::lookup("decision.exists").replace("{slug}", slug));
    }
    if let Err(error) = fs::create_dir_all(&decisions_dir) {
        return err(1, &format!("failed to create meta/decisions: {error}"));
    }
    let content = decision_stub(slug, nodes, informed_by, &today_utc());
    if let Err(error) = fs::write(&target, content) {
        return err(1, &format!("failed to write {file_name}: {error}"));
    }
    ok(format!(
        "{}\n",
        copy::lookup("decision.created").replace("{slug}", slug)
    ))
}

/// True when `slug` is non-empty kebab-case: `[a-z0-9]+(-[a-z0-9]+)*`. Rejects
/// leading/trailing or doubled hyphens so the derived id and heading are sound.
fn is_kebab_slug(slug: &str) -> bool {
    !slug.is_empty()
        && slug.split('-').all(|segment| {
            !segment.is_empty()
                && segment
                    .chars()
                    .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit())
        })
}

/// Collects every value following each occurrence of `flag` in `args`,
/// splitting comma-separated values so `--node a,b` and `--node a --node b`
/// behave the same.
fn flag_values(args: &[String], flag: &str) -> Vec<String> {
    let mut values = Vec::new();
    let mut iter = args.iter().peekable();
    while let Some(arg) = iter.next() {
        if arg == flag
            && let Some(raw) = iter.peek()
            && !raw.starts_with("--")
        {
            for part in raw.split(',') {
                let trimmed = part.trim();
                if !trimmed.is_empty() {
                    values.push(trimmed.to_owned());
                }
            }
        }
    }
    values
}

/// Builds the decision artefact body. Pure: the date is injected so the output
/// is deterministic for a given input.
fn decision_stub(slug: &str, nodes: &[String], informed_by: &[String], date: &str) -> String {
    let mut out = String::new();
    out.push_str("---\n");
    let _ = writeln!(out, "id: dec.{slug}");
    if nodes.is_empty() {
        out.push_str("nodes: []\n");
    } else {
        out.push_str("nodes:\n");
        for node in nodes {
            let _ = writeln!(out, "  - {node}");
        }
    }
    out.push_str("status: proposed\n");
    let _ = writeln!(out, "date: {date}");
    if !informed_by.is_empty() {
        out.push_str("informed_by:\n");
        for id in informed_by {
            let _ = writeln!(out, "  - {id}");
        }
    }
    out.push_str("---\n");
    let _ = writeln!(out, "# {}", title_from_slug(slug));
    out.push_str(
        "\n## Context\n\nDescribe the forces and constraints that make this decision necessary.\n",
    );
    out.push_str("\n## Decision\n\nState the decision in one or two sentences.\n");
    out.push_str("\n## Rationale\n\nExplain why this option was chosen over the alternatives.\n");
    out.push_str(
        "\n## Consequences\n\nNote the trade-offs, follow-ups, and what this commits the project to.\n",
    );
    out
}

/// Turns a kebab-case slug into a Title Case heading.
fn title_from_slug(slug: &str) -> String {
    slug.split('-')
        .filter(|word| !word.is_empty())
        .map(|word| {
            let mut chars = word.chars();
            chars.next().map_or_else(String::new, |first| {
                first.to_uppercase().collect::<String>() + chars.as_str()
            })
        })
        .collect::<Vec<_>>()
        .join(" ")
}

/// Current UTC date as `YYYY-MM-DD`, mirroring the changes archiver.
fn today_utc() -> String {
    std::process::Command::new("date")
        .args(["-u", "+%F"])
        .output()
        .ok()
        .and_then(|output| String::from_utf8(output.stdout).ok())
        .map(|value| value.trim().to_owned())
        .filter(|value| !value.is_empty())
        .unwrap_or_else(|| "1970-01-01".to_owned())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decision_stub_frontmatter_is_well_formed() {
        let stub = decision_stub(
            "my-rule",
            &["app.core".to_owned()],
            &["res.x".to_owned()],
            "2026-06-26",
        );
        assert!(stub.starts_with("---\nid: dec.my-rule\n"));
        assert!(stub.contains("nodes:\n  - app.core\n"));
        assert!(stub.contains("status: proposed\n"));
        assert!(stub.contains("date: 2026-06-26\n"));
        assert!(stub.contains("informed_by:\n  - res.x\n"));
        assert!(stub.contains("# My Rule"));
        assert!(stub.contains("## Context"));
        assert!(stub.contains("## Decision"));
        assert!(stub.contains("## Rationale"));
        assert!(stub.contains("## Consequences"));
    }

    #[test]
    fn test_decision_stub_omits_empty_optional_fields() {
        let stub = decision_stub("solo", &[], &[], "2026-06-26");
        assert!(stub.contains("nodes: []\n"), "empty nodes use inline list");
        assert!(
            !stub.contains("informed_by"),
            "informed_by is omitted when empty"
        );
    }

    #[test]
    fn test_flag_values_splits_and_repeats() {
        let args: Vec<String> = ["decision", "new", "s", "--node", "a,b", "--node", "c"]
            .iter()
            .map(|s| (*s).to_owned())
            .collect();
        assert_eq!(flag_values(&args, "--node"), vec!["a", "b", "c"]);
        assert!(flag_values(&args, "--informed-by").is_empty());
    }

    #[test]
    fn test_flag_values_skips_flag_lookalike_value() {
        // A missing value (next token is another flag) must not be captured.
        let args: Vec<String> = ["new", "s", "--node", "--informed-by", "res.x"]
            .iter()
            .map(|s| (*s).to_owned())
            .collect();
        assert!(
            flag_values(&args, "--node").is_empty(),
            "a flag-lookalike token must not become a value"
        );
        assert_eq!(flag_values(&args, "--informed-by"), vec!["res.x"]);
    }

    #[test]
    fn test_is_kebab_slug_rejects_malformed() {
        assert!(is_kebab_slug("my-rule"));
        assert!(is_kebab_slug("rule1"));
        for bad in ["", "-rule", "rule-", "a--b", "-", "Bad", "a_b", "a b"] {
            assert!(!is_kebab_slug(bad), "{bad:?} must be rejected");
        }
    }

    #[test]
    fn test_run_decision_new_rejects_bad_slug() {
        let dir = tempfile::tempdir().unwrap();
        let result = run_decision_new(dir.path(), "Bad Slug", &[], &[]);
        assert_eq!(result.code, 1);
    }

    #[test]
    fn test_run_decision_new_writes_file_and_refuses_overwrite() {
        let dir = tempfile::tempdir().unwrap();
        let first = run_decision_new(dir.path(), "my-rule", &["app.core".to_owned()], &[]);
        assert_eq!(first.code, 0, "scaffold should succeed: {}", first.stderr);
        assert!(dir.path().join("meta/decisions/dec.my-rule.md").exists());
        let second = run_decision_new(dir.path(), "my-rule", &["app.core".to_owned()], &[]);
        assert_eq!(
            second.code, 1,
            "must refuse to overwrite an existing decision"
        );
    }
}
