# use-currency-code

Small ISO 4217-style currency code primitives for `RustUse`.

## Example

```rust
use use_currency_code::{is_currency_code, parse_currency_code};

let currency = parse_currency_code("usd").unwrap();

assert_eq!(currency.as_str(), "USD");
assert!(is_currency_code("EUR"));
```

## Scope

- Validate 3-letter alphabetic currency code shapes.
- Normalize currency code identifiers to uppercase.
- Provide a small `CurrencyCode` newtype for validated currency identifiers.

## Non-goals

- Exchange rates.
- Money arithmetic.
- Decimal formatting.
- Live currency metadata downloads.
- Finance APIs.

## License

Licensed under either of the following, at your option:

- Apache License, Version 2.0
- MIT license
