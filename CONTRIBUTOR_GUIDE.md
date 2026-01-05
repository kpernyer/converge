# Contributor Guide — Converge
## How to Contribute Without Breaking the Semantics

Welcome, and thank you for your interest in contributing to Converge.

Converge is an open-source project, but it is **not** open-ended.
It has a strong semantic core that must remain intact.

This guide explains how to contribute *productively*.

---

## 1. Before You Contribute

You should understand these documents first:

- ARCHITECTURE.md
- DESIGN_TENETS.md
- TECHNOLOGY_STACK.md
- FAQ_CONVERGE_DESIGN.md

If a proposed change conflicts with these, it will not be merged.

---

## 2. What Contributions Are Welcome

### Highly Encouraged
- Deterministic agents
- Domain capability packages
- Gherkin invariants
- Validators and promotion logic
- Integration examples
- Documentation improvements
- Tests (unit, integration, invariants)

### Conditionally Accepted
- Performance optimizations (must preserve determinism)
- Persistence tooling
- Observability tooling (derived from decisions, not execution)

### Generally Rejected
- Message buses
- Workflow engines
- Actor frameworks
- Background execution
- Eventual consistency mechanisms

---

## 3. Core Contribution Rules

### Rule 1: Do Not Introduce Hidden Control Flow
All execution must be:
- explicit
- engine-driven
- inspectable

### Rule 2: Do Not Dilute Authority
- Agents suggest
- The engine decides
- Humans approve

### Rule 3: Preserve Determinism
If a change makes outcomes non-reproducible, it is unacceptable.

### Rule 4: Prefer Types Over Conventions
If correctness relies on comments or discipline, redesign the API.

---

## 4. Code Style & Testing

- Follow rust.md style guide
- Add tests for every semantic change
- Prefer integration tests that demonstrate convergence

---

## 5. Review Philosophy

PRs are evaluated on:
- semantic clarity
- correctness by construction
- alignment with convergence principles

Performance and features come second.

---

## 6. Final Reminder

> Converge is a semantic engine for alignment, not a playground for autonomy.

Thank you for respecting that.
