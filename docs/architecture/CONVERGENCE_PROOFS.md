# Converge — Convergence Guarantees (Informal)

This document explains *why* Converge converges, without formal proofs.

---

## 1. What Converge Means by Convergence

Convergence means:

Contextₙ₊₁ == Contextₙ

No new facts, intents, or delegations are produced.

---

## 2. Preconditions for Convergence

Convergence is guaranteed if:

1. Fact space is finite
2. Context evolution is monotonic
3. Agents are idempotent
4. Delegation depth is bounded
5. Budgets cap exploration

All Converge runtimes enforce these.

---

## 3. Why Infinite Loops Do Not Occur

- Agents cannot retract facts
- Agents cannot call each other
- Delegations are explicit and bounded
- No recursive execution without new information

This prevents oscillation.

---

## 4. Role of Pruning

Pruning removes:
- dominated branches
- irrelevant agents
- low-confidence paths

This strictly reduces future work.

---

## 5. LLMs and Convergence

LLMs may:
- enrich
- explain
- interpret

They may **not**:
- create unbounded novelty
- control execution
- bypass constraints

Thus, LLMs do not threaten convergence.

---

## 6. Summary

Converge converges because:
- the search space is bounded
- progress is monotonic
- execution is centrally controlled
