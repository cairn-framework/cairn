//! CLI feedback command implementation.
// Reason: child module imports re-exported public surface from parent via use super::*
#![allow(clippy::wildcard_imports)]
use super::super::*;

/// Upstream issue tracker for `cairn feedback` reports.
const FEEDBACK_ISSUE_BASE: &str = "https://github.com/cairn-framework/cairn/issues/new";

pub(crate) fn run_feedback_command(parsed: &ParsedArgs, root: &Path) -> CliResult {
    let message = parsed.command_args[1..].join(" ").trim().to_owned();
    if message.is_empty() {
        return err(2, copy::lookup("feedback.usage"));
    }
    let log_path = root.join(".cairn/feedback.md");
    if let Some(parent) = log_path.parent()
        && let Err(error) = fs::create_dir_all(parent)
    {
        return err(
            1,
            &format!("failed to create {}: {error}", parent.display()),
        );
    }
    let timestamp = super::super::export::current_timestamp_rfc3339();
    let version = env!("CARGO_PKG_VERSION");
    let mut entry = if log_path.exists() {
        String::new()
    } else {
        format!(
            "# Cairn feedback log\n\nFriction recorded by `cairn feedback`. \
             Triage entries into upstream issues at\n\
             {FEEDBACK_ISSUE_BASE}\n"
        )
    };
    let _ = write!(entry, "\n## {timestamp} (cairn {version})\n\n{message}\n");
    let appended = {
        use std::io::Write as _;
        fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(&log_path)
            .and_then(|mut file| file.write_all(entry.as_bytes()))
    };
    if let Err(error) = appended {
        return err(
            1,
            &format!("failed to write {}: {error}", log_path.display()),
        );
    }

    let title: String = message
        .lines()
        .next()
        .unwrap_or("")
        .chars()
        .take(80)
        .collect();
    let body = format!("{message}\n\nRecorded by `cairn feedback` (cairn {version}).");
    let issue_url = format!(
        "{FEEDBACK_ISSUE_BASE}?title={}&body={}",
        encode_query_component(&title),
        encode_query_component(&body)
    );
    if parsed.json {
        return ok(format!(
            "{{\"command\":\"feedback\",\"status\":\"ok\",\"data\":{{\"recorded\":\".cairn/feedback.md\",\"issue_url\":\"{}\"}}}}\n",
            super::super::format::esc(&issue_url)
        ));
    }
    ok(format!(
        "{}\n{}\n{issue_url}\n",
        copy::lookup("feedback.recorded"),
        copy::lookup("feedback.cta")
    ))
}

/// Percent-encodes a string for use as a URL query parameter value.
fn encode_query_component(value: &str) -> String {
    let mut out = String::with_capacity(value.len());
    for byte in value.bytes() {
        match byte {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'.' | b'_' | b'~' => {
                out.push(byte as char);
            }
            _ => {
                let _ = write!(out, "%{byte:02X}");
            }
        }
    }
    out
}
