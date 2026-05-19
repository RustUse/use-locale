#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use core::fmt;

use use_time_zone_id::{TimeZoneId, parse_time_zone_id};

const MAX_OFFSET_MINUTES: i16 = 14 * 60;

/// A time zone represented by either an IANA-shaped identifier or a fixed offset.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum TimeZone {
    /// An IANA-shaped time zone identifier.
    Iana(TimeZoneId),
    /// A fixed UTC offset.
    FixedOffset(TimeZoneOffset),
}

impl TimeZone {
    /// Parses a time zone.
    #[must_use]
    pub fn new(input: &str) -> Option<Self> {
        parse_time_zone(input)
    }

    /// Parses a time zone with diagnostic errors.
    ///
    /// # Errors
    ///
    /// Returns [`TimeZoneParseError`] when the input is empty, contains whitespace,
    /// is an invalid fixed offset, or is not an IANA-shaped identifier.
    pub fn try_new(input: &str) -> Result<Self, TimeZoneParseError> {
        try_parse_time_zone(input)
    }

    /// Returns an IANA time zone value.
    #[must_use]
    pub const fn iana(identifier: TimeZoneId) -> Self {
        Self::Iana(identifier)
    }

    /// Returns a fixed-offset time zone value.
    #[must_use]
    pub const fn fixed_offset(offset: TimeZoneOffset) -> Self {
        Self::FixedOffset(offset)
    }

    /// Returns the IANA identifier when this time zone is identifier-based.
    #[must_use]
    pub const fn as_time_zone_id(&self) -> Option<&TimeZoneId> {
        match self {
            Self::Iana(identifier) => Some(identifier),
            Self::FixedOffset(_) => None,
        }
    }

    /// Returns the fixed offset when this time zone is offset-based.
    #[must_use]
    pub const fn offset(&self) -> Option<TimeZoneOffset> {
        match self {
            Self::Iana(_) => None,
            Self::FixedOffset(offset) => Some(*offset),
        }
    }

    /// Returns `true` when this time zone is an IANA-shaped identifier.
    #[must_use]
    pub const fn is_iana(&self) -> bool {
        matches!(self, Self::Iana(_))
    }

    /// Returns `true` when this time zone is a fixed offset.
    #[must_use]
    pub const fn is_fixed_offset(&self) -> bool {
        matches!(self, Self::FixedOffset(_))
    }
}

impl fmt::Display for TimeZone {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Iana(identifier) => formatter.write_str(identifier.as_str()),
            Self::FixedOffset(offset) => fmt::Display::fmt(offset, formatter),
        }
    }
}

/// A fixed UTC offset in signed minutes.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct TimeZoneOffset {
    minutes: i16,
}

impl TimeZoneOffset {
    /// The zero UTC offset.
    pub const UTC: Self = Self { minutes: 0 };

    /// The minimum civil time zone offset.
    pub const MIN: Self = Self {
        minutes: -MAX_OFFSET_MINUTES,
    };

    /// The maximum civil time zone offset.
    pub const MAX: Self = Self {
        minutes: MAX_OFFSET_MINUTES,
    };

    /// Parses a fixed time zone offset.
    #[must_use]
    pub fn new(input: &str) -> Option<Self> {
        parse_time_zone_offset(input)
    }

    /// Parses a fixed time zone offset with diagnostic errors.
    ///
    /// # Errors
    ///
    /// Returns [`TimeZoneParseError`] when the input is empty, contains whitespace,
    /// is malformed, or falls outside the civil `-14:00..=+14:00` range.
    pub fn try_new(input: &str) -> Result<Self, TimeZoneParseError> {
        try_parse_time_zone_offset(input)
    }

    /// Returns an offset from signed minutes when it is in the civil range.
    #[must_use]
    pub const fn from_minutes(minutes: i16) -> Option<Self> {
        if minutes < -MAX_OFFSET_MINUTES || minutes > MAX_OFFSET_MINUTES {
            None
        } else {
            Some(Self { minutes })
        }
    }

    /// Returns the signed offset in minutes.
    #[must_use]
    pub const fn total_minutes(self) -> i16 {
        self.minutes
    }

    /// Returns `true` when this is the zero UTC offset.
    #[must_use]
    pub const fn is_utc(self) -> bool {
        self.minutes == 0
    }
}

impl fmt::Display for TimeZoneOffset {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_utc() {
            return formatter.write_str("UTC");
        }

        let sign = if self.minutes.is_negative() { '-' } else { '+' };
        let absolute_minutes = self.minutes.unsigned_abs();
        let hours = absolute_minutes / 60;
        let minutes = absolute_minutes % 60;

        write!(formatter, "UTC{sign}{hours:02}:{minutes:02}")
    }
}

/// A time zone or fixed-offset parse error.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum TimeZoneParseError {
    /// The input was empty.
    Empty,
    /// The input contained whitespace.
    ContainsWhitespace,
    /// The fixed-offset input was malformed.
    InvalidOffsetFormat,
    /// The fixed-offset hour field was malformed.
    InvalidOffsetHour,
    /// The fixed-offset minute field was malformed or out of range.
    InvalidOffsetMinute,
    /// The fixed offset was outside the civil `-14:00..=+14:00` range.
    OffsetOutOfRange,
    /// The input was not an IANA-shaped time zone identifier.
    InvalidTimeZoneId,
}

impl fmt::Display for TimeZoneParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let message = match self {
            Self::Empty => "time zone input is empty",
            Self::ContainsWhitespace => "time zone input contains whitespace",
            Self::InvalidOffsetFormat => "fixed time zone offset is malformed",
            Self::InvalidOffsetHour => "fixed time zone offset hour is malformed",
            Self::InvalidOffsetMinute => "fixed time zone offset minute is malformed",
            Self::OffsetOutOfRange => "fixed time zone offset is outside -14:00..=+14:00",
            Self::InvalidTimeZoneId => "time zone identifier is malformed",
        };

        formatter.write_str(message)
    }
}

impl std::error::Error for TimeZoneParseError {}

/// Parses a time zone from an IANA-shaped identifier or fixed offset.
#[must_use]
pub fn parse_time_zone(input: &str) -> Option<TimeZone> {
    try_parse_time_zone(input).ok()
}

/// Parses a time zone from an IANA-shaped identifier or fixed offset with diagnostic errors.
///
/// # Errors
///
/// Returns [`TimeZoneParseError`] when the input is empty, contains whitespace,
/// is an invalid fixed offset, or is not an IANA-shaped identifier.
pub fn try_parse_time_zone(input: &str) -> Result<TimeZone, TimeZoneParseError> {
    reject_empty_or_whitespace(input)?;

    if is_offset_candidate(input) {
        return try_parse_time_zone_offset(input).map(TimeZone::FixedOffset);
    }

    parse_time_zone_id(input)
        .map(TimeZone::Iana)
        .ok_or(TimeZoneParseError::InvalidTimeZoneId)
}

/// Returns `true` when the input is a valid IANA-shaped identifier or fixed offset.
#[must_use]
pub fn is_time_zone(input: &str) -> bool {
    parse_time_zone(input).is_some()
}

/// Parses a fixed time zone offset.
#[must_use]
pub fn parse_time_zone_offset(input: &str) -> Option<TimeZoneOffset> {
    try_parse_time_zone_offset(input).ok()
}

/// Parses a fixed time zone offset with diagnostic errors.
///
/// # Errors
///
/// Returns [`TimeZoneParseError`] when the input is empty, contains whitespace,
/// is malformed, or falls outside the civil `-14:00..=+14:00` range.
pub fn try_parse_time_zone_offset(input: &str) -> Result<TimeZoneOffset, TimeZoneParseError> {
    reject_empty_or_whitespace(input)?;

    if matches!(input, "Z" | "UTC") {
        return Ok(TimeZoneOffset::UTC);
    }

    let signed_offset =
        strip_offset_prefix(input).ok_or(TimeZoneParseError::InvalidOffsetFormat)?;

    parse_signed_offset(signed_offset)
}

/// Returns `true` when the input is a valid fixed time zone offset.
#[must_use]
pub fn is_time_zone_offset(input: &str) -> bool {
    parse_time_zone_offset(input).is_some()
}

fn reject_empty_or_whitespace(input: &str) -> Result<(), TimeZoneParseError> {
    if input.is_empty() {
        return Err(TimeZoneParseError::Empty);
    }

    if input.chars().any(char::is_whitespace) {
        return Err(TimeZoneParseError::ContainsWhitespace);
    }

    Ok(())
}

fn is_offset_candidate(input: &str) -> bool {
    matches!(input, "Z" | "UTC")
        || input.starts_with('+')
        || input.starts_with('-')
        || has_signed_prefix(input, "UTC")
        || has_signed_prefix(input, "GMT")
}

fn has_signed_prefix(input: &str, prefix: &str) -> bool {
    input
        .strip_prefix(prefix)
        .is_some_and(|remainder| remainder.starts_with('+') || remainder.starts_with('-'))
}

fn strip_offset_prefix(input: &str) -> Option<&str> {
    if input.starts_with('+') || input.starts_with('-') {
        return Some(input);
    }

    input
        .strip_prefix("UTC")
        .filter(|remainder| remainder.starts_with('+') || remainder.starts_with('-'))
        .or_else(|| {
            input
                .strip_prefix("GMT")
                .filter(|remainder| remainder.starts_with('+') || remainder.starts_with('-'))
        })
}

fn parse_signed_offset(input: &str) -> Result<TimeZoneOffset, TimeZoneParseError> {
    let (is_negative, body) = split_offset_sign(input)?;
    let bytes = body.as_bytes();
    let (hours, minutes) = match bytes.len() {
        2 => (
            parse_digit_pair(bytes, TimeZoneParseError::InvalidOffsetHour)?,
            0,
        ),
        4 => (
            parse_digit_pair(&bytes[..2], TimeZoneParseError::InvalidOffsetHour)?,
            parse_digit_pair(&bytes[2..], TimeZoneParseError::InvalidOffsetMinute)?,
        ),
        5 if bytes[2] == b':' => (
            parse_digit_pair(&bytes[..2], TimeZoneParseError::InvalidOffsetHour)?,
            parse_digit_pair(&bytes[3..], TimeZoneParseError::InvalidOffsetMinute)?,
        ),
        _ => return Err(TimeZoneParseError::InvalidOffsetFormat),
    };

    if minutes > 59 {
        return Err(TimeZoneParseError::InvalidOffsetMinute);
    }

    let unsigned_minutes = (hours * 60) + minutes;
    let signed_minutes = if is_negative {
        -unsigned_minutes
    } else {
        unsigned_minutes
    };

    TimeZoneOffset::from_minutes(signed_minutes).ok_or(TimeZoneParseError::OffsetOutOfRange)
}

fn split_offset_sign(input: &str) -> Result<(bool, &str), TimeZoneParseError> {
    match (input.strip_prefix('+'), input.strip_prefix('-')) {
        (Some(body), _) => Ok((false, body)),
        (None, Some(body)) => Ok((true, body)),
        (None, None) => Err(TimeZoneParseError::InvalidOffsetFormat),
    }
}

fn parse_digit_pair(bytes: &[u8], error: TimeZoneParseError) -> Result<i16, TimeZoneParseError> {
    let [tens, ones] = bytes else {
        return Err(error);
    };

    if !tens.is_ascii_digit() || !ones.is_ascii_digit() {
        return Err(error);
    }

    Ok((i16::from(*tens - b'0') * 10) + i16::from(*ones - b'0'))
}

#[cfg(test)]
mod tests {
    use super::{
        TimeZone, TimeZoneOffset, TimeZoneParseError, is_time_zone, is_time_zone_offset,
        parse_time_zone, parse_time_zone_offset, try_parse_time_zone, try_parse_time_zone_offset,
    };

    #[test]
    fn parses_iana_time_zone_ids() {
        let zone = parse_time_zone("America/New_York");

        assert!(matches!(zone, Some(TimeZone::Iana(_))));

        if let Some(TimeZone::Iana(identifier)) = zone {
            assert_eq!(identifier.area(), "America");
            assert_eq!(identifier.location(), Some("New_York"));
        } else {
            panic!("expected IANA time zone");
        }
    }

    #[test]
    fn parses_fixed_offset_shapes() {
        for (input, minutes, display) in [
            ("Z", 0, "UTC"),
            ("UTC", 0, "UTC"),
            ("+05:30", 330, "UTC+05:30"),
            ("-08:00", -480, "UTC-08:00"),
            ("+0530", 330, "UTC+05:30"),
            ("-0800", -480, "UTC-08:00"),
            ("+05", 300, "UTC+05:00"),
            ("-08", -480, "UTC-08:00"),
            ("UTC+05:30", 330, "UTC+05:30"),
            ("GMT-08:00", -480, "UTC-08:00"),
        ] {
            let offset = parse_time_zone_offset(input);

            assert_eq!(offset.map(TimeZoneOffset::total_minutes), Some(minutes));
            assert_eq!(
                offset.map(|value| value.to_string()),
                Some(display.to_string())
            );
            assert!(is_time_zone_offset(input));
        }
    }

    #[test]
    fn parses_time_zone_offsets_as_time_zones() {
        let zone = parse_time_zone("UTC+05:30");

        assert!(matches!(zone, Some(TimeZone::FixedOffset(_))));
        assert_eq!(
            zone.map(|value| value.to_string()),
            Some("UTC+05:30".to_string())
        );
        assert!(is_time_zone("UTC+05:30"));
    }

    #[test]
    fn keeps_offsets_in_the_civil_range() {
        assert_eq!(
            TimeZoneOffset::from_minutes(-840),
            Some(TimeZoneOffset::MIN)
        );
        assert_eq!(TimeZoneOffset::from_minutes(840), Some(TimeZoneOffset::MAX));
        assert_eq!(parse_time_zone_offset("-14:00"), Some(TimeZoneOffset::MIN));
        assert_eq!(parse_time_zone_offset("+14:00"), Some(TimeZoneOffset::MAX));
        assert_eq!(parse_time_zone_offset("-14:01"), None);
        assert_eq!(parse_time_zone_offset("+14:01"), None);
    }

    #[test]
    fn rejects_invalid_fixed_offset_shapes() {
        for input in [
            "",
            " +05:00",
            "+05:00 ",
            "UTC +05:00",
            "PST",
            "+5",
            "+05:3",
            "+05:60",
            "+15:00",
            "UTC+99:00",
            "UT+05:00",
        ] {
            assert!(!is_time_zone_offset(input), "{input}");
            assert_eq!(parse_time_zone_offset(input), None, "{input}");
        }
    }

    #[test]
    fn reports_diagnostic_errors() {
        assert_eq!(
            try_parse_time_zone_offset(""),
            Err(TimeZoneParseError::Empty)
        );
        assert_eq!(
            try_parse_time_zone_offset("+05:00 "),
            Err(TimeZoneParseError::ContainsWhitespace)
        );
        assert_eq!(
            try_parse_time_zone_offset("+05:60"),
            Err(TimeZoneParseError::InvalidOffsetMinute)
        );
        assert_eq!(
            try_parse_time_zone_offset("+14:01"),
            Err(TimeZoneParseError::OffsetOutOfRange)
        );
        assert_eq!(
            try_parse_time_zone("America/@Home"),
            Err(TimeZoneParseError::InvalidTimeZoneId)
        );
    }
}
