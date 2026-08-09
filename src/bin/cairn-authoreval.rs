//! Authorability eval runner binary.
//!
//! Runs authoring prompts against a scratch copy of the bootstrap fixture and
//! writes one record per prompt as JSON Lines. Exits 0 whenever every prompt
//! produced a record: a failed authoring attempt is a successful measurement.
//! Non-zero is reserved for instrument faults.

use std::io::Write as _;
use std::{env, process};

use cairn::authoreval::{Invocation, help_text, run_prompt_file, sibling_cairn_bin};

fn main() {
    cairn::report::install_panic_hook();
    let args: Vec<String> = env::args().skip(1).collect();

    if args.iter().any(|arg| arg == "--version" || arg == "-V") {
        println!("{}", cairn::version_label());
        process::exit(0);
    }
    if args.is_empty() || args.iter().any(|arg| arg == "--help" || arg == "-h") {
        eprintln!("{}", help_text());
        process::exit(0);
    }

    let invocation = match Invocation::from_args(&args, sibling_cairn_bin().as_deref()) {
        Ok(invocation) => invocation,
        Err(message) => {
            eprintln!("cairn-authoreval: {message}");
            process::exit(2);
        }
    };

    let mut lines = String::new();
    for prompt in &invocation.prompts {
        match run_prompt_file(&invocation.config, prompt) {
            Ok(record) => match serde_json::to_string(&record) {
                Ok(json) => {
                    lines.push_str(&json);
                    lines.push('\n');
                }
                Err(error) => {
                    eprintln!("cairn-authoreval: failed to serialise a record: {error}");
                    process::exit(1);
                }
            },
            Err(error) => {
                eprintln!("cairn-authoreval: {error}");
                process::exit(1);
            }
        }
    }

    if let Some(path) = &invocation.out {
        if let Err(error) = std::fs::write(path, &lines) {
            eprintln!("cairn-authoreval: failed to write {path}: {error}");
            process::exit(1);
        }
    } else if let Err(error) = std::io::stdout().write_all(lines.as_bytes()) {
        eprintln!("cairn-authoreval: failed to write records: {error}");
        process::exit(1);
    }
}
