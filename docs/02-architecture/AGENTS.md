# Repository Guidelines

## Project Structure & Module Organization
The root directory is documentation heavy: architecture, lifecycle, Gherkin and other specs live as `.md` references. Rust implementation sits entirely in `converge-core/`, a `cargo` crate pinned to edition 2024 and rustc 1.85. Inside `converge-core/src/`, modules are single-file and align with the conceptual model: `context.rs` defines shared state types, `effect.rs` encodes agent effects, `error.rs` centralizes diagnostics, and `lib.rs` wires the public API. Keep new runtime modules colocated under `src/` or add integration tests in `converge-core/tests/` to keep the crate boundary clean.

## Build, Test, and Development Commands
Run commands from `converge-core/`:
- `cargo fmt --all` – applies the canonical `rustfmt` layout, matching CI expectations.
- `cargo clippy --all-targets --all-features -D warnings` – enforces pedantic lint configuration declared in `Cargo.toml`.
- `cargo test --all-targets` – executes unit + integration suites; add `-- --include-ignored` for long convergence checks.
- `cargo doc --no-deps` – validates public docs without pulling in external crates.

## Coding Style & Naming Conventions
Adopt idiomatic Rust: four-space indentation, snake_case for modules/functions, CamelCase for types/traits, and SCREAMING_SNAKE for constants. Keep files focused (one primary type or trait per file) and favor explicit structs/enums over loosely typed maps to preserve schema meaning. Unsafe code is forbidden, so prefer higher-order combinators or state machines over raw pointers. After large refactors, run `cargo fmt` and `cargo clippy` before submitting.

## Testing Guidelines
Unit tests belong beside the code in `#[cfg(test)]` modules; use descriptive names like `handles_monotonic_fact_merging`. Cross-module behavior or convergence proofs should live under `converge-core/tests/` as integration tests named `<feature>_spec.rs`. Target high coverage of context transitions, effect emission, and error propagation, and lean on doc tests in `lib.rs` for public API examples. Gate regressions locally with `cargo test --all-targets` and capture any needed fixtures under `tests/data/`.

## Commit & Pull Request Guidelines
Follow the existing short, imperative commit style: `component: summary`, e.g., `core: tighten context invariants`. Group related work into cohesive commits; avoid mixing formatting with behavior changes. Pull requests should include: a concise problem statement, a numbered change summary, explicit testing evidence (commands + results), linked issues, and screenshots or logs when touching human-in-the-loop flows. Highlight any new invariants or schema changes so reviewers can cross-check the architecture docs.
