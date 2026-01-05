# Converge — Rust Performance & Memory Model

This document addresses questions commonly raised by **senior Rust engineers**
regarding Converge’s ownership model, concurrency story, and memory behavior.

The short answer is:
> Converge is designed to be *boringly explicit* about ownership and memory,
> even if that means rejecting some forms of maximal parallelism.

---

## Q4.1: Context Ownership & Concurrency

### Question recap
`fn execute(&self, ctx: &mut Context) -> AgentEffect;`

- Does `&mut Context` serialize all agents?
- Are contexts cloned?
- Is there interior mutability (`Arc<RwLock<_>>`)?

### Authoritative answer

**Agents do NOT receive `&mut Context` during execution.**

The signature above is **conceptual**, not literal.

The real model is:

```rust
fn execute(&self, ctx: &Context) -> AgentEffect;
```

Only the **engine** ever holds `&mut Context`.

---

### Actual concurrency model

Converge uses a **two-phase model**:

#### Phase 1 — Parallel read & compute
- Agents receive `&Context` (immutable borrow)
- Many agents may execute in parallel
- Agents may:
  - read context
  - allocate local data
  - call tools (LLMs, APIs)
- Agents may NOT:
  - mutate context
  - share mutable state

This phase is embarrassingly parallel.

---

#### Phase 2 — Serialized merge
- The engine takes `&mut Context`
- AgentEffects are merged **one at a time**
- Ordering is deterministic
- Conflicts are detected explicitly

This phase is intentionally serialized.

---

### Why this model

- Avoids `Arc<RwLock>` everywhere
- Avoids clone-heavy contexts
- Preserves deterministic behavior
- Keeps borrow-checker-friendly design

> Think “parallel compute, serialized commit”.

Very similar to database transaction systems.

---

## Q4.2: What is AgentEffect?

### Why not mutate context directly?

Direct mutation would:
- destroy determinism
- hide conflicts
- make convergence unprovable
- complicate auditing

Instead, agents emit **effects**.

---

### Definition

```rust
struct AgentEffect {
    facts: Vec<Fact>,
    intents: Vec<Intent>,
    evaluations: Vec<Evaluation>,
    trace: TraceEvent,
}
```

An AgentEffect is:
- immutable once created
- self-contained
- mergeable

---

### What the effect model enables

#### 1. Transactional semantics
- An agent either contributes everything or nothing
- Panics or timeouts discard the entire effect

#### 2. Conflict detection
- Conflicting facts are detected at merge time
- Resolution is explicit and domain-specific

#### 3. Deterministic ordering
- Effects are applied in a known order
- Replays produce the same context evolution

#### 4. Auditing
- Every contribution has provenance
- Traces reference effect boundaries

---

### Effect batching

Yes — effects may be batched:
- per agent
- per cycle
- per convergence tier

Batching is an optimization, not a semantic change.

---

## Q4.3: Allocation Strategy & Memory Bounds

### Job-scoped allocation

Converge is **job-scoped**.

This enables:
- arena allocation per job
- bulk deallocation when job finishes
- zero long-lived garbage

---

### Facts & context growth

Recommended strategy:

- Facts allocated in a job-local arena
- Context stores lightweight handles / IDs
- Large payloads live behind references

This keeps context cheap to clone or snapshot.

---

### Memory bounds

Memory growth is bounded by:

- Root Intent budgets
  - max facts
  - max agents
  - max cycles
- Pruning rules
- Tiered convergence

If bounds are exceeded:
- execution halts
- partial results are returned
- failure is explicit

---

### Long-running jobs

For extended runtimes:

- snapshots may be taken
- old facts may be summarized
- detail may be moved to Knowledge storage

This is a *semantic decision*, not GC magic.

---

## Why Converge Rejects Some Rust Patterns

Converge intentionally avoids:

- pervasive `Arc<RwLock<...>>`
- shared mutable graphs
- interior mutability as a default
- speculative cloning of context

These patterns optimize for:
- throughput
- liveness

Converge optimizes for:
- correctness
- predictability
- debuggability

---

## Summary

- Agents execute with immutable access to context
- The engine owns all mutable state
- AgentEffect provides transactional semantics
- Allocation is job-scoped and bounded
- Memory is released wholesale at job completion

---

## One-sentence takeaway

> Converge trades maximum parallel mutation for deterministic, auditable evolution — and Rust’s ownership model makes that trade explicit and safe.
