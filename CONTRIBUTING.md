# Contributor Guide — Converge
## How to Contribute Without Breaking the Semantics

Welcome, and thank you for your interest in contributing to Converge.

Converge is an open-source project, but it is **not** open-ended.
It has a strong semantic core that must remain intact.

This guide explains how to contribute *productively*.

---

## 0. Project Structure

### converge-core (Private)
The `converge-core` library is **maintained privately**. It is:
- Published as a compiled library crate
- Source code is not publicly available
- Internal architecture and implementation specs are private
- Public API documentation is available in `docs/public/`

**Contributions to `converge-core` are not accepted.** If you need changes to the core API, please open an issue for discussion.

### Other Modules (Open for Contribution)
The following modules are open for contributions:
- **converge-domain** — Domain-specific agents and capabilities
- **converge-provider** — LLM provider integrations
- **converge-runtime** — Runtime services and APIs
- **converge-tool** — Tooling and utilities

These modules use `converge-core` as a dependency and build on top of its public API.

### Setup for Contributors

The `converge-core` directory is a git submodule pointing to a private repository.
When you clone, the submodule will fail to initialize (expected):

```bash
git clone https://github.com/kpernyer/converge.git
cd converge
git submodule update --init  # Will fail - private repo

# Remove the empty submodule directory
rm -rf converge-core

# Remove converge-core from workspace members in Cargo.toml
# (or cargo will complain about missing path)

# Build - Cargo fetches converge-core from crates.io
cargo build
```

The workspace `Cargo.toml` specifies both `path` and `version` for internal crates.
When the path doesn't exist, Cargo uses the crates.io version automatically.

---

## 1. Before You Contribute

You should understand these documents first:

- `docs/public/` — Public API documentation for using `converge-core`
- `docs/governance/GOVERNANCE.md` — Core principles
- `docs/governance/TERMINOLOGY.md` — Key terms
- `docs/governance/DESIGN_TENETS.md` — Design principles

**Note:** Internal architecture documents (`docs/architecture/`) are for core maintainers only and expose implementation details. Use the public documentation instead.

If a proposed change conflicts with these principles, it will not be merged.

---

## 2. What Contributions Are Welcome

### Highly Encouraged
- Deterministic agents
- Domain capability packages
- Gherkin invariants
- Validators and promotion logic
- Integration examples
- Documentation improvements
- Tests (unit, integration, invariants)

### Conditionally Accepted
- Performance optimizations (must preserve determinism)
- Persistence tooling
- Observability tooling (derived from decisions, not execution)

### Generally Rejected
- Message buses
- Workflow engines
- Actor frameworks
- Background execution
- Eventual consistency mechanisms

---

## 3. Core Contribution Rules

### Rule 1: Do Not Introduce Hidden Control Flow
All execution must be:
- explicit
- engine-driven
- inspectable

### Rule 2: Do Not Dilute Authority
- Agents suggest
- The engine decides
- Humans approve

### Rule 3: Preserve Determinism
If a change makes outcomes non-reproducible, it is unacceptable.

### Rule 4: Prefer Types Over Conventions
If correctness relies on comments or discipline, redesign the API.

---

## 4. Code Style & Testing

- Follow rust.md style guide
- Add tests for every semantic change
- Prefer integration tests that demonstrate convergence

---

## 5. Review Philosophy

PRs are evaluated on:
- semantic clarity
- correctness by construction
- alignment with convergence principles

Performance and features come second.

---

## 6. Final Reminder

> Converge is a semantic engine for alignment, not a playground for autonomy.

Thank you for respecting that.
