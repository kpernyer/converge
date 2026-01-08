# Converge — Agent Lifecycle

## Purpose

This document defines the **lifecycle of an agent** in Converge.

Agents are:
- scheduled by the engine
- controlled by context
- bounded by invariants

---

## Lifecycle Phases

### 1. Registration
Agents are registered at runtime startup.

They declare:
- capabilities
- required context
- optional tools

---

### 2. Eligibility
An agent becomes eligible when:

```text
accepts(context) == true
```

Eligibility is pure and side-effect free.

---

### 3. Execution
Eligible agents execute.

They may:
- read context
- call tools
- compute results

They may not:
- mutate context directly
- call other agents

---

### 4. Emission
Agents emit **effects**:

- facts
- intents
- evaluations
- traces

Effects are buffered, not applied immediately.

---

### 5. Merge
The engine merges effects **serially** into context.

Conflicts are:
- detected
- resolved by rules or governance agents

---

### 6. Pruning
After merge:
- irrelevant agents are skipped
- dominated branches are dropped

---

### 7. Termination
The lifecycle ends when:
- no agents are eligible
- or budgets are exhausted
- or invariants fail

---

## Gherkin Assertion

```gherkin
Scenario: Agent behavior
  Then no agent mutates context directly
  And all agent effects are auditable
```

---

## Summary

Agents are **participants**, not controllers.
The engine owns execution.
