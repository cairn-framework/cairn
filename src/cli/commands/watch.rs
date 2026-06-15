//! CLI watch command implementation.
// Reason: child module imports re-exported public surface from parent via use super::*
#![allow(clippy::wildcard_imports)]
use super::super::*;

/// Watch for finding changes and emit newline-delimited JSON events.
pub(crate) fn run_watch_command(root: &Path, opts: &crate::watch::WatchOpts) -> CliResult {
    let blueprint = root.join("cairn.blueprint");

    // --once: single scan, emit all findings as added, exit.
    if opts.once {
        let findings = match crate::scanner::scan(root, &blueprint) {
            Ok(result) => result.graph.findings,
            Err(error) => {
                return err(1, &format!("scan failed: {error}"));
            }
        };
        let events = crate::watch::diff_findings(&[], &findings);
        for event in events {
            match serde_json::to_string(&event) {
                Ok(line) => println!("{line}"),
                Err(error) => eprintln!("json error: {error}"),
            }
        }
        return ok(String::new());
    }

    let stop = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
    if let Err(error) = crate::signal::install_sigint_handler(std::sync::Arc::clone(&stop)) {
        return err(1, &format!("failed to set Ctrl-C handler: {error}"));
    }

    // Initial scan.
    let mut previous = match crate::scanner::scan(root, &blueprint) {
        Ok(result) => result.graph.findings,
        Err(error) => {
            return err(1, &format!("initial scan failed: {error}"));
        }
    };

    let interval = std::time::Duration::from_secs(opts.interval_secs);

    while !stop.load(std::sync::atomic::Ordering::SeqCst) {
        std::thread::sleep(interval);

        if stop.load(std::sync::atomic::Ordering::SeqCst) {
            break;
        }

        let current = match crate::scanner::scan(root, &blueprint) {
            Ok(result) => result.graph.findings,
            Err(error) => {
                eprintln!("scan error: {error}");
                continue;
            }
        };

        let events = crate::watch::diff_findings(&previous, &current);
        for event in events {
            match serde_json::to_string(&event) {
                Ok(line) => println!("{line}"),
                Err(error) => eprintln!("json error: {error}"),
            }
        }

        previous = current;
    }

    ok("watch stopped\n".to_owned())
}
