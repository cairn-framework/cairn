//! CLI hook command implementation.
#![allow(clippy::wildcard_imports)]
use super::super::*;

pub(crate) fn run_hook_command(
    parsed: &ParsedArgs,
    root: &Path,
    scan_result: &scanner::ScanResult,
    legacy_warning: String,
) -> CliResult {
    let Some(kind) = parsed
        .command_args
        .get(1)
        .and_then(|value| parse_hook_kind(value))
    else {
        return err(2, "usage: cairn hook <structural|interface|tension|all>");
    };
    let changes_dir = root.join(&parsed.changes_dir);
    let report = hooks::run(kind, root, &changes_dir, scan_result);
    CliResult {
        code: report.exit_code(),
        stdout: if parsed.json {
            hooks::render_json(&report)
        } else {
            hooks::render_human(&report)
        },
        stderr: legacy_warning,
    }
}

pub(crate) fn parse_hook_kind(value: &str) -> Option<HookKind> {
    match value {
        "structural" => Some(HookKind::Structural),
        "interface" => Some(HookKind::Interface),
        "tension" => Some(HookKind::Tension),
        "architecture-decision" => Some(HookKind::ArchitectureDecision),
        "all" => Some(HookKind::All),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_hook_kind_all_valid_strings() {
        use crate::hooks::HookKind;
        assert_eq!(parse_hook_kind("structural"), Some(HookKind::Structural));
        assert_eq!(parse_hook_kind("interface"), Some(HookKind::Interface));
        assert_eq!(parse_hook_kind("tension"), Some(HookKind::Tension));
        assert_eq!(
            parse_hook_kind("architecture-decision"),
            Some(HookKind::ArchitectureDecision)
        );
        assert_eq!(parse_hook_kind("all"), Some(HookKind::All));
    }

    #[test]
    fn test_parse_hook_kind_unknown_returns_none() {
        assert!(parse_hook_kind("unknown").is_none());
        assert!(parse_hook_kind("Structural").is_none()); // case-sensitive
        assert!(parse_hook_kind("").is_none());
    }
}
