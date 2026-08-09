//! Invocation parsing for the `cairn-authoreval` binary.
//!
//! Kept beside the instrument rather than in the binary so it is unit
//! testable, matching `lsp::LspOpts`. The `cairn` CLI's own strings live in
//! `docs/design-system/copy.toml`; auxiliary binaries carry their own help
//! text, as `cairn-lsp` and `cairn-mcp` do (copy.toml declares no section for
//! either).

use camino::{Utf8Path, Utf8PathBuf};
use serde::{Deserialize, Serialize};

use super::runner::{BackendSpec, DEFAULT_MAX_REPAIRS, DEFAULT_TIMEOUT_MS, RunConfig};
use crate::error::CairnError;

/// Fixture used when `--fixture` is not given.
pub(crate) const DEFAULT_FIXTURE: &str = "tests/fixtures/cairn-bootstrap";

/// A parsed `cairn-authoreval run` invocation.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Invocation {
    /// Run configuration shared by every prompt.
    pub config: RunConfig,
    /// Prompt files to run, in the order given.
    pub prompts: Vec<Utf8PathBuf>,
    /// Where records are written. Stdout when absent.
    pub out: Option<Utf8PathBuf>,
}

/// Help text for the binary.
#[must_use]
pub fn help_text() -> String {
    format!(
        "{}\n\nUsage: cairn-authoreval run <prompt.json>... [options]\n\n\
         Runs each authoring prompt against a scratch copy of the fixture and\n\
         writes one record per prompt as JSON Lines.\n\n\
         Options:\n  \
         --fixture PATH     Fixture copied into the scratch workspace (default: {DEFAULT_FIXTURE})\n  \
         --cairn PATH       cairn binary used for scoring (default: the `cairn` beside this binary)\n  \
         --backend KIND     replay or command (default: replay)\n  \
         --command PROG     Program to spawn, required by --backend command\n  \
         --command-arg ARG  Argument for the spawned program; repeatable\n  \
         --model NAME       Model identity recorded for --backend command\n  \
         --max-repairs N    Repair attempts after the first response (default: {DEFAULT_MAX_REPAIRS})\n  \
         --timeout-ms MS    Per-call backend deadline (default: {DEFAULT_TIMEOUT_MS})\n  \
         --out PATH         Write records here instead of stdout\n  \
         --version          Print version\n  \
         --help             Print this help",
        crate::version_label()
    )
}

/// The file name of the `cairn` binary on this platform.
#[must_use]
pub(crate) fn cairn_file_name() -> String {
    format!("cairn{}", std::env::consts::EXE_SUFFIX)
}

/// The `cairn` binary sitting beside the running executable.
#[must_use]
pub fn sibling_cairn_bin() -> Option<Utf8PathBuf> {
    let exe = std::env::current_exe().ok()?;
    let dir = exe.parent()?;
    let candidate = Utf8PathBuf::from_path_buf(dir.join(cairn_file_name())).ok()?;
    candidate.is_file().then_some(candidate)
}

impl Invocation {
    /// Parses argv, without the program name.
    ///
    /// `default_cairn_bin` supplies the scoring binary when `--cairn` is not
    /// given; passing it in keeps parsing free of process introspection.
    ///
    /// # Errors
    ///
    /// Returns [`CairnError::AuthorEval`] carrying a usage message when the
    /// arguments are unusable.
    pub fn from_args(
        args: &[String],
        default_cairn_bin: Option<&Utf8Path>,
    ) -> Result<Self, CairnError> {
        let Some((verb, rest)) = args.split_first() else {
            return Err(usage("missing subcommand; expected `run`".to_owned()));
        };
        if verb != "run" {
            return Err(usage(format!(
                "unknown subcommand `{verb}`; expected `run`"
            )));
        }

        let options = Options::scan(rest)?;
        if options.prompts.is_empty() {
            return Err(usage("no prompt files given".to_owned()));
        }
        let backend = options.backend()?;
        let Some(cairn_bin) = options
            .cairn_bin
            .clone()
            .or_else(|| default_cairn_bin.map(Utf8Path::to_path_buf))
        else {
            return Err(usage(
                "no cairn binary found beside this executable; pass --cairn PATH".to_owned(),
            ));
        };

        Ok(Self {
            config: RunConfig {
                fixture: options.fixture,
                cairn_bin,
                backend,
                max_repairs: options.max_repairs,
                timeout_ms: options.timeout_ms,
            },
            prompts: options.prompts,
            out: options.out,
        })
    }
}

/// The raw options one invocation carries, before they are checked together.
struct Options {
    prompts: Vec<Utf8PathBuf>,
    fixture: Utf8PathBuf,
    cairn_bin: Option<Utf8PathBuf>,
    backend_kind: String,
    program: Option<String>,
    command_args: Vec<String>,
    model: Option<String>,
    max_repairs: u32,
    timeout_ms: u64,
    out: Option<Utf8PathBuf>,
}

impl Options {
    /// Reads the option list left to right.
    fn scan(rest: &[String]) -> Result<Self, CairnError> {
        let mut options = Self {
            prompts: Vec::new(),
            fixture: Utf8PathBuf::from(DEFAULT_FIXTURE),
            cairn_bin: None,
            backend_kind: "replay".to_owned(),
            program: None,
            command_args: Vec::new(),
            model: None,
            max_repairs: DEFAULT_MAX_REPAIRS,
            timeout_ms: DEFAULT_TIMEOUT_MS,
            out: None,
        };

        let mut index = 0;
        while index < rest.len() {
            let arg = rest[index].as_str();
            match arg {
                "--fixture" => options.fixture = Utf8PathBuf::from(take(rest, &mut index, arg)?),
                "--cairn" => {
                    options.cairn_bin = Some(Utf8PathBuf::from(take(rest, &mut index, arg)?));
                }
                "--backend" => options.backend_kind = take(rest, &mut index, arg)?,
                "--command" => options.program = Some(take(rest, &mut index, arg)?),
                "--command-arg" => options.command_args.push(take(rest, &mut index, arg)?),
                "--model" => options.model = Some(take(rest, &mut index, arg)?),
                "--max-repairs" => {
                    options.max_repairs = parse_number(&take(rest, &mut index, arg)?, arg)?;
                }
                "--timeout-ms" => {
                    options.timeout_ms = parse_number(&take(rest, &mut index, arg)?, arg)?;
                }
                "--out" => options.out = Some(Utf8PathBuf::from(take(rest, &mut index, arg)?)),
                other if other.starts_with("--") => {
                    return Err(usage(format!("unknown option `{other}`")));
                }
                other => options.prompts.push(Utf8PathBuf::from(other)),
            }
            index += 1;
        }
        Ok(options)
    }

    /// Resolves the backend the options select.
    fn backend(&self) -> Result<BackendSpec, CairnError> {
        match self.backend_kind.as_str() {
            "replay" => Ok(BackendSpec::Replay),
            "command" => Ok(BackendSpec::Command {
                program: non_empty(self.program.as_deref())
                    .ok_or_else(|| usage("--backend command requires --command".to_owned()))?,
                args: self.command_args.clone(),
                // An empty model would put an unattributable record on disk.
                model: non_empty(self.model.as_deref()).ok_or_else(|| {
                    usage("--backend command requires a non-empty --model".to_owned())
                })?,
            }),
            other => Err(usage(format!(
                "unknown backend `{other}`; expected replay or command"
            ))),
        }
    }
}

/// The value, when it is present and not empty.
fn non_empty(value: Option<&str>) -> Option<String> {
    value
        .filter(|value| !value.is_empty())
        .map(ToOwned::to_owned)
}

/// Wraps a usage message in the crate's error type.
fn usage(message: String) -> CairnError {
    CairnError::AuthorEval { message }
}

fn take(rest: &[String], index: &mut usize, flag: &str) -> Result<String, CairnError> {
    *index += 1;
    rest.get(*index)
        .cloned()
        .ok_or_else(|| usage(format!("`{flag}` requires a value")))
}

fn parse_number<T: std::str::FromStr>(raw: &str, flag: &str) -> Result<T, CairnError> {
    raw.parse()
        .map_err(|_| usage(format!("`{flag}` expects a number, got `{raw}`")))
}
