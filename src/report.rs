//! Shared issue-reporting helpers.
//!
//! Nothing here ever sends data automatically. `cairn feedback`, the crash
//! panic hook, and the webui "Report an issue" links all end in a URL the
//! user chooses to open in their own browser; nothing is transmitted on
//! their behalf.

use crate::copy;

/// Upstream issue tracker for prefilled bug and feedback reports.
pub const ISSUE_BASE: &str = "https://github.com/cairn-framework/cairn/issues/new";

/// Builds a prefilled issue URL carrying the `feedback` label.
///
/// `title` and `body` are percent-encoded for use as query parameters.
#[must_use]
pub fn issue_url(title: &str, body: &str) -> String {
    format!(
        "{ISSUE_BASE}?labels=feedback&title={}&body={}",
        encode_query_component(title),
        encode_query_component(body)
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

        let title: String = payload
            .lines()
            .next()
            .unwrap_or(payload)
            .chars()
            .take(80)
            .collect();
        let body = format!("{payload}\n\nat {location}\ncairn {version} on {os}");
        let url = issue_url(&title, &body);

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
    fn test_issue_url_includes_feedback_label_and_encoded_title() {
        let url = issue_url("hello world", "body");
        assert!(url.starts_with(&format!("{ISSUE_BASE}?labels=feedback&title=")));
        assert!(url.contains("title=hello%20world"));
    }

    #[test]
    fn test_issue_url_percent_encodes_special_characters() {
        let url = issue_url("t", "line one\nline two & more: café");
        assert!(url.contains("body=line%20one%0Aline%20two%20%26%20more%3A%20caf%C3%A9"));
        assert!(!url.contains(' '));
        assert!(!url.contains('\n'));
    }
}
