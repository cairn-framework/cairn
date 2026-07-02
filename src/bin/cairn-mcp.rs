//! Binary entrypoint for the Cairn MCP server.

use std::{env, process::ExitCode};

fn main() -> ExitCode {
    cairn::report::install_panic_hook();
    let args = env::args().skip(1).collect::<Vec<_>>();

    if args.iter().any(|a| a == "--version" || a == "-V") {
        println!("{}", cairn::version_label());
        return ExitCode::SUCCESS;
    }

    if args.iter().any(|a| a == "--help" || a == "-h") {
        println!("{}", cairn::version_label());
        println!();
        println!("Usage: cairn-mcp [options]");
        println!();
        println!("Options:");
        println!("  --root <path>             Project root (default: .)");
        println!("  --file <path>             Blueprint path (default: cairn.blueprint)");
        println!("  --changes-dir <path>      Changes directory (default: meta/changes)");
        println!("  --allow-mutating-tools    Expose mutating tools");
        println!("  --version                 Print version");
        println!("  --help                    Print this help");
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
