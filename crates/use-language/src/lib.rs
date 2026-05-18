#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use core::fmt;

/// A normalized 2-letter or 3-letter language subtag.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct LanguageCode {
    value: String,
}

impl LanguageCode {
    /// Parses and normalizes a language subtag.
    #[must_use]
    pub fn new(input: &str) -> Option<Self> {
        parse_language_code(input)
    }

    /// Returns the normalized language subtag.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.value
    }

    /// Consumes the language code and returns the normalized string.
    #[must_use]
    pub fn into_string(self) -> String {
        self.value
    }
}

impl AsRef<str> for LanguageCode {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for LanguageCode {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// Parses a language subtag and normalizes it to lowercase.
#[must_use]
pub fn parse_language_code(input: &str) -> Option<LanguageCode> {
    normalize_language_code(input).map(|value| LanguageCode { value })
}

/// Returns `true` when the input is a simple 2-letter or 3-letter language subtag.
#[must_use]
pub fn is_language_code(input: &str) -> bool {
    normalize_language_code(input).is_some()
}

/// Normalizes a simple language subtag to lowercase.
#[must_use]
pub fn normalize_language_code(input: &str) -> Option<String> {
    let trimmed = input.trim();
    if !matches!(trimmed.len(), 2 | 3) || !trimmed.bytes().all(|byte| byte.is_ascii_alphabetic()) {
        return None;
    }

    Some(trimmed.to_ascii_lowercase())
}

#[cfg(test)]
mod tests {
    use super::{LanguageCode, is_language_code, normalize_language_code, parse_language_code};

    #[test]
    fn accepts_common_language_examples() {
        for language in ["en", "es", "fr", "de", "zh", "ar", "ja"] {
            assert!(is_language_code(language));
            assert_eq!(parse_language_code(language).unwrap().as_str(), language);
        }
    }

    #[test]
    fn normalizes_to_lowercase() {
        assert_eq!(normalize_language_code("EN"), Some("en".to_string()));
        assert_eq!(LanguageCode::new(" ZHO ").unwrap().as_str(), "zho");
    }

    #[test]
    fn rejects_invalid_language_shapes() {
        for language in ["", "e", "engb", "en-US", "e1", "en_", "中文"] {
            assert!(!is_language_code(language));
            assert!(parse_language_code(language).is_none());
        }
    }
}
