#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use core::fmt;

use use_language::{LanguageCode, parse_language_code};
use use_region::{RegionCode, parse_region_code};
use use_script::{ScriptCode, parse_script_code};

/// Parsed locale tag components.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct LocaleTagParts {
    pub language: LanguageCode,
    pub script: Option<ScriptCode>,
    pub region: Option<RegionCode>,
    pub variants: Vec<String>,
    pub extensions: Vec<String>,
    pub private_use: Option<String>,
}

impl LocaleTagParts {
    /// Builds the normalized tag string represented by these parts.
    #[must_use]
    pub fn to_tag_string(&self) -> String {
        let mut subtags = vec![self.language.as_str().to_string()];

        if let Some(script) = &self.script {
            subtags.push(script.as_str().to_string());
        }

        if let Some(region) = &self.region {
            subtags.push(region.as_str().to_string());
        }

        subtags.extend(self.variants.iter().cloned());
        subtags.extend(self.extensions.iter().cloned());

        if let Some(private_use) = &self.private_use {
            subtags.push(private_use.clone());
        }

        subtags.join("-")
    }
}

/// A normalized locale tag.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct LocaleTag {
    value: String,
    parts: LocaleTagParts,
}

impl LocaleTag {
    /// Parses and normalizes a locale tag.
    #[must_use]
    pub fn new(input: &str) -> Option<Self> {
        parse_locale_tag(input)
    }

    /// Returns the normalized locale tag.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.value
    }

    /// Returns the parsed locale tag parts.
    #[must_use]
    pub const fn parts(&self) -> &LocaleTagParts {
        &self.parts
    }

    /// Consumes the locale tag and returns the normalized string.
    #[must_use]
    pub fn into_string(self) -> String {
        self.value
    }
}

impl AsRef<str> for LocaleTag {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for LocaleTag {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// Parses a locale tag and normalizes its core language, script, and region subtags.
#[must_use]
pub fn parse_locale_tag(input: &str) -> Option<LocaleTag> {
    let parts = parse_locale_tag_parts(input)?;
    let value = parts.to_tag_string();

    Some(LocaleTag { value, parts })
}

/// Parses a locale tag into normalized parts.
#[must_use]
pub fn parse_locale_tag_parts(input: &str) -> Option<LocaleTagParts> {
    let trimmed = input.trim();
    if trimmed.is_empty() || trimmed.contains('_') {
        return None;
    }

    let subtags = trimmed.split('-').collect::<Vec<_>>();
    if subtags.iter().any(|subtag| subtag.is_empty()) {
        return None;
    }

    let language = parse_language_code(subtags.first().copied()?)?;
    let mut index = 1;

    let script = subtags
        .get(index)
        .and_then(|subtag| parse_script_code(subtag))
        .inspect(|_| index += 1);

    let region = subtags
        .get(index)
        .and_then(|subtag| parse_region_code(subtag))
        .inspect(|_| index += 1);

    let mut variants = Vec::new();
    let mut extensions = Vec::new();
    let mut private_use = None;
    let mut extension_singletons = Vec::new();

    while index < subtags.len() {
        let subtag = subtags[index];

        if is_private_use_singleton(subtag) {
            let tail = &subtags[index..];
            if tail.len() < 2 || !tail[1..].iter().all(|value| is_private_use_subtag(value)) {
                return None;
            }

            private_use = Some(tail.join("-"));
            index = subtags.len();
        } else if is_extension_singleton(subtag) {
            let singleton = subtag.to_ascii_lowercase();
            if extension_singletons.contains(&singleton) {
                return None;
            }
            extension_singletons.push(singleton);

            let start = index;
            index += 1;
            let payload_start = index;

            while index < subtags.len() && !is_singleton(subtags[index]) {
                if !is_extension_subtag(subtags[index]) {
                    return None;
                }
                index += 1;
            }

            if index == payload_start {
                return None;
            }

            extensions.push(subtags[start..index].join("-"));
        } else if is_variant_subtag(subtag) {
            variants.push(subtag.to_string());
            index += 1;
        } else {
            return None;
        }
    }

    Some(LocaleTagParts {
        language,
        script,
        region,
        variants,
        extensions,
        private_use,
    })
}

/// Normalizes a locale tag when it is syntactically valid for this crate's subset.
#[must_use]
pub fn normalize_locale_tag(input: &str) -> Option<String> {
    parse_locale_tag(input).map(LocaleTag::into_string)
}

/// Returns `true` when the input is a supported locale tag shape.
#[must_use]
pub fn is_locale_tag(input: &str) -> bool {
    parse_locale_tag(input).is_some()
}

fn is_singleton(subtag: &str) -> bool {
    subtag.len() == 1 && subtag.bytes().all(|byte| byte.is_ascii_alphanumeric())
}

fn is_extension_singleton(subtag: &str) -> bool {
    is_singleton(subtag) && !is_private_use_singleton(subtag)
}

const fn is_private_use_singleton(subtag: &str) -> bool {
    subtag.eq_ignore_ascii_case("x")
}

fn is_variant_subtag(subtag: &str) -> bool {
    let length = subtag.len();
    subtag.bytes().all(|byte| byte.is_ascii_alphanumeric())
        && ((5..=8).contains(&length)
            || (length == 4
                && subtag
                    .bytes()
                    .next()
                    .is_some_and(|byte| byte.is_ascii_digit())))
}

fn is_extension_subtag(subtag: &str) -> bool {
    (2..=8).contains(&subtag.len()) && subtag.bytes().all(|byte| byte.is_ascii_alphanumeric())
}

fn is_private_use_subtag(subtag: &str) -> bool {
    (1..=8).contains(&subtag.len()) && subtag.bytes().all(|byte| byte.is_ascii_alphanumeric())
}

#[cfg(test)]
mod tests {
    use super::{
        LocaleTag, is_locale_tag, normalize_locale_tag, parse_locale_tag, parse_locale_tag_parts,
    };

    #[test]
    fn parses_common_locale_tags() {
        for tag in ["en", "en-US", "en-Latn-US", "zh-Hant-TW", "sr-Cyrl-RS"] {
            assert!(is_locale_tag(tag));
            assert_eq!(parse_locale_tag(tag).unwrap().as_str(), tag);
        }
    }

    #[test]
    fn normalizes_core_subtag_casing() {
        assert_eq!(normalize_locale_tag("en-us"), Some("en-US".to_string()));
        assert_eq!(
            normalize_locale_tag("zh-hant-tw"),
            Some("zh-Hant-TW".to_string())
        );
        assert_eq!(LocaleTag::new("SR-cYRL-rs").unwrap().as_str(), "sr-Cyrl-RS");
    }

    #[test]
    fn exposes_normalized_parts() {
        let parts = parse_locale_tag_parts("zh-hant-tw").unwrap();

        assert_eq!(parts.language.as_str(), "zh");
        assert_eq!(parts.script.unwrap().as_str(), "Hant");
        assert_eq!(parts.region.unwrap().as_str(), "TW");
    }

    #[test]
    fn preserves_supported_suffixes() {
        let tag = parse_locale_tag("en-us-oxendict-u-ca-gregory-x-app").unwrap();

        assert_eq!(tag.as_str(), "en-US-oxendict-u-ca-gregory-x-app");
        assert_eq!(tag.parts().variants, vec!["oxendict"]);
        assert_eq!(tag.parts().extensions, vec!["u-ca-gregory"]);
        assert_eq!(tag.parts().private_use.as_deref(), Some("x-app"));
    }

    #[test]
    fn rejects_invalid_locale_tag_shapes() {
        for tag in [
            "",
            "en_ US",
            "en_US",
            "en--US",
            "e-US",
            "en-Lat-US",
            "en-Latn-USA",
            "en-u",
            "en-u-ca-u-nu",
            "en-x",
            "en-@",
        ] {
            assert!(!is_locale_tag(tag));
            assert!(parse_locale_tag(tag).is_none());
        }
    }
}
