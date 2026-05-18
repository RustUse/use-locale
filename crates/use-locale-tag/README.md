# use-locale-tag

Small BCP 47 / Unicode-style locale tag parsing and normalization primitives for `RustUse`.

## Example

```rust
use use_locale_tag::{normalize_locale_tag, parse_locale_tag};

let tag = parse_locale_tag("zh-hant-tw").unwrap();

assert_eq!(tag.as_str(), "zh-Hant-TW");
assert_eq!(normalize_locale_tag("en-us"), Some("en-US".to_string()));
```

## Scope

- Parse locale tags with language, optional script, optional region, and supported suffixes.
- Normalize language subtags to lowercase.
- Normalize script subtags to title case.
- Normalize alphabetic region subtags to uppercase.
- Preserve supported variants, extensions, and private-use suffixes without database canonicalization.

## Non-goals

- Full CLDR canonicalization.
- ICU replacement behavior.
- Translation catalogs.
- Message formatting.
- Locale data loading.

## License

Licensed under either of the following, at your option:

- Apache License, Version 2.0
- MIT license
