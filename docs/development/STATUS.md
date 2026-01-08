# Converge — Implementation Status

This document shows what's built, how pieces connect, and what comes next.

---

## What We Have (Day 1 Complete)

```
┌─────────────────────────────────────────────────────────────────┐
│                         DATA MODEL                               │
├─────────────────────────────────────────────────────────────────┤
│                                                                  │
│  ContextKey          Fact                   ProposedFact         │
│  ───────────         ────                   ────────────         │
│  Seeds               key: ContextKey        key: ContextKey      │
│  Hypotheses          id: String             id: String           │
│  Strategies          content: String        content: String      │
│  Constraints                                confidence: f64      │
│  Signals                                    provenance: String   │
│                                                                  │
│                      ▲                      │                    │
│                      │                      │ TryFrom (validate) │
│                      │                      ▼                    │
│                      └──────────────────────┘                    │
│                                                                  │
├─────────────────────────────────────────────────────────────────┤
│                                                                  │
│  Context                          AgentEffect                    │
│  ───────                          ───────────                    │
│  facts: Map<Key, Vec<Fact>>       facts: Vec<Fact>               │
│  dirty_keys: Vec<ContextKey>      affected_keys() → Vec<Key>     │
│  version: u64                                                    │
│                                                                  │
│  get(key) → &[Fact]                                              │
│  has(key) → bool                                                 │
│  dirty_keys() → &[ContextKey]                                    │
│                                                                  │
└─────────────────────────────────────────────────────────────────┘
```

### Key Insight: The Type Boundary

```
  LLM Agent                    Validation Agent              Context
  ─────────                    ────────────────              ───────
      │                              │                          │
      │  ProposedFact                │                          │
      │  {confidence: 0.7}           │                          │
      ├─────────────────────────────►│                          │
      │                              │                          │
      │                    validate & TryFrom                   │
      │                              │                          │
      │                              │  Fact                    │
      │                              ├─────────────────────────►│
      │                              │                          │

   ❌ LLM cannot add Fact directly (compile error)
   ✓  Must go through validation
```

---

## What We Don't Have Yet (Day 2)

```
┌─────────────────────────────────────────────────────────────────┐
│                         ENGINE (Coming)                          │
├─────────────────────────────────────────────────────────────────┤
│                                                                  │
│  Agent Trait              Agent Registry         Engine          │
│  ───────────              ──────────────         ──────          │
│  dependencies() → [Key]   agents: Vec<Agent>     run_until_      │
│  accepts(&Context)        index: Key → [AgentId]   converged()   │
│  execute(&Context)        register(agent)                        │
│       → AgentEffect       next_id: u32                           │
│                                                                  │
└─────────────────────────────────────────────────────────────────┘
```

---

## The Execution Loop (What We're Building Next)

```
                    ┌─────────────────┐
                    │  Root Intent    │
                    │  (input)        │
                    └────────┬────────┘
                             │
                             ▼
              ┌──────────────────────────────┐
              │     Initialize Context       │
              └──────────────┬───────────────┘
                             │
         ┌───────────────────┼───────────────────┐
         │                   ▼                   │
         │    ┌──────────────────────────────┐   │
         │    │  Which agents are eligible?  │   │
         │    │  (check dirty_keys + index)  │   │
         │    └──────────────┬───────────────┘   │
         │                   │                   │
         │                   ▼                   │
         │    ┌──────────────────────────────┐   │
         │    │  Execute eligible agents     │   │
         │    │  (parallel, read-only ctx)   │   │
         │    └──────────────┬───────────────┘   │
         │                   │                   │
         │                   ▼                   │
         │    ┌──────────────────────────────┐   │
         │    │  Collect AgentEffects        │   │
         │    └──────────────┬───────────────┘   │
         │                   │                   │
         │                   ▼                   │
         │    ┌──────────────────────────────┐   │
         │    │  Merge effects (serial)      │   │
         │    │  Track dirty_keys            │   │
         │    └──────────────┬───────────────┘   │
         │                   │                   │
         │                   ▼                   │
         │         ┌─────────────────┐           │
         │         │ dirty_keys      │           │
         │         │ empty?          │           │
         │         └────────┬────────┘           │
         │                  │                    │
         │          No      │      Yes           │
         │    ┌─────────────┴──────────────┐     │
         │    │                            │     │
         │    ▼                            ▼     │
         └────┘                    ┌─────────────┴─────────────┐
                                   │  CONVERGED                │
                                   │  Return Context           │
                                   └───────────────────────────┘
```

---

## How to See It Evolve

### Right Now: Run the Tests

```bash
cd converge-core
cargo test
```

Output shows 12 tests covering the data model.

### After Day 2: Run a Convergence Example

```rust
// This is what we'll build:
let mut engine = Engine::new();

// Register agents
engine.register(SeedAgent::new());      // Emits initial fact
engine.register(ReactOnceAgent::new()); // Reacts once, then stops

// Run until convergence
let result = engine.run_until_converged(root_intent);

// See what happened
println!("Converged in {} cycles", result.cycles);
println!("Final context: {:?}", result.context);
```

### After Day 3: See Convergence Proof

```rust
#[test]
fn engine_converges_deterministically() {
    let engine = Engine::new();
    // ... setup ...

    let result1 = engine.run_until_converged(intent.clone());
    let result2 = engine.run_until_converged(intent);

    // Same input → same output
    assert_eq!(result1.context, result2.context);
    assert_eq!(result1.cycles, result2.cycles);
}

#[test]
fn engine_cannot_loop_forever() {
    let engine = Engine::new();
    engine.set_budget(Budget { max_cycles: 100 });

    // Even with adversarial agents, we terminate
    let result = engine.run_until_converged(intent);

    assert!(result.cycles <= 100);
}
```

---

## File Structure

```
converge-core/
├── Cargo.toml
└── src/
    ├── lib.rs          # Public API
    ├── context.rs      # Context, ContextKey, Fact, ProposedFact  ✓
    ├── effect.rs       # AgentEffect                              ✓
    ├── error.rs        # ConvergeError                            ✓
    ├── agent.rs        # Agent trait                              ⏳ Day 2
    ├── registry.rs     # Agent registry + index                   ⏳ Day 2
    └── engine.rs       # Engine loop                              ⏳ Day 2
```

---

## Evolution Path

| Phase | What | Proves |
|-------|------|--------|
| Day 1 ✓ | Data model | Types are correct |
| Day 2 | Engine loop | Convergence works |
| Day 3 | Test agents | System is usable |
| Phase 3 | Gherkin | Correctness is enforced |
| Phase 5 | LLM agents | ProposedFact boundary works |

---

## Quick Reference: What Each Type Does

| Type | Purpose | Key Property |
|------|---------|--------------|
| `ContextKey` | Categories of facts | Enables dependency indexing |
| `Fact` | Trusted assertion | Immutable, has provenance |
| `ProposedFact` | LLM suggestion | Must validate to become Fact |
| `Context` | Job state | Append-only, tracks changes |
| `AgentEffect` | Agent output | Buffered, merged serially |
| `Agent` (Day 2) | Capability | Never mutates directly |
| `Engine` (Day 2) | Coordinator | Owns convergence |

---

## One Command to Verify

```bash
cd converge-core && cargo test && cargo clippy
```

If this passes, the foundation is solid.
