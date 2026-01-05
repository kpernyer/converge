# Converge — Convergence Semantics

This document addresses the **core concern of convergence** in Converge.
It makes the convergence model precise enough to satisfy systems engineers,
while remaining implementable and explainable.

Convergence is the *defining property* of Converge.

---

## Framing: Convergence is Engine-Controlled, Not Emergent

Before answering specific questions, this must be explicit:

> **Convergence in Converge is not emergent behavior.**
> It is an explicit property enforced by the engine.

Agents do not decide when to stop.
The engine does.

---

## Q5.1: Agent Eligibility — How Are Agents Selected?

### Question recap
- What makes an agent eligible?
- Polling vs subscription?
- How does this scale?

### Answer

### 1. Eligibility is data-driven, not blind polling

Agents declare **eligibility predicates** over context.

Conceptually:

```rust
trait Agent {
    fn accepts(&self, ctx: &ContextView) -> bool;
}
```

However, the engine does **not** naively poll all agents every cycle.

---

### 2. Context keys & dependency indexing

Each agent also declares **which parts of context it depends on**:

```rust
trait Agent {
    fn dependencies(&self) -> &'static [ContextKey];
}
```

Examples:
- `Signals`
- `Competitors`
- `Hypotheses`
- `Constraints`

The engine maintains an **index**:

```
ContextKey → Agents interested in this key
```

When context changes:
- only agents whose dependencies changed are reconsidered

This is closer to:
- incremental computation
- rule engines
- dataflow systems

Not brute-force polling.

---

### 3. Eligibility pipeline

Eligibility proceeds as:

1. Context changes in merge phase
2. Changed keys are identified
3. Dependent agents are enqueued
4. `accepts()` is evaluated only for those agents

This scales linearly with *change*, not with *agent count*.

---

## Q5.2: Termination Guarantees — Why Infinite Loops Cannot Occur

### Question recap
- What prevents Agent A ↔ Agent B loops?
- Is there cycle detection?
- Are budgets involved?

### Answer

Termination is guaranteed by **three independent mechanisms**.

---

### 1. Monotonicity of context

Context evolution is **monotonic in meaning**.

Rules:
- facts are only added, never retracted
- refinements increase precision or confidence
- invalidations are explicit facts, not deletions

This prevents oscillation.

You cannot “undo” a fact silently.

---

### 2. Bounded fact space (critical)

Every Root Intent defines **finite domains**:

- finite segment list
- finite channel list
- finite time windows
- finite resource sets

Agents are forbidden from inventing unbounded new symbols.

This makes the state space finite.

---

### 3. Budgets (hard stop)

Each job has explicit budgets:

- max cycles
- max facts
- max delegations
- max wall-clock time

If any budget is exceeded:
- execution halts
- result is marked partial
- no false convergence is reported

Budgets are **not a fallback** — they are part of the correctness model.

---

### 4. Why cycle detection is unnecessary

Classic cycle detection is needed when:
- state can oscillate
- transitions are reversible

Neither is true in Converge.

The system progresses toward saturation of a finite space.

---

## Q5.3: Progressive Convergence — When Is “Good Enough”?

### Question recap
- Who decides early vs primary vs extended?
- What triggers transitions?
- Is this time-based or quality-based?

### Answer

### 1. Progressive convergence is RootIntent-driven

The Root Intent declares **convergence tiers** explicitly.

Example (conceptual):

```rust
ConvergencePolicy {
    early: Criteria { min_strategies: 2, confidence: 0.5 },
    primary: Criteria { min_strategies: 3, confidence: 0.75 },
    extended: Criteria { exhaustiveness: true },
}
```

“Good enough” is *declared*, not guessed.

---

### 2. Engine evaluates convergence criteria

At the end of each cycle, the engine evaluates:

- success criteria
- confidence thresholds
- remaining eligible agents
- marginal utility of further work

If criteria for a tier are met:
- that tier is declared converged
- results are emitted

---

### 3. Who decides?

- **Root Intent** sets the bar
- **Engine** evaluates progress
- **Agents** never decide convergence

Humans may:
- accept early convergence
- request deeper convergence

But the engine enforces rules.

---

### 4. Exploratory / background work

Exploratory convergence is allowed when:

- primary criteria are met
- optional agents remain eligible
- budgets allow continued execution

Exploratory work:
- cannot invalidate earlier tiers
- may only add refinements or alternatives

---

## Putting It All Together

Convergence in Converge is:

- data-driven (eligibility indexing)
- bounded (finite domains + budgets)
- monotonic (no oscillation)
- explicit (RootIntent criteria)
- enforced (engine-owned)

---

## Summary

- Agents are triggered by **context changes**, not polling
- Eligibility scales with data changes
- Infinite loops are structurally impossible
- Convergence tiers are explicitly declared
- The engine — not agents — decides when to stop

---

## One-sentence takeaway

> Convergence in Converge is not a hope — it is a contract enforced by structure, bounds, and explicit policy.
