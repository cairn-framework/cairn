//! Cairn LSP server binary.
//!
//! This is a minimal stub implementation. Full LSP features (diagnostics,
//! completion, hover, go-to-definition, document symbols) are planned for
//! a future release once a compatible LSP type library is available.

use std::{env, process};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.iter().any(|a| a == "--version" || a == "-V") {
        println!("{}", cairn::version_label());
        process::exit(0);
    }
    eprintln!("cairn-lsp: Language server protocol support is not yet fully implemented.");
    eprintln!(
        "Planned features: diagnostics, completion, hover, go-to-definition, document symbols."
    );
    process::exit(1);
}
