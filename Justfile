# Task helpers for Converge development

set shell := ["/bin/bash", "-c"]

# Build all crates with release optimizations (in dependency order)
build:
	cd converge-core && cargo build --release
	cd converge-provider && cargo build --release
	cd converge-domain && cargo build --release
	cd converge-tool && cargo build --release
	cd converge-runtime && cargo build --release

# Build core library only
build-core:
	cd converge-core && cargo build --release

# Build provider library only
build-provider:
	cd converge-provider && cargo build --release

# Build domain library only
build-domain:
	cd converge-domain && cargo build --release

# Build tool library only
build-tool:
	cd converge-tool && cargo build --release

# Build runtime server only
build-runtime:
	cd converge-runtime && cargo build --release

# Install the core crate locally for smoke-testing
install:
	cd converge-core && cargo install --path . --force

# Execute unit and integration suites before merging
test:
	cd converge-core && cargo test --all-targets
	cd converge-provider && cargo test --all-targets
	cd converge-domain && cargo test --all-targets
	cd converge-tool && cargo test --all-targets
	cd converge-runtime && cargo test --all-targets

# Test core only
test-core:
	cd converge-core && cargo test --all-targets

# Test provider only
test-provider:
	cd converge-provider && cargo test --all-targets

# Test domain only
test-domain:
	cd converge-domain && cargo test --all-targets

# Test tool only
test-tool:
	cd converge-tool && cargo test --all-targets

# Test runtime only
test-runtime:
	cd converge-runtime && cargo test --all-targets

# Run the HTTP server
run-server:
	cd converge-runtime && cargo run

# Run server with tracing
run-server-trace:
	RUST_LOG=info cd converge-runtime && cargo run

# Format all code
fmt:
	cd converge-core && cargo fmt
	cd converge-provider && cargo fmt
	cd converge-domain && cargo fmt
	cd converge-tool && cargo fmt
	cd converge-runtime && cargo fmt

# Lint all code
lint:
	cd converge-core && cargo clippy --all-targets --all-features -- -D warnings
	cd converge-provider && cargo clippy --all-targets --all-features -- -D warnings
	cd converge-domain && cargo clippy --all-targets --all-features -- -D warnings
	cd converge-tool && cargo clippy --all-targets --all-features -- -D warnings
	cd converge-runtime && cargo clippy --all-targets --all-features -- -D warnings

# Placeholder until deployment automation is defined
deploy:
	@echo "Define deployment steps once the release pipeline is ready."
