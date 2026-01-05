# Converge — Root Intent Schema

## Purpose

This document defines the **Root Intent schema** used by Converge.
The Root Intent is the *only* entry point into a Converge runtime.

It defines:
- the universe of discourse
- what is allowed to happen
- what success means

---

## Definition

A Root Intent is a **typed declaration**, not a prompt.

```rust
struct RootIntent {
    id: IntentId,
    kind: IntentKind,
    objective: Objective,
    scope: Scope,
    constraints: Constraints,
    success_criteria: SuccessCriteria,
    budgets: Budgets,
}
```

---

## Core Fields

### IntentKind
Defines the class of problem.

Examples:
- GrowthStrategy
- Scheduling
- ResourceOptimization

Used to:
- select eligible agents
- load domain constraints

---

### Objective
What the system is trying to improve.

Examples:
- IncreaseDemand
- MinimizeTime
- MaximizeFeasibility

---

### Scope
Defines what is in-bounds.

Examples:
- Market (Nordic B2B)
- Time window (Next week)
- Geography
- Product

Nothing outside the scope may appear in context.

---

### Constraints
Hard and soft limits.

Examples:
- Budget class
- Brand safety
- Regulatory limits

Violating hard constraints aborts convergence.

---

### SuccessCriteria
Defines when the job is considered successful.

Examples:
- At least one viable strategy exists
- A valid schedule is found
- All tasks are allocated

---

### Budgets
Limits exploration.

Examples:
- Max cycles
- Max agents
- Time limit

Budgets guarantee termination.

---

## Gherkin Mapping

Root Intent legitimacy is expressed via Gherkin.

```gherkin
Scenario: Valid root intent
  Given the intent scope is defined
  And success criteria are explicit
  Then the system may begin execution
```

---

## Summary

The Root Intent is the **constitution** of a Converge job.
Nothing may override it.
