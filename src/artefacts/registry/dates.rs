//! Strict civil-date arithmetic shared by artefact validation and the
//! query layer: `YYYY-MM-DD` parsing to whole days since the Unix epoch.

/// Strict `YYYY-MM-DD` to whole days since the Unix epoch. Rejects malformed
/// shapes and out-of-range calendar components.
#[must_use]
pub(crate) fn date_to_days(value: &str) -> Option<i64> {
    let bytes = value.as_bytes();
    if bytes.len() != 10 || bytes[4] != b'-' || bytes[7] != b'-' {
        return None;
    }
    if !bytes
        .iter()
        .enumerate()
        .all(|(i, b)| matches!(i, 4 | 7) || b.is_ascii_digit())
    {
        return None;
    }
    let year: i64 = value[0..4].parse().ok()?;
    let month: i64 = value[5..7].parse().ok()?;
    let day: i64 = value[8..10].parse().ok()?;
    if !(1..=12).contains(&month) {
        return None;
    }
    let leap = year % 4 == 0 && (year % 100 != 0 || year % 400 == 0);
    let month_len = [
        31,
        if leap { 29 } else { 28 },
        31,
        30,
        31,
        30,
        31,
        31,
        30,
        31,
        30,
        31,
    ][usize::try_from(month - 1).ok()?];
    if !(1..=month_len).contains(&day) {
        return None;
    }
    Some(days_from_civil(year, month, day))
}

/// Days from 1970-01-01 for a proleptic-Gregorian civil date (Howard
/// Hinnant's `days_from_civil`).
pub(crate) const fn days_from_civil(y: i64, m: i64, d: i64) -> i64 {
    let y = if m <= 2 { y - 1 } else { y };
    let era = y.div_euclid(400);
    let yoe = y - era * 400;
    let doy = (153 * (if m > 2 { m - 3 } else { m + 9 }) + 2) / 5 + d - 1;
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
    era * 146_097 + doe - 719_468
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_days_from_civil_known_values() {
        assert_eq!(days_from_civil(1970, 1, 1), 0);
        assert_eq!(days_from_civil(1970, 1, 2), 1);
        assert_eq!(days_from_civil(2000, 3, 1), 11_017);
        assert_eq!(days_from_civil(2026, 7, 30), 20_664);
        assert_eq!(days_from_civil(1969, 12, 31), -1);
    }

    #[test]
    fn test_date_to_days_rejects_malformed_values() {
        for value in [
            "2026-7-30",
            "2026/07/30",
            "2026-13-01",
            "2026-00-10",
            "2026-02-30",
            "2025-02-29",
            "+026-01-01",
            "yesterday!!",
            "2026-07-30T00:00:00Z",
            "",
        ] {
            assert!(date_to_days(value).is_none(), "{value:?} must not parse");
        }
        assert_eq!(
            date_to_days("2024-02-29"),
            Some(days_from_civil(2024, 2, 29))
        );
    }
}
