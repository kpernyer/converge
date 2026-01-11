# Publishing converge-core

This document describes how to publish `converge-core` as a library-only crate.

## Library-Only Configuration

The `Cargo.toml` is configured to publish only the library (no binary target). The crate is:
- **Library-only** — No `[[bin]]` targets
- **Source private** — Source code remains in private repository
- **Compiled distribution** — Only compiled artifacts are published

## Publishing to crates.io

### Prerequisites

1. Create a crates.io account
2. Get an API token from https://crates.io/me
3. Run `cargo login <token>`

### Publishing Steps

```bash
cd converge-core

# Verify the package
cargo package --list

# Dry run (check what will be published)
cargo publish --dry-run

# Publish
cargo publish
```

### What Gets Published

- All source files in `src/`
- `Cargo.toml` (with metadata)
- `README.md` (shown on crates.io)
- `LICENSE` file (if present)

### What Does NOT Get Published

- `tests/` directory (dev-only)
- Internal documentation in `docs/`
- Implementation specs

## Publishing to Private Registry

If using a private registry (e.g., GitHub Packages, GitLab Package Registry):

1. Configure registry in `Cargo.toml`:
```toml
[package]
publish = ["your-registry-name"]
```

2. Configure registry in `~/.cargo/config.toml`:
```toml
[registries.your-registry-name]
index = "https://your-registry-url"
```

3. Publish:
```bash
cargo publish --registry your-registry-name
```

## Documentation

Public documentation is available at:
- `docs/public/` — High-level API docs
- `README.md` — Quick start and overview
- `docs.rs/converge-core` — Auto-generated API docs (after publishing)

Internal documentation (implementation details) remains private in:
- `docs/architecture/` — Internal architecture
- `docs/development/` — Development decisions

## Version Management

- Use semantic versioning
- Update version in workspace `Cargo.toml`
- Tag releases in git
- Update `CHANGELOG.md` (if maintained)

## Post-Publishing

After publishing:
1. Documentation will be available at `https://docs.rs/converge-core`
2. Users can add to `Cargo.toml`: `converge-core = "0.1.0"`
3. Source code remains in your private repository

