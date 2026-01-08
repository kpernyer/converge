# Repository Guidelines

## Project Structure & Module Organization

The workspace is organized into multiple crates:

- **`converge-core/`** — Runtime machinery + abstractions (engine, context, agents, effects, invariants)
- **`converge-provider/`** — LLM provider implementations (e.g., `AnthropicProvider`)
- **`converge-domain/`** — Domain-specific agents (e.g., `growth_strategy`)
- **`converge-tool/`** — Development tools (e.g., Gherkin validator)
- **`converge-runtime/`** — HTTP/gRPC server and API handlers

The root directory contains documentation: architecture, lifecycle, Gherkin and other specs live as `.md` references.

Inside each crate's `src/`, modules are single-file and align with the conceptual model. For example, in `converge-core/src/`: `context.rs` defines shared state types, `effect.rs` encodes agent effects, `error.rs` centralizes diagnostics, and `lib.rs` wires the public API.

Keep new runtime modules colocated under each crate's `src/` or add integration tests in `<crate>/tests/` to keep crate boundaries clean.

## Build, Test, and Development Commands

Run commands from the workspace root:
- `cargo fmt --all` – applies the canonical `rustfmt` layout, matching CI expectations.
- `cargo clippy --all-targets --all-features -D warnings` – enforces pedantic lint configuration across all crates.
- `cargo test --all-targets` – executes unit + integration suites across all crates; add `-- --include-ignored` for long convergence checks.
- `cargo doc --no-deps` – validates public docs without pulling in external crates.

To work on a specific crate, `cd` into its directory and run the same commands.

## Coding Style & Naming Conventions
Adopt idiomatic Rust: four-space indentation, snake_case for modules/functions, CamelCase for types/traits, and SCREAMING_SNAKE for constants. Keep files focused (one primary type or trait per file) and favor explicit structs/enums over loosely typed maps to preserve schema meaning. Unsafe code is forbidden, so prefer higher-order combinators or state machines over raw pointers. After large refactors, run `cargo fmt` and `cargo clippy` before submitting.

## Testing Guidelines
Unit tests belong beside the code in `#[cfg(test)]` modules; use descriptive names like `handles_monotonic_fact_merging`. Cross-module behavior or convergence proofs should live under `<crate>/tests/` as integration tests named `<feature>_spec.rs`. Target high coverage of context transitions, effect emission, and error propagation, and lean on doc tests in `lib.rs` for public API examples. Gate regressions locally with `cargo test --all-targets` from the workspace root and capture any needed fixtures under `<crate>/tests/data/`.

## Commit & Pull Request Guidelines
Follow the existing short, imperative commit style: `component: summary`, e.g., `core: tighten context invariants`. Group related work into cohesive commits; avoid mixing formatting with behavior changes. Pull requests should include: a concise problem statement, a numbered change summary, explicit testing evidence (commands + results), linked issues, and screenshots or logs when touching human-in-the-loop flows. Highlight any new invariants or schema changes so reviewers can cross-check the architecture docs.
