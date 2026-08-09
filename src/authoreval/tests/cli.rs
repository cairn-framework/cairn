//! Invocation parsing and scoring-binary resolution.

use camino::{Utf8Path, Utf8PathBuf};

use super::super::cli::{self, Invocation};
use super::super::runner::{BackendSpec, DEFAULT_MAX_REPAIRS, DEFAULT_TIMEOUT_MS};
use super::super::scorer::absolute_bin;
use super::args;

#[test]
fn test_from_args_defaults_to_the_offline_backend() {
    let invocation = Invocation::from_args(
        &args(&["run", "prompts/a.json"]),
        Some(Utf8Path::new("/bin/cairn")),
    )
    .expect("parse");

    assert_eq!(invocation.config.backend, BackendSpec::Replay);
    assert_eq!(invocation.config.max_repairs, DEFAULT_MAX_REPAIRS);
    assert_eq!(invocation.config.timeout_ms, DEFAULT_TIMEOUT_MS);
    assert_eq!(invocation.config.cairn_bin, Utf8Path::new("/bin/cairn"));
    assert_eq!(
        invocation.prompts,
        vec![Utf8PathBuf::from("prompts/a.json")]
    );
    assert!(invocation.out.is_none());
}

#[test]
fn test_from_args_builds_a_command_backend() {
    let invocation = Invocation::from_args(
        &args(&[
            "run",
            "p.json",
            "--backend",
            "command",
            "--command",
            "omp",
            "--command-arg",
            "-p",
            "--model",
            "some-model",
            "--max-repairs",
            "1",
            "--timeout-ms",
            "500",
        ]),
        Some(Utf8Path::new("/bin/cairn")),
    )
    .expect("parse");

    assert_eq!(
        invocation.config.backend,
        BackendSpec::Command {
            program: "omp".to_owned(),
            args: vec!["-p".to_owned()],
            model: "some-model".to_owned(),
        }
    );
    assert_eq!(invocation.config.max_repairs, 1);
    assert_eq!(invocation.config.timeout_ms, 500);
}

#[test]
fn test_from_args_rejects_unusable_invocations() {
    for (raw, expected) in [
        (vec!["run"], "no prompt files given"),
        (vec!["walk", "p.json"], "unknown subcommand"),
        (vec!["run", "p.json", "--nope"], "unknown option"),
        (
            vec!["run", "p.json", "--backend", "psychic"],
            "unknown backend",
        ),
        (
            vec!["run", "p.json", "--backend", "command"],
            "requires --command",
        ),
        (
            vec!["run", "p.json", "--max-repairs", "many"],
            "expects a number",
        ),
        (vec!["run", "p.json", "--fixture"], "requires a value"),
    ] {
        let error = Invocation::from_args(&args(&raw), Some(Utf8Path::new("/bin/cairn")))
            .expect_err("must be rejected");
        assert!(
            error.to_string().contains(expected),
            "`{raw:?}` gave `{error}`, expected it to mention `{expected}`"
        );
    }
}

#[test]
fn test_from_args_requires_a_scoring_binary() {
    let error = Invocation::from_args(&args(&["run", "p.json"]), None)
        .expect_err("no scoring binary must be rejected");
    assert!(error.to_string().contains("--cairn"));
}

#[test]
fn test_absolute_bin_absolutises_paths_but_leaves_bare_names_to_path_lookup() {
    let absolute = Utf8Path::new("/usr/local/bin/cairn");
    assert_eq!(
        absolute_bin(absolute).expect("absolute"),
        absolute,
        "an absolute path is already unambiguous"
    );

    let bare = Utf8Path::new("cairn");
    assert_eq!(
        absolute_bin(bare).expect("bare"),
        bare,
        "a bare name means a PATH lookup; rewriting it to cwd/cairn would break that"
    );

    for relative in ["./cairn", "target/debug/cairn"] {
        let resolved = absolute_bin(Utf8Path::new(relative)).expect("relative");
        assert!(
            resolved.is_absolute(),
            "`{relative}` is a path, so it must be resolved against the process cwd \
             before the child's working directory is changed; got `{resolved}`"
        );
    }
}

#[test]
fn test_the_sibling_binary_name_carries_the_platform_suffix() {
    let name = cli::cairn_file_name();
    assert_eq!(
        name,
        format!("cairn{}", std::env::consts::EXE_SUFFIX),
        "the documented default is the cairn beside this binary; on a platform \
         with an executable suffix, looking for a bare `cairn` would never find it"
    );
    assert!(name.starts_with("cairn"));
}

#[test]
fn test_a_backend_must_be_attributable() {
    for raw in [
        vec![
            "run",
            "p.json",
            "--backend",
            "command",
            "--command",
            "",
            "--model",
            "m",
        ],
        vec![
            "run",
            "p.json",
            "--backend",
            "command",
            "--command",
            "omp",
            "--model",
            "",
        ],
    ] {
        let error = Invocation::from_args(&args(&raw), Some(Utf8Path::new("/bin/cairn")))
            .expect_err("an unattributable backend must be refused");
        assert!(
            error.to_string().contains("requires"),
            "unexpected error: {error}"
        );
    }
}
