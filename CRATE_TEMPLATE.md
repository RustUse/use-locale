# RustUse Crate Template

Use this checklist when adding a new focused crate or expanding the `use-locale` facade.

## Target Layout

```text
crates/use-example/
  Cargo.toml
  README.md
  src/
    lib.rs
```

`examples/`, `tests/`, and helper modules are optional, but new crates should stay small and include meaningful unit tests.

## Cargo.toml Pattern

```toml
[package]
name = "use-example"
description = "Small example identifier primitives for RustUse"
publish = true
authors.workspace = true
version.workspace = true
edition.workspace = true
homepage.workspace = true
license.workspace = true
repository.workspace = true
rust-version.workspace = true
readme = "README.md"
documentation = "https://docs.rs/use-example"
keywords = ["locale", "identifier", "rustuse"]
categories = ["internationalization", "parser-implementations"]

[lints]
workspace = true
```

Checklist:

- Keep package metadata inherited from the workspace wherever possible.
- Default new release-surface crates to `publish = true` only after naming, docs, and tests are reviewed.
- Prefer zero dependencies for v0.1 unless a dependency is clearly justified.
- Use primitive-first descriptions and avoid framework language.

## src/lib.rs Pattern

```rust
#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

pub struct ExampleCode {
    value: String,
}
```

Checklist:

- Re-export the focused public API at the crate root.
- Use `parse_*`, `is_*`, and `normalize_*` helpers for identifier primitives.
- Prefer `Option` for simple syntax validation failure.
- Implement `AsRef<str>` instead of adding an inherent `as_ref` method.

## README Structure

Keep crate README files short and consistent.

Required sections:

- title and one-line summary
- `Example`
- `Scope`
- `Non-goals`
- `License`

Guidelines:

- Keep examples runnable as doctests.
- Backtick product names like `RustUse` when the README is included in rustdoc.
- Make `Non-goals` explicit about framework behavior and external data that is intentionally out of scope.

## Facade Checklist

If a new focused crate should be available through `use-locale`, also update:

- `crates/use-locale/Cargo.toml` dependencies
- `crates/use-locale/src/lib.rs` root re-exports
- `crates/use-locale/src/prelude.rs`
- root `Cargo.toml` workspace members and dependencies
- root `README.md`
- `release-plz.toml`, `Makefile`, release docs, and publish-readiness workflows

## Validation Checklist

Run the full workspace suite before opening a pull request:

```sh
cargo fmt --all --check
cargo check --workspace --all-features
cargo check --workspace --all-features --examples
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo deny check
cargo audit
cargo test --workspace --all-features
cargo test --workspace --no-default-features
cargo doc --workspace --all-features --no-deps
```
