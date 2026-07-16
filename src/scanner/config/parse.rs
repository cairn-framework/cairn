//! Hand-rolled cairn.config.yaml parser (single-pass state machine).

use std::collections::BTreeMap;
use std::path::PathBuf;

use super::{Config, GateStep, IntentionalAsymmetry, TargetConfig};
use crate::map::graph::{Finding, FindingSeverity};
use crate::reconcile::target::{VALID_CONFIG_LANGUAGES, language_error_message};

// Reason: config parser is a single-pass state machine with many small
// transitions; extracting every arm would obscure the linear flow.
#[allow(clippy::collapsible_if, clippy::too_many_lines)]
pub(super) fn parse_config(source: &str, config: &mut Config) {
    const KNOWN_TOP_LEVEL_KEYS: &[&str] = &[
        "context",
        "rules",
        "artefact_types",
        "targets",
        "multi_target",
        "ignore",
        "gates",
    ];

    let mut in_ignore = false;
    let mut in_rules = false;
    let mut in_artefacts = false;
    let mut in_targets = false;
    let mut in_asymmetry = false;
    let mut in_asymmetry_targets = false;
    let mut in_gates = false;
    let mut current_target: Option<TargetConfig> = None;
    let mut current_asymmetry: Option<IntentionalAsymmetry> = None;
    let mut current_gate: Option<GateStep> = None;
    let mut warned_keys = std::collections::BTreeSet::new();

    for line in source.lines() {
        let trimmed = line.trim();
        // Top-level key detection: unindented `key:` not in the known set.
        // Require a colon so bare lines / document markers are not treated as keys.
        if indentation(line) == 0
            && !trimmed.is_empty()
            && !trimmed.starts_with('#')
            && !trimmed.starts_with('-')
            && trimmed.contains(':')
            && let Some(key) = trimmed.split(':').next().map(str::trim)
            && !key.is_empty()
            && !KNOWN_TOP_LEVEL_KEYS.contains(&key)
            && warned_keys.insert(key.to_owned())
        {
            let message = if key == "reconcilers" {
                "unknown config key `reconcilers` (and nested `tree_sitter_languages`) is not supported; use a top-level `targets:` override for language, and `ignore:` for ignore patterns".to_owned()
            } else {
                format!(
                    "unknown config key `{key}`; recognised top-level keys are: {}",
                    KNOWN_TOP_LEVEL_KEYS.join(", ")
                )
            };
            config.findings.push(Finding {
                code: "CAIRN_CONFIG_UNKNOWN_KEY".to_owned(),
                severity: FindingSeverity::Warning,
                message,
                node: None,
                target: None,
                path: Some("cairn.config.yaml".to_owned()),
                deferred_by: None,
            });
        }

        if trimmed.starts_with("context:") {
            config.context = value_after_colon(trimmed);
            in_rules = false;
            in_ignore = false;
            in_artefacts = false;
            in_targets = false;
            in_asymmetry = false;
            in_gates = false;
            flush_gate(&mut current_gate, config);
        } else if trimmed.starts_with("rules:") {
            in_rules = true;
            in_ignore = false;
            in_artefacts = false;
            in_targets = false;
            in_asymmetry = false;
            in_gates = false;
            flush_gate(&mut current_gate, config);
        } else if trimmed.starts_with("artefact_types:") {
            in_artefacts = true;
            in_rules = false;
            in_ignore = false;
            in_targets = false;
            in_asymmetry = false;
            in_gates = false;
            flush_gate(&mut current_gate, config);
            config.artefact_types.push_str(trimmed);
            config.artefact_types.push('\n');
        } else if !in_asymmetry && trimmed.starts_with("targets:") {
            in_targets = true;
            in_rules = false;
            in_ignore = false;
            in_artefacts = false;
            in_gates = false;
            flush_gate(&mut current_gate, config);
        } else if trimmed.starts_with("multi_target:") {
            in_asymmetry = true;
            in_rules = false;
            in_ignore = false;
            in_artefacts = false;
            in_targets = false;
            in_gates = false;
            flush_gate(&mut current_gate, config);
        } else if trimmed.starts_with("ignore:") {
            in_ignore = true;
            in_rules = false;
            in_artefacts = false;
            in_targets = false;
            in_asymmetry = false;
            in_gates = false;
            flush_gate(&mut current_gate, config);
        } else if trimmed.starts_with("gates:") {
            in_gates = true;
            config.gates_configured = true;
            in_ignore = false;
            in_rules = false;
            in_artefacts = false;
            in_targets = false;
            in_asymmetry = false;
        } else if in_ignore && trimmed.starts_with('-') {
            config.ignores.push(
                trimmed
                    .trim_start_matches('-')
                    .trim()
                    .trim_matches('"')
                    .to_owned(),
            );
        } else if in_gates {
            if trimmed.starts_with('-') {
                flush_gate(&mut current_gate, config);
                let mut new_gate = GateStep::default();
                let rest = trimmed.trim_start_matches('-').trim();
                if let Some((key, value)) = rest.split_once(':') {
                    let value = value.trim().trim_matches('"').to_owned();
                    match key.trim() {
                        "name" => new_gate.name = value,
                        "command" => new_gate.command = value,
                        _ => {}
                    }
                }
                current_gate = Some(new_gate);
            } else if let Some((key, value)) = trimmed.split_once(':') {
                if let Some(gate) = &mut current_gate {
                    let value = value.trim().trim_matches('"').to_owned();
                    match key.trim() {
                        "name" => gate.name = value,
                        "command" => gate.command = value,
                        _ => {}
                    }
                }
            }
        } else if in_rules && trimmed.contains(':') {
            if let Some((key, value)) = trimmed.split_once(':') {
                config.rules.insert(
                    key.trim().to_owned(),
                    value.trim().trim_matches('"').to_owned(),
                );
            }
        } else if in_artefacts {
            config.artefact_types.push_str(line);
            config.artefact_types.push('\n');
        } else if in_targets {
            if trimmed.starts_with('-') {
                if let Some(target) = current_target.take() {
                    config.targets.push(target);
                }
                let mut new_target = TargetConfig {
                    node_id: String::new(),
                    path: PathBuf::new(),
                    language: String::new(),
                    contract_role: String::new(),
                };
                // Parse an optional inline key-value on the same line as the
                // list marker: `- node: app.api` should set node_id, not discard it.
                let rest = trimmed.trim_start_matches('-').trim();
                if let Some((key, value)) = rest.split_once(':') {
                    let value = value.trim().trim_matches('"').to_owned();
                    match key.trim() {
                        "node" => new_target.node_id = value,
                        "path" => new_target.path = PathBuf::from(value),
                        "language" => new_target.language = value,
                        "contract_role" => new_target.contract_role = value,
                        _ => {}
                    }
                }
                current_target = Some(new_target);
            } else if let Some(target) = &mut current_target
                && let Some((key, value)) = trimmed.split_once(':')
            {
                let value = value.trim().trim_matches('"').to_owned();
                match key.trim() {
                    "node" => target.node_id = value,
                    "path" => target.path = PathBuf::from(value),
                    "language" => {
                        if !VALID_CONFIG_LANGUAGES.contains(&value.as_str()) {
                            eprintln!(
                                "error: unsupported language `{value}`; {}",
                                language_error_message()
                            );
                        }
                        target.language = value;
                    }
                    "contract_role" => target.contract_role = value,
                    _ => {}
                }
            }
        } else if in_asymmetry {
            if trimmed == "intentional_asymmetry:" {
                if let Some(asym) = current_asymmetry.take() {
                    if !asym.node.is_empty() {
                        config.intentional_asymmetries.push(asym);
                    }
                }
                in_asymmetry_targets = false;
                current_asymmetry = Some(IntentionalAsymmetry {
                    node: String::new(),
                    contract_role: String::new(),
                    targets: Vec::new(),
                    reason: String::new(),
                });
            } else if trimmed.starts_with('-') {
                let rest = trimmed.trim_start_matches('-').trim();
                if in_asymmetry_targets {
                    if let Some(asym) = &mut current_asymmetry {
                        asym.targets.push(PathBuf::from(rest.trim_matches('"')));
                    }
                } else {
                    if let Some(asym) = current_asymmetry.take() {
                        if !asym.node.is_empty() {
                            config.intentional_asymmetries.push(asym);
                        }
                    }
                    current_asymmetry = Some(IntentionalAsymmetry {
                        node: String::new(),
                        contract_role: String::new(),
                        targets: Vec::new(),
                        reason: String::new(),
                    });
                    if let Some((key, value)) = rest.split_once(':') {
                        let value = value.trim().trim_matches('"').to_owned();
                        if let Some(asym) = &mut current_asymmetry {
                            match key.trim() {
                                "node" => asym.node = value,
                                "contract_role" => asym.contract_role = value,
                                "reason" => asym.reason = value,
                                _ => {}
                            }
                        }
                    }
                }
            } else if trimmed == "targets:" {
                in_asymmetry_targets = true;
            } else if let Some(asym) = &mut current_asymmetry
                && let Some((key, value)) = trimmed.split_once(':')
            {
                in_asymmetry_targets = false;
                let value = value.trim().trim_matches('"').to_owned();
                match key.trim() {
                    "node" => asym.node = value,
                    "contract_role" => asym.contract_role = value,
                    "reason" => asym.reason = value,
                    _ => {}
                }
            }
        }
    }
    if let Some(target) = current_target.take() {
        config.targets.push(target);
    }
    if let Some(asym) = current_asymmetry.take() {
        if !asym.node.is_empty() {
            config.intentional_asymmetries.push(asym);
        }
    }
    flush_gate(&mut current_gate, config);
    parse_context_rules_blocks(source, config);
}

fn flush_gate(current: &mut Option<GateStep>, config: &mut Config) {
    if let Some(gate) = current.take()
        && (!gate.name.is_empty() || !gate.command.is_empty())
    {
        config.gates.push(gate);
    }
}

fn parse_context_rules_blocks(source: &str, config: &mut Config) {
    let lines = source.lines().collect::<Vec<_>>();
    let mut index = 0;
    while index < lines.len() {
        let line = lines[index];
        let trimmed = line.trim();
        if trimmed.starts_with("context:") {
            let indent = indentation(line);
            let value = value_after_colon(trimmed);
            if matches!(value.as_str(), "|" | ">") {
                let (block, next) = collect_block(&lines, index + 1, indent);
                config.context = block;
                index = next;
                continue;
            }
            config.context = value;
        } else if trimmed == "rules:" {
            let (rules, next) = collect_rules(&lines, index + 1, indentation(line));
            if !rules.is_empty() {
                config.rules = rules;
            }
            index = next;
            continue;
        }
        index += 1;
    }
}

fn collect_rules(
    lines: &[&str],
    start: usize,
    base_indent: usize,
) -> (BTreeMap<String, String>, usize) {
    let mut rules = BTreeMap::new();
    let mut index = start;
    while index < lines.len() {
        let line = lines[index];
        let trimmed = line.trim();
        if trimmed.is_empty() {
            index += 1;
            continue;
        }
        let indent = indentation(line);
        if indent <= base_indent {
            break;
        }
        if let Some((key, raw_value)) = trimmed.split_once(':') {
            let value = raw_value.trim().trim_matches('"').to_owned();
            if matches!(value.as_str(), "|" | ">") {
                let (block, next) = collect_block(lines, index + 1, indent);
                rules.insert(key.trim().to_owned(), block);
                index = next;
                continue;
            }
            rules.insert(key.trim().to_owned(), value);
        }
        index += 1;
    }
    (rules, index)
}

fn collect_block(lines: &[&str], start: usize, base_indent: usize) -> (String, usize) {
    let mut block = Vec::new();
    let mut index = start;
    while index < lines.len() {
        let line = lines[index];
        if !line.trim().is_empty() && indentation(line) <= base_indent {
            break;
        }
        block.push(line.trim_start().to_owned());
        index += 1;
    }
    (block.join("\n").trim_end().to_owned(), index)
}

fn indentation(line: &str) -> usize {
    line.chars()
        .take_while(|character| *character == ' ')
        .count()
}

fn value_after_colon(line: &str) -> String {
    line.split_once(':')
        .map(|(_, value)| value.trim().trim_matches('"').to_owned())
        .unwrap_or_default()
}
