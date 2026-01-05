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

## Summary Table

| Concern | Decision |
|---------|----------|
| Effect merge order | Stable `AgentId` registration order |
| Eligibility | Dependency index + dirty keys |
| ProposedFact boundary | Separate types, compile-time enforced |
| Convergence check | Dirty-key tracking |
| Fact storage | `HashMap<ContextKey, Vec<Fact>>` |

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
