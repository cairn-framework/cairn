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
#[derive(Clone, Debug, Eq, PartialEq)]
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
    /// Selected state backend (e.g., "filesystem", "beads").
    pub state_backend: String,
}
impl Default for Config {
    fn default() -> Self {
        Self {
            ignores: Vec::new(),
            context: String::new(),
            rules: BTreeMap::new(),
            artefact_types: String::new(),
            targets: Vec::new(),
            intentional_asymmetries: Vec::new(),
            state_backend: "filesystem".to_owned(),
        }
    }
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

// Reason: config parser is a single-pass state machine with many small
// transitions; extracting every arm would obscure the linear flow.
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
        } else if trimmed.starts_with("state_backend:") {
            config.state_backend = value_after_colon(trimmed);
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
    parse_context_rules_blocks(source, config);
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
mod tests {
    use super::*;

    fn ignores(patterns: &[&str]) -> Vec<String> {
        patterns.iter().map(ToString::to_string).collect()
    }

    // ── is_ignored ────────────────────────────────────────────────────────────

    #[test]
    fn test_exact_match() {
        assert!(is_ignored("foo.txt", &ignores(&["foo.txt"])));
    }

    #[test]
    fn test_directory_prefix_match() {
        // "src" must match "src/main.rs" and all files under src/.
        assert!(is_ignored("src/main.rs", &ignores(&["src"])));
        assert!(is_ignored("src/deep/nested.rs", &ignores(&["src"])));
    }

    #[test]
    fn test_suffix_match() {
        // "node_modules" must match "lib/node_modules" — useful for
        // mono-repos where the pattern appears at any depth.
        assert!(is_ignored("lib/node_modules", &ignores(&["node_modules"])));
        assert!(is_ignored(
            "packages/app/node_modules",
            &ignores(&["node_modules"])
        ));
    }

    #[test]
    fn test_glob_extension_match() {
        assert!(is_ignored("dist/bundle.js", &ignores(&["*.js"])));
        assert!(is_ignored("src/image.png", &ignores(&["*.png"])));
    }

    #[test]
    fn test_glob_does_not_match_wrong_extension() {
        // "*.js" must not match ".js.map" files — suffix must be exact.
        assert!(!is_ignored("file.js.map", &ignores(&["*.js"])));
    }

    #[test]
    fn test_no_match_returns_false() {
        assert!(!is_ignored("src/main.rs", &ignores(&["dist"])));
        assert!(!is_ignored("src/main.rs", &ignores(&["*.js"])));
    }

    #[test]
    fn test_partial_prefix_not_matched() {
        // "src" must not match "srcmap/main.rs" — prefix check requires trailing slash.
        assert!(!is_ignored("srcmap/main.rs", &ignores(&["src"])));
    }

    #[test]
    fn test_empty_pattern_is_skipped() {
        assert!(!is_ignored("foo.txt", &ignores(&[""])));
        // slash-only pattern trims to empty and is also skipped
        assert!(!is_ignored("foo.txt", &ignores(&["/"])));
    }

    #[test]
    fn test_trailing_slash_in_pattern_is_trimmed() {
        // "src/" must behave identically to "src".
        assert!(is_ignored("src/main.rs", &ignores(&["src/"])));
        assert!(is_ignored("src", &ignores(&["src/"])));
    }

    // ── is_protected (prevents ignoring sentinel files) ───────────────────────

    #[test]
    fn test_blueprint_file_is_protected() {
        assert!(!is_ignored(
            "cairn.blueprint",
            &ignores(&["cairn.blueprint"])
        ));
    }

    #[test]
    fn test_config_file_is_protected() {
        assert!(!is_ignored(
            "cairn.config.yaml",
            &ignores(&["cairn.config.yaml"])
        ));
    }

    #[test]
    fn test_meta_prefix_is_protected() {
        assert!(!is_ignored("meta/decisions/d1.md", &ignores(&["meta"])));
        assert!(!is_ignored("meta", &ignores(&["meta"])));
    }

    #[test]
    fn test_cairn_hidden_dir_is_protected() {
        assert!(!is_ignored(".cairn/state.json", &ignores(&[".cairn"])));
        assert!(!is_ignored(".cairn", &ignores(&[".cairn"])));
    }

    #[test]
    fn test_normal_paths_are_not_protected() {
        // Sanity: protection only applies to the sentinel paths.
        assert!(is_ignored("target", &ignores(&["target"])));
        assert!(is_ignored("dist/app.js", &ignores(&["*.js"])));
    }

    #[test]
    fn test_meta_prefix_not_matched_for_metadata() {
        // "metadata/" starts with "meta" but not "meta/" — must not be protected.
        assert!(is_ignored("metadata/foo.txt", &ignores(&["metadata"])));
    }

    // ── parse_config ──────────────────────────────────────────────────────────

    #[test]
    fn test_parse_config_context_field() {
        let mut config = Config::default();
        parse_config("context: \"System-level AI agent\"\n", &mut config);
        assert_eq!(config.context, "System-level AI agent");
    }

    #[test]
    fn test_parse_config_state_backend_field() {
        let mut config = Config::default();
        parse_config("state_backend: beads\n", &mut config);
        assert_eq!(config.state_backend, "beads");
    }

    #[test]
    fn test_parse_config_ignore_list() {
        let mut config = Config::default();
        parse_config("ignore:\n  - dist\n  - \"*.lock\"\n", &mut config);
        assert!(config.ignores.contains(&"dist".to_owned()));
        assert!(config.ignores.contains(&"*.lock".to_owned()));
    }

    #[test]
    fn test_parse_config_rules_map() {
        let mut config = Config::default();
        parse_config("rules:\n  tone: concise\n  format: json\n", &mut config);
        assert_eq!(
            config.rules.get("tone").map(String::as_str),
            Some("concise")
        );
        assert_eq!(config.rules.get("format").map(String::as_str), Some("json"));
    }

    #[test]
    fn test_parse_config_targets() {
        let mut config = Config::default();
        parse_config(
            "targets:\n  - node: app.api\n    path: src/api\n    language: rust\n",
            &mut config,
        );
        assert_eq!(config.targets.len(), 1);
        assert_eq!(config.targets[0].node_id, "app.api");
        assert_eq!(config.targets[0].path, std::path::PathBuf::from("src/api"));
        assert_eq!(config.targets[0].language, "rust");
    }

    // ── load_ignore_file ──────────────────────────────────────────────────────

    #[test]
    fn test_load_ignore_file_strips_comments_and_blanks() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(
            dir.path().join(".cairnignore"),
            "# comment\ndist\n\n*.log\n",
        )
        .unwrap();
        let patterns = load_ignore_file(&dir.path().join(".cairnignore")).unwrap();
        assert!(
            !patterns.iter().any(|p| p.starts_with('#')),
            "comments must be stripped"
        );
        assert!(patterns.contains(&"dist".to_owned()));
        assert!(patterns.contains(&"*.log".to_owned()));
        assert!(
            !patterns.contains(&String::new()),
            "blank lines must be stripped"
        );
    }

    #[test]
    fn test_load_ignore_file_missing_returns_empty() {
        let dir = tempfile::tempdir().unwrap();
        let patterns = load_ignore_file(&dir.path().join(".cairnignore")).unwrap();
        assert!(patterns.is_empty());
    }

    // ── IntentionalAsymmetry::matches ─────────────────────────────────────────

    #[test]
    fn test_asymmetry_matches_exact_node_role_and_paths() {
        let p = std::path::PathBuf::from("src/api.ts");
        let asym = IntentionalAsymmetry {
            node: "app.api".to_owned(),
            contract_role: "public_api".to_owned(),
            targets: vec![p.clone()],
            reason: "generated".to_owned(),
        };
        assert!(asym.matches("app.api", "public_api", &[&p]));
    }

    #[test]
    fn test_asymmetry_rejects_wrong_node() {
        let p = std::path::PathBuf::from("src/api.ts");
        let asym = IntentionalAsymmetry {
            node: "app.api".to_owned(),
            contract_role: "public_api".to_owned(),
            targets: vec![p.clone()],
            reason: "generated".to_owned(),
        };
        assert!(!asym.matches("app.db", "public_api", &[&p]));
    }

    #[test]
    fn test_asymmetry_rejects_wrong_role() {
        let p = std::path::PathBuf::from("src/api.ts");
        let asym = IntentionalAsymmetry {
            node: "app.api".to_owned(),
            contract_role: "public_api".to_owned(),
            targets: vec![p.clone()],
            reason: "generated".to_owned(),
        };
        assert!(!asym.matches("app.api", "internal", &[&p]));
    }

    #[test]
    fn test_asymmetry_rejects_extra_path() {
        let p1 = std::path::PathBuf::from("src/a.ts");
        let p2 = std::path::PathBuf::from("src/b.ts");
        let asym = IntentionalAsymmetry {
            node: "app.api".to_owned(),
            contract_role: "public_api".to_owned(),
            targets: vec![p1.clone()],
            reason: "generated".to_owned(),
        };
        // targets has p1 only; passing [p1, p2] is a different set.
        assert!(!asym.matches("app.api", "public_api", &[&p1, &p2]));
    }
}
