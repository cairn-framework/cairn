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

/// Dispatches `cairn ruling <subcommand>`: the maintainer read surface over
/// coordination ruling facts.
pub(crate) fn run_ruling_command(parsed: &ParsedArgs, root: &Path) -> CliResult {
    match parsed.command_args.get(1).map(String::as_str) {
        Some("list") => coordination_query(parsed, root, "ruling list", None),
        Some("show") => {
            let Some(fact_id) = parsed.command_args.get(2) else {
                return err(1, copy::lookup("ruling.show-usage"));
            };
            coordination_query(parsed, root, "ruling show", Some(fact_id.clone()))
        }
        _ => err(1, copy::lookup("ruling.usage")),
    }
}

/// Dispatches `cairn lease <subcommand>`: the read surface over lease and
/// driver-singleton facts.
pub(crate) fn run_lease_command(parsed: &ParsedArgs, root: &Path) -> CliResult {
    match parsed.command_args.get(1).map(String::as_str) {
        Some("list") => coordination_query(parsed, root, "lease list", None),
        _ => err(1, copy::lookup("lease.usage")),
    }
}

/// Runs a coordination read tool and renders it: raw pretty JSON under
/// `--json`, one line per fact otherwise. The wire carries no derived
/// verdicts, so the human form prints raw facts too.
fn coordination_query(
    parsed: &ParsedArgs,
    root: &Path,
    tool: &str,
    fact_id: Option<String>,
) -> CliResult {
    let request = crate::query_api::QueryRequest {
        tool: tool.to_owned(),
        node: fact_id,
        at: flag_value(&parsed.command_args, "--at").map(ToOwned::to_owned),
        since: flag_value(&parsed.command_args, "--since").map(ToOwned::to_owned),
        ..Default::default()
    };
    let changes_dir = root.join(&parsed.changes_dir);
    match crate::query_api::execute(root, &parsed.file, &changes_dir, &request) {
        Ok(response) => {
            if parsed.json {
                return ok(format!("{}\n", response.data));
            }
            ok(render_coordination_human(&response.data))
        }
        Err(error) => err(1, &format!("{error}")),
    }
}

fn render_coordination_human(data: &serde_json::Value) -> String {
    if data["store_state"] == "uninitialised" {
        return copy::lookup("coord.store-uninitialised").to_owned();
    }
    let mut lines = Vec::new();
    let facts = ["rulings", "leases"]
        .iter()
        .find_map(|key| data[*key].as_array())
        .cloned()
        .or_else(|| {
            data["ruling"]
                .as_object()
                .map(|one| vec![serde_json::Value::Object(one.clone())])
        })
        .unwrap_or_default();
    for fact in &facts {
        let target = fact["payload"]["target"]
            .as_str()
            .or_else(|| fact["payload"]["unit_id"].as_str())
            .unwrap_or("-");
        lines.push(format!(
            "{}  {}  {}  {}",
            fact["recorded_at"].as_str().unwrap_or("-"),
            fact["kind"].as_str().unwrap_or("-"),
            fact["fact_id"].as_str().unwrap_or("-"),
            target,
        ));
    }
    if lines.is_empty() {
        lines.push(copy::lookup("coord.no-facts").to_owned());
    }
    lines.join("\n")
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
