# Publishing Guide

## Repository Structure

```
converge.hey.sh/              (PUBLIC - open source ecosystem)
├── .gitmodules               (declares submodule relationship)
├── converge-core/            (PRIVATE - git submodule)
│   └── → github.com/kpernyer/converge-core (private repo)
├── converge-provider/        (public)
├── converge-domain/          (public)
├── converge-tool/            (public)
└── converge-runtime/         (public)
```

**Strategy:** `converge-core` source is private. Published to crates.io as compiled library.
External users get the `.rlib` binary, not the source code.

---

## Publishing Order

Crates must be published in dependency order:

```
1. converge-core      (no internal deps)
2. converge-provider  (depends on core)
3. converge-domain    (depends on core, provider)
4. converge-tool      (depends on core, provider)
5. converge-runtime   (depends on core)
```

---

## Pre-Publish Checklist

### For converge-core (from private repo)

```bash
cd converge-core

# 1. Verify clean state
git status  # Should be clean
cargo test  # All tests pass
cargo clippy -- -D warnings  # No warnings

# 2. Verify Cargo.toml metadata
#    - name, version, description: set
#    - license: MIT (or your choice)
#    - documentation: https://docs.rs/converge-core
#    - keywords, categories: set
#    - repository: leave commented out (private)

# 3. Dry run
cargo publish --dry-run

# 4. Publish
cargo publish
```

### For public crates

```bash
cd converge-provider  # (or domain, tool, runtime)

# 1. Wait for converge-core to appear on crates.io (~5 min)

# 2. Verify dependency resolves
cargo update
cargo check

# 3. Dry run and publish
cargo publish --dry-run
cargo publish
```

---

## Version Bumping

When releasing a new version:

1. Update `[workspace.package] version` in root `Cargo.toml`
2. Update `converge-core = { path = "...", version = "X.Y" }` in workspace deps
3. Commit: `git commit -am "chore: bump version to X.Y.Z"`
4. Tag: `git tag vX.Y.Z`
5. Publish in order (see above)

---

## For Contributors

Contributors clone the public repo but **cannot access converge-core source**:

```bash
# Clone public repo
git clone https://github.com/your-org/converge.git
cd converge

# Submodule init will fail (private repo)
git submodule update --init  # ERROR: Permission denied

# Instead, use published crate
# The workspace Cargo.toml specifies both path and version:
#   converge-core = { path = "converge-core", version = "0.1" }
#
# When path doesn't exist, Cargo falls back to crates.io version
```

**Contributor workflow:**

```bash
# Remove the empty submodule directory
rm -rf converge-core

# Edit workspace Cargo.toml - remove converge-core from members:
# members = ["converge-provider", "converge-domain", ...]

# Now cargo will fetch from crates.io
cargo build
```

---

## Crates.io API Token

```bash
# Login once (stores token in ~/.cargo/credentials.toml)
cargo login

# Or set via environment
export CARGO_REGISTRY_TOKEN=your-token
```

---

## Verification After Publishing

```bash
# In a fresh directory, verify the published crate works
cargo new test-converge && cd test-converge
echo 'converge-core = "0.1"' >> Cargo.toml
cargo build
```

---

## Troubleshooting

**"crate not found" after publish:**
- crates.io index updates every ~5 minutes
- Run `cargo update` to refresh

**"version already exists":**
- Bump version in Cargo.toml
- Crates.io versions are immutable

**Submodule issues for contributors:**
- See "For Contributors" section above
- They use crates.io version, not source
