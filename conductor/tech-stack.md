# Technology Stack: Converge

## Core Philosophy
**Converge Core defines semantics and guarantees; runtimes and infrastructure are optional layers built around it.**

## Core Technology
- **Programming Language:** Rust (1.85+, 2024 Edition)
- **Error Handling:** `thiserror`
- **Architecture:** Monorepo with a dedicated `converge-core` library serving as the semantic authority.

## Infrastructure & Runtimes (Optional Layers)
- The core engine is agnostic to specific infrastructure or deployment targets.
- Runtimes are treated as adapters around the core guarantees.
