# use-timezone

Small time zone and fixed offset primitives for `RustUse`.

## Example

```rust
use use_timezone::{TimeZone, TimeZoneOffset, parse_time_zone};

let zone = parse_time_zone("America/New_York").unwrap();
let offset = TimeZoneOffset::new("UTC+05:30").unwrap();

assert!(matches!(zone, TimeZone::Iana(_)));
assert_eq!(offset.total_minutes(), 330);
assert_eq!(parse_time_zone("-0800").unwrap().to_string(), "UTC-08:00");
```

## Scope

- Represent an IANA-shaped time zone identifier or a fixed UTC/GMT offset.
- Reuse `use-time-zone-id` for IANA-shaped identifier syntax.
- Parse fixed offsets such as `Z`, `UTC`, `+05:30`, `-0800`, and `GMT-08:00`.
- Keep fixed offsets within the civil `-14:00..=+14:00` range.

## Non-goals

- Time arithmetic.
- Daylight-saving calculations.
- Date/time conversion.
- Local system time zone discovery.
- Bundled tzdb data.
- Replacement for `time`, `chrono`, or `tz-rs`.

## License

Licensed under either of the following, at your option:

- Apache License, Version 2.0
- MIT license
