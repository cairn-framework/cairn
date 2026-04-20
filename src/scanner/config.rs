//! Phase 1 configuration and ignore handling.

use std::{collections::BTreeMap, error::Error, fmt, fs, path::Path, path::PathBuf};

use crate::reconcile::target::{SUPPORTED_LANGUAGES, language_error_message};

/// Target configuration from cairn.config.yaml.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct TargetConfig {
    /// Node ID for this target.
    pub node_id: String,
    /// Path for this target.
    pub path: PathBuf,
    /// Language override for this target.
    pub language: String,
    /// Contract role for this target.
    pub contract_role: String,
}

/// Intentional asymmetry entry marking specific targets as intentionally divergent.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct IntentionalAsymmetry {
    /// Node ID for this asymmetry.
    pub node: String,
    /// Contract role this asymmetry applies to.
    pub contract_role: String,
    /// Target paths that are intentionally asymmetric.
    pub targets: Vec<PathBuf>,
    /// Human-readable reason for the asymmetry.
    pub reason: String,
}

/// Loaded configuration.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Config {
    /// Combined ignore patterns.
    pub ignores: Vec<String>,
    /// Raw context text.
    pub context: String,
    /// Raw rules map.
    pub rules: BTreeMap<String, String>,
    /// Retained raw artefact type section.
    pub artefact_types: String,
    /// Target configurations.
    pub targets: Vec<TargetConfig>,
    /// Intentional asymmetry entries.
    pub intentional_asymmetries: Vec<IntentionalAsymmetry>,
}

/// Config load error.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ConfigError {
    /// Stable code.
    pub code: String,
    /// Message.
    pub message: String,
}

impl fmt::Display for ConfigError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}: {}", self.code, self.message)
    }
}

impl Error for ConfigError {}

/// Loads config and layered ignore rules.
///
/// # Errors
///
/// Returns a config error when a config or ignore file cannot be read.
pub fn load(root: &Path) -> Result<Config, ConfigError> {
    let mut config = Config {
        ignores: built_in_ignores(),
        ..Config::default()
    };
    config
        .ignores
        .extend(load_ignore_file(&root.join(".gitignore"))?);
    config
        .ignores
        .extend(load_ignore_file(&root.join(".cairnignore"))?);
    let config_path = root.join("cairn.config.yaml");
    if config_path.exists() {
        let source = fs::read_to_string(&config_path).map_err(|error| ConfigError {
            code: "CAIRN_CONFIG_READ_FAILED".to_owned(),
            message: format!("failed to read {}: {error}", config_path.display()),
        })?;
        parse_config(&source, &mut config);
    }
    config.ignores.sort();
    config.ignores.dedup();
    Ok(config)
}

/// Returns true when a relative path is ignored.
#[must_use]
pub fn is_ignored(path: &str, ignores: &[String]) -> bool {
    if is_protected(path) {
        return false;
    }
    ignores.iter().any(|pattern| {
        let pattern = pattern.trim().trim_matches('/');
        if pattern.is_empty() {
            return false;
        }
        path == pattern
            || path.starts_with(&format!("{pattern}/"))
            || path.ends_with(&format!("/{pattern}"))
            || (pattern.starts_with("*.") && path.ends_with(&pattern[1..]))
    })
}

fn built_in_ignores() -> Vec<String> {
    [".git", "target", "node_modules", ".DS_Store"]
        .iter()
        .map(ToString::to_string)
        .collect()
}

fn load_ignore_file(path: &Path) -> Result<Vec<String>, ConfigError> {
    if !path.exists() {
        return Ok(Vec::new());
    }
    let source = fs::read_to_string(path).map_err(|error| ConfigError {
        code: "CAIRN_IGNORE_READ_FAILED".to_owned(),
        message: format!("failed to read {}: {error}", path.display()),
    })?;
    Ok(source
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty() && !line.starts_with('#'))
        .map(ToOwned::to_owned)
        .collect())
}

#[allow(clippy::collapsible_if, clippy::too_many_lines)]
fn parse_config(source: &str, config: &mut Config) {
    let mut in_ignore = false;
    let mut in_rules = false;
    let mut in_artefacts = false;
    let mut in_targets = false;
    let mut in_asymmetry = false;
    let mut in_asymmetry_targets = false;
    let mut current_target: Option<TargetConfig> = None;
    let mut current_asymmetry: Option<IntentionalAsymmetry> = None;

    for line in source.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("context:") {
            config.context = value_after_colon(trimmed);
            in_rules = false;
            in_ignore = false;
            in_artefacts = false;
            in_targets = false;
            in_asymmetry = false;
        } else if trimmed.starts_with("rules:") {
            in_rules = true;
            in_ignore = false;
            in_artefacts = false;
            in_targets = false;
            in_asymmetry = false;
        } else if trimmed.starts_with("artefact_types:") {
            in_artefacts = true;
            in_rules = false;
            in_ignore = false;
            in_targets = false;
            in_asymmetry = false;
            config.artefact_types.push_str(trimmed);
            config.artefact_types.push('\n');
        } else if !in_asymmetry && trimmed.starts_with("targets:") {
            in_targets = true;
            in_rules = false;
            in_ignore = false;
            in_artefacts = false;
        } else if trimmed.starts_with("multi_target:") {
            in_asymmetry = true;
            in_rules = false;
            in_ignore = false;
            in_artefacts = false;
            in_targets = false;
        } else if trimmed.starts_with("ignore:") {
            in_ignore = true;
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
                current_target = Some(TargetConfig {
                    node_id: String::new(),
                    path: PathBuf::new(),
                    language: String::new(),
                    contract_role: String::new(),
                });
            } else if let Some(ref mut target) = current_target
                && let Some((key, value)) = trimmed.split_once(':')
            {
                let value = value.trim().trim_matches('"').to_owned();
                match key.trim() {
                    "node" => target.node_id = value,
                    "path" => target.path = PathBuf::from(value),
                    "language" => {
                        if !SUPPORTED_LANGUAGES.contains(&value.as_str()) {
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
                    if let Some(ref mut asym) = current_asymmetry {
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
                        if let Some(ref mut asym) = current_asymmetry {
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
            } else if let Some(ref mut asym) = current_asymmetry
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
}

fn value_after_colon(line: &str) -> String {
    line.split_once(':')
        .map(|(_, value)| value.trim().trim_matches('"').to_owned())
        .unwrap_or_default()
}

fn is_protected(path: &str) -> bool {
    matches!(path, "cairn.blueprint" | "cairn.config.yaml")
        || path.starts_with("meta/")
        || path == "meta"
        || path.starts_with(".cairn/")
        || path == ".cairn"
}

impl IntentionalAsymmetry {
    /// Returns true when this asymmetry matches the given node, contract role, and paths.
    #[must_use]
    pub fn matches(&self, node: &str, contract_role: &str, paths: &[&std::path::PathBuf]) -> bool {
        if self.node != node || self.contract_role != contract_role {
            return false;
        }
        if self.targets.len() != paths.len() {
            return false;
        }
        for path in paths {
            if !self.targets.contains(path) {
                return false;
            }
        }
        true
    }
}

impl Config {
    /// Returns the intentional asymmetry entry if one matches the given node, contract role, and paths.
    #[must_use]
    pub fn is_intentional_asymmetry(
        &self,
        node: &str,
        contract_role: &str,
        paths: &[&std::path::PathBuf],
    ) -> Option<&IntentionalAsymmetry> {
        self.intentional_asymmetries
            .iter()
            .find(|asym| asym.matches(node, contract_role, paths))
    }
}
