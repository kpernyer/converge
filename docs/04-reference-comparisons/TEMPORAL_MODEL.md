# Converge — Temporal / Durable Execution Model

This document addresses questions commonly raised by **Temporal / durable workflow architects**
when evaluating Converge.

Temporal and Converge solve **adjacent but different problems**.
This document clarifies where they align, where they differ, and why.

---

## Framing: Converge is not a Durable Workflow Engine

Before answering specifics, this must be explicit:

> **Converge is not a durable workflow engine like Temporal.**

Temporal is designed to:
- run workflows that may last days or months
- survive crashes transparently
- replay execution deterministically
- guarantee exactly-once semantics

Converge is designed to:
- reason over bounded decision spaces
- converge to a correct result
- be explainable and auditable
- stop when a fixed point is reached

Durability is **optional and explicit**, not implicit.

---

## Q3.1: Durability Guarantees

### Question recap
> Is execution replayable after crash?  
> Is context persisted or in-memory?  
> Can a job resume where it left off?

### Answer

#### 1. Context persistence is a deployment choice

Converge supports **optional context persistence**.

A runtime may:
- keep context purely in memory (default, simplest)
- persist context snapshots to disk or database
- persist after each merge cycle or at checkpoints

Persistence is:
- explicit
- coarse-grained
- outside the core engine

This avoids coupling correctness to storage semantics.

---

#### 2. Replay model (not event sourcing)

Converge does **not** use full event sourcing.

Instead:
- Context snapshots may be persisted
- Trace records *what happened*, not *how to replay*

After a crash:
- a job may restart from the Root Intent
- or resume from the last persisted context snapshot

This is **state restoration**, not deterministic replay.

---

#### 3. Why full replay is not guaranteed

Full replay assumes:
- deterministic execution
- stable external dependencies

Converge explicitly allows:
- non-deterministic tools (LLMs, APIs)
- heuristic exploration

Therefore:

> Converge guarantees **correctness of results**, not **bitwise replayability of execution**.

---

## Q3.2: Determinism vs LLM Agents

### Question recap
> How do you reconcile deterministic agents with non-deterministic LLMs?  
> Is LLM output captured and replayed?

### Answer

#### 1. Separation of concerns

Converge separates:

- **Control logic** (engine, scheduling, convergence)
- **Knowledge contributions** (agent effects)

Only control logic must be deterministic.

---

#### 2. LLM agents as effect generators

LLM agents:
- read context
- produce effects (facts, interpretations, explanations)
- never control execution flow

Their outputs are treated as **inputs**, not code.

---

#### 3. Trace captures LLM outputs

When durability is enabled:
- LLM outputs are captured in the Trace
- Effects merged into context are persisted

On resume:
- the system continues from persisted context
- LLMs are **not re-invoked for past steps**

This is similar to:
- caching activity results in Temporal
- but without strict replay semantics

---

#### 4. Determinism guarantee (precise)

Converge guarantees:

- deterministic convergence **given a context**
- not deterministic reproduction of every intermediate step

This is sufficient for:
- explainability
- audit
- trust

---

## Q3.3: Compensating Actions & Rollback

### Question recap
> If a job partially converges then fails, how do you rollback?  
> Are there compensating agents?

### Answer

#### 1. There is no implicit rollback

Converge does **not** implement saga-style rollback.

Why:
- context evolution is monotonic
- facts represent observations, not side effects

You cannot “undo” knowledge.

---

#### 2. Compensation is semantic, not mechanical

If rollback is needed, it is expressed as:

- new facts
- revised evaluations
- explicit invalidation markers

Example:
- “Strategy A is invalid under new regulation”

This refines context instead of reverting it.

---

#### 3. External side effects

Converge itself does **not** perform irreversible side effects.

If:
- emails are sent
- orders are placed
- resources are allocated

Those actions must occur **outside** Converge,
using its outputs as recommendations.

Saga-style compensation belongs there.

---

## Comparison: Temporal vs Converge

| Dimension | Temporal | Converge |
|---------|----------|----------|
| Primary goal | Durable execution | Correct decision-making |
| Execution model | Event sourcing | Fixed-point convergence |
| Replay | Deterministic | Best-effort via snapshots |
| LLM support | External activities | First-class but bounded |
| Rollback | Compensating actions | Context refinement |
| Duration | Long-lived workflows | Bounded jobs |

---

## Summary

- Converge is **not** a Temporal replacement
- Durability is optional and explicit
- Replay restores state, not execution
- LLM non-determinism is contained and traced
- Rollback is semantic, not mechanical

---

## One-sentence takeaway

> Temporal guarantees that workflows keep running.  
> Converge guarantees that decisions converge correctly.
