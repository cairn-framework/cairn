//! Integration test: CLI command registry matches docs/commands.md.
//!
//! Guards against silent code drops like issue #86: documented commands
//! that don't exist in the binary (or vice versa).

use std::fs;

/// Extracts `cairn <command>` names from docs/commands.md tables.
fn documented_commands() -> Vec<String> {
    let content =
        fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/docs/commands.md")).unwrap();
    let mut commands = Vec::new();
    for line in content.lines() {
        // Match table rows like `| `cairn status` | ... |`
        let trimmed = line.trim();
        if trimmed.starts_with("| `cairn ")
            && let Some(start) = trimmed.find("`cairn ")
        {
            let rest = &trimmed[start + 7..];
            if let Some(end) = rest.find('`') {
                let cmd = rest[..end].split_whitespace().next().unwrap_or("");
                if !cmd.is_empty() {
                    commands.push(cmd.to_owned());
                }
            }
        }
    }
    commands.sort_unstable();
    commands.dedup();
    commands
}

/// Extracts CLI command names from the binary registry.
fn registered_commands() -> Vec<String> {
    let mut names: Vec<String> = cairn::cli::registry()
        .iter()
        .filter(|t| t.cli_name != "init_from_code")
        .map(|t| t.cli_name.to_owned())
        .collect();

    // EXTRA_CLI_COMMANDS from src/cli/mod.rs
    let extra = [
        "accept",
        "change",
        "check",
        "export",
        "import-openspec",
        "onboard",
        "refine",
        "watch",
    ];
    for cmd in extra {
        if !names.contains(&cmd.to_owned()) {
            names.push(cmd.to_owned());
        }
    }

    names.sort_unstable();
    names
}

#[test]
fn test_all_documented_commands_exist_in_cli() {
    let registered = registered_commands();
    let missing: Vec<_> = documented_commands()
        .iter()
        .filter(|cmd| !registered.contains(cmd))
        .cloned()
        .collect();

    assert!(
        missing.is_empty(),
        "commands documented in docs/commands.md but missing from CLI: {missing:?}"
    );
}

#[test]
fn test_all_registered_commands_are_documented() {
    let documented = documented_commands();
    let missing: Vec<_> = registered_commands()
        .iter()
        .filter(|cmd| !documented.contains(cmd))
        .cloned()
        .collect();

    assert!(
        missing.is_empty(),
        "commands in CLI but missing from docs/commands.md: {missing:?}"
    );
}

#[test]
fn test_architecture_decision_hook_is_documented() {
    let content =
        fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/docs/commands.md")).unwrap();
    assert!(
        content.contains("architecture-decision"),
        "docs/commands.md should document `cairn hook architecture-decision`"
    );
}

#[test]
fn test_architecture_decision_hook_in_integration_contract() {
    let content = fs::read_to_string(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/docs/integration-contract.md"
    ))
    .unwrap();
    assert!(
        content.contains("architecture-decision"),
        "docs/integration-contract.md should list architecture-decision hook kind"
    );
}

#[test]
fn test_ch001_in_error_registry() {
    let content = fs::read_to_string(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/openspec/registries/error-codes.md"
    ))
    .unwrap();
    assert!(
        content.contains("CH001"),
        "openspec/registries/error-codes.md should list CH001"
    );
}

#[test]
fn test_hooks_doc_lists_architecture_decision() {
    let content =
        fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/docs/hooks.md")).unwrap();
    assert!(
        content.contains("architecture-decision"),
        "docs/hooks.md should document architecture-decision hook"
    );
}

#[test]
fn test_command_reference_doc_up_to_date() {
    // If this test fails, a command was added or removed without updating
    // docs/commands.md. The error message lists the delta.
    let doc = documented_commands();
    let reg = registered_commands();
    assert_eq!(
        doc, reg,
        "docs/commands.md command list diverges from CLI registry"
    );
}

#[test]
fn test_all_registered_commands_in_integration_contract() {
    let content = fs::read_to_string(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/docs/integration-contract.md"
    ))
    .unwrap();
    let missing: Vec<_> = registered_commands()
        .iter()
        .filter(|cmd| !content.contains(&format!("`{cmd}")))
        .cloned()
        .collect();
    assert!(
        missing.is_empty(),
        "registered commands missing from integration-contract.md: {missing:?}"
    );
}
