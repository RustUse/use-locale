# use-locale-match

Small locale preference and fallback primitives for `RustUse`.

## Example

```rust
use use_locale_match::{best_locale_match, fallback_chain};

assert_eq!(fallback_chain("zh-Hant-TW"), vec!["zh-Hant-TW", "zh-Hant", "zh"]);
assert_eq!(best_locale_match("en-US", ["en", "fr"]).unwrap().available, "en");
```

## Scope

- Build simple locale fallback chains.
- Match one requested locale against available locale tags.
- Normalize locale tags before comparison.

## Non-goals

- HTTP framework integration.
- Full Accept-Language parsing.
- Translation catalogs.
- Message formatting.
- CLDR likely-subtag matching.

## License

Licensed under either of the following, at your option:

- Apache License, Version 2.0
- MIT license
