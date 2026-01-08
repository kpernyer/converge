# Converge — Authoritative Implementation Decisions

This document records **locked-in implementation decisions** for Converge v1.
These are not open for debate during implementation. Changes require explicit review.

Guiding principle:
> Favor correctness over cleverness. Determinism first, sophistication later.

---

## 1. Effect Merge Ordering

**Decision:** Deterministic merge order = agent registration order + stable cycle index

**Implementation:**
- Agents are assigned a stable `AgentId` at registration time
- Merge effects in ascending `AgentId` order
- Within one agent, preserve effect emission order

```rust
type AgentId = u32; // monotonic assignment
```

**Rationale:**
- Simple and fully deterministic
- Easy to reason about
- No hidden coupling between agents

**Why not priority or dependency-based:**
- Priority systems leak policy into infrastructure
- Topological ordering implies inter-agent dependency graphs (complex, unnecessary now)

**Evolution path:** Priorities can be added later as an optional field without breaking semantics.

---

## 2. Dependency Index Maintenance

**Decision:** Incremental maintenance on agent registration + context-key dirty tracking

**Implementation:**
```rust
// On register
fn on_register(agent: &Agent) {
    for key in agent.dependencies() {
        index[key].push(agent_id);
    }
}

// On merge
fn on_merge(effect: &AgentEffect) -> Vec<ContextKey> {
    effect.affected_keys() // returns dirty_keys
}

// Eligibility
fn eligible_agents(dirty_keys: &[ContextKey]) -> Vec<AgentId> {
    dirty_keys.iter()
        .flat_map(|key| &index[key])
        .collect()
}
```

**Rationale:**
- O(changes), not O(agents)
- No rebuild cost per cycle
- Matches incremental computation / rule engine patterns

**Why not rebuild per cycle:**
- Wasteful
- Hides bugs (everything runs "just in case")
- Undermines scalability claims

**Invariant:** An agent only runs if something it depends on changed.

---

## 3. ProposedFact Type Boundary

**Decision:** Compile-time separation with distinct types

**Implementation:**
```rust
struct ProposedFact { /* ... */ }
struct Fact { /* ... */ }

impl TryFrom<ProposedFact> for Fact {
    type Error = ValidationError;
    // Explicit conversion required
}
```

**Rationale:**
- Rust makes illegal states unrepresentable
- Cannot "accidentally" treat a suggestion as truth
- LLM containment enforced by the type system

**Why not enum variants:**
```rust
// NOT THIS:
enum FactLike { Proposed(ProposedFact), Accepted(Fact) }
```
- Too easy to forget to check
- Reviewers will miss a branch
- Loses one of Rust's biggest advantages

**Rule:** If something is dangerous, make it impossible to misuse.

---

## 4. Context Equality / Convergence Check

**Decision:** Dirty-key tracking (not hashing, not deep compare)

**Implementation:**
```rust
struct MergeResult {
    dirty_keys: Vec<ContextKey>,
}

// Convergence condition
fn has_converged(result: &MergeResult) -> bool {
    result.dirty_keys.is_empty()
}
```

**Rationale:**
- O(changes), not O(context size)
- No hashing pitfalls
- No deep comparison cost
- Matches dependency-index logic

**Why not hashing:**
- Expensive for large contexts
- Subtle bugs with non-canonical ordering
- Harder to debug

**Why not deep compare:**
- Slow and scales poorly
- Totally unnecessary given our structure

**Principle:** You already know what changed. Use that.

---

## 5. Fact Storage

**Decision:** `HashMap<ContextKey, Vec<Fact>>` with duplicate detection by `id`

**Implementation:**
```rust
struct Context {
    facts: HashMap<ContextKey, Vec<Fact>>,
    dirty_keys: Vec<ContextKey>,
    version: u64,
}
```

**Rationale:**
- Simple and sufficient for v1
- O(1) lookup by key category
- Version counter enables quick convergence checks

**Evolution path:** Can migrate to arena allocation if memory pressure requires.

---

## 6. Crate Layering & Dependency Discipline

**Decision:** Strict layered architecture with unidirectional dependencies

**Implementation:**
```
                        ┌─────────────────┐
                        │ converge-runtime│ (HTTP, gRPC, TUI)
                        └────────┬────────┘
                                 │
         ┌───────────────────────┼───────────────────────┐
         │                       │                       │
         ▼                       ▼                       ▼
┌─────────────────┐   ┌─────────────────┐   ┌─────────────────┐
│ converge-domain │   │  converge-tool  │   │    (future)     │
│  (GrowthStrategy,   │  (Gherkin, CLI) │   │                 │
│   Scheduling...)│   │                 │   │                 │
└────────┬────────┘   └────────┬────────┘   └─────────────────┘
         │                     │
         └──────────┬──────────┘
                    ▼
         ┌─────────────────┐
         │converge-provider│ (Anthropic, OpenAI, Gemini...)
         └────────┬────────┘
                  │
                  ▼
         ┌─────────────────┐
         │  converge-core  │ (Engine, Context, Agent, traits)
         └─────────────────┘
```

**Layer Responsibilities:**

| Crate | Contains | Does NOT Contain |
|-------|----------|------------------|
| `converge-core` | Traits, abstractions, engine, context, agent model | Provider implementations, API keys, HTTP clients |
| `converge-provider` | LLM implementations, model metadata, provider factory | Domain logic, business rules |
| `converge-domain` | Domain agents, invariants, use-case pipelines | Provider selection, raw LLM calls |
| `converge-tool` | Dev tools, validators, CLI utilities | Runtime servers |
| `converge-runtime` | HTTP/gRPC servers, TUI | Business logic, agent implementations |

**Rules:**

1. **Dependencies flow downward only** — Never `core → provider` or `core → domain`
2. **Core is provider-agnostic** — Traits in core, implementations in provider
3. **No vendor lock-in in core** — No `anthropic`, `openai`, `reqwest` in core
4. **Domain doesn't know providers** — Domain uses traits, not concrete types
5. **Runtime is a thin shell** — Just wiring, no business logic

**Rationale:**
- Prevents circular dependencies
- Enables testing core without network
- Allows swapping providers without touching core
- Keeps compile times manageable
- Makes the architecture legible

**Why this matters:**

```rust
// WRONG — Core depending on provider specifics
// converge-core/src/llm.rs
use reqwest;  // ❌ HTTP client in core
pub struct AnthropicProvider { ... }  // ❌ Concrete provider in core

// RIGHT — Abstract in core, concrete in provider
// converge-core/src/llm.rs
pub trait LlmProvider { ... }  // ✅ Trait only

// converge-provider/src/anthropic.rs
pub struct AnthropicProvider { ... }  // ✅ Implementation here
```

**Anti-patterns to avoid:**
- Adding `reqwest`, `tokio`, or HTTP types to core
- Putting provider-specific model lists in core
- Making domain depend directly on provider implementations
- Circular imports between crates

**Evolution path:** If core grows too large, extract `converge-context` and `converge-engine`.

---

## Summary Table

| Concern | Decision |
|---------|----------|
| Effect merge order | Stable `AgentId` registration order |
| Eligibility | Dependency index + dirty keys |
| ProposedFact boundary | Separate types, compile-time enforced |
| Convergence check | Dirty-key tracking |
| Fact storage | `HashMap<ContextKey, Vec<Fact>>` |
| Crate layering | Strict downward-only dependencies |

---

## Usage Note

When implementing or reviewing code, reference this document:

> "Implement Converge v1 using deterministic agent registration order for effect merges,
> dependency-indexed eligibility driven by dirty ContextKeys, compile-time separation
> between ProposedFact and Fact, and convergence detection via dirty-key tracking
> (no hashing or deep comparison). Favor clarity over optimization."

---

## Change Policy

These decisions are locked for v1. To change:
1. Document the problem with the current approach
2. Propose alternative with rationale
3. Get explicit approval
4. Update this document

Do not deviate silently.
