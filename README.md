# use-locale

`use-locale` provides composable locale identifier primitives for `RustUse`.

It is a `RustUse` crate set for identifying, parsing, validating, normalizing, comparing, and composing locale-related identifiers without adopting a full localization framework.

## Workspace crates

- `use-locale`: thin facade crate that reexports the focused crates with a shared prelude
- `use-locale-tag`: BCP 47 / Unicode-style locale tag parsing and normalization primitives
- `use-language`: language subtag primitives
- `use-script`: writing script subtag primitives
- `use-region`: region subtag primitives for locale identifiers
- `use-currency-code`: ISO 4217-style currency code primitives
- `use-time-zone-id`: IANA time zone identifier syntax primitives
- `use-timezone`: time zone and fixed UTC/GMT offset primitives
- `use-locale-match`: locale preference and fallback primitives

## What it is useful for

- Validating locale tags.
- Normalizing language, script, and region subtags.
- Handling currency code identifiers.
- Handling time zone ID identifiers.
- Handling fixed UTC/GMT offsets.
- Building simple locale fallback chains.
- Composing locale-aware systems without adopting a full i18n framework.

## Umbrella crate

Use `use-locale` when you want a single dependency for the full set.

```rust
use use_locale::prelude::{best_locale_match, normalize_locale_tag, parse_currency_code};

assert_eq!(normalize_locale_tag("zh-hant-tw"), Some("zh-Hant-TW".to_string()));
assert_eq!(parse_currency_code("usd").unwrap().as_str(), "USD");
assert_eq!(best_locale_match("en-US", ["en", "fr"]).unwrap().available, "en");
```

## Scope

`use-locale` focuses on small identifier primitives grounded in BCP 47 language tags, Unicode CLDR/LDML locale concepts, ISO country and currency code shapes, IANA time zone identifier shapes, and fixed UTC/GMT offset shapes.

It intentionally performs syntax validation and casing normalization only. It does not ship CLDR, IANA tzdb, ISO registry snapshots, exchange-rate data, or language metadata databases.

## Non-goals

This repository is not:

- A translation framework.
- A CLDR runtime.
- An ICU replacement.
- A date/time library.
- A money library.
- A geocoding system.
- A full web negotiation library.

It also avoids translation catalogs, message formatting, language detection, NLP, currency formatting, time-zone offset calculation, daylight-saving rules, HTTP framework integration, network calls, build-time downloads, generated service data, and global runtime assumptions.

## License

Licensed under either of the following, at your option:

- Apache License, Version 2.0
- MIT license
