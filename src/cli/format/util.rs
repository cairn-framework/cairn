//! Utility helpers for CLI formatting: argument parsing, graph queries, and output builders.
// Reason: child module imports re-exported public surface from parent via use super::*
#![allow(clippy::wildcard_imports)]
use super::super::*;

pub(crate) fn node_arg(args: &[String]) -> Result<&str, Finding> {
    args.get(1).map(String::as_str).ok_or_else(|| Finding {
        code: "CAIRN_CLI_MISSING_NODE".to_owned(),
        severity: FindingSeverity::Error,
        message: "node argument is required".to_owned(),
        node: None,
        target: None,
        path: None,
        deferred_by: None,
    })
}

/// Finds the positional node token: the first non-flag token after the
/// command name, skipping the values of value-taking flags so that
/// `todos --status open app.kernel` resolves `app.kernel`, not `open`.
/// Returns `None` when no positional token is present.
pub(crate) fn positional_node(args: &[String]) -> Option<&String> {
    // Flags that consume the following token as their value.
    const VALUE_FLAGS: &[&str] = &["--direction", "--grep", "--language", "--status"];
    let mut tokens = args.iter().skip(1);
    while let Some(token) = tokens.next() {
        if VALUE_FLAGS.contains(&token.as_str()) {
            tokens.next();
        } else if !token.starts_with("--") {
            return Some(token);
        }
    }
    None
}

pub(crate) fn flag_value<'a>(args: &'a [String], flag: &str) -> Option<&'a str> {
    args.windows(2)
        .find_map(|pair| (pair[0] == flag).then_some(pair[1].as_str()))
}

pub(crate) fn findings_output(json: bool, verbose: bool, findings: &[Finding]) -> CliResult {
    CliResult {
        code: 1,
        stdout: super::render::render_findings(findings, json, verbose),
        stderr: String::new(),
    }
}

pub(crate) fn finding_output(json: bool, verbose: bool, finding: Finding) -> CliResult {
    findings_output(json, verbose, &[finding])
}

pub(crate) fn error_output(json: bool, code: &str, message: &str) -> CliResult {
    let finding = Finding {
        code: code.to_owned(),
        severity: FindingSeverity::Error,
        message: message.to_owned(),
        node: None,
        target: None,
        path: None,
        deferred_by: None,
    };
    finding_output(json, false, finding)
}

pub(crate) fn ok(stdout: String) -> CliResult {
    CliResult {
        code: 0,
        stdout,
        stderr: String::new(),
    }
}

/// Build an error `CliResult` with the given exit code and message.
///
/// # Exit codes
///
/// - **0**: success (clean, no findings)
/// - **1**: success with advisory findings, or operational error
/// - **2**: argument/usage error (bad args, unknown command)
pub(crate) fn err(code: u8, message: &str) -> CliResult {
    CliResult {
        code,
        stdout: String::new(),
        stderr: format!("{message}\n"),
    }
}

pub(crate) fn lines(values: &[String]) -> String {
    if values.is_empty() {
        "None".to_owned()
    } else {
        let mut out = String::new();
        for (i, value) in values.iter().enumerate() {
            if i > 0 {
                out.push('\n');
            }
            let _ = write!(out, "- {value}");
        }
        out
    }
}

pub(crate) fn esc(value: &str) -> std::borrow::Cow<'_, str> {
    if value.bytes().all(|b| {
        b != b'\\'
            && b != b'"'
            && b != b'\n'
            && b != b'\r'
            && b != b'\t'
            && b != b'\x08'
            && b != b'\x0C'
    }) {
        return std::borrow::Cow::Borrowed(value);
    }
    std::borrow::Cow::Owned(
        value
            .replace('\\', "\\\\")
            .replace('"', "\\\"")
            .replace('\n', "\\n")
            .replace('\r', "\\r")
            .replace('\t', "\\t")
            .replace('\u{08}', "\\b")
            .replace('\u{0C}', "\\f"),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    // ── esc ───────────────────────────────────────────────────────────────────

    #[test]
    fn test_esc_empty_string() {
        assert_eq!(esc(""), "");
    }

    #[test]
    fn test_esc_no_special_chars() {
        assert_eq!(esc("hello world"), "hello world");
    }

    #[test]
    fn test_esc_backslash() {
        assert_eq!(esc("a\\b"), "a\\\\b");
    }

    #[test]
    fn test_esc_double_quote() {
        assert_eq!(esc(r#"say "hi""#), r#"say \"hi\""#);
    }

    #[test]
    fn test_esc_newline() {
        assert_eq!(esc("line1\nline2"), "line1\\nline2");
    }

    #[test]
    fn test_esc_carriage_return() {
        assert_eq!(esc("a\rb"), "a\\rb");
    }

    #[test]
    fn test_esc_tab() {
        assert_eq!(esc("a\tb"), "a\\tb");
    }

    #[test]
    fn test_esc_backspace() {
        assert_eq!(esc("a\u{08}b"), "a\\bb");
    }

    #[test]
    fn test_esc_form_feed() {
        assert_eq!(esc("a\u{0C}b"), "a\\fb");
    }

    #[test]
    fn test_esc_backslash_is_escaped_first_so_replacements_do_not_double_escape() {
        // Input: the two chars '\\' and 'n' (a literal backslash followed by 'n').
        // esc must produce '\\\\n' (JSON: literal backslash then 'n'), NOT '\\\\\\\\n'.
        // If the newline-replacement step ran before the backslash step it would
        // turn '\\\\' (already escaped backslash) into '\\\\\\\\' — double escaping.
        assert_eq!(esc("\\n"), "\\\\n");
    }

    #[test]
    fn test_esc_all_specials_in_one_value() {
        let input = "a\\\"\n\r\t\u{08}\u{0C}z";
        let want = "a\\\\\\\"\\n\\r\\t\\b\\fz";
        assert_eq!(esc(input), want);
    }

    // ── lines ─────────────────────────────────────────────────────────────────

    #[test]
    fn test_lines_empty_returns_none() {
        assert_eq!(lines(&[]), "None");
    }

    #[test]
    fn test_lines_single_value() {
        assert_eq!(lines(&["foo".to_owned()]), "- foo");
    }

    #[test]
    fn test_lines_multiple_values_joined_by_newline() {
        assert_eq!(lines(&["a".to_owned(), "b".to_owned()]), "- a\n- b");
    }

    // ── flag_value ───────────────────────────────────────────────────────────

    #[test]
    fn test_flag_value_found() {
        let args = args(&["--format", "json"]);
        assert_eq!(flag_value(&args, "--format"), Some("json"));
    }

    #[test]
    fn test_flag_value_not_present_returns_none() {
        let args = args(&["--node", "app.api"]);
        assert_eq!(flag_value(&args, "--format"), None);
    }

    #[test]
    fn test_flag_value_flag_at_last_position_returns_none_not_panic() {
        // flag is present but has no following argument.
        let args = args(&["--node", "app.api", "--format"]);
        assert_eq!(flag_value(&args, "--format"), None);
    }

    #[test]
    fn test_flag_value_empty_args_returns_none() {
        assert_eq!(flag_value(&[], "--format"), None);
    }

    #[test]
    fn test_flag_value_returns_first_occurrence_when_repeated() {
        let args = args(&["--format", "json", "--format", "mermaid"]);
        assert_eq!(flag_value(&args, "--format"), Some("json"));
    }

    // ── positional_node ──────────────────────────────────────────────────────

    #[test]
    fn test_positional_node_returns_trailing_node_after_value_flag() {
        let a = args(&["todos", "--status", "open", "app.kernel"]);
        assert_eq!(positional_node(&a), Some(&"app.kernel".to_owned()));
    }

    #[test]
    fn test_positional_node_returns_leading_node_before_flags() {
        let a = args(&["todos", "app.kernel", "--status", "open"]);
        assert_eq!(positional_node(&a), Some(&"app.kernel".to_owned()));
    }

    #[test]
    fn test_positional_node_flag_only_invocation_returns_none() {
        let a = args(&["todos", "--status", "open"]);
        assert_eq!(positional_node(&a), None);
    }

    #[test]
    fn test_positional_node_bare_command_returns_none() {
        let a = args(&["todos"]);
        assert_eq!(positional_node(&a), None);
    }

    #[test]
    fn test_positional_node_skips_boolean_flag_but_not_its_neighbour() {
        // `--transitive` takes no value, so the following token is the node.
        let a = args(&["rationale", "--transitive", "app.kernel"]);
        assert_eq!(positional_node(&a), Some(&"app.kernel".to_owned()));
    }

    // ── node_arg ─────────────────────────────────────────────────────────────

    #[test]
    fn test_node_arg_returns_second_element() {
        // args[0] is the subcommand name, args[1] is the node id.
        let a = args(&["node", "app.api"]);
        assert_eq!(node_arg(&a), Ok("app.api"));
    }

    #[test]
    fn test_node_arg_missing_returns_cli_missing_node_finding() {
        let a = args(&["node"]);
        let err = node_arg(&a).unwrap_err();
        assert_eq!(err.code, "CAIRN_CLI_MISSING_NODE");
    }

    #[test]
    fn test_node_arg_empty_slice_returns_error() {
        let err = node_arg(&[]).unwrap_err();
        assert_eq!(err.code, "CAIRN_CLI_MISSING_NODE");
    }

    // ── err / ok ─────────────────────────────────────────────────────────────

    #[test]
    fn test_err_result_has_given_exit_code_and_stderr_message() {
        let result = err(2, "bad args");
        assert_eq!(result.code, 2);
        assert_eq!(result.stderr, "bad args\n");
        assert_eq!(result.stdout, "");
    }

    #[test]
    fn test_ok_result_has_exit_code_zero_and_stdout_message() {
        let result = ok("done\n".to_owned());
        assert_eq!(result.code, 0);
        assert_eq!(result.stdout, "done\n");
        assert_eq!(result.stderr, "");
    }

    // ── helpers ───────────────────────────────────────────────────────────────

    fn args(strs: &[&str]) -> Vec<String> {
        strs.iter().map(|s| (*s).to_owned()).collect()
    }
}
