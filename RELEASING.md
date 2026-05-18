# Releasing

This repository uses a staged first-wave release flow because the facade crate and two focused crates depend on sibling crates.

## Current release state

`use-locale` publishes independent focused crates first, then dependent focused crates, then the `use-locale` facade crate.

## Canonical release guide

Use `RELEASE.md` as the authoritative release policy for:

- first-wave publish scope
- focused-crate publish ordering
- publish readiness checks
- trusted publishing setup after the first public wave
- maintainer release checklist

If the repository history is being reset before the first public push, follow `docs/history-reset-and-republish.md`.

## Current automation

The repository includes the specialized workflows that match this release shape:

- `publish-readiness.yml`
- `facade-publish-readiness.yml`
- `release-plz-pr.yml`
- `release-plz-release.yml`

The pre-publication readiness path dry-runs independent focused crates. The dependent focused crates and facade have separate post-propagation validation targets because they require sibling crates to resolve from crates.io.

This file exists to keep the top-level release entrypoint consistent with the other RustUse repositories while preserving the more detailed custom guidance in `RELEASE.md`.
