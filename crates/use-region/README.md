# use-region

Small region subtag primitives for `RustUse`.

## Example

```rust
use use_region::{is_region_code, parse_region_code};

let region = parse_region_code("us").unwrap();

assert_eq!(region.as_str(), "US");
assert!(is_region_code("419"));
```

## Scope

- Validate 2-letter region subtags.
- Validate 3-digit UN M.49-style region subtags.
- Normalize alphabetic region subtags to uppercase.

## Non-goals

- Geocoding.
- Geopolitical opinions.
- Address formatting.
- Maps.
- Subdivision data.

## License

Licensed under either of the following, at your option:

- Apache License, Version 2.0
- MIT license
