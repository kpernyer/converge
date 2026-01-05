# Minimal task helpers for converge-core development

set shell := ["/bin/bash", "-c"]

# Build all crates with release optimizations
build:
	cd converge-core && cargo build --release

# Install the core crate locally for smoke-testing
install:
	cd converge-core && cargo install --path . --force

# Execute unit and integration suites before merging
test:
	cd converge-core && cargo test --all-targets

# Placeholder until deployment automation is defined
deploy:
	@echo "Define deployment steps once the release pipeline is ready."
