# Converge Core — Usage Guide

This guide shows how to use the Converge Core library.

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
converge-core = "0.1.0"
```

## Basic Usage

### Creating an Engine

```rust
use converge_core::Engine;

let mut engine = Engine::new();
```

### Registering Agents

```rust
use converge_core::agents::SeedAgent;

engine.register(SeedAgent::new("seed-1", "initial data"));
```

### Running to Convergence

```rust
use converge_core::Context;

let context = Context::new();
let result = engine.run(context)?;

if result.converged {
    println!("Converged in {} cycles", result.cycles);
} else {
    println!("Terminated: {:?}", result.termination_reason);
}
```

## Implementing Custom Agents

### Simple Deterministic Agent

```rust
use converge_core::{Agent, AgentId, Context, AgentEffect, ContextKey};

struct MyAgent {
    id: AgentId,
}

impl Agent for MyAgent {
    fn accepts(&self, ctx: &Context) -> bool {
        // Determine if this agent should run
        ctx.has(ContextKey::Seeds)
    }

    fn dependencies(&self) -> &'static [ContextKey] {
        &[ContextKey::Seeds]
    }

    fn execute(&self, ctx: &Context) -> AgentEffect {
        // Read from context
        let seeds = ctx.get(ContextKey::Seeds);
        
        // Produce effects
        AgentEffect::new()
            .with_fact(/* ... */)
    }
}
```

### LLM-Backed Agent

```rust
use converge_core::{Agent, Context, AgentEffect, ProposedFact};
use converge_core::llm::LlmClient;

struct LlmAgent {
    llm: LlmClient,
}

impl Agent for LlmAgent {
    fn execute(&self, ctx: &Context) -> AgentEffect {
        // Call LLM
        let response = self.llm.generate(/* prompt */)?;
        
        // Return as ProposedFact (requires validation)
        AgentEffect::new()
            .with_proposed_fact(ProposedFact::from_llm(response))
    }
}
```

## Error Handling

All operations return `Result` types:

```rust
use converge_core::{Engine, Context, ConvergeError};

match engine.run(context) {
    Ok(result) => {
        // Handle success
    }
    Err(ConvergeError::BudgetExceeded { .. }) => {
        // Handle budget exceeded
    }
    Err(ConvergeError::ValidationError { .. }) => {
        // Handle validation error
    }
    Err(e) => {
        // Handle other errors
    }
}
```

## Observability

The library uses `tracing` for structured logging. Set up a subscriber:

```rust
use tracing_subscriber;

tracing_subscriber::fmt::init();

// Now all engine operations will emit traces
let result = engine.run(context)?;
```

## Best Practices

1. **Declare dependencies accurately** — This enables efficient eligibility checking
2. **Keep agents stateless** — State should live in context
3. **Validate LLM outputs** — Always validate `ProposedFact` before promotion
4. **Set appropriate budgets** — Prevent infinite execution
5. **Use structured logging** — Enable observability in production

