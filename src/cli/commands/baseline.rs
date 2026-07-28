//! CLI contract-baseline record and drop command.
// Reason: child module imports re-exported public surface from parent via use super::*
#![allow(clippy::wildcard_imports)]
use super::super::*;
use crate::summariser::{BaselineError, describe_baseline_error, drop_baseline, record_baseline};

/// Dispatches `cairn baseline <subcommand> <node>`.
///
/// The non-generative counterpart to the accept-time baseline writer: it needs
/// no summariser backend, so a repository running with the summariser disabled
/// can still record, re-record, and prune contract baselines.
pub(crate) fn run_baseline_command(parsed: &ParsedArgs, root: &Path) -> CliResult {
    if parsed.command_args.len() != 3 {
        return err(2, copy::lookup("baseline.usage"));
    }
    let subcommand = parsed.command_args[1].as_str();
    let node = &parsed.command_args[2];
    match subcommand {
        "record" => run_record(root, &parsed.file, node, parsed.json),
        "drop" => run_drop(root, &parsed.file, node, parsed.json),
        _ => err(2, copy::lookup("baseline.usage")),
    }
}

fn run_record(root: &Path, blueprint_path: &Path, node: &str, json: bool) -> CliResult {
    match record_baseline(root, blueprint_path, node) {
        Err(e) => baseline_error(&e),
        Ok(entry) => {
            if json {
                ok(serde_json::json!({
                    "node": node,
                    "action": "record",
                    "baseline": entry,
                })
                .to_string())
            } else {
                ok(copy::lookup("baseline.recorded").replace("{node}", node))
            }
        }
    }
}

fn run_drop(root: &Path, blueprint_path: &Path, node: &str, json: bool) -> CliResult {
    match drop_baseline(root, blueprint_path, node) {
        Err(e) => baseline_error(&e),
        Ok(()) => {
            if json {
                ok(serde_json::json!({
                    "node": node,
                    "action": "drop",
                })
                .to_string())
            } else {
                ok(copy::lookup("baseline.dropped").replace("{node}", node))
            }
        }
    }
}

/// Renders a failure from `copy.toml`. Every `BaselineError` leaves the state
/// file untouched, so exit 1 with the cause is the whole contract.
fn baseline_error(error: &BaselineError) -> CliResult {
    err(1, &describe_baseline_error(error))
}
