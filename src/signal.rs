//! POSIX signal handling: SIGINT flag via `signal-hook` (no unsafe code).
//!
//! `signal-hook` uses a pipe internally, which is async-signal-safe.
//! This avoids the `ctrlc` crate which pulls `dispatch2` / ObjC2 /
//! Foundation.framework and adds ~2ms to binary startup on macOS.
//!
//! Unix only (`signal-hook`'s `extended-siginfo-raw` feature compiles a `cc`
//! build step against POSIX `siginfo_t`, unavailable on Windows). Windows
//! gets a no-op stub below: `Ctrl-C` still terminates the process via the
//! OS default handler, it just skips this crate's flag-based graceful-
//! shutdown path. Advisory-only for both callers (`src/ui/mod.rs`,
//! `src/cli/commands/watch.rs`): each already treats a handler-registration
//! failure as non-fatal (falls back to the OS default Ctrl-C behaviour), so
//! a stub that always succeeds preserves that same fallback shape.
use std::sync::Arc;
use std::sync::atomic::AtomicBool;

/// Installs a SIGINT handler that sets `flag` to `true` when Ctrl-C is received.
///
/// # Errors
///
/// Returns an error string if handler registration fails.
#[cfg(unix)]
pub fn install_sigint_handler(flag: Arc<AtomicBool>) -> Result<(), String> {
    signal_hook::flag::register(signal_hook::consts::SIGINT, flag)
        .map(|_| ())
        .map_err(|e| e.to_string())
}

/// No-op stub for non-Unix targets: `signal-hook` is Unix-only in this crate
/// (see module docs). Always succeeds; the OS default Ctrl-C behaviour still
/// applies, it just isn't hooked to `flag`.
///
/// # Errors
///
/// Never returns an error; the signature matches the Unix implementation so
/// callers do not need their own `#[cfg]`.
#[cfg(not(unix))]
#[allow(clippy::unnecessary_wraps)] // Reason: signature parity with the Unix implementation
pub fn install_sigint_handler(_flag: Arc<AtomicBool>) -> Result<(), String> {
    Ok(())
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
