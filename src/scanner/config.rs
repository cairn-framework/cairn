//! Phase 1 configuration and ignore handling.

use std::{collections::BTreeMap, error::Error, fmt, fs, path::Path};

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

fn parse_config(source: &str, config: &mut Config) {
    let mut in_ignore = false;
    let mut in_rules = false;
    let mut in_artefacts = false;
    for line in source.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("context:") {
            config.context = value_after_colon(trimmed);
            in_rules = false;
            in_ignore = false;
            in_artefacts = false;
        } else if trimmed.starts_with("rules:") {
            in_rules = true;
            in_ignore = false;
            in_artefacts = false;
        } else if trimmed.starts_with("artefact_types:") {
            in_artefacts = true;
            in_rules = false;
            in_ignore = false;
            config.artefact_types.push_str(trimmed);
            config.artefact_types.push('\n');
        } else if trimmed.starts_with("ignore:") {
            in_ignore = true;
            in_rules = false;
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
        }
    }
}

fn value_after_colon(line: &str) -> String {
    line.split_once(':')
        .map(|(_, value)| value.trim().trim_matches('"').to_owned())
        .unwrap_or_default()
}

fn is_protected(path: &str) -> bool {
    matches!(path, "cairn.dsl" | "cairn.config.yaml")
        || path.starts_with("meta/")
        || path == "meta"
        || path.starts_with(".cairn/")
        || path == ".cairn"
}
