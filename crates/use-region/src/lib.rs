#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use core::fmt;

/// A normalized region subtag.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct RegionCode {
    value: String,
}

impl RegionCode {
    /// Parses and normalizes a region subtag.
    #[must_use]
    pub fn new(input: &str) -> Option<Self> {
        parse_region_code(input)
    }

    /// Returns the normalized region subtag.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.value
    }

    /// Consumes the region code and returns the normalized string.
    #[must_use]
    pub fn into_string(self) -> String {
        self.value
    }

    /// Returns `true` when this region subtag is numeric.
    #[must_use]
    pub fn is_numeric(&self) -> bool {
        self.value.bytes().all(|byte| byte.is_ascii_digit())
    }
}

impl AsRef<str> for RegionCode {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for RegionCode {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// Parses a region subtag and normalizes alphabetic regions to uppercase.
#[must_use]
pub fn parse_region_code(input: &str) -> Option<RegionCode> {
    normalize_region_code(input).map(|value| RegionCode { value })
}

/// Returns `true` when the input is a 2-letter or 3-digit region subtag.
#[must_use]
pub fn is_region_code(input: &str) -> bool {
    normalize_region_code(input).is_some()
}

/// Normalizes a 2-letter or 3-digit region subtag.
#[must_use]
pub fn normalize_region_code(input: &str) -> Option<String> {
    let trimmed = input.trim();
    if trimmed.len() == 2 && trimmed.bytes().all(|byte| byte.is_ascii_alphabetic()) {
        return Some(trimmed.to_ascii_uppercase());
    }

    if trimmed.len() == 3 && trimmed.bytes().all(|byte| byte.is_ascii_digit()) {
        return Some(trimmed.to_string());
    }

    None
}

#[cfg(test)]
mod tests {
    use super::{RegionCode, is_region_code, normalize_region_code, parse_region_code};

    #[test]
    fn accepts_common_region_examples() {
        for region in ["US", "GB", "CA", "JP", "DE", "419"] {
            assert!(is_region_code(region));
            assert_eq!(parse_region_code(region).unwrap().as_str(), region);
        }
    }

    #[test]
    fn normalizes_alpha_regions_to_uppercase() {
        assert_eq!(normalize_region_code("us"), Some("US".to_string()));
        assert_eq!(normalize_region_code("  gb  "), Some("GB".to_string()));
        assert_eq!(RegionCode::new("419").unwrap().as_str(), "419");
        assert!(RegionCode::new("419").unwrap().is_numeric());
    }

    #[test]
    fn rejects_invalid_region_shapes() {
        for region in ["", "U", "USA", "U1", "41A", "1234", "US-CA", "日本"] {
            assert!(!is_region_code(region));
            assert!(parse_region_code(region).is_none());
        }
    }
}
