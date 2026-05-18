# use-language

Small language subtag primitives for `RustUse`.

## Example

```rust
use use_language::{is_language_code, parse_language_code};

let language = parse_language_code("EN").unwrap();

assert_eq!(language.as_str(), "en");
assert!(is_language_code("zh"));
```

## Scope

- Validate simple 2-letter and 3-letter language subtags.
- Normalize language subtags to lowercase.
- Provide a small `LanguageCode` newtype for validated language subtags.

## Non-goals

- Translation.
- Language detection.
- NLP.
- Full ISO 639 registry data.

## License

Licensed under either of the following, at your option:

- Apache License, Version 2.0
- MIT license
