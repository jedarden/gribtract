//! Shared utility functions for gribtract-fetch

/// Return `YYYYMMDD` for `days_ago` days before today (UTC).
///
/// Uses a proleptic Gregorian algorithm (Hinnant) — no `chrono` dependency.
/// This allows computing recent dates for NOAA GRIB2 file URLs without
/// pulling in the full chrono dependency.
pub fn probe_date_str(days_ago: u64) -> String {
    let secs = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    let days = (secs / 86400).saturating_sub(days_ago) as i64;
    let (y, m, d) = civil_date(days);
    format!("{:04}{:02}{:02}", y, m, d)
}

/// Days-since-1970-01-01 → (year, month, day).
///
/// Algorithm: <https://howardhinnant.github.io/date_algorithms.html>
pub fn civil_date(days: i64) -> (i32, u32, u32) {
    let z = days + 719_468;
    let era: i64 = if z >= 0 { z } else { z - 146_096 } / 146_097;
    let doe = (z - era * 146_097) as u32; // day of era [0, 146096]
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146_096) / 365; // [0, 399]
    let y: i64 = yoe as i64 + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100); // [0, 365]
    let mp = (5 * doy + 2) / 153; // [0, 11]
    let d = doy - (153 * mp + 2) / 5 + 1; // [1, 31]
    let m = if mp < 10 { mp + 3 } else { mp - 9 }; // [1, 12]
    let y = if m <= 2 { y + 1 } else { y };
    (y as i32, m, d)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn civil_date_known_dates() {
        // 1970-01-01 = day 0
        assert_eq!(civil_date(0), (1970, 1, 1));
        // 2000-01-01 = 10957 days since epoch
        // (30 years, accounting for leap years)
        assert_eq!(civil_date(10957), (2000, 1, 1));
        // 2026-06-20 = 20624 days since epoch
        assert_eq!(civil_date(20624), (2026, 6, 20));
    }

    #[test]
    fn probe_date_str_is_8_digits() {
        let s = probe_date_str(2);
        assert_eq!(s.len(), 8, "YYYYMMDD must be 8 chars: {s}");
        assert!(s.chars().all(|c| c.is_ascii_digit()));
    }

    #[test]
    fn probe_date_str_lookback() {
        // Test that days_ago parameter works correctly
        let today = probe_date_str(0);
        let yesterday = probe_date_str(1);
        let two_days_ago = probe_date_str(2);

        // All should be valid 8-digit dates
        assert_eq!(today.len(), 8);
        assert_eq!(yesterday.len(), 8);
        assert_eq!(two_days_ago.len(), 8);

        // Dates should be different (unless run right at midnight UTC)
        assert!(today != yesterday || today != two_days_ago);
    }
}
