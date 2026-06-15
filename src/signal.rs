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

#[cfg(test)]
mod tests {
    use std::sync::atomic::Ordering;

    use super::*;

    #[test]
    fn install_sigint_handler_can_be_called_multiple_times() {
        let flag1 = Arc::new(AtomicBool::new(false));
        let flag2 = Arc::new(AtomicBool::new(false));

        install_sigint_handler(Arc::clone(&flag1)).expect("first install should succeed");
        install_sigint_handler(Arc::clone(&flag2)).expect("second install should also succeed");

        // The flags should share the same SIGINT registration semantics: we can
        // only verify that installation succeeded without panicking; actually
        // raising SIGINT is platform-specific and flaky in unit tests.
        assert!(!flag1.load(Ordering::SeqCst));
        assert!(!flag2.load(Ordering::SeqCst));
    }
}
