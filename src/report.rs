//! Shared issue-reporting helpers.
//!
//! Nothing here ever sends data automatically. `cairn feedback` prints a
//! blank-issue link prefilled with the user's message; the crash panic hook
//! prints a bug-report-template link with the version and panic context
//! prefilled; the webui "Report an issue" links open the same template with
//! the version and a short report seed. Nothing is transmitted on the user's
//! behalf; the user chooses whether to open any of these links.

use crate::copy;

/// Upstream issue tracker for prefilled bug and feedback reports.
pub const ISSUE_BASE: &str = "https://github.com/cairn-framework/cairn/issues/new";

/// Builds a prefilled, freeform issue URL carrying the `feedback` label.
///
/// `cairn feedback` keeps its own blank issue: the bug report form forces a
/// required "what you expected" field that does not fit open-ended friction.
/// `title` and `body` are percent-encoded for use as query parameters.
#[must_use]
pub fn issue_url(title: &str, body: &str) -> String {
    format!(
        "{ISSUE_BASE}?labels=feedback&title={}&body={}",
        encode_query_component(title),
        encode_query_component(body)
    )
}

/// Builds a bug-report template URL prefilled with the cairn `version` and the
/// `what-happened` panic context.
///
/// The template itself applies the `bug` label; passing `labels` in the URL
/// needs triage permission on the repository and can 404 for outside
/// reporters, so the URL carries only the template and field ids.
///
/// Routing crashes through `.github/ISSUE_TEMPLATE/bug-report.yml` keeps the
/// reporter inside the structured form; the `version` input and `what-happened`
/// textarea accept field ids as query parameters.
#[must_use]
fn crash_issue_url(version: &str, panic_context: &str) -> String {
    format!(
        "{ISSUE_BASE}?template=bug-report.yml&version={}&what-happened={}",
        encode_query_component(version),
        encode_query_component(panic_context)
    )
}

/// Installs a panic hook that prints a crash report and a prefilled issue
/// link to stderr, then runs the previously installed hook so default
/// backtrace output (`RUST_BACKTRACE=1`) still prints.
///
/// Nothing is sent automatically: the hook only prints a link the user can
/// choose to open. Safe to call once at process start; later calls replace
/// the previously installed hook.
pub fn install_panic_hook() {
    let previous = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        use std::io::Write as _;

        let version = env!("CARGO_PKG_VERSION");
        let os = std::env::consts::OS;
        let payload = info.payload_as_str().unwrap_or("unknown panic");
        let location = info
            .location()
            .map_or_else(|| "unknown location".to_owned(), ToString::to_string);

        let panic_context = format!("{payload}\n\nat {location}\ncairn {version} on {os}");
        let url = crash_issue_url(version, &panic_context);

        let heading = copy::lookup("report.crash.heading").replace("{version}", version);
        let mut stderr = std::io::stderr();
        let _ = writeln!(stderr, "{heading}");
        let _ = writeln!(stderr, "  {payload}");
        let _ = writeln!(stderr, "  at {location}");
        let _ = writeln!(stderr, "{}", copy::lookup("report.crash.cta"));
        let _ = writeln!(stderr, "{url}");
        let _ = writeln!(stderr, "{}", copy::lookup("report.crash.transparency"));
        let _ = stderr.flush();

        previous(info);
    }));
}

/// Percent-encodes a string for use as a URL query parameter value.
fn encode_query_component(value: &str) -> String {
    let mut out = String::with_capacity(value.len());
    for byte in value.bytes() {
        match byte {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'.' | b'_' | b'~' => {
                out.push(byte as char);
            }
            _ => {
                use std::fmt::Write as _;
                let _ = write!(out, "%{byte:02X}");
            }
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_issue_url_builds_exact_feedback_url() {
        let url = issue_url("hello world", "body");
        assert_eq!(
            url,
            "https://github.com/cairn-framework/cairn/issues/new?labels=feedback&title=hello%20world&body=body"
        );
    }

    #[test]
    fn test_issue_url_percent_encodes_special_characters() {
        let url = issue_url("t", "line one\nline two & more: café");
        assert!(url.contains("body=line%20one%0Aline%20two%20%26%20more%3A%20caf%C3%A9"));
        assert!(!url.contains(' '));
        assert!(!url.contains('\n'));
    }

    #[test]
    fn test_crash_issue_url_prefills_bug_form_fields() {
        let url = crash_issue_url(
            "0.7.0",
            "panic: boom\n\nat src/lib.rs:42\ncairn 0.7.0 on macos",
        );
        assert_eq!(
            url,
            "https://github.com/cairn-framework/cairn/issues/new?template=bug-report.yml&version=0.7.0&what-happened=panic%3A%20boom%0A%0Aat%20src%2Flib.rs%3A42%0Acairn%200.7.0%20on%20macos"
        );
    }
}
