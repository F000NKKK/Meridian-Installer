//! Sizes for people.

const UNITS: [&str; 4] = ["B", "KB", "MB", "GB"];
const STEP: f64 = 1024.0;

/// `bytes` as `59 MB` or `2.10 GB`.
#[must_use]
#[expect(clippy::cast_precision_loss, reason = "a size shown to two digits")]
pub(crate) fn format_bytes(bytes: u64) -> String {
    let mut value = bytes as f64;
    let mut unit = 0;
    while value >= STEP && unit < UNITS.len() - 1 {
        value /= STEP;
        unit += 1;
    }
    let digits = usize::from(unit >= 3) * 2;
    format!("{value:.digits$} {}", UNITS[unit])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sizes_are_written_in_the_largest_unit() {
        assert_eq!(format_bytes(0), "0 B");
        assert_eq!(format_bytes(1536), "2 KB");
        assert_eq!(format_bytes(61_865_984), "59 MB");
        assert_eq!(format_bytes(2_254_857_830), "2.10 GB");
    }
}
