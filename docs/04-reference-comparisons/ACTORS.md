# Converge — Actors & Collaboration

This document shows the main actors in the engine and how they collaborate.

---

## The Actors

```
┌─────────────────────────────────────────────────────────────────────────┐
│                                                                         │
│   ┌─────────┐     ┌─────────┐     ┌─────────┐     ┌─────────┐          │
│   │  Agent  │     │  Agent  │     │  Agent  │     │  Agent  │          │
│   │  Seed   │     │  React  │     │ Strategy│     │   ...   │          │
│   └────┬────┘     └────┬────┘     └────┬────┘     └────┬────┘          │
│        │               │               │               │                │
│        └───────────────┴───────────────┴───────────────┘                │
│                                │                                        │
│                                │ register()                             │
│                                ▼                                        │
│                    ┌───────────────────────┐                            │
│                    │        ENGINE         │                            │
│                    │  ─────────────────    │                            │
│                    │  • Agent Registry     │                            │
│                    │  • Dependency Index   │                            │
│                    │  • Convergence Loop   │                            │
│                    └───────────┬───────────┘                            │
│                                │                                        │
│                                │ run()                                  │
│                                ▼                                        │
│                    ┌───────────────────────┐                            │
│                    │       CONTEXT         │                            │
│                    │  ─────────────────    │                            │
│                    │  • Facts by Key       │                            │
│                    │  • Dirty Keys         │                            │
│                    │  • Version            │                            │
│                    └───────────────────────┘                            │
│                                                                         │
└─────────────────────────────────────────────────────────────────────────┘
```

---

## Actor Responsibilities

### 1. Agent (Many)

**Role:** Semantic capability that observes and contributes.

```rust
trait Agent {
    fn name(&self) -> &str;
    fn dependencies(&self) -> &[ContextKey];  // What I care about
    fn accepts(&self, ctx: &Context) -> bool; // Should I run?
    fn execute(&self, ctx: &Context) -> AgentEffect; // Do work
}
```

**Constraints:**
- ❌ Cannot call other agents
- ❌ Cannot mutate context directly
- ❌ Cannot control when it runs
- ✅ Can only read context and emit effects

---

### 2. Engine (One)

**Role:** Coordinator that owns convergence.

```rust
struct Engine {
    agents: Vec<Box<dyn Agent>>,      // Registered agents
    index: HashMap<ContextKey, Vec<AgentId>>, // Dependency index
    always_eligible: Vec<AgentId>,    // Agents with no deps
    budget: Budget,                   // Termination limits
}
```

**Responsibilities:**
- Register agents, assign IDs
- Build dependency index
- Run convergence loop
- Merge effects (deterministic order)
- Enforce budgets

---

### 3. Context (One per job)

**Role:** The shared truth — append-only, monotonic.

```rust
struct Context {
    facts: HashMap<ContextKey, Vec<Fact>>,  // The knowledge
    dirty_keys: Vec<ContextKey>,            // What changed
    version: u64,                           // Monotonic counter
}
```

**Properties:**
- Only Engine can mutate (`&mut Context`)
- Agents only read (`&Context`)
- Changes are tracked via `dirty_keys`

---

### 4. AgentEffect (Transient)

**Role:** Buffered output from an agent.

```rust
struct AgentEffect {
    facts: Vec<Fact>,  // New facts to add
}
```

**Properties:**
- Immutable once created
- All-or-nothing (transactional)
- Merged serially by Engine

---

## Collaboration Flow

```
┌──────────────────────────────────────────────────────────────────────────┐
│                         CONVERGENCE LOOP                                 │
├──────────────────────────────────────────────────────────────────────────┤
│                                                                          │
│  ┌─────────────────────────────────────────────────────────────────┐     │
│  │ 1. ELIGIBILITY                                                  │     │
│  │                                                                 │     │
│  │    Engine                                                       │     │
│  │      │                                                          │     │
│  │      │  "Which agents should run?"                              │     │
│  │      │                                                          │     │
│  │      ├──► Check dirty_keys                                      │     │
│  │      │      │                                                   │     │
│  │      │      ▼                                                   │     │
│  │      │    Index lookup: dirty_keys → candidate agents           │     │
│  │      │      │                                                   │     │
│  │      │      ▼                                                   │     │
│  │      └──► For each candidate: agent.accepts(&context)?          │     │
│  │             │                                                   │     │
│  │             ▼                                                   │     │
│  │           eligible_agents: Vec<AgentId>                         │     │
│  │                                                                 │     │
│  └─────────────────────────────────────────────────────────────────┘     │
│                              │                                           │
│                              ▼                                           │
│  ┌─────────────────────────────────────────────────────────────────┐     │
│  │ 2. EXECUTION (Parallel Read)                                    │     │
│  │                                                                 │     │
│  │    For each eligible agent:                                     │     │
│  │                                                                 │     │
│  │      Agent ◄─── &Context (read-only)                            │     │
│  │        │                                                        │     │
│  │        │  agent.execute(&context)                               │     │
│  │        │                                                        │     │
│  │        ▼                                                        │     │
│  │      AgentEffect { facts: [...] }                               │     │
│  │                                                                 │     │
│  │    All agents see the SAME context snapshot.                    │     │
│  │    No agent sees another's output yet.                          │     │
│  │                                                                 │     │
│  └─────────────────────────────────────────────────────────────────┘     │
│                              │                                           │
│                              ▼                                           │
│  ┌─────────────────────────────────────────────────────────────────┐     │
│  │ 3. MERGE (Serial Commit)                                        │     │
│  │                                                                 │     │
│  │    Engine                                                       │     │
│  │      │                                                          │     │
│  │      │  Sort effects by AgentId (deterministic)                 │     │
│  │      │                                                          │     │
│  │      │  For each effect (in order):                             │     │
│  │      │    │                                                     │     │
│  │      │    ├──► context.add_fact(fact)                           │     │
│  │      │    │      │                                              │     │
│  │      │    │      ├── Check duplicate (by id)                    │     │
│  │      │    │      ├── Add to facts[key]                          │     │
│  │      │    │      ├── Track dirty_key                            │     │
│  │      │    │      └── Increment version                          │     │
│  │      │    │                                                     │     │
│  │      │    └──► Next effect                                      │     │
│  │      │                                                          │     │
│  │      ▼                                                          │     │
│  │    dirty_keys = [keys that changed this cycle]                  │     │
│  │                                                                 │     │
│  └─────────────────────────────────────────────────────────────────┘     │
│                              │                                           │
│                              ▼                                           │
│  ┌─────────────────────────────────────────────────────────────────┐     │
│  │ 4. CONVERGENCE CHECK                                            │     │
│  │                                                                 │     │
│  │    if dirty_keys.is_empty() {                                   │     │
│  │        // Nothing changed → CONVERGED                           │     │
│  │        return Ok(ConvergeResult { converged: true, ... })       │     │
│  │    }                                                            │     │
│  │                                                                 │     │
│  │    if cycles > budget.max_cycles {                              │     │
│  │        // Too many cycles → BUDGET EXHAUSTED                    │     │
│  │        return Err(BudgetExhausted)                              │     │
│  │    }                                                            │     │
│  │                                                                 │     │
│  │    // More work to do → LOOP AGAIN                              │     │
│  │                                                                 │     │
│  └─────────────────────────────────────────────────────────────────┘     │
│                              │                                           │
│                              │ (loop back to step 1)                     │
│                              ▼                                           │
└──────────────────────────────────────────────────────────────────────────┘
```

---

## Data Flow Example

```
Cycle 1:
─────────────────────────────────────────────────────────────
  dirty_keys: [] (empty, first cycle)

  Eligible: SeedAgent (no deps → always eligible)
            SeedAgent.accepts(ctx) → true (no seeds yet)

  Execute:  SeedAgent.execute(ctx) → Effect { facts: [Seed("s1")] }

  Merge:    context.add_fact(Seed("s1"))
            dirty_keys: [Seeds]

Cycle 2:
─────────────────────────────────────────────────────────────
  dirty_keys: [Seeds]

  Eligible: SeedAgent (no deps → always eligible)
            SeedAgent.accepts(ctx) → false (seed exists)

            ReactOnceAgent (deps: [Seeds] ∩ dirty_keys ✓)
            ReactOnceAgent.accepts(ctx) → true (seeds exist, no hyp)

  Execute:  ReactOnceAgent.execute(ctx) → Effect { facts: [Hyp("h1")] }

  Merge:    context.add_fact(Hyp("h1"))
            dirty_keys: [Hypotheses]

Cycle 3:
─────────────────────────────────────────────────────────────
  dirty_keys: [Hypotheses]

  Eligible: SeedAgent (no deps → always eligible)
            SeedAgent.accepts(ctx) → false

            ReactOnceAgent (deps: [Seeds] ∩ [Hypotheses] = ∅)
            NOT ELIGIBLE (deps don't intersect dirty)

  Execute:  (no eligible agents)

  Result:   CONVERGED in 3 cycles
```

---

## Key Invariants

| Invariant | Enforced By |
|-----------|-------------|
| Agents never call each other | Type system: no agent handle in `execute()` |
| Agents can't mutate context | Type system: `&Context` not `&mut Context` |
| Merge order is deterministic | Engine sorts by `AgentId` |
| Convergence is detectable | `dirty_keys.is_empty()` |
| Termination is guaranteed | Budget limits (cycles, facts) |

---

## Actor Communication Summary

```
┌─────────┐                    ┌─────────┐
│  Agent  │ ──── registers ──► │ Engine  │
└─────────┘                    └────┬────┘
                                    │
                                    │ builds
                                    ▼
                            ┌──────────────┐
                            │   Index      │
                            │ Key→Agents   │
                            └──────────────┘

┌─────────┐                    ┌─────────┐
│  Agent  │ ◄── &Context ───── │ Engine  │
└────┬────┘                    └────┬────┘
     │                              │
     │ returns                      │
     ▼                              │
┌─────────┐                         │
│ Effect  │ ─────────────────────►  │
└─────────┘        collected        │
                                    │
                                    │ merges into
                                    ▼
                            ┌──────────────┐
                            │   Context    │
                            └──────────────┘
```

**Communication is unidirectional:**
- Agents → Engine (via Effects)
- Engine → Agents (via &Context)
- Agents ↮ Agents (never)
