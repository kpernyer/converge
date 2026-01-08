# Converge — Reference Architectures

This document shows **how Converge fits into real systems**,
rather than replacing everything.

---

## 1. Converge + Temporal

### When to use
- Temporal runs business processes
- Converge makes decisions inside those processes

### Architecture

```
Temporal Workflow
  |
  | calls
  v
Converge Job
  |
  | returns decision
  v
Temporal continues execution
```

Temporal handles:
- durability
- retries
- side effects

Converge handles:
- reasoning
- constraints
- convergence

---

## 2. Converge + MCP

### When to use
- MCP standardizes access to models and tools
- Converge governs execution

### Architecture

```
LLMs / Tools (MCP)
        |
        v
Converge Runtime
        |
        v
Decisions & Artifacts
```

MCP is the **tool boundary**, not the control plane.

---

## 3. Converge + Human-in-the-Loop

### When to use
- strategic or high-risk decisions
- regulatory environments

### Architecture

```
Human sets Root Intent
        |
        v
Converge explores & converges
        |
        v
Human reviews / approves
```

Humans:
- set scope
- approve outcomes
- never micromanage agents

---

## 4. Converge as a Decision Microservice

### When to use
- many small independent decisions
- API-driven systems

### Architecture

```
Client
  |
  v
Converge API
  |
  v
Job-specific runtime
```

Each request spawns a bounded decision runtime.

---

## 5. What Converge Never Is

Converge is never:
- the system of record
- the side-effect executor
- the message broker

It is a **decision engine**.

---

## Summary

Converge composes with existing systems.
It does not replace them.

---

## One-sentence takeaway

> Converge decides; other systems execute.
