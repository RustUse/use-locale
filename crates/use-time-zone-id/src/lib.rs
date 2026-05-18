#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use core::fmt;

/// A syntactically valid IANA-shaped time zone identifier.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct TimeZoneId {
    value: String,
}

impl TimeZoneId {
    /// Parses a time zone identifier.
    #[must_use]
    pub fn new(input: &str) -> Option<Self> {
        parse_time_zone_id(input)
    }

    /// Returns the identifier text.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.value
    }

    /// Consumes the time zone identifier and returns the string.
    #[must_use]
    pub fn into_string(self) -> String {
        self.value
    }

    /// Returns the first segment, such as `America` or `UTC`.
    #[must_use]
    pub fn area(&self) -> &str {
        self.value
            .split_once('/')
            .map_or(self.as_str(), |(area, _)| area)
    }

    /// Returns the remaining location path after the area segment.
    #[must_use]
    pub fn location(&self) -> Option<&str> {
        self.value.split_once('/').map(|(_, location)| location)
    }

    /// Returns the identifier segments.
    #[must_use]
    pub fn segments(&self) -> Vec<&str> {
        self.value.split('/').collect()
    }
}

impl AsRef<str> for TimeZoneId {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for TimeZoneId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// Parses a syntactically valid IANA-shaped time zone identifier.
#[must_use]
pub fn parse_time_zone_id(input: &str) -> Option<TimeZoneId> {
    is_time_zone_id(input).then(|| TimeZoneId {
        value: input.to_string(),
    })
}

/// Returns `true` when the input is a syntactically valid IANA-shaped identifier.
#[must_use]
pub fn is_time_zone_id(input: &str) -> bool {
    let trimmed = input.trim();
    if trimmed.is_empty()
        || trimmed != input
        || trimmed.starts_with('/')
        || trimmed.ends_with('/')
        || trimmed.contains("//")
        || trimmed.chars().any(char::is_whitespace)
    {
        return false;
    }

    trimmed.split('/').all(is_time_zone_segment)
}

/// Splits a valid time zone identifier into owned segments.
#[must_use]
pub fn split_time_zone_id(input: &str) -> Option<Vec<String>> {
    parse_time_zone_id(input).map(|zone| {
        zone.as_str()
            .split('/')
            .map(ToOwned::to_owned)
            .collect::<Vec<_>>()
    })
}

fn is_time_zone_segment(segment: &str) -> bool {
    !segment.is_empty()
        && !matches!(segment, "." | "..")
        && segment
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'_' | b'-' | b'.' | b'+'))
}

#[cfg(test)]
mod tests {
    use super::{TimeZoneId, is_time_zone_id, parse_time_zone_id, split_time_zone_id};

    #[test]
    fn accepts_common_time_zone_id_shapes() {
        for zone in [
            "UTC",
            "America/New_York",
            "America/Indiana/Indianapolis",
            "Europe/London",
            "Asia/Tokyo",
        ] {
            assert!(is_time_zone_id(zone));
            assert_eq!(parse_time_zone_id(zone).unwrap().as_str(), zone);
        }
    }

    #[test]
    fn splits_area_and_location() {
        let zone = TimeZoneId::new("America/Indiana/Indianapolis").unwrap();

        assert_eq!(zone.area(), "America");
        assert_eq!(zone.location(), Some("Indiana/Indianapolis"));
        assert_eq!(zone.segments(), vec!["America", "Indiana", "Indianapolis"]);
        assert_eq!(split_time_zone_id("UTC"), Some(vec!["UTC".to_string()]));
    }

    #[test]
    fn rejects_invalid_time_zone_id_shapes() {
        for zone in [
            "",
            " America/New_York",
            "America/New_York ",
            "America//Indianapolis",
            "/America/New_York",
            "America/New_York/",
            "America/New York",
            "America/..",
            "America/@Home",
        ] {
            assert!(!is_time_zone_id(zone));
            assert!(parse_time_zone_id(zone).is_none());
        }
    }
}
