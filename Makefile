.PHONY: help fmt check lint test test-minimal build doc examples audit deny sbom publish-dry-run-independent publish-dry-run-dependent publish-dry-run-focused publish-dry-run-facade release-readiness dependent-post-publish-validation facade-post-publish-validation verify

INDEPENDENT_CRATES := use-language use-script use-region use-currency-code use-time-zone-id
DEPENDENT_CRATES := use-locale-tag use-locale-match
FOCUSED_CRATES := $(INDEPENDENT_CRATES) $(DEPENDENT_CRATES)

help:
	@printf "%s\n" \
		"help                           Show available repository tasks" \
		"fmt                            Check formatting with rustfmt" \
		"check                          Run cargo check for the workspace" \
		"lint                           Run clippy with warnings denied" \
		"test                           Run workspace tests with all features" \
		"test-minimal                   Run workspace tests with no default features" \
		"build                          Build the workspace with all features" \
		"doc                            Build workspace docs without dependencies" \
		"examples                       Check all examples" \
		"audit                          Run cargo-audit" \
		"deny                           Run cargo-deny" \
		"sbom                           Generate a CycloneDX SBOM for use-locale" \
		"publish-dry-run-independent    List package contents and dry-run publish independent crates" \
		"publish-dry-run-dependent      Dry-run dependent focused crates after dependency propagation" \
		"publish-dry-run-focused        Pre-publication focused-crate readiness path" \
		"publish-dry-run-facade         Dry-run publish use-locale after crates.io propagation" \
		"release-readiness              Run the pre-release focused-crate validation path" \
		"dependent-post-publish-validation Dry-run dependent focused crates after independent crates are live" \
		"facade-post-publish-validation Dry-run the facade crate after focused crates are live" \
		"verify                         Run the main workspace validation path"

fmt:
	cargo fmt --all -- --check

check:
	cargo check --workspace --all-features

lint:
	cargo clippy --workspace --all-targets --all-features -- -D warnings

test:
	cargo test --workspace --all-features

test-minimal:
	cargo test --workspace --no-default-features

build:
	cargo build --workspace --all-features

doc:
	cargo doc --workspace --all-features --no-deps

examples:
	cargo check --workspace --all-features --examples

audit:
	cargo audit

deny:
	cargo deny check

sbom:
	cargo cyclonedx --manifest-path crates/use-locale/Cargo.toml --all-features --format json --spec-version 1.5 --override-filename sbom.cyclonedx

publish-dry-run-independent:
	@for crate in $(INDEPENDENT_CRATES); do \
		cargo package --allow-dirty --list -p $$crate; \
		cargo publish --dry-run --allow-dirty -p $$crate; \
	done

publish-dry-run-dependent:
	@for crate in $(DEPENDENT_CRATES); do \
		cargo package --allow-dirty --list -p $$crate; \
		cargo publish --dry-run --allow-dirty -p $$crate; \
	done

publish-dry-run-focused: publish-dry-run-independent
	@printf "%s\n" "Deferred dependent crate dry-runs until sibling dependencies are live on crates.io: $(DEPENDENT_CRATES)"

publish-dry-run-facade:
	cargo package --allow-dirty --list -p use-locale
	cargo publish --dry-run --allow-dirty -p use-locale

release-readiness: verify examples test-minimal publish-dry-run-focused

dependent-post-publish-validation: publish-dry-run-dependent

facade-post-publish-validation: publish-dry-run-facade

verify: fmt lint test build