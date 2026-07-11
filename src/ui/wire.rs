//! Wire-level helpers for the embedded web server: URL percent-decoding and
//! the legacy-layout API error body.

/// Decodes percent-encoded sequences and `+` as space, preserving invalid or
/// truncated sequences verbatim (browser / RFC 3986 §2.1 behaviour).
pub(super) fn percent_decode(value: &str) -> String {
    let mut output = String::new();
    let mut chars = value.chars();
    while let Some(ch) = chars.next() {
        if ch == '%' {
            let hi = chars.next();
            let lo = chars.next();
            let decoded = hi.zip(lo).and_then(|(h, l)| {
                h.to_digit(16)
                    .zip(l.to_digit(16))
                    .map(|(hv, lv)| (hv, lv, h, l))
            });
            if let Some((hi_val, lo_val, _, _)) = decoded
                && let Ok(byte) = u8::try_from(hi_val * 16 + lo_val)
            {
                output.push(char::from(byte));
                continue;
            }
            // Fallback: preserve `%` and any consumed characters as-is.
            output.push('%');
            if let Some((_, _, h, l)) = decoded {
                output.push(h);
                output.push(l);
            } else {
                // One or both hex chars missing (truncated input).
                if let Some(h) = hi {
                    output.push(h);
                }
                if let Some(l) = lo {
                    output.push(l);
                }
            }
            continue;
        }
        output.push(ch);
    }
    output.replace('+', " ")
}

fn esc(value: &str) -> String {
    value
        .replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n")
        .replace('\r', "\\r")
}

/// API error body in the legacy `finding_json` byte layout so error responses
/// stay wire-identical across the `query_api` spine flip.
pub(super) fn error_json(code: &str, message: &str) -> String {
    format!(
        "{{\"code\":\"{}\",\"severity\":\"error\",\"message\":\"{}\",\"node\":null,\"path\":null}}",
        esc(code),
        esc(message)
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    // ── percent_decode ────────────────────────────────────────────────────────

    #[test]
    fn test_percent_decode_valid_sequence_decoded() {
        assert_eq!(percent_decode("%20"), " ");
    }

    #[test]
    fn test_percent_decode_plus_becomes_space() {
        assert_eq!(percent_decode("hello+world"), "hello world");
    }

    #[test]
    fn test_percent_decode_empty_string() {
        assert_eq!(percent_decode(""), "");
    }

    #[test]
    fn test_percent_decode_no_encoding_unchanged() {
        assert_eq!(percent_decode("app.api"), "app.api");
    }

    #[test]
    fn test_percent_decode_mixed_valid_and_plain() {
        assert_eq!(percent_decode("%41pp%2eapi"), "App.api");
    }

    /// An invalid hex pair must NOT silently drop the characters after `%`.
    /// Browser / RFC 3986 §2.1 behaviour: treat `%` followed by non-hex as
    /// literal `%` + the two characters unchanged.
    #[test]
    fn test_percent_decode_invalid_hex_pair_preserves_both_chars() {
        assert_eq!(percent_decode("%GHtest"), "%GHtest");
    }

    #[test]
    fn test_percent_decode_truncated_percent_at_end_preserved() {
        assert_eq!(percent_decode("end%"), "end%");
    }

    #[test]
    fn test_percent_decode_truncated_percent_one_char_preserved() {
        // `%G` at end — the `G` must not be silently dropped.
        assert_eq!(percent_decode("end%G"), "end%G");
    }

    // ── error_json ───────────────────────────────────────────────────────────

    #[test]
    fn test_error_json_escapes_quotes_and_newlines_and_preserves_layout() {
        assert_eq!(
            error_json("CODE\"X", "line1\nline2\r\n\"end\""),
            r#"{"code":"CODE\"X","severity":"error","message":"line1\nline2\r\n\"end\"","node":null,"path":null}"#
        );
    }
}
