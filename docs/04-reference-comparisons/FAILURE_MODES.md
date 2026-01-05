# Converge — Failure Modes & Recovery

This document describes how Converge behaves under failure conditions.
The design principle is simple:

> Converge prefers **no answer or partial answer** over an **incorrect answer**.

---

## 1. Agent Failures

### 1.1 Agent panic or exception
- The agent execution is aborted
- No partial writes are applied to context
- Failure is recorded in the trace
- Other agents may still execute

Agent failures do **not** crash the runtime.

---

### 1.2 Agent timeout
- Agent execution is canceled
- Context remains unchanged
- Agent may be retried depending on policy

Timeouts are treated as *absence of contribution*, not errors.

---

## 2. Tool Failures (LLMs, APIs, Solvers)

External tools are assumed unreliable.

### Behavior:
- Timeouts, retries, circuit breakers
- Tool failure produces a structured failure fact
- Dependent agents may degrade or skip

No tool failure may corrupt context.

---

## 3. Invariant Violations

If a Gherkin invariant is violated:
- Convergence is aborted
- Result is marked invalid
- Full trace is preserved for inspection

Invariant violations are **hard failures**.

---

## 4. Runtime Failures

### 4.1 Process crash
- Job fails
- Context snapshot may be persisted
- Job can be retried from Root Intent

### 4.2 Resource exhaustion
- Execution halts
- Partial results may be emitted
- Failure reason is explicit

---

## 5. Recovery Philosophy

Converge does not attempt:
- transparent replay
- hidden retries
- silent correction

Recovery is:
- explicit
- observable
- auditable

---

## 6. Summary

Failures never produce false correctness.
They either:
- reduce confidence
- reduce completeness
- or stop execution
