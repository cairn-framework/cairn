//! LSP diagnostics server for OMP integration.
//!
//! Provides a synchronous, stdio-based language server that publishes Cairn
//! findings as `textDocument/publishDiagnostics` notifications. The server
//! runs a background watch loop and re-publishes diagnostics whenever the
//! finding set changes.

pub mod diagnostics;
pub mod server;

pub use self::server::run;

use camino::Utf8PathBuf;

use crate::lsp::diagnostics::MIN_INTERVAL_SECS;

/// Configuration for the Cairn LSP server.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LspOpts {
    /// Project root override. If `None`, the first workspace folder is used.
    pub root: Option<Utf8PathBuf>,
    /// Seconds between background scans.
    pub interval_secs: u64,
}

impl Default for LspOpts {
    fn default() -> Self {
        Self {
            root: None,
            interval_secs: 5,
        }
    }
}

impl LspOpts {
    /// Parses command-line arguments into LSP options.
    ///
    /// # Errors
    ///
    /// Returns a descriptive message on unknown flags or missing values.
    pub fn from_args(args: &[String]) -> Result<Self, String> {
        let mut opts = Self::default();
        let mut i = 0;
        while i < args.len() {
            match args[i].as_str() {
                "--root" => {
                    let value = args.get(i + 1).ok_or("--root requires a value")?;
                    opts.root = Some(Utf8PathBuf::from(value));
                    i += 2;
                }
                "--interval" => {
                    let value = args.get(i + 1).ok_or("--interval requires a value")?;
                    let secs: u64 = value
                        .parse()
                        .map_err(|_| format!("invalid interval: {value}"))?;
                    if secs < MIN_INTERVAL_SECS {
                        return Err(format!(
                            "interval must be at least {MIN_INTERVAL_SECS} second"
                        ));
                    }
                    opts.interval_secs = secs;
                    i += 2;
                }
                "--help" | "-h" => return Err(help_text()),
                _ => return Err(format!("unknown flag: {}", args[i])),
            }
        }
        Ok(opts)
    }
}

/// Returns the `--help` text for `cairn-lsp`.
#[must_use]
pub fn help_text() -> String {
    format!(
        "{}\n\nUsage: cairn-lsp [options]\n\nOptions:\n  --root PATH      Project root (defaults to workspace folder)\n  --interval SECS  Seconds between scans (default: {}, min: {MIN_INTERVAL_SECS})\n  --version        Print version\n  --help           Print this help",
        crate::version_label(),
        LspOpts::default().interval_secs
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lsp_opts_default_uses_five_second_interval() {
        let opts = LspOpts::default();
        assert_eq!(opts.interval_secs, 5);
        assert!(opts.root.is_none());
    }

    #[test]
    fn test_from_args_root_sets_root() {
        let opts = LspOpts::from_args(&["--root".to_owned(), "/project".to_owned()]).unwrap();
        assert_eq!(opts.root, Some(Utf8PathBuf::from("/project")));
    }

    #[test]
    fn test_from_args_interval_sets_interval() {
        let opts = LspOpts::from_args(&["--interval".to_owned(), "10".to_owned()]).unwrap();
        assert_eq!(opts.interval_secs, 10);
    }

    #[test]
    fn test_from_args_interval_too_small_returns_err() {
        let result = LspOpts::from_args(&["--interval".to_owned(), "0".to_owned()]);
        assert!(result.is_err());
    }

    #[test]
    fn test_from_args_unknown_flag_returns_err() {
        let result = LspOpts::from_args(&["--bogus".to_owned()]);
        assert!(result.is_err());
    }
}
