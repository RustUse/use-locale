# Changelog

## Unreleased

## [0.1.0](https://github.com/RustUse/use-locale/releases/tag/use-locale-v0.1.0) - 2026-05-18

### Changed

- Add initial use-locale workspace and CI

### Added

- Added the initial `use-locale` RustUse workspace with a thin facade crate and focused locale identifier primitives.
- Added `use-language`, `use-script`, `use-region`, `use-currency-code`, and `use-time-zone-id` for small identifier validation and normalization.
- Added `use-locale-tag` for conservative BCP 47 / Unicode-style locale tag parsing and normalization.
- Added `use-locale-match` for simple locale fallback chains and best-match selection.
- Added repo-readiness tooling, release metadata, local validation shortcuts, and CI configuration following the `use-math` repository shape.
