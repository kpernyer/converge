# Converge
## A Semantic Engine for Intent-Driven Business Systems

**Website:** [converge.zone](https://converge.zone) | **Docs:** [docs.rs](https://docs.rs/converge-core) | **Crates.io:** [converge-core](https://crates.io/crates/converge-core)

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

## Repository Structure

This repository uses git submodules for clean separation of concerns:

```
converge/
├── converge-core/      → github.com/kpernyer/converge-core
├── converge-provider/  → github.com/kpernyer/converge-provider
├── converge-domain/    → github.com/kpernyer/converge-domain
├── converge-runtime/   (inline - HTTP API server)
└── converge-tool/      (inline - development tools)
```

### Clone with Submodules

```bash
git clone --recurse-submodules https://github.com/kpernyer/converge.git
```

Or if already cloned:

```bash
git submodule update --init --recursive
```

---

## Crates

| Crate | Version | Description |
|-------|---------|-------------|
| [converge-core](https://crates.io/crates/converge-core) | 0.6.0 | Runtime engine, agent traits, capability abstractions |
| [converge-provider](https://crates.io/crates/converge-provider) | 0.2.2 | 14+ LLM providers, model selection, vector stores |
| [converge-domain](https://crates.io/crates/converge-domain) | 0.2.2 | 12 business use cases with deterministic + LLM variants |

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
converge-core = "0.6"
converge-provider = "0.2"  # Optional: LLM providers
converge-domain = "0.2"    # Optional: Business use cases
```

Or use cargo:

```bash
cargo add converge-core
cargo add converge-provider  # LLM providers (Anthropic, OpenAI, etc.)
cargo add converge-domain    # Domain-specific agents (12 use cases)
```

---

## Quick Start

```rust
use converge_core::{Context, ContextKey, Fact, Engine, Agent, AgentEffect};

// Define a simple agent
struct GreetingAgent;

impl Agent for GreetingAgent {
    fn name(&self) -> &str { "greeting" }
    fn dependencies(&self) -> &[ContextKey] { &[ContextKey::Seeds] }

    fn accepts(&self, ctx: &Context) -> bool {
        ctx.has(ContextKey::Seeds) && !ctx.has(ContextKey::Signals)
    }

    fn execute(&self, _ctx: &Context) -> AgentEffect {
        AgentEffect::with_fact(Fact::new(
            ContextKey::Signals,
            "greeting-response",
            "Hello from Converge!",
        ))
    }
}

fn main() {
    // Create engine and register agent
    let mut engine = Engine::new();
    engine.register(GreetingAgent);

    // Create initial context with seed
    let mut ctx = Context::new();
    ctx.add_fact(Fact::new(ContextKey::Seeds, "input", "Start")).unwrap();

    // Run to convergence
    let result = engine.run(ctx).expect("should converge");

    println!("Converged in {} cycles", result.cycles);
    for fact in result.context.get(ContextKey::Signals) {
        println!("Signal: {}", fact.content);
    }
}
```

---

## Business Use Cases (converge-domain)

The domain crate includes 12 production-ready use cases:

1. **Growth Strategy** - Market analysis and strategy development
2. **Meeting Scheduler** - Calendar coordination with constraints
3. **Resource Routing** - Task-resource matching and optimization
4. **Release Readiness** - Engineering quality gates
5. **Supply Chain** - Multi-warehouse routing and forecasting
6. **Inventory Rebalancing** - Cross-region transfers
7. **Strategic Sourcing** - Vendor assessment and negotiation
8. **Catalog Enrichment** - Product deduplication and validation
9. **CRM Account Health** - Churn risk and upsell identification
10. **Compliance Monitoring** - Regulation parsing and violation detection
11. **HR Policy Alignment** - Policy distribution with understanding signals
12. **SDR Sales** - Sales qualification funnel

Each use case includes:
- Deterministic base agents
- Optional LLM-enhanced variants
- Invariant definitions
- Comprehensive tests

---

## LLM Providers (converge-provider)

Supported providers:
- **Anthropic** (Claude)
- **OpenAI** (GPT-4)
- **Google Gemini**
- **Alibaba Qwen**
- **DeepSeek**
- **Mistral**
- **xAI Grok**
- **Perplexity**
- **OpenRouter**
- **Ollama** (local)
- And more...

Features:
- Cost/latency/quality-aware model selection
- YAML-based model registry
- Vector stores (in-memory, LanceDB)
- Embedding and reranking capabilities

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
