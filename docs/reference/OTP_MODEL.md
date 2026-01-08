# Converge — OTP / Actor Model Considerations

This document addresses common questions raised by **Erlang / Elixir (OTP)** practitioners
when evaluating Converge.

Some questions stem from valid distributed-systems instincts.
Others assume properties Converge **intentionally does not have**.

This document clarifies the differences.

---

## Framing: Converge is *not* an Actor System

Before answering specific questions, this must be explicit:

> **Converge is not an actor model system.**

It does not aim to replicate:
- Erlang processes
- mailboxes
- supervision trees
- distributed fault tolerance at the process level

Instead, Converge is closer to:
- a deterministic execution engine
- a constraint solver
- a decision runtime

The unit of correctness is **the job**, not the process.

---

## Q2.1: Fault Isolation & “Let it crash”

### Question recap
> What happens when an agent panics?  
> Is context corrupted?  
> How do you recover partial execution?  
> Where is the supervision hierarchy?

### Answer

#### 1. Agent panics are contained

Agents **never mutate context directly**.

Execution model:

1. Agent executes in isolation
2. Agent emits effects into a buffer
3. Engine merges effects *after* execution

If an agent panics:
- its execution is aborted
- its effect buffer is discarded
- context remains unchanged
- the failure is recorded in the trace

This gives **transaction-like semantics** per agent.

There is no partial context corruption.

---

#### 2. Why there is no supervision tree

OTP supervision trees exist to:
- restart long-lived processes
- maintain availability of services
- recover from crashes transparently

Converge does **not** run long-lived agent processes.

Agents are:
- short-lived
- pure functions over context
- scheduled explicitly by the engine

There is nothing to “restart”.

> In Converge, **crash recovery is job-level**, not agent-level.

If correctness is compromised, the job fails or degrades.
It is never silently repaired.

This is a conscious tradeoff.

---

#### 3. Recovery model

Converge recovery options:
- retry agent execution
- skip agent contribution
- downgrade convergence tier
- abort job

Recovery is:
- explicit
- observable
- auditable

There is no hidden self-healing.

---

## Q2.2: Backpressure & Throughput

### Question recap
> What if agents produce facts faster than others consume?  
> Is there flow control?

### Answer

#### 1. There is no unbounded message flow

Converge has **no mailboxes**.

Agents do not send messages to each other.

All communication happens via:
- a bounded context
- effect buffers
- engine-controlled merge cycles

This eliminates classic mailbox overflow problems.

---

#### 2. Backpressure is structural, not reactive

Backpressure is enforced via:

- **Budgets**
  - max agents per cycle
  - max facts
  - max cycles

- **Eligibility rules**
  - most agents are inactive most of the time

- **Pruning**
  - dominated or irrelevant agents are removed

If too much information is produced:
- weaker facts are dropped
- branches are pruned
- exploration is capped

This is closer to **search-space control** than message flow control.

---

#### 3. Throughput vs correctness

OTP systems optimize for:
- sustained throughput
- availability under load

Converge optimizes for:
- bounded reasoning
- correctness
- explainability

These are different goals.

---

## Q2.3: Process Isolation & Shared Memory

### Question recap
> Are agents isolated like Erlang processes?  
> Does &mut Context serialize all execution?

### Answer

#### 1. Agents share memory by design

Agents are **not isolated processes**.

They:
- run in the same runtime
- observe the same context
- are coordinated by a single engine

This is intentional.

> Converge chooses **shared memory with strict control**
> over message passing with emergent behavior.

---

#### 2. &mut Context does NOT mean serialized computation

Key distinction:

- Agents may execute **in parallel** internally
- They may read context concurrently
- They may call tools concurrently

What is serialized is:
- the **merge of effects into context**

This provides:
- determinism
- reproducibility
- clear causality

Think of it as:
> *parallel compute, serialized commit*

Very similar to database transaction models.

---

#### 3. Why actor isolation is rejected

Actor isolation implies:
- eventual consistency
- nondeterministic ordering
- emergent behavior

This is incompatible with:
- fixed-point convergence
- semantic invariants
- explainability

Converge explicitly rejects:
> “If it converges, it’s probably fine.”

Instead it enforces:
> “If it converged, we can prove why.”

---

## Mapping OTP Concepts to Converge

| OTP Concept | Converge Equivalent |
|------------|--------------------|
| Process | Agent execution |
| Mailbox | Context |
| Supervisor | Engine |
| Crash | Agent failure (isolated) |
| Restart | Retry or skip |
| Availability | Tiered convergence |
| Throughput | Bounded exploration |

This mapping is conceptual, not literal.

---

## Summary

- Converge is **not** an actor system
- Fault isolation is achieved via **effect buffering**
- There are no mailboxes, hence no mailbox backpressure
- Shared memory is controlled, not concurrent
- Serialization happens at **commit time**, not compute time
- Supervision is replaced by **explicit correctness rules**

---

## One-sentence takeaway

> Erlang optimizes for systems that must never stop.  
> Converge optimizes for systems that must never be wrong.
