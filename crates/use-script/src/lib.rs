#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use core::fmt;

/// A normalized 4-letter writing script subtag.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ScriptCode {
    value: String,
}

impl ScriptCode {
    /// Parses and normalizes a script subtag.
    #[must_use]
    pub fn new(input: &str) -> Option<Self> {
        parse_script_code(input)
    }

    /// Returns the normalized script subtag.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.value
    }

    /// Consumes the script code and returns the normalized string.
    #[must_use]
    pub fn into_string(self) -> String {
        self.value
    }
}

impl AsRef<str> for ScriptCode {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for ScriptCode {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// Parses a script subtag and normalizes it to title case.
#[must_use]
pub fn parse_script_code(input: &str) -> Option<ScriptCode> {
    normalize_script_code(input).map(|value| ScriptCode { value })
}

/// Returns `true` when the input is a 4-letter script subtag.
#[must_use]
pub fn is_script_code(input: &str) -> bool {
    normalize_script_code(input).is_some()
}

/// Normalizes a 4-letter script subtag to title case.
#[must_use]
pub fn normalize_script_code(input: &str) -> Option<String> {
    let trimmed = input.trim();
    if trimmed.len() != 4 || !trimmed.bytes().all(|byte| byte.is_ascii_alphabetic()) {
        return None;
    }

    let mut normalized = String::with_capacity(4);
    for (index, character) in trimmed.chars().enumerate() {
        if index == 0 {
            normalized.push(character.to_ascii_uppercase());
        } else {
            normalized.push(character.to_ascii_lowercase());
        }
    }

    Some(normalized)
}

#[cfg(test)]
mod tests {
    use super::{ScriptCode, is_script_code, normalize_script_code, parse_script_code};

    #[test]
    fn accepts_common_script_examples() {
        for script in ["Latn", "Cyrl", "Arab", "Hans", "Hant"] {
            assert!(is_script_code(script));
            assert_eq!(parse_script_code(script).unwrap().as_str(), script);
        }
    }

    #[test]
    fn normalizes_to_title_case() {
        assert_eq!(normalize_script_code("latn"), Some("Latn".to_string()));
        assert_eq!(normalize_script_code("CYRL"), Some("Cyrl".to_string()));
        assert_eq!(ScriptCode::new(" hAnT ").unwrap().as_str(), "Hant");
    }

    #[test]
    fn rejects_invalid_script_shapes() {
        for script in ["", "Lat", "Latnn", "La1n", "Latn-US", "漢字"] {
            assert!(!is_script_code(script));
            assert!(parse_script_code(script).is_none());
        }
    }
}
