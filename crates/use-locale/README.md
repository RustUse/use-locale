# use-locale

Thin facade crate for `RustUse` locale identifier primitives.

## Example

```rust
use use_locale::prelude::{best_locale_match, normalize_locale_tag, parse_currency_code};

assert_eq!(normalize_locale_tag("en-us"), Some("en-US".to_string()));
assert_eq!(parse_currency_code("usd").unwrap().as_str(), "USD");
assert_eq!(best_locale_match("en-US", ["en", "fr"]).unwrap().available, "en");
```

## Scope

- Reexport the focused `use-locale` child crates.
- Provide a shared prelude for common identifier parsing and matching helpers.
- Keep implementation logic in focused child crates.

## Non-goals

- Translation frameworks.
- CLDR runtimes.
- ICU replacement APIs.
- Date/time libraries.
- Money libraries.
- Geocoding systems.
- Full web negotiation libraries.

## License

Licensed under either of the following, at your option:

- Apache License, Version 2.0
- MIT license
