//! Binary entrypoint for the Cairn MCP server.

use std::{env, process::ExitCode};

fn main() -> ExitCode {
    let args = env::args().skip(1).collect::<Vec<_>>();

    if args.iter().any(|a| a == "--version" || a == "-V") {
        println!("{}", cairn::version_label());
        return ExitCode::SUCCESS;
    }

    let config = match cairn::mcp::config_from_args(&args) {
        Ok(config) => config,
        Err(message) => {
            eprintln!("{message}");
            return ExitCode::from(2);
        }
    };
    match cairn::mcp::serve_stdio(&config) {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("cairn-mcp failed: {error}");
            ExitCode::from(1)
        }
    }
}
