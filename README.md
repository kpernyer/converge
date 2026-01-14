# Converge: Semantic Governance & Alignment

> Converge is a vision for **semantic governance**. We move from fragmented intent to unified, converged states through a deterministic alignment engine. Our mission is to provide a stable foundation for complex decision-making where human authority and AI agency coexist in a transparent, explainable ecosystem.

## Converge Platform

The core platform for the Converge semantic governance system. This repository contains the foundational crates: `converge-core`, `converge-provider`, and `converge-domain`.

**Website:** [converge.zone](https://converge.zone) | **Docs:** [docs.rs](https://docs.rs/converge-core) | **Crates.io:** [converge-core](https://crates.io/crates/converge-core)

---

## Quick Start

```bash
# Clone with submodules
git clone --recurse-submodules https://github.com/kpernyer/converge.git

# Or if already cloned
git submodule update --init --recursive

# Build
cargo build --release

# Run tests
cargo test
```

---

## What This Is

Converge Platform is the **semantic convergence engine** that powers Converge. It provides:

- **converge-core**: Runtime engine, agent traits, capability abstractions
- **converge-provider**: 14+ LLM providers, model selection, vector stores
- **converge-domain**: 12 business use cases with deterministic + LLM variants

Converge is not an agent framework. It is not a workflow engine. It is not event-driven.

It is a system for **alignment** — moving from intent to stable, explainable outcomes.

---

## Documentation

- **Getting Started:** See [converge.zone](https://converge.zone)
- **Architecture:** See [converge-business/knowledgebase/platform-ARCHITECTURE.md](../converge-business/knowledgebase/platform-ARCHITECTURE.md)
- **Knowledgebase:** See [converge-business/knowledgebase/](../converge-business/knowledgebase/)
- **For LLMs:** See [AGENTS.md](AGENTS.md) in this repository

---

## Crates

| Crate | Version | Description |
|-------|---------|-------------|
| [converge-core](https://crates.io/crates/converge-core) | 0.6.1 | Runtime engine, agent traits, capability abstractions |
| [converge-provider](https://crates.io/crates/converge-provider) | 0.2.3 | 14+ LLM providers, model selection, vector stores |
| [converge-domain](https://crates.io/crates/converge-domain) | 0.2.3 | 12 business use cases with deterministic + LLM variants |

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

## Related Projects

- [converge-runtime](../converge-runtime) - HTTP/gRPC server
- [converge-application](../converge-application) - Application distribution
- [converge-business](../converge-business) - Documentation and strategy
- [converge-ios](../converge-ios) - iOS mobile client
- [converge-android](../converge-android) - Android mobile client

---

## License

- **converge-core**: Proprietary (Aprio One AB)
- **converge-provider, converge-domain**: MIT

---

## Contributing

We welcome contributors who value:

- correctness
- clarity
- restraint

See `CONTRIBUTING.md` and `CODE_OF_CONDUCT.md`.
