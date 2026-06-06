//! POSIX signal handling: SIGINT flag via `signal-hook` (no unsafe code).
//!
//! `signal-hook` uses a pipe internally, which is async-signal-safe.
//! This avoids the `ctrlc` crate which pulls `dispatch2` / ObjC2 /
//! Foundation.framework and adds ~2ms to binary startup on macOS.
use std::sync::Arc;
use std::sync::atomic::AtomicBool;

/// Installs a SIGINT handler that sets `flag` to `true` when Ctrl-C is received.
///
/// # Errors
///
/// Returns an error string if handler registration fails.
pub fn install_sigint_handler(flag: Arc<AtomicBool>) -> Result<(), String> {
    signal_hook::flag::register(signal_hook::consts::SIGINT, flag)
        .map(|_| ())
        .map_err(|e| e.to_string())
}
