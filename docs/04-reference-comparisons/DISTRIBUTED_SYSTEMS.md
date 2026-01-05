# Converge — Distributed Systems Model

This document explains how **Converge** relates to classical **distributed systems concerns**.
It answers common questions about topology, consistency, convergence, and failure handling.

The goal is clarity: Converge is intentionally conservative about distribution in order to
guarantee correctness, explainability, and convergence.

---

## 1. Topology: What kind of system is Converge?

### 1.1 Authoritative Answer

**Converge is a single-runtime, single-context system per job.**

This is a deliberate design choice.

A *runtime* may be deployed:
- as a single process on one node (most common)
- as multiple runtimes across machines (for scale)

But:

> **A single Converge job never spans multiple runtimes or nodes.**

There is no distributed shared context.

---

### 1.2 Why Converge is not a distributed context system

Detecting convergence requires detecting a **fixed point**:

```
Contextₙ₊₁ == Contextₙ
```

In distributed systems, detecting *global quiescence* requires:
- vector clocks
- distributed termination detection
- agreement on absence of in-flight messages

This is:
- complex
- expensive
- fragile
- hostile to explainability

Converge explicitly avoids this.

> **Distribution happens at the job level, not inside the job.**

---

### 1.3 How Converge scales instead

Converge scales via:

- many independent jobs
- many independent runtimes
- horizontal replication
- message queues / APIs between jobs

Each runtime:
- owns its context
- owns its convergence logic
- is fully deterministic and auditable

This is similar to how:
- databases scale queries
- CI systems scale builds
- workflow engines scale runs

---

## 2. Consistency Model

### 2.1 What “monotonic context” means

Statements like:
- “context is append-only in meaning”
- “context evolves monotonically”

do **not** imply a distributed CRDT model.

They mean:

> **Within a single runtime, context updates are totally ordered and deterministic.**

---

### 2.2 Consistency guarantees

Within a runtime:

- **Strong consistency**
- Single-writer semantics enforced by the engine
- Deterministic merge order
- No concurrent writes to context

Agents do **not** write concurrently.

Instead:
1. Eligible agents are selected
2. They execute (possibly in parallel internally)
3. Their effects are collected
4. Effects are merged *serially* into context

This gives:

- reproducibility
- debuggability
- clear causality

---

### 2.3 Conflicting facts

Conflicts are handled **explicitly**, never implicitly.

Options include:
- dominance rules (newer > weaker)
- confidence scoring
- constraint rejection
- aggregation agents

Example:

- Agent A emits: “Channel X is promising (confidence 0.6)”
- Agent B emits: “Channel X is saturated (confidence 0.9)”

The system does **not** auto-merge.
An aggregation or governance agent resolves the conflict.

> Conflict resolution is a **domain concern**, not an infrastructure concern.

---

## 3. Failure, Availability, and Partition Tolerance

### 3.1 External tool failures (LLMs, APIs)

External tools are treated as **unreliable dependencies**.

An unavailable tool does **not** crash the runtime.

Instead, tools are invoked with:
- timeouts
- retries
- circuit breakers
- failure classification

---

### 3.2 What happens when a tool fails

If a tool becomes unavailable:

- The agent depending on it fails gracefully
- Partial context remains valid
- Other agents may still proceed
- Convergence may still be reached at a lower tier

Example:
- Web search unavailable
- Relationship analysis still runs
- Strategy synthesis proceeds with reduced confidence

---

### 3.3 Blocking vs non-blocking work

Converge distinguishes between:

- **Required agents** (must succeed for convergence tier)
- **Optional agents** (best-effort enrichment)

A job may:
- produce an early answer
- mark parts of context as incomplete
- continue background work when dependencies recover

This supports **anytime answers**.

---

## 4. Partition Tolerance (CAP Perspective)

Converge does not try to satisfy CAP at the *job level*.

Instead:

- **Consistency**: Strong (within a runtime)
- **Availability**: Best-effort, tiered
- **Partition tolerance**: Handled at the *deployment* level

If a runtime is partitioned:
- the job may fail or degrade
- but no incorrect convergence is reported

> Converge prefers **no answer** or **partial answer**
> over an incorrect one.

---

## 5. Why this design is intentional

Many agent frameworks attempt:
- distributed agents
- shared mutable state
- peer-to-peer reasoning

This leads to:
- non-reproducible behavior
- hidden races
- unprovable convergence
- untestable systems

Converge rejects this.

> **Correctness beats distribution.**

Distribution is achieved by:
- running more jobs
- not by fragmenting a single job

---

## 6. Summary

- Converge is **single-runtime per job**
- Context is **not distributed**
- Consistency is **strong and deterministic**
- Conflicts are **explicitly resolved**
- External failures degrade gracefully
- Scaling happens **around** the runtime, not inside it

---

## 7. One-sentence takeaway

> Converge is not a distributed agent system; it is a distributed *decision service* built from many small, convergent runtimes.
