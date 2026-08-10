//! CLI onboard command implementation.
// Reason: child module imports re-exported public surface from parent via use super::*
#![allow(clippy::wildcard_imports)]
use super::super::*;

/// Dispatches `cairn onboard [decisions]`.
///
/// With no subcommand the orphan report is unchanged. `decisions` selects the
/// deterministic decision-evidence index. Any other positional subcommand is a
/// usage error rather than a silent fall-back to the orphan report
/// (`dec.brownfield-extraction-mechanism` clause 1).
pub(crate) fn run_onboard_command(parsed: &ParsedArgs) -> CliResult {
    match parsed.command_args.get(1).map(String::as_str) {
        None => run_orphan_report(parsed),
        Some("decisions") => run_decision_evidence(parsed),
        Some(_) => err(
            2,
            &format!("usage: {}", copy::lookup("help.commands.onboard.usage")),
        ),
    }
}

fn onboard_root(parsed: &ParsedArgs) -> &Path {
    parsed
        .file
        .parent()
        .filter(|path| !path.as_os_str().is_empty())
        .unwrap_or_else(|| Path::new("."))
}

fn run_orphan_report(parsed: &ParsedArgs) -> CliResult {
    let root = onboard_root(parsed);

    let (blueprint_path, _temp_dir) = if parsed.file.exists() {
        (parsed.file.clone(), None)
    } else {
        let dir = std::env::temp_dir().join(format!(
            "cairn-onboard-{}",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map_or(0, |d| d.as_nanos())
        ));
        let _ = fs::create_dir_all(&dir);
        let stub = dir.join("cairn.blueprint");
        let _ = fs::write(&stub, "System Stub \"onboard stub\" id \"stub\" {\n}\n");
        (stub, Some(dir))
    };

    match crate::scanner::load_project(root, &blueprint_path) {
        Ok(result) => {
            let report = crate::brownfield::onboard::analyze(&result.graph.findings);
            let output = if parsed.json {
                let inner = crate::brownfield::onboard::render_json(&report);
                let inner = inner.trim();
                format!("{{\"command\":\"onboard\",\"status\":\"ok\",\"data\":{inner}}}\n")
            } else {
                crate::brownfield::onboard::render_human(&report)
            };
            CliResult {
                code: 0,
                stdout: output,
                stderr: String::new(),
            }
        }
        Err(error) => onboard_error("onboard", parsed.json, &error),
    }
}

/// Runs the decision-evidence index.
///
/// Unlike the orphan report this branch never synthesises a stub blueprint: a
/// draft without a real `nodes:` binding does not meet the flow's contract, so
/// an absent or unloadable blueprint is a clear error instead. A blueprint that
/// parses but carries structural errors (duplicate ids, invalid edges) is
/// rejected the same way every map-reading query is: binding evidence against a
/// partial graph would resolve owners the blueprint does not really declare.
fn run_decision_evidence(parsed: &ParsedArgs) -> CliResult {
    let root = onboard_root(parsed);
    if !parsed.file.exists() {
        let message = copy::lookup("onboard.decisions.no-blueprint")
            .replace("{path}", &parsed.file.display().to_string());
        return onboard_error("onboard decisions", parsed.json, &message);
    }
    let result = match crate::scanner::load_project(root, &parsed.file) {
        Ok(result) => result,
        Err(error) => return onboard_error("onboard decisions", parsed.json, &error),
    };
    if result.graph.has_errors() {
        return findings_output(parsed.json, parsed.verbose, &result.graph.findings);
    }
    let report = match crate::brownfield::decisions::index(root, &result.graph) {
        Ok(report) => report,
        Err(error) => {
            return onboard_error("onboard decisions", parsed.json, &error.to_string());
        }
    };
    let output = if parsed.json {
        let inner = crate::brownfield::decisions::render_json(&report);
        let inner = inner.trim();
        format!("{{\"command\":\"onboard decisions\",\"status\":\"ok\",\"data\":{inner}}}\n")
    } else {
        crate::brownfield::decisions::render_human(&report)
    };
    CliResult {
        code: 0,
        stdout: output,
        stderr: String::new(),
    }
}

fn onboard_error(command: &str, json: bool, message: &str) -> CliResult {
    if json {
        CliResult {
            code: 1,
            stdout: format!(
                "{{\"command\":\"{command}\",\"status\":\"error\",\"data\":{{\"message\":\"{}\"}}}}\n",
                format::esc(message)
            ),
            stderr: String::new(),
        }
    } else {
        err(1, message)
    }
}
