#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use use_locale_tag::{LocaleTagParts, normalize_locale_tag, parse_locale_tag_parts};

/// A normalized locale preference with an explicit priority.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LocalePreference {
    pub locale: String,
    pub priority: usize,
}

impl LocalePreference {
    /// Parses and normalizes a locale preference.
    #[must_use]
    pub fn new(locale: &str, priority: usize) -> Option<Self> {
        normalize_locale_tag(locale).map(|locale| Self { locale, priority })
    }
}

/// A simple locale match result.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LocaleMatch {
    pub requested: String,
    pub available: String,
    pub fallback_index: usize,
}

impl LocaleMatch {
    /// Returns `true` when the match was exact.
    #[must_use]
    pub const fn is_exact(&self) -> bool {
        self.fallback_index == 0
    }
}

/// Builds a most-specific to least-specific fallback chain for a locale tag.
#[must_use]
pub fn fallback_chain(input: &str) -> Vec<String> {
    let Some(mut parts) = parse_locale_tag_parts(input) else {
        return Vec::new();
    };

    let mut chain = Vec::new();
    push_unique_tag(&mut chain, &parts);

    if parts.private_use.take().is_some() {
        push_unique_tag(&mut chain, &parts);
    }

    while parts.extensions.pop().is_some() {
        push_unique_tag(&mut chain, &parts);
    }

    while parts.variants.pop().is_some() {
        push_unique_tag(&mut chain, &parts);
    }

    if parts.region.take().is_some() {
        push_unique_tag(&mut chain, &parts);
    }

    if parts.script.take().is_some() {
        push_unique_tag(&mut chain, &parts);
    }

    chain
}

/// Finds the best available locale for a requested locale.
#[must_use]
pub fn best_locale_match<I, S>(requested: &str, available: I) -> Option<LocaleMatch>
where
    I: IntoIterator<Item = S>,
    S: AsRef<str>,
{
    let chain = fallback_chain(requested);
    let requested = chain.first()?.clone();
    let available = available
        .into_iter()
        .filter_map(|locale| normalize_locale_tag(locale.as_ref()))
        .collect::<Vec<_>>();

    for (fallback_index, candidate) in chain.iter().enumerate() {
        if let Some(matched) = available.iter().find(|locale| *locale == candidate) {
            return Some(LocaleMatch {
                requested,
                available: matched.clone(),
                fallback_index,
            });
        }
    }

    None
}

fn push_unique_tag(chain: &mut Vec<String>, parts: &LocaleTagParts) {
    let candidate = parts.to_tag_string();
    if chain.last() != Some(&candidate) {
        chain.push(candidate);
    }
}

#[cfg(test)]
mod tests {
    use super::{LocalePreference, best_locale_match, fallback_chain};

    #[test]
    fn builds_expected_fallback_chains() {
        assert_eq!(fallback_chain("en-US"), vec!["en-US", "en"]);
        assert_eq!(
            fallback_chain("zh-Hant-TW"),
            vec!["zh-Hant-TW", "zh-Hant", "zh"]
        );
    }

    #[test]
    fn removes_suffixes_before_core_subtags() {
        assert_eq!(
            fallback_chain("en-US-oxendict-u-ca-gregory-x-app"),
            vec![
                "en-US-oxendict-u-ca-gregory-x-app",
                "en-US-oxendict-u-ca-gregory",
                "en-US-oxendict",
                "en-US",
                "en",
            ]
        );
    }

    #[test]
    fn best_match_uses_fallback_order() {
        let matched = best_locale_match("en-US", ["en", "fr"]).unwrap();

        assert_eq!(matched.requested, "en-US");
        assert_eq!(matched.available, "en");
        assert_eq!(matched.fallback_index, 1);
        assert!(!matched.is_exact());
    }

    #[test]
    fn exact_matches_win() {
        let matched = best_locale_match("en-US", ["en", "en-us"]).unwrap();

        assert_eq!(matched.available, "en-US");
        assert!(matched.is_exact());
    }

    #[test]
    fn invalid_requested_locale_has_no_match() {
        assert!(best_locale_match("not_a_locale", ["en"]).is_none());
        assert!(fallback_chain("not_a_locale").is_empty());
    }

    #[test]
    fn builds_normalized_preferences() {
        let preference = LocalePreference::new("ZH-hant-tw", 0).unwrap();

        assert_eq!(preference.locale, "zh-Hant-TW");
        assert_eq!(preference.priority, 0);
    }
}
