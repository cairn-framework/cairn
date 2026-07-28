//! Phase 1 configuration and ignore handling.

use std::{collections::BTreeMap, error::Error, fmt, fs, path::Path, path::PathBuf};

use crate::map::graph::Finding;

mod parse;
use parse::parse_config;

/// Default accepted-decision count above which
/// `CAIRN_DECISION_ACCUMULATION` fires, overridable with the
/// `decision_accumulation_threshold` config key.
pub(super) const DEFAULT_DECISION_ACCUMULATION_THRESHOLD: usize = 10;

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

/// A single accept-gate verification step from `gates:` in cairn.config.yaml.
///
/// `command` is split on whitespace into program + args (no shell quoting).
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct GateStep {
    /// Display name for the step (shown in accept findings).
    pub name: String,
    /// Command line to run; whitespace-split into argv (no shell).
    pub command: String,
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
    /// Accept-gate steps from a top-level `gates:` block (language-agnostic).
    pub gates: Vec<GateStep>,
    /// True when a top-level `gates:` key was present (even if the list is empty).
    ///
    /// Distinguishes "no gates configured" (fall back by language) from an
    /// explicit empty `gates:` list (run zero language steps).
    pub gates_configured: bool,
    /// Accepted-decision count above which a node is flagged for
    /// consolidation. `None` means the default applies.
    pub decision_accumulation_threshold: Option<usize>,
    /// Non-fatal config findings (unknown keys, etc.).
    pub findings: Vec<Finding>,
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
    for pattern in &mut config.ignores {
        *pattern = pattern.trim().trim_matches('/').to_owned();
    }
    Ok(config)
}

/// Returns true when a relative path is ignored.
#[must_use]
pub fn is_ignored(path: &str, ignores: &[String]) -> bool {
    if is_protected(path) {
        return false;
    }
    ignores.iter().any(|pattern| {
        if pattern.is_empty() {
            return false;
        }
        path == pattern
            || (path.starts_with(pattern) && path.as_bytes().get(pattern.len()) == Some(&b'/'))
            || (path.len() > pattern.len()
                && path.as_bytes()[path.len() - pattern.len() - 1] == b'/'
                && path.ends_with(pattern))
            || (pattern.starts_with("*.") && path.ends_with(&pattern[1..]))
    })
}

fn built_in_ignores() -> Vec<String> {
    [".git", "target", "node_modules", ".DS_Store", ".claude"]
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

#[cfg(test)]
mod tests;
