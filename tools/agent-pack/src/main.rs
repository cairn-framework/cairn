// Reason: the dev-only binary shares `tempfile` with the library for cross-platform
// atomic replacement, bringing target-specific versions already present in the lockfile.
#![allow(clippy::multiple_crate_versions)]

//! Command line interface for `cairn-agent-pack`.
//!
//! Supports `--check` and `--write` modes for agent pack manifest validation,
//! render plan construction, drift verification, and atomic file rendering.

use cairn_agent_pack::{run_check, run_write};
use std::env;
use std::path::PathBuf;
use std::process::ExitCode;

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();

    let mut check_mode = false;
    let mut write_mode = false;
    let mut manifest_path = PathBuf::from("tools/agent-pack/manifest.toml");
    let mut repo_root = PathBuf::from(".");

    let mut idx = 1;
    while idx < args.len() {
        match args[idx].as_str() {
            "--check" => check_mode = true,
            "--write" => write_mode = true,
            "--manifest" => {
                idx += 1;
                if idx < args.len() {
                    manifest_path = PathBuf::from(&args[idx]);
                } else {
                    eprintln!("Error: --manifest requires a path argument");
                    return ExitCode::FAILURE;
                }
            }
            "--root" => {
                idx += 1;
                if idx < args.len() {
                    repo_root = PathBuf::from(&args[idx]);
                } else {
                    eprintln!("Error: --root requires a path argument");
                    return ExitCode::FAILURE;
                }
            }
            "--help" | "-h" => {
                print_help();
                return ExitCode::SUCCESS;
            }
            arg => {
                eprintln!("Error: unknown argument '{arg}'");
                print_help();
                return ExitCode::FAILURE;
            }
        }
        idx += 1;
    }

    if !check_mode && !write_mode {
        eprintln!("Error: specify either --check or --write");
        print_help();
        return ExitCode::FAILURE;
    }

    if check_mode && write_mode {
        eprintln!("Error: --check and --write are mutually exclusive");
        return ExitCode::FAILURE;
    }

    if check_mode {
        match run_check(&manifest_path, &repo_root) {
            Ok(()) => {
                println!("Agent pack check succeeded: no drift detected.");
                ExitCode::SUCCESS
            }
            Err(err) => {
                eprintln!("Agent pack check failed:\n{err}");
                ExitCode::FAILURE
            }
        }
    } else {
        match run_write(&manifest_path, &repo_root) {
            Ok(()) => {
                println!("Agent pack write succeeded.");
                ExitCode::SUCCESS
            }
            Err(err) => {
                eprintln!("Agent pack write failed:\n{err}");
                ExitCode::FAILURE
            }
        }
    }
}

fn print_help() {
    println!(
        "cairn-agent-pack CLI\n\n\
        Usage: cairn-agent-pack [--check | --write] [--manifest PATH] [--root PATH]\n\n\
        Flags:\n  \
          --check          Validate manifest and check disk for drift\n  \
          --write          Validate manifest and write rendered files atomically\n  \
          --manifest PATH  Path to manifest.toml (default: tools/agent-pack/manifest.toml)\n  \
          --root PATH      Target repository root directory (default: .)\n"
    );
}
