//! CLI store-admin verbs for the coordination fact store:
//! `cairn coord verify` and `cairn coord compact --before <date>`.
// Reason: child module imports re-exported public surface from parent via use super::*
#![allow(clippy::wildcard_imports)]
use super::super::*;
use crate::coord::verify::{compact, verify};

/// Dispatches `cairn coord <subcommand>`.
pub(crate) fn run_coord_command(parsed: &ParsedArgs, root: &Path) -> CliResult {
    match parsed.command_args.get(1).map(String::as_str) {
        Some("verify") => run_coord_verify(root, parsed.json),
        Some("compact") => {
            let Some(before) = flag_value(&parsed.command_args, "--before") else {
                return err(1, copy::lookup("coord.compact-usage"));
            };
            run_coord_compact(root, before, parsed.json)
        }
        _ => err(1, copy::lookup("coord.usage")),
    }
}

fn run_coord_verify(root: &Path, json: bool) -> CliResult {
    match verify(root) {
        Ok(()) => {
            let message = copy::lookup("coord.verify-clean");
            if json {
                ok(serde_json::json!({
                    "schema_version": crate::query_api::SCHEMA_VERSION,
                    "clean": true,
                    "message": message,
                })
                .to_string())
            } else {
                ok(message.to_owned())
            }
        }
        Err(violation) => {
            if json {
                err(
                    1,
                    &serde_json::json!({
                        "schema_version": crate::query_api::SCHEMA_VERSION,
                        "clean": false,
                        "violation": violation,
                    })
                    .to_string(),
                )
            } else {
                err(1, &violation)
            }
        }
    }
}

fn run_coord_compact(root: &Path, before: &str, json: bool) -> CliResult {
    match compact(root, before) {
        Ok(moved) => {
            if json {
                ok(serde_json::json!({
                    "schema_version": crate::query_api::SCHEMA_VERSION,
                    "moved": moved,
                })
                .to_string())
            } else if moved.is_empty() {
                ok(copy::lookup("coord.compact-none").to_owned())
            } else {
                ok(
                    copy::lookup("coord.compact-moved")
                        .replace("{count}", &moved.len().to_string()),
                )
            }
        }
        Err(error) => err(1, &error),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn repo() -> tempfile::TempDir {
        let dir = tempfile::TempDir::new().expect("tempdir");
        let output = std::process::Command::new("git")
            .arg("-C")
            .arg(dir.path())
            .args(["init", "-q"])
            .output()
            .expect("git runs");
        assert!(output.status.success());
        dir
    }

    fn parsed(args: &[&str], json: bool) -> ParsedArgs {
        ParsedArgs {
            json,
            strict: false,
            verbose: false,
            brief: false,
            file: PathBuf::from("cairn.blueprint"),
            changes_dir: PathBuf::from("meta/changes"),
            command: "coord".to_owned(),
            command_args: std::iter::once("coord")
                .chain(args.iter().copied())
                .map(ToOwned::to_owned)
                .collect(),
        }
    }

    fn run(dir: &Path, args: &[&str], json: bool) -> CliResult {
        run_coord_command(&parsed(args, json), dir)
    }

    #[test]
    fn verify_reports_clean_on_an_uninitialised_store() {
        let dir = repo();
        let result = run(dir.path(), &["verify"], false);
        assert_eq!(result.code, 0, "{}", result.stderr);
        assert_eq!(result.stdout.trim(), copy::lookup("coord.verify-clean"));
        let json = run(dir.path(), &["verify"], true);
        assert_eq!(json.code, 0);
        let value: serde_json::Value =
            serde_json::from_str(json.stdout.trim()).expect("json output");
        assert_eq!(value["clean"], serde_json::json!(true));
        assert_eq!(
            value["schema_version"],
            serde_json::json!(crate::query_api::SCHEMA_VERSION)
        );
    }

    #[test]
    fn compact_requires_a_before_date_and_reports_moves_as_json() {
        let dir = repo();
        let missing = run(dir.path(), &["compact"], false);
        assert_eq!(missing.code, 1);
        assert_eq!(missing.stderr.trim(), copy::lookup("coord.compact-usage"));
        let json = run(dir.path(), &["compact", "--before", "2026-08-01"], true);
        assert_eq!(json.code, 0, "{}", json.stderr);
        let value: serde_json::Value =
            serde_json::from_str(json.stdout.trim()).expect("json output");
        assert_eq!(value["moved"], serde_json::json!([]));
    }

    #[test]
    fn unknown_subcommand_prints_usage() {
        let dir = repo();
        let result = run(dir.path(), &["bogus"], false);
        assert_eq!(result.code, 1);
        assert_eq!(result.stderr.trim(), copy::lookup("coord.usage"));
    }
}
