# Converge — Terminology

This document defines normative terms used in Converge.

---

## Agent
A semantic capability that observes context and emits facts or intents.

---

## Agent OS
A runtime that manages context, schedules agents, and guarantees convergence.

---

## Context
The shared, typed, evolving representation of a job.

---

## Root Intent
The typed declaration that defines scope, constraints, and success.

---

## Convergence
The fixed point where further execution produces no new context changes.

---

## Execution Graph
The implicit, data-driven graph formed by agent activations.

---

## Fact
A typed assertion added to context.

---

## Intent
A declaration of desired exploration or change within a job.

---

## Delegation
The act of emitting a sub-intent into context.

---

## Aggregation
Explicit synthesis of multiple facts into a higher-level outcome.

---

## Deterministic Agent
An agent with fully predictable behavior.

---

## LLM Agent
An agent that uses a language model as an internal tool.

---

## Solver Agent
An agent that uses deterministic optimization techniques.

---

## Governance Agent
An agent responsible for enforcing safety or correctness.

---

## Knowledge
Curated, long-lived information extracted from completed jobs.

---

## Tool
An external capability invoked by agents via MCP.

---

## Trace
An auditable log of agent execution and decisions.

---

## Fixed Point
The formal condition defining convergence.

---

## Universe of Discourse
The total set of allowable facts and intents defined by a Root Intent.
