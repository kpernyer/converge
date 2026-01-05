# Converge — When to Use (and Not Use) Converge

This document helps architects and engineers decide **when Converge is the right tool**,
and when other systems are a better fit.

Converge is deliberately opinionated.

---

## Use Converge When

### 1. You are making a *decision*, not running a process

Converge is ideal when the system’s job is to:
- explore a decision space
- weigh tradeoffs
- enforce constraints
- converge on a justified outcome

Examples:
- growth or go-to-market strategy
- planning under uncertainty
- multi-constraint scheduling
- recommendation with rationale

---

### 2. Correctness matters more than availability

Use Converge when:
- a wrong answer is worse than no answer
- partial answers are acceptable if clearly labeled
- explainability is required

---

### 3. You need bounded reasoning

Converge excels when:
- the search space must be controlled
- exploration must terminate
- infinite loops are unacceptable

---

### 4. You want LLMs *inside* a governed system

Use Converge if:
- LLMs add value (interpretation, synthesis)
- but must not control execution
- and must not violate invariants

---

## Do NOT Use Converge When

### 1. You need a long-lived durable workflow

If you need:
- months-long execution
- automatic replay
- exactly-once side effects

Use **Temporal**.

---

### 2. You need a highly available reactive service

If you need:
- always-on services
- high-throughput message handling
- fault-tolerant concurrency

Use **Erlang/OTP**, **Akka**, or **Orleans**.

---

### 3. You need simple linear orchestration

If your logic is:
- step-by-step
- well-defined
- short-lived

Use:
- plain code
- a workflow engine
- a task queue

---

### 4. You want emergent agent behavior

If you want:
- agents talking freely
- message passing
- emergent coordination

Converge is the *wrong* tool by design.

---

## Comparison Summary

| Need | Use |
|----|-----|
| Correct decision-making | Converge |
| Durable workflows | Temporal |
| High availability services | OTP |
| LLM chaining | LangChain |
| Graph-based prompting | LangGraph |

---

## One-sentence rule

> If you can’t clearly state what convergence means, don’t use Converge.
