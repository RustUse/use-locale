# Release Policy

RustUse/use-locale is not published yet. The root workspace metadata keeps `publish = false` as the default, while the crate manifests under `crates/` opt in with `publish = true`.

For the exact same-repository reset and first publish sequence, use `docs/history-reset-and-republish.md`.

Because `use-locale-tag`, `use-locale-match`, and `use-locale` depend on sibling crates, some dry-run paths are intentionally staged around crates.io index propagation.

## First Publish Wave

Publish in this order:

1. Independent focused crates: `use-language`, `use-script`, `use-region`, `use-currency-code`, `use-time-zone-id`.
2. Dependent focused crate: `use-locale-tag` after `use-language`, `use-script`, and `use-region` resolve from crates.io.
3. Dependent focused crate: `use-locale-match` after `use-locale-tag` resolves from crates.io.
4. Facade crate: `use-locale` after all child crates resolve from crates.io.

## Publish Surface

Before the first publish wave, confirm that the release surface:

- keeps the workspace-level default at `publish = false`
- keeps every crate under `crates/` at `publish = true`
- leaves any future non-release crates opted out until they are intentionally reviewed

## Versioning

- The workspace currently uses lockstep `0.x.y` versioning.
- Before `1.0`, breaking changes should bump the minor version.
- Before `1.0`, additive compatible changes should bump the patch version.
- The facade crate should only advertise actively supported crates.

## Automated Release Validation

The repository includes release-validation paths modeled after `use-math`:

- `.github/workflows/publish-readiness.yml` runs on pull requests, pushes to `main`, and manual dispatch.
- `make release-readiness` runs local validation, examples, no-default-features coverage, and independent focused-crate publish dry-runs.
- `make dependent-post-publish-validation` dry-runs `use-locale-tag` and `use-locale-match` after their sibling dependencies are live on crates.io.
- `.github/workflows/facade-publish-readiness.yml` is a manual post-publication check that dry-runs `use-locale` after the child crates are live on crates.io.
- The facade workflow fails fast unless every child crate already resolves from crates.io.

## Branch Protection Gate

Before the first public release, the canonical GitHub repository should require `Publish Readiness / Release Readiness Checks` on `main`.

This repository can document the required check name, but it cannot enforce branch protection from version-controlled files alone.

## Version and Changelog Automation

The repository includes `release-plz` configuration in `release-plz.toml` and maintainer workflows under `.github/workflows/release-plz-*.yml`.

- `Release PR Automation` opens or updates a release PR with lockstep version changes for every publishable crate in the workspace.
- The workspace is configured with one `version_group` so all published crates keep the same version.
- The root `CHANGELOG.md` remains the shared changelog and is updated through the `use-locale` package entry, including focused-crate commits.
- `Release Publish Automation` can publish automatically on pushes to `main` after the initial manual wave is complete, crates.io trusted publishing is configured for every published crate, and the `CRATES_IO_AUTOPUBLISH_ENABLED` repository variable is set to `true`.

One-time post-initial-release setup:

- Configure crates.io Trusted Publishing for each published crate with repository owner `RustUse`, repository name `use-locale`, and workflow filename `release-plz-release.yml`.
- Leave the crates.io environment field empty unless you intentionally add a matching GitHub Actions environment to the workflow later.
- Set the repository variable `CRATES_IO_AUTOPUBLISH_ENABLED` to `true` only after the initial manual crates.io wave is complete.
- Do not set `CARGO_REGISTRY_TOKEN` for this workflow when using trusted publishing.

## Publish Readiness Checklist

1. Confirm `cargo fmt` is clean.
2. Confirm `cargo check --workspace --all-features` passes.
3. Confirm `cargo check --workspace --all-features --examples` passes.
4. Confirm `cargo test --workspace --all-features` passes.
5. Confirm `cargo test --workspace --no-default-features` passes.
6. Confirm `cargo clippy --workspace --all-targets --all-features -- -D warnings` passes.
7. Confirm `cargo deny check` and `cargo audit` pass.
8. Review README examples, crate metadata, repository health files, `Cargo.lock`, and changelog entries.
9. Confirm the organization-level security, support, and code of conduct defaults, plus the local governance docs, reflect the current public launch posture.
10. Confirm the crates under `crates/` are the only intentionally publishable crates.
11. Publish all focused crates in dependency order, then wait for crates.io index resolution.
12. Confirm `cargo publish --dry-run --allow-dirty -p use-locale` passes once the matching child-crate versions are available on crates.io.
