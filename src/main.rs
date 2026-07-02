//! Binary entrypoint for the Cairn CLI.

use std::{env, process::ExitCode};

fn main() -> ExitCode {
    cairn::report::install_panic_hook();
    let args = env::args().skip(1).collect::<Vec<_>>();
    let result = cairn::cli::run(&args);
    print!("{}", result.stdout);
    eprint!("{}", result.stderr);
    ExitCode::from(result.code)
}
