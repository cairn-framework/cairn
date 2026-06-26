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
        "feedback",
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
        "/docs/registries/error-codes.md"
    ))
    .unwrap();
    assert!(
        content.contains("CH001"),
        "docs/registries/error-codes.md should list CH001"
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

#[test]
fn test_claude_md_documents_debate_format() {
    let content = fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/CLAUDE.md")).unwrap();
    assert!(
        content.contains("Debate format"),
        "CLAUDE.md must document the debate format convention"
    );
    assert!(
        content.contains("**For**")
            && content.contains("**Against**")
            && content.contains("**Verdict**"),
        "Debate format must include bold For, Against, and Verdict markers"
    );
}

#[test]
fn test_integration_contract_exit_codes_match_cli() {
    let content = fs::read_to_string(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/docs/integration-contract.md"
    ))
    .unwrap();
    // Exit code 1 must be described as blocking (not advisory).
    let exit1_idx = content
        .find("| 1 |")
        .expect("exit code 1 must be documented");
    let exit1_line = content[exit1_idx..exit1_idx + 120].lines().next().unwrap();
    assert!(
        exit1_line.contains("blocking") || exit1_line.contains("Error"),
        "integration contract must describe exit 1 as blocking findings, not advisory: {exit1_line}"
    );
    // --strict must be mentioned.
    assert!(
        content.contains("--strict"),
        "integration contract must document --strict flag"
    );
}

#[test]
fn test_tui_graph_viewer_design_note_exists_and_covers_questions() {
    let content = fs::read_to_string(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/archive/research/tui-graph-viewer.md"
    ))
    .unwrap();
    assert!(
        content.contains("Compatibility") || content.contains("terminal"),
        "design note must cover terminal compatibility"
    );
    assert!(
        content.contains("Library") || content.contains("ratatui") || content.contains("mermaid"),
        "design note must cover library choice"
    );
    assert!(
        content.contains("Scope")
            || content.contains("full-fidelity")
            || content.contains("neighbour"),
        "design note must cover scope"
    );
    assert!(
        content.contains("Pros") || content.contains("Cons"),
        "design note must cover pros and cons"
    );
}

// ---------- issue #70: CLAUDE.md progressive disclosure split ----------

#[test]
fn test_agent_voice_doc_exists() {
    let content =
        fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/docs/agent/voice.md")).unwrap();
    assert!(
        content.contains("em-dash") || content.contains("audience"),
        "voice.md must cover voice and audience guidance"
    );
}

#[test]
fn test_agent_principles_doc_exists() {
    let content = fs::read_to_string(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/docs/agent/principles.md"
    ))
    .unwrap();
    assert!(
        content.contains("artefact") || content.contains("provenance"),
        "principles.md must cover CAIRN positive principles"
    );
}

#[test]
fn test_claude_md_has_pointers_to_agent_subdocs() {
    let content = fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/CLAUDE.md")).unwrap();
    assert!(
        content.contains("docs/agent/voice.md"),
        "CLAUDE.md must point to docs/agent/voice.md"
    );
    assert!(
        content.contains("docs/agent/principles.md"),
        "CLAUDE.md must point to docs/agent/principles.md"
    );
}

#[test]
fn test_claude_md_preserves_load_bearing_rules() {
    let content = fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/CLAUDE.md")).unwrap();
    assert!(
        content.contains("em-dash") || content.contains("Em-dash"),
        "CLAUDE.md must still contain the em-dash ban"
    );
    assert!(
        content.contains("no-verify"),
        "CLAUDE.md must still contain the hook-skip ban"
    );
    assert!(
        content.contains("What to avoid"),
        "CLAUDE.md must still contain the What to avoid section"
    );
}
