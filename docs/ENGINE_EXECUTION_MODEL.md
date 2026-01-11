# Engine Execution Model

This document defines the architectural invariants of the Converge execution engine.

## Core Principle

> **Agent execution is a pure function of context evolution.**
> No agent can starve. No agent can run twice.
> Convergence is deterministic and explainable.

## The Convergence Loop

```
initialize context
mark all existing keys as dirty (first cycle only)

repeat:
  clear dirty flags
  find eligible agents (dirty deps ∩ accepts())
  execute eligible agents (parallel read)
  merge effects (serial, deterministic order)
  track which keys changed (new dirty keys)
until no keys changed OR budget exhausted
```

## Non-Negotiable Invariants

### 1. Context is the Only Shared State
- No agent-local lifecycle state (`has_run`, counters, flags, caches)
- Agents are pure functions: `Context → AgentEffect`

### 2. Eligibility is Data-Driven
- No polling
- No pending agent queues
- No background re-check loops
- Agents are reconsidered when their declared dependencies become dirty

### 3. Idempotency via Context
- "Has this agent already contributed?" must be derivable from facts in context
- Pattern: check output key for existing contribution with agent's ID prefix
- **LLM agents special case**: Must check both `ContextKey::Proposals` (pending) and `target_key` (validated) since they emit to Proposals first

### 4. Dependency Completeness
- Any `ContextKey` read in `accepts()` or `execute()` must be declared in `dependencies()`
- Violation causes **starvation**: agent misses execution opportunities

### 5. Deterministic Convergence
- No implicit engine memory
- No hidden scheduling heuristics
- Same context + same agents = same result

## Canonical Idempotency Pattern

```rust
fn accepts(&self, ctx: &Context) -> bool {
    // Preconditions
    if !ctx.has(ContextKey::Seeds) {
        return false;
    }

    // Idempotency: check if we already contributed
    let my_prefix = format!("{}-", self.name());
    !ctx.get(self.output_key)
        .iter()
        .any(|f| f.id.starts_with(&my_prefix))
}
```

### LLM Agent Idempotency Pattern (Special Case)

LLM agents emit to `ContextKey::Proposals` first, then ValidationAgent promotes to `target_key`. The idempotency check must check **both** places:

```rust
fn accepts(&self, ctx: &Context) -> bool {
    // Precondition: at least one input dependency has data
    let has_input = self.config.dependencies.iter().any(|k| ctx.has(*k));
    if !has_input {
        return false;
    }

    // Idempotency: check if we've already contributed
    let my_prefix = format!("{}-", self.name);
    
    // Check Proposals (pending contributions before validation)
    let has_pending_proposal = ctx
        .get(ContextKey::Proposals)
        .iter()
        .any(|f| {
            // Proposal IDs are: "proposal:{target_key}:{agent_name}-{uuid}"
            f.id.contains(&my_prefix)
        });
    
    // Check target_key (validated contributions after validation)
    let has_validated_fact = ctx
        .get(self.config.target_key)
        .iter()
        .any(|f| f.id.starts_with(&my_prefix));

    // Run if we haven't contributed (no pending proposal AND no validated fact)
    !has_pending_proposal && !has_validated_fact
}
```

**Why both checks are needed:**
- Agent emits proposal to `Proposals` with ID `proposal:{target_key}:{agent_name}-{uuid}`
- ValidationAgent promotes to `target_key` with ID `{agent_name}-{uuid}`
- Agent must check both to avoid duplicate execution
- This ensures idempotency throughout the proposal → validation → fact pipeline

## Proof of Correctness

The test `tests/engine_convergence_bones.rs` defines the **minimal correctness contract** of the engine. It proves:

1. **Multi-precondition agents** wait correctly for ALL dependencies
2. **Out-of-order dirty keys** don't cause starvation
3. **Context-derived idempotency** prevents duplicate execution
4. **Broken agents** (undeclared dependencies) ARE starved

Run with traces:
```bash
cargo test --test engine_convergence_bones -- --nocapture
```

## What Converge Is NOT

- ❌ A workflow engine
- ❌ A task queue
- ❌ An agent supervisor
- ❌ A retry/backoff system

## What Converge IS

> A semantic convergence engine that reaches a fixed point over a monotonic context.

Every design choice must preserve that truth.
