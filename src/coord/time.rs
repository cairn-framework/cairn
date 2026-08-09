//! UTC timestamp formatting for coordination facts, without a time crate.
//!
//! The store needs two spellings of one instant: RFC 3339 for the envelope's
//! `recorded_at` and a compact form for fact filenames. Both derive from the
//! standard civil-from-days algorithm (Howard Hinnant), the inverse of
//! `artefacts::registry::dates::days_from_civil`; the CLI export lane keeps
//! its own formatter because coord never depends on the CLI.

use std::time::{SystemTime, UNIX_EPOCH};

use crate::artefacts::registry::dates::date_to_days;

/// Whole seconds per civil day.
const SECS_PER_DAY: u64 = 86_400;

/// Civil date from days since 1970-01-01 (proleptic Gregorian).
const fn civil_from_days(days: i64) -> (i64, u8, u8) {
    let z = days + 719_468;
    let era = z.div_euclid(146_097);
    let doe = z - era * 146_097;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146_096) / 365;
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    let y = if m <= 2 { y + 1 } else { y };
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    // Reason: month and day are bounded to 1..=12 and 1..=31 by the algorithm.
    (y, m as u8, d as u8)
}

/// Splits a `SystemTime` into civil UTC components.
fn utc_parts(t: SystemTime) -> (i64, u8, u8, u8, u8, u8) {
    let secs = t
        .duration_since(UNIX_EPOCH)
        .map_or(0, |duration| duration.as_secs());
    #[allow(clippy::cast_possible_wrap)]
    // Reason: seconds-since-epoch stays far below i64::MAX for any realistic clock.
    let days = (secs / SECS_PER_DAY) as i64;
    let rem = secs % SECS_PER_DAY;
    let (year, month, day) = civil_from_days(days);
    #[allow(clippy::cast_possible_truncation)]
    // Reason: hour, minute, and second are bounded by the modulo above.
    (
        year,
        month,
        day,
        (rem / 3600) as u8,
        ((rem % 3600) / 60) as u8,
        (rem % 60) as u8,
    )
}

/// Formats an instant as RFC 3339 UTC without subseconds,
/// e.g. `2026-08-07T03:45:12Z`.
pub(crate) fn rfc3339_utc(t: SystemTime) -> String {
    let (year, month, day, hour, minute, second) = utc_parts(t);
    format!("{year:04}-{month:02}-{day:02}T{hour:02}:{minute:02}:{second:02}Z")
}

/// Validates the whole-second UTC spelling emitted by [`rfc3339_utc`].
pub(crate) fn validate_rfc3339_utc(value: &str) -> Result<(), String> {
    let bytes = value.as_bytes();
    if bytes.len() != 20 || !has_timestamp_separators(bytes) || !digits_at(bytes) {
        return Err(format!("`{value}` is not an RFC 3339 UTC timestamp"));
    }
    let year = decimal(bytes, 0, 4);
    let month = decimal(bytes, 5, 7);
    let day = decimal(bytes, 8, 10);
    let hour = decimal(bytes, 11, 13);
    let minute = decimal(bytes, 14, 16);
    let second = decimal(bytes, 17, 19);
    if month == 0 || month > 12 || day == 0 || day > days_in_month(year, month) {
        return Err(format!("`{value}` has an invalid calendar date"));
    }
    if hour > 23 || minute > 59 || second > 59 {
        return Err(format!("`{value}` has an invalid UTC time"));
    }
    Ok(())
}

/// An RFC 3339 instant normalized to UTC for temporal comparisons.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct Rfc3339Instant<'a> {
    pub(crate) seconds: i64,
    pub(crate) fraction: &'a [u8],
}

fn parse_digits(bytes: &[u8], start: usize, length: usize) -> Option<u32> {
    bytes
        .get(start..start.checked_add(length)?)?
        .iter()
        .try_fold(0_u32, |value, byte| {
            byte.is_ascii_digit()
                .then_some(value * 10 + u32::from(byte - b'0'))
        })
}

fn parse_fraction(bytes: &[u8], mut cursor: usize) -> Option<(&[u8], usize)> {
    if bytes.get(cursor) != Some(&b'.') {
        return Some((&[], cursor));
    }
    cursor += 1;
    let start = cursor;
    while bytes.get(cursor).is_some_and(u8::is_ascii_digit) {
        cursor += 1;
    }
    (start < cursor).then_some((&bytes[start..cursor], cursor))
}

fn parse_offset(bytes: &[u8], cursor: usize) -> Option<i64> {
    match bytes.get(cursor) {
        Some(zone) if zone.eq_ignore_ascii_case(&b'Z') && cursor + 1 == bytes.len() => Some(0),
        Some(sign @ (b'+' | b'-'))
            if cursor + 6 == bytes.len() && bytes.get(cursor + 3) == Some(&b':') =>
        {
            let hours = parse_digits(bytes, cursor + 1, 2)?;
            let minutes = parse_digits(bytes, cursor + 4, 2)?;
            if hours > 23 || minutes > 59 {
                return None;
            }
            let seconds = i64::from(hours * 3_600 + minutes * 60);
            Some(if *sign == b'-' { -seconds } else { seconds })
        }
        _ => None,
    }
}

/// Parses an RFC 3339 instant, retaining arbitrary fractional precision.
pub(crate) fn parse_rfc3339(value: &str) -> Option<Rfc3339Instant<'_>> {
    let bytes = value.as_bytes();
    if bytes.len() < 20
        || bytes
            .get(10)
            .is_none_or(|byte| !byte.eq_ignore_ascii_case(&b'T'))
        || bytes.get(13) != Some(&b':')
        || bytes.get(16) != Some(&b':')
    {
        return None;
    }
    let days = date_to_days(value.get(..10)?)?;
    let hour = parse_digits(bytes, 11, 2)?;
    let minute = parse_digits(bytes, 14, 2)?;
    let second = parse_digits(bytes, 17, 2)?;
    if hour > 23 || minute > 59 || second > 59 {
        return None;
    }
    let (fraction, cursor) = parse_fraction(bytes, 19)?;
    let offset_seconds = parse_offset(bytes, cursor)?;
    let local_seconds = days
        .checked_mul(86_400)?
        .checked_add(i64::from(hour * 3_600 + minute * 60 + second))?;
    Some(Rfc3339Instant {
        seconds: local_seconds.checked_sub(offset_seconds)?,
        fraction,
    })
}

fn compare_fractions(left: &[u8], right: &[u8]) -> std::cmp::Ordering {
    let length = left.len().max(right.len());
    (0..length)
        .map(|index| {
            (
                left.get(index).copied().unwrap_or(b'0'),
                right.get(index).copied().unwrap_or(b'0'),
            )
        })
        .find_map(|(left, right)| (left != right).then_some(left.cmp(&right)))
        .unwrap_or(std::cmp::Ordering::Equal)
}

pub(crate) fn compare_instants(
    left: Rfc3339Instant<'_>,
    right: Rfc3339Instant<'_>,
) -> std::cmp::Ordering {
    left.seconds
        .cmp(&right.seconds)
        .then_with(|| compare_fractions(left.fraction, right.fraction))
}

fn has_timestamp_separators(bytes: &[u8]) -> bool {
    bytes.len() == 20
        && bytes[4] == b'-'
        && bytes[7] == b'-'
        && bytes[10] == b'T'
        && bytes[13] == b':'
        && bytes[16] == b':'
        && bytes[19] == b'Z'
}

fn digits_at(bytes: &[u8]) -> bool {
    [0..4, 5..7, 8..10, 11..13, 14..16, 17..19]
        .into_iter()
        .all(|range| bytes[range].iter().all(u8::is_ascii_digit))
}

fn decimal(bytes: &[u8], start: usize, end: usize) -> u32 {
    bytes[start..end]
        .iter()
        .fold(0, |value, digit| value * 10 + u32::from(*digit - b'0'))
}

fn days_in_month(year: u32, month: u32) -> u32 {
    match month {
        2 if is_leap_year(year) => 29,
        2 => 28,
        4 | 6 | 9 | 11 => 30,
        _ => 31,
    }
}

fn is_leap_year(year: u32) -> bool {
    year.is_multiple_of(4) && (!year.is_multiple_of(100) || year.is_multiple_of(400))
}

/// Compacts an RFC 3339 UTC timestamp into the filename form,
/// e.g. `20260807T034512Z`: the same instant, separators stripped, so a
/// fact's filename always matches its `recorded_at`.
pub(crate) fn compact_rfc3339(recorded_at: &str) -> String {
    recorded_at
        .chars()
        .filter(|c| *c != '-' && *c != ':')
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    fn at(secs: u64) -> SystemTime {
        UNIX_EPOCH + Duration::from_secs(secs)
    }

    #[test]
    fn known_vectors_format_correctly() {
        assert_eq!(rfc3339_utc(at(0)), "1970-01-01T00:00:00Z");
        assert_eq!(rfc3339_utc(at(1_786_074_312)), "2026-08-07T03:45:12Z");
        // Leap day.
        assert_eq!(rfc3339_utc(at(1_709_208_000)), "2024-02-29T12:00:00Z");
        // Year boundary.
        assert_eq!(rfc3339_utc(at(946_684_799)), "1999-12-31T23:59:59Z");
    }

    #[test]
    fn compact_form_strips_separators_only() {
        assert_eq!(compact_rfc3339("2026-08-07T03:45:12Z"), "20260807T034512Z");
    }
    #[test]
    fn fractional_seconds_are_rejected_for_stored_timestamps() {
        let error = validate_rfc3339_utc("2026-08-07T03:45:12.500Z")
            .expect_err("stored coordination timestamps are whole seconds");
        assert!(error.contains("RFC 3339 UTC timestamp"), "{error}");
    }
}
