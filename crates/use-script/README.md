# use-script

Small writing script subtag primitives for `RustUse`.

## Example

```rust
use use_script::{is_script_code, parse_script_code};

let script = parse_script_code("latn").unwrap();

assert_eq!(script.as_str(), "Latn");
assert!(is_script_code("Cyrl"));
```

## Scope

- Validate 4-letter script subtags.
- Normalize script subtags to title case.
- Provide a small `ScriptCode` newtype for validated script subtags.

## Non-goals

- Unicode text shaping.
- Font selection.
- Transliteration.
- Script detection.

## License

Licensed under either of the following, at your option:

- Apache License, Version 2.0
- MIT license
