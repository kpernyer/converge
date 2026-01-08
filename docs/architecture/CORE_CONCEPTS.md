# Converge Core — Core Concepts

This document explains the essential concepts of Converge without exposing implementation details.

## Context-Driven Architecture

In Converge, **context is the API**. Agents don't call each other—they read from and write to a shared context. This design ensures:

- **Decoupling**: Agents don't need to know about each other
- **Parallelism**: Agents can execute concurrently
- **Auditability**: All communication is visible in context
- **Determinism**: Same context state produces same results

## Convergence

Convergence is the property that execution halts when the context reaches a stable state. A cycle is considered converged when no new facts are added to the context.

**Why it matters:**
- Guarantees termination (with budgets)
- Makes outcomes explainable
- Enables reproducible execution

## Agent Model

Agents are stateless capabilities that:
1. Read from context
2. Produce effects (facts)
3. Never call other agents
4. Never mutate context directly

**Agent Types:**
- Deterministic agents — Pure functions
- LLM-backed agents — Use LLMs as tools, outputs require validation
- Solver agents — Use external solvers (e.g., constraint solvers)
- IO agents — Interact with external systems

## Fact Model

Facts are typed, immutable pieces of information added to context.

**Properties:**
- Typed — Each fact has a known type
- Provenanced — Every fact knows its source
- Validated — Facts are validated before being added
- Monotonic — Facts are never retracted (invalidations are explicit facts)

## Invariants

Invariants are business rules expressed as predicates. They constrain what facts can exist in context.

**Invariant Classes:**
- Structural — Checked on every merge
- Semantic — Checked at end of each cycle
- Acceptance — Checked when convergence is claimed

## Budgets

Budgets prevent infinite execution:

- **Max cycles** — Maximum number of execution cycles
- **Max facts** — Maximum number of facts in context
- **Max time** — Maximum execution time

When a budget is exceeded, execution terminates with an error.

## LLM Integration

LLMs are treated as tools, not authorities. Their outputs are:

1. Captured as `ProposedFact` (not `Fact`)
2. Validated deterministically
3. Promoted to `Fact` only if validation passes

This ensures LLM outputs cannot:
- Override constraints
- Create invalid states
- Bypass validation

