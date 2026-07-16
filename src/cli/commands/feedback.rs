//! CLI feedback command implementation.
// Reason: child module imports re-exported public surface from parent via use super::*
#![allow(clippy::wildcard_imports)]
use super::super::*;

/// Upstream issue tracker for `cairn feedback` reports.
const FEEDBACK_ISSUE_BASE: &str = crate::report::ISSUE_BASE;

/// Maximum generated issue title length in characters.
const TITLE_MAX_CHARS: usize = 80;

pub(crate) fn run_feedback_command(parsed: &ParsedArgs, root: &Path) -> CliResult {
    let (message, area, severity) = match parse_feedback_args(&parsed.command_args[1..]) {
        Ok(parts) => parts,
        Err(result) => return result,
    };
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
    let fields = [("area", &area), ("severity", &severity)]
        .iter()
        .filter_map(|(name, value)| value.as_deref().map(|v| format!("{name}: {v}")))
        .collect::<Vec<_>>()
        .join("\n");
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
    if !fields.is_empty() {
        let _ = write!(entry, "\n{fields}\n");
    }
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

    let title = truncate_title(message.lines().next().unwrap_or(""));
    let body = if fields.is_empty() {
        format!("{message}\n\nRecorded by `cairn feedback` (cairn {version}).")
    } else {
        format!("{message}\n\n{fields}\n\nRecorded by `cairn feedback` (cairn {version}).")
    };
    let issue_url = crate::report::issue_url(&title, &body);
    if parsed.json {
        let esc = super::super::format::esc;
        let mut data = String::from("{\"recorded\":\".cairn/feedback.md\"");
        if let Some(area) = &area {
            let _ = write!(data, ",\"area\":\"{}\"", esc(area));
        }
        if let Some(severity) = &severity {
            let _ = write!(data, ",\"severity\":\"{}\"", esc(severity));
        }
        let _ = write!(data, ",\"issue_url\":\"{}\"}}", esc(&issue_url));
        return ok(format!(
            "{{\"command\":\"feedback\",\"status\":\"ok\",\"data\":{data}}}\n"
        ));
    }
    ok(format!(
        "{}\n{}\n{issue_url}\n",
        copy::lookup("feedback.recorded"),
        copy::lookup("feedback.cta")
    ))
}

/// Splits feedback args into the free-form message plus optional `--area`
/// and `--severity` values, so the flags never leak into the title or body.
fn parse_feedback_args(
    args: &[String],
) -> Result<(String, Option<String>, Option<String>), CliResult> {
    let mut area = None;
    let mut severity = None;
    let mut words: Vec<&str> = Vec::new();
    let mut iter = args.iter();
    while let Some(arg) = iter.next() {
        let target = match arg.as_str() {
            "--area" => &mut area,
            "--severity" => &mut severity,
            _ => {
                words.push(arg);
                continue;
            }
        };
        match iter.next() {
            Some(value) if !value.starts_with("--") => *target = Some(value.clone()),
            _ => return Err(err(2, &format!("{arg} requires a value"))),
        }
    }
    Ok((words.join(" ").trim().to_owned(), area, severity))
}

/// Truncates a generated issue title to [`TITLE_MAX_CHARS`] characters,
/// cutting at the last word boundary instead of mid-word. Falls back to a
/// hard cut when the prefix contains no whitespace.
fn truncate_title(line: &str) -> String {
    if line.chars().count() <= TITLE_MAX_CHARS {
        return line.to_owned();
    }
    let prefix: String = line.chars().take(TITLE_MAX_CHARS).collect();
    // A word ending exactly at the limit is already whole; keep it.
    if line
        .chars()
        .nth(TITLE_MAX_CHARS)
        .is_some_and(char::is_whitespace)
    {
        return prefix.trim_end().to_owned();
    }
    match prefix.rfind(char::is_whitespace) {
        Some(index) => prefix[..index].trim_end().to_owned(),
        None => prefix,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn owned(args: &[&str]) -> Vec<String> {
        args.iter().map(|s| (*s).to_owned()).collect()
    }

    #[test]
    fn test_parse_feedback_args_extracts_area_and_severity() {
        let args = owned(&[
            "--area",
            "scanner",
            "scan said X,",
            "expected Y",
            "--severity",
            "high",
        ]);
        let Ok((message, area, severity)) = parse_feedback_args(&args) else {
            panic!("parse failed");
        };
        assert_eq!(message, "scan said X, expected Y");
        assert_eq!(area.as_deref(), Some("scanner"));
        assert_eq!(severity.as_deref(), Some("high"));
    }

    #[test]
    fn test_parse_feedback_args_without_flags_keeps_message() {
        let args = owned(&["scan said X,", "expected Y"]);
        let Ok((message, area, severity)) = parse_feedback_args(&args) else {
            panic!("parse failed");
        };
        assert_eq!(message, "scan said X, expected Y");
        assert_eq!(area, None);
        assert_eq!(severity, None);
    }

    #[test]
    fn test_parse_feedback_args_missing_value_errors() {
        let args = owned(&["something broke", "--area"]);
        let Err(result) = parse_feedback_args(&args) else {
            panic!("expected missing-value error");
        };
        assert_eq!(result.code, 2);
        assert!(result.stderr.contains("--area requires a value"));
    }

    #[test]
    fn test_parse_feedback_args_flag_as_value_errors() {
        let args = owned(&["--area", "--severity", "high", "broke"]);
        let Err(result) = parse_feedback_args(&args) else {
            panic!("expected missing-value error");
        };
        assert_eq!(result.code, 2);
        assert!(result.stderr.contains("--area requires a value"));
    }

    #[test]
    fn test_truncate_title_short_line_unchanged() {
        assert_eq!(truncate_title("scan said X"), "scan said X");
    }

    #[test]
    fn test_truncate_title_cuts_at_word_boundary() {
        let head = "a".repeat(70);
        // The 80-char cut lands inside the final word; the whole word drops.
        let line = format!("{head} unbroken-final-word");
        assert_eq!(truncate_title(&line), head);
    }

    #[test]
    fn test_truncate_title_keeps_word_ending_at_limit() {
        let head = "a".repeat(TITLE_MAX_CHARS);
        let line = format!("{head} tail");
        assert_eq!(truncate_title(&line), head);
    }

    #[test]
    fn test_truncate_title_no_whitespace_hard_cut() {
        let line = "x".repeat(120);
        assert_eq!(truncate_title(&line), "x".repeat(TITLE_MAX_CHARS));
    }
}
