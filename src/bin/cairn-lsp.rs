//! Cairn LSP server binary.
//!
//! Serves Cairn findings as LSP diagnostics over stdio. OMP and other
//! orchestrators can subscribe to `textDocument/publishDiagnostics` for
//! on-write feedback.

use std::{env, process};

use lsp_server::Connection;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.iter().any(|arg| arg == "--version" || arg == "-V") {
        println!("{}", cairn::version_label());
        process::exit(0);
    }
    if args.iter().any(|arg| arg == "--help" || arg == "-h") {
        eprintln!("{}", cairn::lsp::help_text());
        process::exit(0);
    }

    let opts = match cairn::lsp::LspOpts::from_args(&args) {
        Ok(opts) => opts,
        Err(message) => {
            eprintln!("cairn-lsp: {message}");
            process::exit(2);
        }
    };

    let (connection, io_threads) = Connection::stdio();
    if let Err(error) = cairn::lsp::run(&connection, &opts) {
        eprintln!("cairn-lsp: {error}");
        process::exit(1);
    }
    if let Err(error) = io_threads.join() {
        eprintln!("cairn-lsp: io thread error: {error:?}");
        process::exit(1);
    }
}
