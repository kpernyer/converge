# Converge — Gherkin Model & Invariant Semantics

This document clarifies the **real role of Gherkin in Converge**.

Gherkin in Converge is **not primarily a testing language**.
It is a **semantic constraint language** used to declare correctness,
legitimacy, and acceptance conditions for a decision runtime.

---

## Framing: Gherkin Is Not Control Flow

Before addressing specifics, this must be explicit:

> **Gherkin does not drive execution in Converge.**
> It constrains execution.

Agents do not “follow” Gherkin scenarios.
The engine **enforces** them.

---

## Q6.1: When Are Invariants Checked?

### Question recap
- Every cycle?
- Only at convergence?
- Continuously?

### Answer

Converge supports **three classes of invariant checks**.

---

### 1. Structural invariants (continuous)

Structural invariants are checked **on every merge**.

Examples:
- Context schema validity
- Type correctness
- Scope violations
- Forbidden fact combinations

These are:
- cheap to check
- deterministic
- non-negotiable

Violation causes **immediate failure**.

---

### 2. Semantic invariants (per-cycle)

Semantic invariants are checked:
- at the end of each execution cycle
- after effects are merged

Examples:
- “No strategy violates brand safety”
- “All scheduled meetings respect working hours”

These invariants:
- may depend on multiple facts
- are domain-specific

Violations:
- block convergence
- mark the current state invalid

Execution may continue **only** if recovery is possible.

---

### 3. Acceptance invariants (convergence gates)

Acceptance invariants are checked:
- when the engine believes convergence has been reached
- or when a convergence tier is claimed

Examples:
- “At least two viable strategies exist”
- “A feasible schedule exists”

These invariants:
- decide whether results may be emitted
- are the final authority on correctness

---

## Q6.2: What Happens on Violation?

### Question recap
- Halt?
- Emit facts?
- Compensate?

### Answer

Violation handling depends on **invariant class**.

---

### 1. Structural invariant violation

Behavior:
- execution halts immediately
- job is marked failed
- trace records the violation

No recovery is attempted.

---

### 2. Semantic invariant violation

Behavior:
- violation is recorded as a **governance fact**
- convergence is blocked
- relevant agents may attempt remediation

Example:
- “Campaign violates brand safety”
- Governance agent prunes the campaign

If remediation succeeds, execution continues.

---

### 3. Acceptance invariant violation

Behavior:
- convergence is rejected
- results are not emitted
- job may:
  - continue exploring
  - downgrade convergence tier
  - fail explicitly

No invalid result is ever emitted.

---

### 4. No automatic compensation

Converge does **not** perform mechanical rollback.

Compensation is:
- semantic
- explicit
- represented as new facts or invalidations

---

## Q6.3: How Are Gherkin Specs Executed?

### Question recap
- Interpreted?
- Compiled?
- External process?

### Answer

### 1. Gherkin is compiled, not interpreted

Gherkin specs are:
- parsed at build time or startup
- compiled into **typed invariant predicates**

There is no runtime string interpretation.

---

### 2. Compilation target

Each scenario becomes a Rust function:

```rust
fn invariant(ctx: &Context) -> InvariantResult;
```

Where `InvariantResult` may be:
- Ok
- Violated { reason, severity }

---

### 3. Execution location

Invariant evaluation occurs:
- inside the engine
- synchronously
- with full access to typed context

No external validator process is required.

---

### 4. Why not external validation?

External validators:
- break determinism
- complicate deployment
- fragment observability

Converge keeps invariants **inside the runtime**.

---

## How Gherkin Differs from BDD Testing

| BDD Testing | Converge Gherkin |
|-----------|-----------------|
| Test-time | Runtime |
| Verifies behavior | Enforces correctness |
| After-the-fact | Always-on |
| Linear scenarios | Global invariants |
| Drives confidence | Gates legitimacy |

---

## What Gherkin Is Best At

Gherkin excels at:
- expressing business intent
- defining “must never happen”
- defining “must be true before accepting results”
- bridging domain experts and engineers

This is why Converge uses it.

---

## Summary

- Gherkin expresses invariants, not workflows
- Invariants are checked continuously, per-cycle, and at convergence
- Violations are handled by severity
- Specs are compiled into Rust predicates
- Gherkin is part of the engine, not a test harness

---

## One-sentence takeaway

> In Converge, Gherkin is not a test — it is the law the runtime must obey.
