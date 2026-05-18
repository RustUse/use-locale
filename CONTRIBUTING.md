# Contributing

RustUse/use-locale is intentionally small. Contributions should favor correctness, clear naming, and minimal locale identifier primitives over broad framework behavior.

For routing and organization-wide policy, use the RustUse defaults for
[support](https://github.com/RustUse/.github/blob/main/SUPPORT.md),
[security](https://github.com/RustUse/.github/blob/main/SECURITY.md), and the
[code of conduct](https://github.com/RustUse/.github/blob/main/CODE_OF_CONDUCT.md),
alongside `GOVERNANCE.md` and `MAINTAINERS.md`.

## Development Flow

1. Make the smallest useful change that improves the current crates.
2. Add or update unit tests for every public function you introduce or change.
3. Keep dependencies lightweight unless there is a strong justification.
4. Preserve the primitive-first API direction and avoid framework features.

## Local Validation

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

## Tooling Shortcuts

The repository ships cross-platform Cargo aliases in `.cargo/config.toml`:

```sh
cargo xcheck
cargo xlint
cargo xtest
cargo xtest-minimal
cargo xexamples
cargo xdoc
```

VS Code users also get checked-in task definitions in `.vscode/tasks.json` and extension recommendations in `.vscode/extensions.json`.

## Optional Dev Tool Bootstrap

Optional Cargo tooling used by local release and advisory flows can be installed with either bootstrap script:

```sh
bash scripts/bootstrap-dev-tools.sh
pwsh -File scripts/bootstrap-dev-tools.ps1
```

These scripts install `cargo-deny`, `cargo-audit`, `cargo-cyclonedx`, `release-plz`, and `cargo-machete`.

## Documentation

- Update the root README when the crate list or facade story changes.
- Keep crate README examples small and runnable as doctests.
- Keep docs explicit about non-goals: no translation framework, CLDR runtime, ICU replacement, date/time library, money library, geocoder, or full web negotiation library.
- Follow `CRATE_TEMPLATE.md` when introducing a new focused crate or expanding the facade surface.

## Release Policy

- The workspace-level default keeps `publish = false`, while publishable crate manifests opt in with `publish = true`.
- Versions move in lockstep at `0.x.y` for now.
- `Cargo.lock` is committed intentionally for reproducible CI, security checks, and release dry runs in this library workspace.
- Before any newly reviewed crate is made publishable, it should have stable naming, README coverage, unit tests, and changelog notes.

`release-plz` drives release PRs and changelog generation for publishable crates. Prefer commit subjects that match `type: summary` or `type(scope)!: summary`, using prefixes such as `feat:`, `fix:`, `docs:`, `refactor:`, `build:`, `ci:`, `test:`, and `chore:`.
