//! Summariser configuration loader.
//!
//! Reads the `summariser` section from `cairn.config.yaml` and deserialises
//! it into a strongly-typed shape. The section is optional; when absent the
//! summariser defaults to disabled mode.

use std::{fs, path::Path};

/// Raw container so serde can parse the nested `summariser:` key.
#[derive(Clone, Debug, Default, Eq, PartialEq, serde::Deserialize)]
struct RootConfig {
    #[serde(default)]
    summariser: SummariserConfig,
}

/// Summariser-specific configuration inside `cairn.config.yaml`.
#[derive(Clone, Debug, Default, Eq, PartialEq, serde::Deserialize)]
struct SummariserConfig {
    #[serde(default)]
    mode: String,
    #[serde(default = "default_timeout_ms")]
    timeout_ms: u64,
    #[serde(default = "default_max_prompt_bytes")]
    max_prompt_bytes: usize,
    #[serde(default = "default_max_sample_bytes_per_file")]
    max_sample_bytes_per_file: usize,
    #[serde(default)]
    local_command: Option<LocalCommandConfig>,
    #[serde(default)]
    hosted_api: Option<HostedApiConfig>,
}

#[derive(Clone, Debug, Default, Eq, PartialEq, serde::Deserialize)]
struct LocalCommandConfig {
    #[serde(default)]
    argv: Vec<String>,
}

#[derive(Clone, Debug, Default, Eq, PartialEq, serde::Deserialize)]
struct HostedApiConfig {
    provider: String,
    model: String,
    #[serde(default)]
    credential_env: String,
}

const fn default_timeout_ms() -> u64 {
    30_000
}

const fn default_max_prompt_bytes() -> usize {
    24_000
}

const fn default_max_sample_bytes_per_file() -> usize {
    4_000
}

/// Loaded summariser settings with fallible construction from disk.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SummariserSettings {
    /// Active backend mode.
    pub mode: crate::summariser::SummariserMode,
    /// Per-invocation timeout.
    pub timeout_ms: u64,
    /// Hard limit on the serialised prompt size.
    pub max_prompt_bytes: usize,
    /// Per-file ceiling for code samples.
    pub max_sample_bytes_per_file: usize,
}

impl Default for SummariserSettings {
    fn default() -> Self {
        Self {
            mode: crate::summariser::SummariserMode::default(),
            timeout_ms: default_timeout_ms(),
            max_prompt_bytes: default_max_prompt_bytes(),
            max_sample_bytes_per_file: default_max_sample_bytes_per_file(),
        }
    }
}

impl SummariserSettings {
    /// Reads `cairn.config.yaml` in `root` and extracts the summariser section.
    ///
    /// Returns defaults when the file is missing or the section is absent.
    ///
    /// # Errors
    ///
    /// Returns an error only when the file exists but the `summariser:` block
    /// cannot be parsed.
    pub fn load(root: &Path) -> Result<Self, String> {
        let path = root.join("cairn.config.yaml");
        let Ok(source) = fs::read_to_string(&path) else {
            return Ok(Self::default());
        };
        let root_config: RootConfig = serde_yaml::from_str(&source)
            .map_err(|e| format!("failed to parse summariser config: {e}"))?;
        let cfg = root_config.summariser;
        let mode = match cfg.mode.as_str() {
            "local_command" => {
                let lc = cfg.local_command.ok_or_else(|| {
                    "summariser mode is local_command but local_command section is missing"
                        .to_owned()
                })?;
                let command = lc.argv.first().cloned().unwrap_or_default();
                let args = lc.argv.into_iter().skip(1).collect();
                crate::summariser::SummariserMode::LocalCommand {
                    command,
                    args,
                    timeout_ms: cfg.timeout_ms,
                }
            }
            "hosted_api" => {
                let ha = cfg.hosted_api.ok_or_else(|| {
                    "summariser mode is hosted_api but hosted_api section is missing".to_owned()
                })?;
                crate::summariser::SummariserMode::Hosted {
                    adapter: ha.provider,
                }
            }
            _ => crate::summariser::SummariserMode::Disabled,
        };
        Ok(Self {
            mode,
            timeout_ms: cfg.timeout_ms,
            max_prompt_bytes: cfg.max_prompt_bytes,
            max_sample_bytes_per_file: cfg.max_sample_bytes_per_file,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_missing_file_returns_defaults() {
        let dir = tempfile::tempdir().unwrap();
        let settings = SummariserSettings::load(dir.path()).unwrap();
        assert!(matches!(
            settings.mode,
            crate::summariser::SummariserMode::Disabled
        ));
        assert_eq!(settings.timeout_ms, 30_000);
        assert_eq!(settings.max_prompt_bytes, 24_000);
        assert_eq!(settings.max_sample_bytes_per_file, 4_000);
    }

    #[test]
    fn test_load_disabled_mode_from_file() {
        let dir = tempfile::tempdir().unwrap();
        fs::write(
            dir.path().join("cairn.config.yaml"),
            "summariser:\n  mode: disabled\n",
        )
        .unwrap();
        let settings = SummariserSettings::load(dir.path()).unwrap();
        assert!(matches!(
            settings.mode,
            crate::summariser::SummariserMode::Disabled
        ));
    }

    #[test]
    fn test_load_local_command_mode() {
        let dir = tempfile::tempdir().unwrap();
        fs::write(
            dir.path().join("cairn.config.yaml"),
            "summariser:\n  mode: local_command\n  timeout_ms: 5000\n  max_prompt_bytes: 1000\n  max_sample_bytes_per_file: 200\n  local_command:\n    argv: [\"/bin/echo\"]\n",
        )
        .unwrap();
        let settings = SummariserSettings::load(dir.path()).unwrap();
        assert!(
            matches!(settings.mode, crate::summariser::SummariserMode::LocalCommand { command, timeout_ms, .. } if command == "/bin/echo" && timeout_ms == 5000)
        );
        assert_eq!(settings.max_prompt_bytes, 1_000);
        assert_eq!(settings.max_sample_bytes_per_file, 200);
    }
}
