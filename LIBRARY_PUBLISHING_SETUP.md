# Library Publishing Setup — Summary

This document summarizes the changes made to configure `converge-core` as a library-only crate with separated public/internal documentation.

## What Was Done

### 1. Library-Only Configuration

**File:** `converge-core/Cargo.toml`
- Added metadata for publishing (repository, documentation links)
- Configured as library-only (no binary targets)
- Ready for publishing to crates.io or private registry

### 2. Public Documentation Structure

**Directory:** `docs/public/`
- `README.md` — Overview of public documentation
- `API_OVERVIEW.md` — High-level API description
- `CORE_CONCEPTS.md` — Essential concepts without implementation details
- `USAGE_GUIDE.md` — How to use the library

These documents describe:
- ✅ What the library does
- ✅ How to use it
- ✅ Public API surface
- ✅ Behavioral guarantees

These documents do NOT describe:
- ❌ Implementation details
- ❌ Internal architecture decisions
- ❌ Execution model specifics
- ❌ Design rationale

### 3. Internal Documentation Organization

**Directory:** `docs/internal/`
- `README.md` — Explains what's internal and why

**Existing Internal Docs:**
- `docs/02-architecture/` — Marked as internal (implementation details)
- `docs/05-development/` — Marked as internal (development decisions)

### 4. Public-Facing README

**File:** `converge-core/README.md`
- Quick start guide
- Core concepts overview
- Links to public documentation
- No implementation details

### 5. Publishing Guide

**File:** `converge-core/PUBLISHING.md`
- Instructions for publishing to crates.io
- Instructions for private registries
- What gets published vs. what stays private

### 6. Updated Contributor Guide

**File:** `CONTRIBUTOR_GUIDE.md`
- Clarified that `converge-core` is private
- Explained that other modules are open for contribution
- Updated references to use public documentation

### 7. Updated Documentation Index

**File:** `docs/README.md`
- Added `public/` section
- Marked `02-architecture/` as internal
- Clarified separation between public and internal docs

## How It Works

### For Library Users

1. **Install:** `cargo add converge-core`
2. **Read:** `docs/public/` for usage
3. **Use:** Public API as documented
4. **Source:** Not available (compiled library only)

### For Contributors

1. **Core:** Not open for contribution (private)
2. **Other Modules:** Open for contribution
   - `converge-domain`
   - `converge-provider`
   - `converge-runtime`
   - `converge-tool`
3. **Documentation:** Use `docs/public/` for API understanding

### For Core Maintainers

1. **Internal Docs:** `docs/02-architecture/`, `docs/05-development/`
2. **Implementation:** Full source access
3. **Publishing:** See `converge-core/PUBLISHING.md`

## Publishing Strategy

### Option 1: Public (crates.io)
```bash
cd converge-core
cargo publish
```

### Option 2: Private Registry
Configure registry in `Cargo.toml` and `~/.cargo/config.toml`, then:
```bash
cargo publish --registry your-registry
```

### What Gets Published
- ✅ All `src/` files (compiled)
- ✅ `Cargo.toml`
- ✅ `README.md`
- ✅ `LICENSE`

### What Stays Private
- ❌ `tests/` directory
- ❌ Internal documentation
- ❌ Implementation specs
- ❌ Source repository (if kept private)

## Next Steps

1. **Review** the public documentation in `docs/public/`
2. **Test** publishing with `cargo publish --dry-run`
3. **Update** repository URL in `Cargo.toml` if needed
4. **Publish** when ready

## Notes

- The library is configured as library-only (no binary)
- Public docs focus on "what" and "how", not "why" (implementation details)
- Internal specs remain in `docs/02-architecture/` and `docs/05-development/`
- Contributors work on other modules, not core
- Core maintainers have full access to internal documentation

