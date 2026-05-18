# History Reset And Republishing Runbook

This runbook assumes the current private GitHub repository stays in place and you replace its commit history with one clean public root commit before the first public launch.

## Preconditions

- The working tree already contains exactly the files you want to publish.
- The repo-owned DX and release files are present, including `.cargo/config.toml`, `.github/workflows/`, `release-plz.toml`, `RELEASE.md`, and this runbook.
- You have admin access to `RustUse/use-locale`.
- You have a crates.io account ready for the first manual publish wave.

## Quick command sequence

Replace `YYYYMMDD` with the actual launch date before running the backup commands.

```bash
git switch main
git status --short
cargo xcheck
cargo xlint
cargo xtest
cargo xtest-minimal
cargo xexamples
bash scripts/bootstrap-dev-tools.sh --dry-run
pwsh -File scripts/bootstrap-dev-tools.ps1 -DryRun
git branch backup/private-main-YYYYMMDD
git tag private-pre-public-YYYYMMDD
git bundle create ../use-locale-private-history-YYYYMMDD.bundle --all
git switch --orphan public-main
git add -A
git status --short
git commit -m "chore: initialize public use-locale workspace"
git branch -M main
git push --force-with-lease origin main
```

After the clean public push, verify GitHub settings manually, then run the release-readiness and first-publish sequence.

## First crates.io publish sequence

Publish independent focused crates first:

```bash
for crate in use-language use-script use-region use-currency-code use-time-zone-id; do
	cargo publish -p "$crate"
done
```

Wait for crates.io index propagation, then publish dependent focused crates:

```bash
cargo publish -p use-locale-tag
cargo publish -p use-locale-match
```

Wait for crates.io index propagation again, then verify and publish the facade:

```bash
cargo publish --dry-run -p use-locale
cargo publish -p use-locale
```

## Finish the first public release state

After the focused crates and facade are live:

- Confirm docs.rs builds and crate metadata render correctly.
- Push the release tag if you want the repository history to carry the first public version explicitly.
- Keep using `release-plz` only for subsequent releases, after the initial manual publish wave is complete.
