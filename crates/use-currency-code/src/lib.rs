#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use core::fmt;

/// Common currency code examples used in docs and tests.
pub const COMMON_CURRENCY_CODES: &[&str] = &["USD", "EUR", "GBP", "JPY", "CAD", "AUD", "CHF"];

/// A normalized 3-letter currency code identifier.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct CurrencyCode {
    value: String,
}

impl CurrencyCode {
    /// Parses and normalizes a currency code identifier.
    #[must_use]
    pub fn new(input: &str) -> Option<Self> {
        parse_currency_code(input)
    }

    /// Returns the normalized currency code identifier.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.value
    }

    /// Consumes the currency code and returns the normalized string.
    #[must_use]
    pub fn into_string(self) -> String {
        self.value
    }
}

impl AsRef<str> for CurrencyCode {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for CurrencyCode {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// Parses a currency code identifier and normalizes it to uppercase.
#[must_use]
pub fn parse_currency_code(input: &str) -> Option<CurrencyCode> {
    normalize_currency_code(input).map(|value| CurrencyCode { value })
}

/// Returns `true` when the input is a 3-letter currency code identifier.
#[must_use]
pub fn is_currency_code(input: &str) -> bool {
    normalize_currency_code(input).is_some()
}

/// Normalizes a 3-letter currency code identifier to uppercase.
#[must_use]
pub fn normalize_currency_code(input: &str) -> Option<String> {
    let trimmed = input.trim();
    if trimmed.len() != 3 || !trimmed.bytes().all(|byte| byte.is_ascii_alphabetic()) {
        return None;
    }

    Some(trimmed.to_ascii_uppercase())
}

#[cfg(test)]
mod tests {
    use super::{
        COMMON_CURRENCY_CODES, CurrencyCode, is_currency_code, normalize_currency_code,
        parse_currency_code,
    };

    #[test]
    fn accepts_common_currency_examples() {
        for currency in COMMON_CURRENCY_CODES {
            assert!(is_currency_code(currency));
            assert_eq!(parse_currency_code(currency).unwrap().as_str(), *currency);
        }
    }

    #[test]
    fn normalizes_to_uppercase() {
        assert_eq!(normalize_currency_code("usd"), Some("USD".to_string()));
        assert_eq!(normalize_currency_code(" eur "), Some("EUR".to_string()));
        assert_eq!(CurrencyCode::new("gbp").unwrap().as_str(), "GBP");
    }

    #[test]
    fn rejects_invalid_currency_shapes() {
        for currency in ["", "US", "USDA", "U1D", "USD-", "€€€"] {
            assert!(!is_currency_code(currency));
            assert!(parse_currency_code(currency).is_none());
        }
    }
}
