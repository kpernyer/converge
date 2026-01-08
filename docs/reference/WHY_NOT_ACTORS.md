# Converge — Why Not Actors?

This document explains why **Converge intentionally does not use the Actor Model**,
despite superficial similarities (agents, messages, isolation).

This is not a critique of actors.
It is a statement of **fit-for-purpose**.

---

## 1. What the Actor Model Is Good At

Actor systems (Erlang/OTP, Akka, Orleans) excel at:

- Long-lived concurrent processes
- Fault-tolerant services
- High-throughput message handling
- Eventual consistency
- Availability under partial failure

They are ideal when:
- the system must never stop
- individual correctness is less important than uptime
- behavior can be emergent

---

## 2. What Converge Is Optimizing For

Converge optimizes for:

- Correctness over availability
- Deterministic behavior
- Explainability and auditability
- Bounded reasoning
- Semantic convergence

Converge jobs:
- start
- reason
- converge
- stop

They are not services.
They are **decisions**.

---

## 3. The Core Mismatch

| Actor Model | Converge |
|------------|----------|
| Message passing | Shared, typed context |
| Eventual consistency | Strong consistency |
| Emergent behavior | Enforced invariants |
| Process autonomy | Engine-controlled execution |
| Availability first | Correctness first |

Actors assume:
> “If the system keeps running, things will probably be fine.”

Converge assumes:
> “If the system converged, we can explain why.”

---

## 4. Why Actor Isolation Breaks Convergence

Convergence requires:

- global visibility of state
- deterministic ordering
- absence of hidden messages
- explicit termination

Actor systems make these difficult because:
- messages may be in flight
- ordering is nondeterministic
- termination is implicit

This makes fixed-point detection unreliable.

---

## 5. Why Supervision Trees Don’t Apply

Supervision trees exist to:
- restart failed processes
- preserve service availability

Converge does not preserve:
- agent processes
- partial state
- hidden progress

If correctness is compromised, the job degrades or fails.
It is never silently repaired.

---

## 6. Summary

Actors are excellent for *systems that must run forever*.

Converge is designed for *decisions that must be right*.

---

## One-sentence takeaway

> Actor systems keep the world running.  
> Converge decides what the world should do next.
