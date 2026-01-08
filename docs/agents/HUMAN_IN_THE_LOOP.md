# Human-in-the-Loop (HITL) in Converge

This document defines **exactly how humans participate in Converge**
without breaking convergence, determinism, or architectural guarantees.

It is intentionally explicit and conservative.

---

## Executive Summary

> **In Converge, humans are never part of the convergence process itself.**
> They act only as **initiators**, **validators**, or **authorities of last resort**.

When human input is required:
- the engine enters a **Waiting state**
- execution halts cleanly
- state is persisted explicitly
- convergence is neither advanced nor invalidated
- execution resumes deterministically when input arrives

Humans are **outside the fixed-point loop**.

---

## Core Principle

> **Humans provide authority, not computation.**

A human:
- does not execute agents
- does not trigger control flow
- does not participate in eligibility
- does not influence scheduling

A human decision is always represented as **data** and merged through the engine.

---

## Where Humans Can Appear

Humans participate in exactly three places.

---

## 1. Humans as Root Intent Authors (Before Execution)

This is the simplest case.

- A human defines or approves the `RootIntent`
- Scope, budgets, and convergence policy are locked before execution
- No runtime interaction is required

This happens **outside** the engine runtime.

---

## 2. Humans as Validators (Convergence Gates)

This is the canonical HITL case.

### Example
> “A growth strategy must be approved by a human before acceptance.”

---

### What triggers waiting

- The engine reaches a candidate convergence state
- A Gherkin **acceptance invariant** requires human approval
- The engine cannot legally emit results

---

### Engine behavior

When this happens, the engine:

1. Stops execution immediately
2. Emits a `WaitingForHuman` status
3. Persists state
4. Returns control to the caller

No background execution continues.

---

## The Waiting State (Critical Concept)

The waiting state is **not** a running workflow.

It is a **persisted snapshot of intent and context**.

### Waiting means:

- No agents are executing
- No context mutations occur
- No timers are running
- No retries are happening

The engine is **quiescent**.

---

## Persistence During Waiting

### What is persisted

At minimum:

- Job ID
- RootIntent
- Current Context
- Convergence tier reached
- Required human action(s)
- Trace (for explanation)

---

### Where persistence lives

Converge does not mandate storage, but recommends:

- **SurrealDB** for:
  - flexible schemas
  - graph relations
  - provenance tracking

Persistence is explicit and external to the engine.

---

### Persistence model

```text
Job
 ├─ job_id
 ├─ root_intent
 ├─ context_snapshot
 ├─ waiting_reason
 ├─ required_action
 ├─ created_at
 └─ last_updated
```

The engine can be safely terminated after persistence.

---

## Correlating a Future Human Reply

This is critical for long waits (hours, days, weeks).

---

### Correlation mechanism

- Every waiting state includes a **stable Job ID**
- Human-facing systems reference this ID
- Human responses must include it

No implicit matching.
No heuristics.

---

### Human response format

Human input is converted into a **Fact**:

```text
HumanDecisionFact {
  job_id
  decision: Approved | Rejected | Refine
  rationale
  actor_id
  timestamp
}
```

This fact is the *only* way humans affect execution.

---

## Resuming Execution

When a human decision arrives:

1. The persisted job is loaded
2. The `HumanDecisionFact` is merged
3. Dirty keys are updated
4. The engine runs another cycle

Execution resumes exactly as if the fact had been produced internally.

---

## Determinism Guarantee

Determinism is preserved because:

- Waiting introduces no side effects
- Resumption is triggered by explicit data
- The same input produces the same next state

Time does not affect execution semantics.

---

## Humans and Convergence (Important Clarification)

### Are humans part of convergence?

**No.**

Humans:
- do not participate in fixed-point detection
- do not affect eligibility
- do not influence iteration order

They only:
- unblock acceptance
- add authoritative facts

Convergence remains an engine property.

---

## Humans as Authorities of Last Resort

This is common with LLM outputs.

If:
- a ProposedFact cannot be deterministically validated

Then:
- human approval is required
- promotion to Fact happens explicitly

Again:
- data in
- data out

---

## What Humans Are Explicitly Forbidden To Do

To preserve correctness:

Humans must NOT:
- mutate context directly
- trigger agents manually
- resume execution implicitly
- bypass Gherkin invariants
- act without leaving a trace

If an action cannot be represented as a Fact, it is not allowed.

---

## Relationship to Queues and Workflows

Human waiting does **not** require:
- queues
- Pub/Sub
- workflow engines

Waiting is a **state**, not a process.

This keeps Converge from becoming a durable workflow system.

---

## Summary

- Humans never participate in convergence
- Waiting halts execution cleanly
- State is explicitly persisted
- SurrealDB is a natural persistence choice
- Resumption is deterministic and data-driven
- Humans act as authorities, not agents

---

## One-Sentence Takeaway

> **In Converge, humans pause the system as authorities and resume it with data — they never join the convergence loop itself.**
