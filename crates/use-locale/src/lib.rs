#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

pub use use_currency_code;
pub use use_language;
pub use use_locale_match;
pub use use_locale_tag;
pub use use_region;
pub use use_script;
pub use use_time_zone_id;

pub mod prelude;

#[cfg(test)]
mod tests {
    use super::prelude::{
        best_locale_match, fallback_chain, normalize_locale_tag, parse_currency_code,
        parse_language_code, parse_locale_tag, parse_region_code, parse_script_code,
        parse_time_zone_id,
    };

    #[test]
    fn facade_exposes_common_locale_primitives() {
        assert_eq!(parse_language_code("EN").unwrap().as_str(), "en");
        assert_eq!(parse_script_code("latn").unwrap().as_str(), "Latn");
        assert_eq!(parse_region_code("us").unwrap().as_str(), "US");
        assert_eq!(parse_currency_code("usd").unwrap().as_str(), "USD");
        assert_eq!(
            parse_time_zone_id("America/New_York").unwrap().area(),
            "America"
        );
        assert_eq!(
            parse_locale_tag("zh-hant-tw").unwrap().as_str(),
            "zh-Hant-TW"
        );
        assert_eq!(normalize_locale_tag("en-us"), Some("en-US".to_string()));
        assert_eq!(fallback_chain("en-US"), vec!["en-US", "en"]);
        assert_eq!(
            best_locale_match("en-US", ["en", "fr"]).unwrap().available,
            "en"
        );
    }
}
