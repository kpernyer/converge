# Converge
## A Semantic Engine for Intent-Driven Business Systems

Converge is an open-source **semantic convergence engine**.

It enables systems to move from an initial *intent*
to a stable, explainable outcome by accumulating facts,
enforcing invariants, and converging on truth.

Converge is not an agent framework.
It is not a workflow engine.
It is not event-driven.

It is a system for **alignment**.

---

## Why Converge Exists

Modern business software fails because:
- configuration replaces intent
- automation hides authority
- workflows fossilize assumptions

Converge replaces:
- configuration → intent
- workflows → convergence
- automation → explainable decisions

---

## Core Concepts

- **Root Intent:** the scope and goal of execution
- **Shared Context:** append-only facts visible to all agents
- **Agents:** deterministic or LLM-backed contributors
- **Convergence:** execution until a fixed point is reached
- **Invariants:** business laws expressed in Gherkin
- **Human-in-the-Loop:** explicit approval and gating

---

## What Makes Converge Different

- Single semantic authority per intent
- No message queues
- No background execution
- No eventual consistency
- Full provenance and auditability

Every outcome can be explained.

---

## Who Is This For?

Converge is designed to power:
- intent-driven CRM alternatives
- marketing and growth systems
- SMB business platforms
- explainable AI-assisted decision systems

---

## Project Status

- Core engine: Stable
- Deterministic agents: Implemented
- Invariant enforcement: Implemented
- HITL support: Implemented
- LLM integration: Governed and optional

---

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
converge-core = "0.4"
```

Or use cargo:

```bash
cargo add converge-core
```

### Additional Crates

```bash
cargo add converge-provider  # LLM providers (Anthropic, OpenAI, etc.)
cargo add converge-domain    # Domain-specific agents
cargo add converge-tool      # Development tools (Gherkin validation)
```

---

## Quick Start

```rust
use converge_core::{Context, ContextKey, Fact};

fn main() {
    let mut ctx = Context::new();

    // Create a fact using the new() constructor
    let fact = Fact::new(ContextKey::Seeds, "greeting", "Hello from Converge!");

    ctx.add_fact(fact).expect("should add");

    let facts = ctx.get(ContextKey::Seeds);
    for f in facts {
        println!("Content: {}", f.content);
    }

    println!("Context version: {}", ctx.version());
}
```

---

## Documentation

- [API Docs (docs.rs)](https://docs.rs/converge-core)
- `AGENTS.md` — Comprehensive guide for AI assistants
- `docs/governance/DESIGN_TENETS.md` — Design principles
- `docs/architecture/ARCHITECTURE.md` — System architecture
- `docs/deployment/TECHNOLOGY_STACK.md` — Technology choices

---

## Contributing

We welcome contributors who value:
- correctness
- clarity
- restraint

See `CONTRIBUTING.md` and `CODE_OF_CONDUCT.md`.

---

## License

- **converge-core**: Proprietary (Aprio One AB)
- **converge-provider, converge-domain, converge-tool, converge-runtime**: MIT
