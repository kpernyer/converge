# Communication & Distribution in Converge
## Why gRPC Is Allowed — and Queues / Pub‑Sub Are Not

This document makes an **explicit, opinionated statement** about how Converge
communicates across process and machine boundaries.

It exists to prevent accidental architectural drift.

---

## Executive Summary

> **Converge permits synchronous RPC (e.g. gRPC + Protobuf) for explicit,
> engine‑controlled interactions across process boundaries, and explicitly
> forbids asynchronous messaging systems (queues, Pub/Sub, event buses).**

This is not a tooling preference.
It is a **semantic requirement** of the convergence model.

---

## The Core Invariant

Converge enforces one foundational invariant:

> **The engine must always retain causal control over execution.**

That means the engine must know:
- *when* something runs
- *why* it runs
- *what* changed as a result

Any communication mechanism that breaks this invariant is disallowed.

---

## Why Queues and Pub/Sub Are Forbidden

Queues, Pub/Sub systems, and event buses introduce **implicit behavior** that
violates Converge’s execution guarantees.

### What they introduce implicitly

- Hidden buffering
- Hidden retries
- Hidden ordering semantics
- Hidden concurrency
- Hidden durability
- Eventual consistency

These properties are often desirable in distributed systems —
but they are **toxic to convergence**.

---

### The fundamental problem

With queues or Pub/Sub:

```text
Producer emits message
System decides when / if it is processed
Consumer reacts later
```

Control flow escapes the engine.

This makes it impossible to:
- reason about convergence
- guarantee determinism
- explain causality
- enforce invariants reliably

For Converge, this is unacceptable.

---

## Why gRPC + Protobuf Are Explicitly Allowed

gRPC is permitted because it preserves **explicit control**.

### What gRPC guarantees

- Caller decides *when* the call happens
- Caller waits for the result
- Caller owns retries (or not)
- Caller observes failure explicitly
- Caller decides what to do next

In other words:

> gRPC is a typed function call across a process boundary.

This preserves the engine’s authority.

---

## Approved gRPC Usage Patterns

### 1. Engine → Tool calls

Examples:
- LLM inference service
- Optimization solver
- External knowledge lookup
- MCP-compatible tools

```text
Converge Engine
   |
   | gRPC (explicit call)
   v
External Tool
```

Rules:
- Engine initiates the call
- Engine blocks or times out explicitly
- Output is treated as data, never authority

---

### 2. Process isolation for safety or performance

Examples:
- Native solvers
- GPU-backed inference
- Sandboxed execution

Here, gRPC is simply **process isolation**, not orchestration.

---

### 3. Optional: Agent execution out-of-process

Allowed only if:
- The engine schedules execution
- The engine owns context
- The engine merges effects
- The agent is stateless

```text
Engine
  |
  | gRPC: ExecuteAgent(ContextView)
  v
Agent Worker
```

This does **not** turn agents into independent actors.

---

## Explicitly Disallowed Patterns

### ❌ Agent‑to‑Agent communication

```text
Agent A → gRPC → Agent B   ❌
```

This recreates:
- emergent behavior
- hidden dependencies
- non-deterministic ordering

---

### ❌ Event‑driven orchestration

```text
Agent emits event → subscriber reacts later   ❌
```

This is Pub/Sub with extra steps.

---

### ❌ Durable execution via messaging

If gRPC is wrapped with:
- automatic retries
- background task execution
- queues for durability

You have rebuilt a workflow engine.
That is **out of scope** for Converge.

---

## Why Protobuf Is the Preferred IDL

Protobuf aligns naturally with Converge’s philosophy:

- Explicit schemas
- Strong versioning
- No ambient behavior
- Language‑agnostic contracts

It matches:
- Context schemas
- Fact definitions
- Effect boundaries
- Traceability requirements

This is **mechanical sympathy**, not fashion.

---

## Relationship to Distributed Systems

Converge is **distributed at the job level**, not the execution level.

- One job = one runtime
- Horizontal scale = many jobs
- No shared mutable context across nodes

gRPC enables distribution **without changing semantics**.

Queues would.

---

## The Rule to Remember

> **If a communication mechanism allows work to happen “later” or “elsewhere”
> without the engine’s direct involvement, it is not allowed in Converge.**

This single rule explains every decision in this document.

---

## One‑Sentence Takeaway

> **Queues distribute control.  
> gRPC preserves control.  
> Converge requires control.**
