# use-time-zone-id

Small IANA time zone identifier syntax primitives for `RustUse`.

## Example

```rust
use use_time_zone_id::{is_time_zone_id, parse_time_zone_id};

let zone = parse_time_zone_id("America/New_York").unwrap();

assert_eq!(zone.area(), "America");
assert_eq!(zone.location(), Some("New_York"));
assert!(is_time_zone_id("UTC"));
```

## Scope

- Validate IANA-shaped time zone identifier syntax.
- Support identifiers like `UTC`, `America/New_York`, and `Europe/London`.
- Split valid identifiers into area and location components.

## Non-goals

- Time arithmetic.
- Daylight-saving calculations.
- Date/time conversion.
- Bundled tzdb data.
- Replacement for `time`, `chrono`, or `tz-rs`.

## License

Licensed under either of the following, at your option:

- Apache License, Version 2.0
- MIT license
