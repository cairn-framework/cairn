//! Binary entrypoint for the Cairn foundation CLI.
//!
//! Phase 0 supports only deterministic package metadata output. Functional
//! Cairn commands are intentionally left for later implementation phases.

use std::{env, process::ExitCode};

fn main() -> ExitCode {
    let mut args = env::args().skip(1);

    if let (Some("--version"), None) = (args.next().as_deref(), args.next()) {
        println!("{}", cairn::version_label());
        ExitCode::SUCCESS
    } else {
        eprintln!("usage: cairn --version");
        ExitCode::from(2)
    }
}
