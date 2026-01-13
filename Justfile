# Converge Platform development commands
# Install just: cargo install just

# Default: show available commands
default:
    @just --list

# ============================================
# Development Checks
# ============================================

# Run all pre-push checks (fmt, clippy, test, doc)
check: fmt-check clippy test doc
    @echo "✓ All checks passed!"

# Check formatting (CI equivalent)
fmt-check:
    cargo fmt --check

# Apply formatting fixes
fmt:
    cargo fmt

# Run clippy lints
clippy:
    cargo clippy --all-targets --all-features -- -D warnings

# Run tests
test:
    cargo test

# Build documentation
doc:
    cargo doc --no-deps

# Run verbose axiom tests
axioms:
    cargo test --package converge-core -- axioms --nocapture

# Quick check before push (fmt + clippy + doc)
pre-push: fmt-check clippy doc
    @echo "✓ Ready to push!"

# ============================================
# Build
# ============================================

# Build all packages
build:
    cargo build

# Build release
build-release:
    cargo build --release

# Clean build artifacts
clean:
    cargo clean
