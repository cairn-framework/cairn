//! Unit tests for per-command help routing.

use super::super::{RETIRED_TOP_LEVEL, all_command_names};
use super::*;

#[test]
fn bare_help_flags_resolve_to_global() {
    for argv in [
        args(&["--help"]),
        args(&["-h"]),
        args(&["--json", "--help"]),
        args(&["--file", "x.blueprint", "-h"]),
    ] {
        assert_eq!(help_request(&argv), Some(HelpTarget::Global), "{argv:?}");
    }
}

#[test]
fn command_help_flags_resolve_to_command() {
    let cases = [
        (args(&["neighbourhood", "--help"]), "neighbourhood"),
        (args(&["accept", "-h"]), "accept"),
        (args(&["--json", "frontier", "--help"]), "frontier"),
        // Longest-prefix: change accept, not bare change.
        (args(&["change", "accept", "--help"]), "change accept"),
        (args(&["draft", "list", "-h"]), "draft list"),
        (args(&["todo", "set", "--help"]), "todo set"),
    ];
    for (argv, expected) in cases {
        assert_eq!(
            help_request(&argv),
            Some(HelpTarget::Command(expected)),
            "{argv:?}"
        );
    }
}

#[test]
fn no_help_flag_yields_none() {
    assert_eq!(help_request(&args(&["neighbourhood", "app.api"])), None);
    assert_eq!(help_request(&args(&["--json", "status"])), None);
}

#[test]
fn every_recognised_command_has_help() {
    for name in all_command_names() {
        let text = command_help_text(name)
            .unwrap_or_else(|| panic!("missing help page for registered command `{name}`"));
        assert!(
            text.contains("Usage:"),
            "`{name}` help missing Usage line:\n{text}"
        );
        // Must not be the global command catalogue.
        assert!(
            !text.contains("Commands:\n  backlog"),
            "`{name}` help fell back to global list"
        );
    }
    for name in RETIRED_TOP_LEVEL {
        let text = command_help_text(name)
            .unwrap_or_else(|| panic!("missing help page for retired command `{name}`"));
        assert!(
            !text.contains("Commands:\n  backlog"),
            "retired `{name}` help fell back to global list"
        );
        assert!(
            text.contains("Prefer:"),
            "retired `{name}` help should point at the preferred spelling:\n{text}"
        );
    }
}

#[test]
fn help_specs_cover_all_top_level_and_retired() {
    let spec_names: Vec<&str> = COMMAND_HELP.iter().map(|s| s.name).collect();
    // No duplicates.
    let mut sorted = spec_names.clone();
    sorted.sort_unstable();
    let mut dedup = sorted.clone();
    dedup.dedup();
    assert_eq!(sorted, dedup, "duplicate entries in COMMAND_HELP");

    for name in all_command_names() {
        assert!(
            spec_names.contains(&name),
            "COMMAND_HELP missing top-level `{name}`"
        );
    }
    for name in RETIRED_TOP_LEVEL {
        assert!(
            spec_names.contains(name),
            "COMMAND_HELP missing retired `{name}`"
        );
    }
    // Compound registry tools must also surface.
    for tool in crate::query_api::registry() {
        if tool.cli_name.contains(' ') {
            assert!(
                spec_names.contains(&tool.cli_name),
                "COMMAND_HELP missing compound registry tool `{}`",
                tool.cli_name
            );
        }
    }
}

#[test]
fn neighbourhood_help_lists_command_flags() {
    let text = command_help_text("neighbourhood").expect("neighbourhood help");
    for flag in [
        "--include-research",
        "--include-todos",
        "--include-reviews",
        "--include-deprecated-decisions",
        "--include-changes",
        "--json",
    ] {
        assert!(text.contains(flag), "missing {flag} in:\n{text}");
    }
    assert!(text.contains("<node>"), "missing node arg in:\n{text}");
    assert!(!text.contains("Commands:\n  backlog"));
    // Retired with gh:#236: edges are always shown, the flag is gone.
    assert!(
        !text.contains("--include-orphans"),
        "retired --include-orphans must not be advertised:\n{text}"
    );
}

#[test]
fn accept_help_is_per_command_not_global() {
    let text = command_help_text("accept").expect("accept help");
    assert!(text.contains("accept"), "{text}");
    assert!(text.contains("--json"), "{text}");
    assert!(!text.contains("Commands:\n  backlog"), "{text}");
    assert!(text.contains("change accept"), "{text}");
}

#[test]
fn change_accept_help_is_accept_specific() {
    let text = command_help_text("change accept").expect("change accept help");
    assert!(text.contains("change accept"), "{text}");
    assert!(
        text.contains("[change-id]") || text.contains("change-id"),
        "{text}"
    );
    assert!(text.contains("--json"), "{text}");
    assert!(!text.contains("Commands:\n  backlog"), "{text}");
    // Must not be the generic change family overview.
    assert!(
        !text.contains("new|list|show|accept|apply|archive"),
        "change accept help should not list the whole family:\n{text}"
    );
}

#[test]
fn context_help_lists_depth_and_scope() {
    let text = command_help_text("context").expect("context help");
    assert!(text.contains("--depth"), "{text}");
    assert!(text.contains("--scope"), "{text}");
    assert!(text.contains("--mermaid"), "{text}");
}

#[test]
fn frontier_help_is_per_command() {
    let text = command_help_text("frontier").expect("frontier help");
    assert!(text.contains("frontier"), "{text}");
    assert!(!text.contains("Commands:\n  backlog"), "{text}");
}

#[test]
fn unknown_command_has_no_help_page() {
    assert!(command_help_text("not-a-real-command").is_none());
    assert!(!is_known_help_command("not-a-real-command"));
}

#[test]
fn flag_and_usage_copy_keys_resolve() {
    for spec in COMMAND_HELP {
        for key in spec.flags {
            let path = format!("help.flags.{key}");
            let value = copy::lookup(&path);
            assert_ne!(
                value, path,
                "missing copy key {path} (referenced by `{}`)",
                spec.name
            );
            assert!(
                value.contains("--") || *key == "help",
                "flag copy {path} should document a flag: {value}"
            );
        }
        let usage_key = format!("help.commands.{}.usage", spec.copy_key);
        let usage = copy::lookup(&usage_key);
        assert_ne!(
            usage, usage_key,
            "missing usage copy for `{}` (key {usage_key})",
            spec.name
        );
    }
}

fn args(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

#[test]
fn scan_help_does_not_list_node() {
    let text = command_help_text("scan").expect("scan help");
    assert!(
        !text.contains("--node"),
        "scan does not honour --node; help must not advertise it:\n{text}"
    );
    for flag in ["--strict", "--verbose", "--json", "--file"] {
        assert!(text.contains(flag), "scan help missing {flag}:\n{text}");
    }
}

#[test]
fn lint_help_lists_node() {
    let text = command_help_text("lint").expect("lint help");
    assert!(
        text.contains("--node"),
        "lint help must list --node:\n{text}"
    );
    assert!(text.contains("--strict"), "{text}");
}

#[test]
fn workspace_lint_help_lists_strict_and_verbose() {
    let text = command_help_text("workspace lint").expect("workspace lint help");
    assert!(text.contains("--strict"), "{text}");
    assert!(text.contains("--verbose"), "{text}");
    assert!(text.contains("--json"), "{text}");
    assert!(
        text.contains("Aggregate lint"),
        "workspace lint must use its own description, not the family blurb:\n{text}"
    );
}

#[test]
fn todo_set_help_lists_json_and_own_description() {
    let text = command_help_text("todo set").expect("todo set help");
    assert!(text.contains("--json"), "{text}");
    assert!(
        text.contains("Update the status"),
        "todo set must use its own description:\n{text}"
    );
    assert!(
        !text.contains("Scaffold a new todo"),
        "todo set must not use the family scaffold blurb:\n{text}"
    );
}

#[test]
fn draft_family_help_does_not_list_edited() {
    let text = command_help_text("draft").expect("draft help");
    assert!(
        !text.contains("--edited"),
        "only draft accept honours --edited:\n{text}"
    );
    let accept = command_help_text("draft accept").expect("draft accept help");
    assert!(accept.contains("--edited"), "{accept}");
}

#[test]
fn hook_family_help_does_not_list_pre_push() {
    let text = command_help_text("hook").expect("hook help");
    assert!(
        !text.contains("--pre-push"),
        "only hook install/status/uninstall honour --pre-push:\n{text}"
    );
    let install = command_help_text("hook install").expect("hook install help");
    assert!(install.contains("--pre-push"), "{install}");
}

#[test]
fn change_accept_has_own_description() {
    let text = command_help_text("change accept").expect("change accept help");
    assert!(
        text.contains("Run the acceptance gate"),
        "change accept must not use the family Manage changes blurb:\n{text}"
    );
    assert!(
        !text.contains("Manage changes:"),
        "change accept must not use the family blurb:\n{text}"
    );
}

#[test]
fn pre_command_value_flags_do_not_become_command_tokens() {
    let cases = [
        (args(&["--depth", "2", "context", "--help"]), "context"),
        (
            args(&["--scope", "app.api", "context", "--help"]),
            "context",
        ),
        (args(&["--port", "9999", "ui", "--help"]), "ui"),
        (args(&["--status", "open", "todos", "--help"]), "todos"),
        (args(&["--interval", "3", "watch", "--help"]), "watch"),
        (args(&["--node", "app.api", "lint", "--help"]), "lint"),
    ];
    for (argv, expected) in cases {
        assert_eq!(
            help_request(&argv),
            Some(HelpTarget::Command(expected)),
            "pre-command value flag must not hijack routing: {argv:?}"
        );
    }
}

#[test]
fn refine_help_does_not_list_json() {
    let text = command_help_text("refine").expect("refine help");
    assert!(
        !text.contains("--json"),
        "refine ignores --json (plain text only):\n{text}"
    );
    assert!(
        text.contains("--file"),
        "refine uses project_root from --file:\n{text}"
    );
}

#[test]
fn change_new_help_does_not_list_json() {
    let text = command_help_text("change new").expect("change new help");
    assert!(
        !text.contains("--json"),
        "change new ignores --json:\n{text}"
    );
    assert!(
        text.contains("--file"),
        "change new writes under project_root from --file:\n{text}"
    );
}

#[test]
fn gap_help_lists_json_and_verbose() {
    let text = command_help_text("gap").expect("gap help");
    assert!(text.contains("--json"), "{text}");
    assert!(text.contains("--verbose"), "{text}");
    assert!(text.contains("--question"), "{text}");
}

#[test]
fn feedback_help_lists_json() {
    let text = command_help_text("feedback").expect("feedback help");
    assert!(text.contains("--json"), "{text}");
}

#[test]
fn change_archive_help_lists_verbose() {
    let text = command_help_text("change archive").expect("change archive help");
    assert!(text.contains("--verbose"), "{text}");
    assert!(text.contains("--json"), "{text}");
}

#[test]
fn todo_family_help_options_do_not_list_node() {
    // --node is only consumed by `todo new`. The family page may mention it in
    // the Arguments synopsis, but must not list it under Options.
    let text = command_help_text("todo").expect("todo help");
    let options = text.split("Options:").nth(1).unwrap_or("");
    assert!(
        !options.contains("--node"),
        "family Options must not advertise subcommand-only --node:\n{text}"
    );
    let new_page = command_help_text("todo new").expect("todo new help");
    assert!(new_page.contains("--node"), "{new_page}");
}

#[test]
fn init_help_lists_json_and_file() {
    // project_root comes from --file (mod.rs:104-108); --json is preserved on
    // `init --from-code --apply` via the archive delegate (mod.rs:167-173).
    let text = command_help_text("init").expect("init help");
    assert!(text.contains("--json"), "{text}");
    assert!(text.contains("--file"), "{text}");
    for flag in ["--from-code", "--apply", "--wire", "--force"] {
        assert!(text.contains(flag), "init help missing {flag}:\n{text}");
    }
}

#[test]
fn change_list_help_lists_changes_dir() {
    let text = command_help_text("change list").expect("change list help");
    assert!(text.contains("--changes-dir"), "{text}");
    assert!(text.contains("--file"), "{text}");
    assert!(text.contains("--json"), "{text}");
}

#[test]
fn draft_accept_help_lists_file_and_edited() {
    let text = command_help_text("draft accept").expect("draft accept help");
    assert!(text.contains("--edited"), "{text}");
    assert!(text.contains("--file"), "{text}");
    assert!(text.contains("--changes-dir"), "{text}");
}

#[test]
fn ui_help_lists_file() {
    let text = command_help_text("ui").expect("ui help");
    assert!(
        text.contains("--file"),
        "ui uses parsed.file as blueprint_path:\n{text}"
    );
    assert!(text.contains("--port"), "{text}");
}

#[test]
fn workspace_lint_help_lists_file() {
    let text = command_help_text("workspace lint").expect("workspace lint help");
    assert!(
        text.contains("--file"),
        "workspace root from --file parent:\n{text}"
    );
    assert!(text.contains("--strict"), "{text}");
}

#[test]
fn hook_install_help_lists_file() {
    // Lifecycle hooks receive root from parsed.file.parent() (mod.rs run path
    // + hook.rs:49). Advertise --file alongside --pre-push.
    let text = command_help_text("hook install").expect("hook install help");
    assert!(text.contains("--file"), "{text}");
    assert!(text.contains("--pre-push"), "{text}");
    assert!(text.contains("--json"), "{text}");
}
