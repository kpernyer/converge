# Convergence Proof Tests

This document describes the test suite that proves the correctness of the Converge engine's execution model. These tests are not just "passing tests" — they are **axiom proof harnesses**: mathematical proofs that would fail catastrophically if any of the core invariants were violated.

## Why This Matters

The distinction between "integration test" and "axiom proof" is important:

- **Integration tests** assert behavior
- **Axiom proofs** demonstrate invariants under execution

The proof tests in this suite are structured as:

1. **Formal statement** — the axiom being proven
2. **Concrete execution** — agents running against real context
3. **Observed evidence** — trace output showing what happened
4. **Explicit verification** — assertions that prove the axiom held
5. **Falsification attempt** — negative tests that would break if the axiom failed

This structure follows how operating system and database correctness papers are written.

**These tests serve as:**

- A **living specification** of engine semantics
- A **non-negotiable regression barrier**
- A **design contract** — if someone asks "why does the engine work this way?", point here

## Quick Reference

| Test File | What It Proves | Run Command |
|-----------|----------------|-------------|
| `engine_convergence_axioms.rs` | All 5 core axioms (step-by-step) | `cargo test --test engine_convergence_axioms -- --nocapture` |
| `hitl_pause_resume_axioms.rs` | HITL pause/resume (Axioms 6-7) | `cargo test --test hitl_pause_resume_axioms -- --nocapture` |
| `snapshot_resume_axioms.rs` | Snapshot/recovery (Axioms 8-9) | `cargo test --test snapshot_resume_axioms -- --nocapture` |
| `engine_convergence_bones.rs` | Core convergence contract | `cargo test --test engine_convergence_bones -- --nocapture` |
| `no_starvation.rs` | Multi-precondition agents | `cargo test --test no_starvation` |
| `property_tests.rs` | Random input invariants | `cargo test --test property_tests` |
| `convergence.rs` | Sequential agent execution | `cargo test --test convergence` |
| `proposal_promotion.rs` | LLM output validation | `cargo test --test proposal_promotion` |
| `root_intent_anchor.rs` | Semantic compliance/RootIntent | `cargo test --test root_intent_anchor` |

---

## The Nine Axioms

The engine is built on nine non-negotiable axioms. All proof tests verify these:

### AXIOM 1: Context is the ONLY Shared State

> No agent has hidden lifecycle state (`has_run`, counters, flags, caches).
> Agents are pure functions: `Context → AgentEffect`

**Proof**: The restart safety test in `engine_convergence_axioms.rs` runs the engine twice on the same context. If any agent had hidden state, it would run again on the second pass.

### AXIOM 2: Eligibility is Data-Driven

> Agents are reconsidered when their declared dependencies become dirty.
> No polling, no pending queues, no background re-check loops.

**Proof**: `engine_convergence_bones.rs` demonstrates that DeltaAgent (which needs Seeds, Hypotheses, AND Signals) correctly waits until all three exist, even though Seeds becomes dirty first.

### AXIOM 3: Idempotency via Context

> "Has this agent already contributed?" must be derivable from facts in context.
> Pattern: check output key for existing contribution with agent's ID prefix.

**Proof**: Every agent in the proof tests checks its output key before running. The restart safety test proves this works across engine invocations.

### AXIOM 4: Dependency Completeness

> Any `ContextKey` read in `accepts()` or `execute()` must be declared in `dependencies()`.
> Violation causes **starvation**: agent misses execution opportunities.

**Proof**: `BrokenAgent` in `engine_convergence_axioms.rs` reads Evaluations but doesn't declare it. It is intentionally starved, proving the contract.

**Why this matters**: Most frameworks try to *prevent* starvation. Converge does something better: it makes starvation **predictable, explainable, and contractual**. If you violate the dependency contract, you get starved — and the test proves exactly why. This is the correct move for a semantic engine.

### AXIOM 5: Deterministic Convergence

> Same context + same agents = same result.
> No implicit engine memory, no hidden scheduling heuristics.

**Proof**: `convergence_is_order_independent` in `engine_convergence_bones.rs` registers agents in different orders and verifies identical final contexts.

### AXIOM 6: Pause/Resume Safety

> Engine execution can be paused at any cycle boundary, serialized, and resumed without affecting correctness.
> Context is the complete state — no hidden engine state needs to be saved.

**Proof**: `hitl_pause_resume_axioms.rs` serializes context mid-execution, deserializes it, adds a human decision, and resumes. The final result matches straight-through execution with the same decision pre-provided.

### AXIOM 7: Human Decision Integration

> Human decisions are facts like any other. They do not bypass the agent contract.
> They simply provide data that unblocks waiting agents.

**Proof**: `ApprovalGatedAgent` in `hitl_pause_resume_axioms.rs` waits for an `approval:granted` fact. When a human adds this fact during pause, the agent becomes eligible and runs. No special HITL machinery is needed — the engine doesn't know or care where facts come from.

**Why this matters**: This means HITL workflows are not a special case. The same convergence guarantees that apply to automated agents apply to human-in-the-loop scenarios. Humans are just another source of facts.

### AXIOM 8: Snapshot Completeness

> Context is the complete state needed to resume execution.
> No hidden engine state, no external dependencies, no implicit ordering.
> A snapshot taken at cycle N contains everything needed to continue from cycle N+1.

**Proof**: `snapshot_resume_axioms.rs` serializes context mid-execution, loads it on a fresh engine, and verifies the final result matches straight-through execution.

### AXIOM 9: Cross-Instance Determinism

> Resuming from a snapshot on a DIFFERENT engine instance produces identical results to continuing on the original instance.
> The engine has no identity — only context matters.

**Proof**: `snapshot_resume_axioms.rs` runs Engine A → snapshot → Engine B → Engine C (baseline). All final states are identical, proving the engine is stateless.

**Why this matters**: This enables crash recovery, horizontal scaling, and distributed execution. You can save context to any storage, restore it anywhere, and get the same result. The engine is truly stateless.

---

## Test Details

### `engine_convergence_axioms.rs` — The Correctness Contract

**Purpose**: Visual, educational proof of all five axioms with detailed trace output. This is the **keystone test** — it proves the engine's semantic guarantees are real.

**Run**:

```bash
cargo test -p converge-core --test engine_convergence_axioms -- --nocapture
```

**Why "convergence axioms"?** Because what this test verifies is not implementation details — it's **semantic law**. The five axioms are the load-bearing invariants of the entire system. Everything else (performance, parallelism, distribution) is derivative.

<details>
<summary><b>Full Output (click to expand)</b></summary>

```
================================================================================
  CONVERGE ENGINE AXIOM PROOF - VERBOSE STEP-BY-STEP EXECUTION
================================================================================

  This test proves the following axioms:

  AXIOM 1: Context is the ONLY shared state
           → No agent has hidden lifecycle state (has_run, counters, etc.)

  AXIOM 2: Eligibility is data-driven
           → Agents are reconsidered when declared dependencies become dirty

  AXIOM 3: Idempotency via context
           → "Has this agent contributed?" is derived from context facts

  AXIOM 4: Dependency completeness
           → Keys read in accepts()/execute() MUST be declared in dependencies()
           → Violation causes STARVATION (agent never runs)

  AXIOM 5: Deterministic convergence
           → Same context + same agents = same result

================================================================================

┌─────────────────────────────────────────────────────────────────────────────┐
│ PHASE 1: AGENT REGISTRATION                                                 │
└─────────────────────────────────────────────────────────────────────────────┘
   [0] SeedProvider     → Seeds
   [1] AlphaAgent       → Seeds → Hypotheses
   [2] BetaAgent        → Hypotheses → Signals
   [3] GammaAgent       → Signals → Strategies
   [4] DeltaAgent       → Seeds+Hypotheses+Signals → Evaluations (MULTI-PRECONDITION)
   [5] BrokenAgent      → Evaluations → Competitors (UNDECLARED DEP - WILL STARVE)

   Dependency Graph:
   ┌────────────┐
   │ SeedProvider │
   └──────┬─────┘
          │ Seeds
          ▼
   ┌────────────┐
   │ AlphaAgent │
   └──────┬─────┘
          │ Hypotheses
          ▼
   ┌────────────┐      ┌────────────┐
   │ BetaAgent  │      │ DeltaAgent │ (waits for Seeds+Hypotheses+Signals)
   └──────┬─────┘      └──────┬─────┘
          │ Signals           │ Evaluations
          ▼                   ▼
   ┌────────────┐      ┌────────────┐
   │ GammaAgent │      │ BrokenAgent│ (STARVED - undeclared dep on Evaluations)
   └──────┬─────┘      └────────────┘
          │ Strategies
          ▼

┌─────────────────────────────────────────────────────────────────────────────┐
│ PHASE 2: CONVERGENCE LOOP EXECUTION                                         │
└─────────────────────────────────────────────────────────────────────────────┘

   Legend:
     accepts(✓) = agent is eligible to run
     accepts(✗) = agent is NOT eligible (preconditions not met or already ran)
     EXECUTING  = agent is producing facts
     deps=[...]  = current state of agent's declared dependencies


╔══════════════════════════════════════════════════════════════════════════════╗
║                    CONVERGE ENGINE - STEP BY STEP                            ║
╚══════════════════════════════════════════════════════════════════════════════╝

┌─────────────────────────────────────────────────────────────────────────────┐
│ INITIAL STATE                                                               │
└─────────────────────────────────────────────────────────────────────────────┘
   Version: 0
   Dirty Keys: []
      [SeedProvider        ] accepts(✓) deps=[]
      [SeedProvider        ] EXECUTING...
      [SeedProvider        ]   → Seeds:market-data = "Nordic B2B SaaS market analysis input"
      [AlphaAgent          ] accepts(✓) deps=[Seeds=1, Hypotheses=∅]
      [DeltaAgent          ] accepts(✗) deps=[Seeds=1, Hypotheses=∅, Signals=∅, Evaluations=∅]
      [SeedProvider        ] accepts(✗) deps=[]
      [AlphaAgent          ] EXECUTING...
      [AlphaAgent          ]   → Hypotheses:alpha-hypothesis = "Initial hypothesis from Alpha based on seeds"
      [BetaAgent           ] accepts(✓) deps=[Hypotheses=1, Signals=∅]
      [DeltaAgent          ] accepts(✗) deps=[Seeds=1, Hypotheses=1, Signals=∅, Evaluations=∅]
      [AlphaAgent          ] accepts(✗) deps=[Seeds=1, Hypotheses=1]
      [SeedProvider        ] accepts(✗) deps=[]
      [BetaAgent           ] EXECUTING...
      [BetaAgent           ]   → Signals:beta-signal = "Signal derived from hypothesis by Beta"
      [BetaAgent           ] accepts(✗) deps=[Hypotheses=1, Signals=1]
      [GammaAgent          ] accepts(✓) deps=[Signals=1, Strategies=∅]
      [DeltaAgent          ] accepts(✓) deps=[Seeds=1, Hypotheses=1, Signals=1, Evaluations=∅]
      [SeedProvider        ] accepts(✗) deps=[]
      [GammaAgent          ] EXECUTING...
      [GammaAgent          ]   → Strategies:gamma-strategy = "Strategy synthesized from signals by Gamma"
      [DeltaAgent          ] EXECUTING...
      [DeltaAgent          ]   → Evaluations:delta-evaluation = "Comprehensive evaluation from Delta (needs seeds+h..."
      [SeedProvider        ] accepts(✗) deps=[]
      [DeltaAgent          ] accepts(✗) deps=[Seeds=1, Hypotheses=1, Signals=1, Evaluations=1]
      [GammaAgent          ] accepts(✗) deps=[Signals=1, Strategies=1]

┌─────────────────────────────────────────────────────────────────────────────┐
│ FINAL STATE                                                                 │
└─────────────────────────────────────────────────────────────────────────────┘
   Version: 5
   Dirty Keys: [Strategies, Evaluations]
   Seeds:
      • market-data = "Nordic B2B SaaS market analysis input"
   Hypotheses:
      • alpha-hypothesis = "Initial hypothesis from Alpha based on seeds"
   Signals:
      • beta-signal = "Signal derived from hypothesis by Beta"
   Strategies:
      • gamma-strategy = "Strategy synthesized from signals by Gamma"
   Evaluations:
      • delta-evaluation = "Comprehensive evaluation from Delta (needs seeds+hypotheses+..."

┌─────────────────────────────────────────────────────────────────────────────┐
│ CONVERGENCE SUMMARY                                                         │
└─────────────────────────────────────────────────────────────────────────────┘
   Converged: true
   Cycles: 5
   Context Version: 5

┌─────────────────────────────────────────────────────────────────────────────┐
│ PHASE 3: AXIOM VERIFICATION                                                 │
└─────────────────────────────────────────────────────────────────────────────┘
   ✓ AXIOM 5: Engine converged deterministically
   ✓ AXIOM 3: AlphaAgent ran exactly once (idempotency via context)
   ✓ AXIOM 3: BetaAgent ran exactly once (idempotency via context)
   ✓ AXIOM 3: GammaAgent ran exactly once (idempotency via context)
   ✓ AXIOM 2: DeltaAgent ran exactly once (multi-precondition NOT starved)
   ✓ AXIOM 4: BrokenAgent was STARVED (undeclared dependency on Evaluations)

┌─────────────────────────────────────────────────────────────────────────────┐
│ PHASE 4: RESTART SAFETY TEST (proves AXIOM 1 - no hidden state)             │
└─────────────────────────────────────────────────────────────────────────────┘

   Running engine AGAIN on the converged context...
   If any agent has hidden state, it would run again.

      [AlphaAgent          ] accepts(✗) deps=[Seeds=1, Hypotheses=1]
      [SeedProvider        ] accepts(✗) deps=[]
      [DeltaAgent          ] accepts(✗) deps=[Seeds=1, Hypotheses=1, Signals=1, Evaluations=1]
      [GammaAgent          ] accepts(✗) deps=[Signals=1, Strategies=1]
      [BetaAgent           ] accepts(✗) deps=[Hypotheses=1, Signals=1]

   ✓ AXIOM 1: Second run converged in 1 cycle
              All agents correctly detected 'already contributed' from context
              This proves idempotency is CONTEXT-DERIVED, not STATE-DERIVED
   ✓ Context unchanged after restart

╔══════════════════════════════════════════════════════════════════════════════╗
║                              VERDICT                                         ║
╚══════════════════════════════════════════════════════════════════════════════╝

   ✓ AXIOM 1: Context is the ONLY shared state
   ✓ AXIOM 2: Eligibility is data-driven
   ✓ AXIOM 3: Idempotency via context
   ✓ AXIOM 4: Dependency completeness (violations cause starvation)
   ✓ AXIOM 5: Deterministic convergence

   Agent execution is a pure function of context evolution.
   No agent was starved (except the intentionally broken one).
   No agent ran twice.
   Convergence is deterministic and explainable.

   THE ENGINE IS NOT BUILT ON JELLY.

test verbose_axiom_proof ... ok
```

</details>

**Agents**:

- `SeedProvider` — Bootstrap (no deps)
- `AlphaAgent` — Seeds → Hypotheses
- `BetaAgent` — Hypotheses → Signals
- `GammaAgent` — Signals → Strategies
- `DeltaAgent` — Seeds+Hypotheses+Signals → Evaluations (MULTI-PRECONDITION)
- `BrokenAgent` — Undeclared dep on Evaluations (STARVED)

**Phases**:

1. Agent registration with dependency graph
2. Convergence loop with step-by-step trace
3. Axiom verification with assertions
4. Restart safety test (proves no hidden state)

---

### `hitl_pause_resume_axioms.rs` — Human-In-The-Loop Proof

**Purpose**: Proves that HITL workflows follow the same convergence guarantees as fully automated workflows. Human decisions are just facts.

**Run**:

```bash
cargo test -p converge-core --test hitl_pause_resume_axioms -- --nocapture
```

**What it proves**:

1. Context can be serialized mid-execution (pause)
2. Deserialized context produces identical behavior (resume)
3. Human decisions added during pause influence execution correctly
4. Agents waiting for human input are not starved
5. The final result is deterministic regardless of when pause occurred

**The Pause/Resume Pattern**:

```text
Phase 1: Agents run → one emits approval_request
Phase 2: Engine converges (gated agents blocked)

--- PAUSE: Serialize context to JSON/database ---

Phase 3: Human reviews and adds approval:granted fact

--- RESUME: Run engine on modified context ---

Phase 4: Blocked agents now eligible, run and converge
Phase 5: Final state matches straight-through execution
```

**Agents**:

- `AnalysisAgent` — Seeds → Signals (analysis:summary)
- `ApprovalRequestAgent` — Signals → Signals (approval:request)
- `ApprovalGatedAgent` — Signals + Constraints → Strategies (BLOCKED until approval)
- `EvaluationAgent` — Strategies → Evaluations

**Key Insight**: The engine doesn't know or care where facts come from. A fact added by a human during pause is indistinguishable from a fact added by an agent. This is why HITL "just works" — it's not a special case.

---

### `snapshot_resume_axioms.rs` — Snapshot/Recovery Proof

**Purpose**: Proves that context snapshots enable crash recovery and cross-instance execution. The engine is stateless — only context matters.

**Run**:

```bash
cargo test -p converge-core --test snapshot_resume_axioms -- --nocapture
```

**What it proves**:

1. Context snapshots are complete (no missing state)
2. Snapshots can be persisted to any storage (JSON, database, file)
3. Resuming on a fresh engine produces identical results
4. Multiple resume points converge to the same final state
5. Crash recovery is safe (just reload last snapshot)

**The Snapshot Pattern**:

```text
Engine A: Run cycles 1-3, take snapshot S1
Engine B: Load S1, run cycles 4-6, take snapshot S2
Engine C: Load S2, run to convergence

Engine D: Run straight through (no snapshots)

VERIFY: Engine C final state == Engine D final state
```

**Tests**:

- `snapshot_resume_axiom_proof` — Multi-engine snapshot/resume workflow
- `snapshot_preserves_version` — Context version survives serialization
- `snapshot_dirty_keys_on_resume` — Dirty key handling on resume
- `crash_recovery_simulation` — Recovery from multiple checkpoint scenarios

**Key Insight**: The engine is stateless. Context is everything. This enables horizontal scaling, crash recovery, and distributed execution without any special coordination.

---

### `engine_convergence_bones.rs` — Core Convergence Contract

**Purpose**: Minimal correctness contract that defines the engine's behavior.

**Run**:

```bash
cargo test -p converge-core --test engine_convergence_bones -- --nocapture
```

**What it proves**:

1. Multi-precondition agents wait correctly for ALL dependencies
2. Out-of-order dirty keys don't cause starvation
3. Context-derived idempotency prevents duplicate execution
4. Broken agents (undeclared dependencies) ARE starved

**Expected execution trace**:

```
Cycle 1: AlphaAgent → Seeds:alpha
Cycle 2: BetaAgent → Hypotheses:beta
Cycle 3: GammaAgent → Signals:gamma
Cycle 4: DeltaAgent → Strategies:delta
Cycle 5: EpsilonAgent → Evaluations:epsilon
Cycle 6: CONVERGENCE (no eligible agents)
```

---

### `no_starvation.rs` — Starvation Prevention

**Purpose**: Regression test that proves agents cannot be starved by the dependency-indexed eligibility model.

**Run**:

```bash
cargo test -p converge-core --test no_starvation
```

**The starvation scenario (now impossible)**:

1. Agent is eligible (dependency dirty)
2. `accepts()` returns false (precondition not met)
3. Precondition becomes true later
4. Agent is never reconsidered ← BUG (now fixed)

**The fix**: Agents must declare ALL keys read in `accepts()` as dependencies.

**Tests**:

- `multi_precondition_agent_is_not_starved` — Agent needing both Seeds AND Hypotheses runs correctly
- `idempotency_prevents_duplicate_execution` — Same agent doesn't run twice
- `agent_with_undeclared_dependency_would_be_starved` — Negative control

---

### `property_tests.rs` — Random Input Invariants

**Purpose**: Property-based testing using proptest to verify invariants hold across random inputs.

**Run**:

```bash
cargo test -p converge-core --test property_tests
```

**Properties tested**:

- Context preserves fact content
- Context tracks dirty keys correctly
- Fact IDs must be unique per key
- Version increments on mutation
- Budget limits are enforced
- AgentRequirements validation
- RootIntent constraint validation
- Invariant violation detection

---

### `convergence.rs` — Sequential Agent Execution

**Purpose**: Basic integration test for agent chain execution.

**Run**:

```bash
cargo test -p converge-core --test convergence
```

**Tests**:

- `converges_through_sequential_agents` — Seeds → Strategy → Constraint chain completes
- Verifies cycle count matches expected (at least 3)

---

### `proposal_promotion.rs` — LLM Output Validation

**Purpose**: Tests the LLM containment model where untrusted outputs cannot corrupt trusted context without validation.

**Run**:

```bash
cargo test -p converge-core --test proposal_promotion
```

**Tests**:

- `engine_promotes_proposals_in_merge_phase` — Valid proposals become facts
- `engine_rejects_invalid_proposals` — Empty content proposals are rejected

---

### `transparent_determinism.rs` — Graceful Error Handling

**Purpose**: Tests that the engine handles partial failures gracefully.

**Run**:

```bash
cargo test -p converge-core --test transparent_determinism
```

**Tests**:

- `engine_handles_partial_failures_gracefully` — One valid + one invalid proposal: only valid is promoted

---

### `validation_verbose.rs` — Validation Agent Proof

**Purpose**: Comprehensive test of the ValidationAgent that gates LLM outputs.

**Run**:

```bash
cargo test -p converge-core --test validation_verbose -- --nocapture
```

**What it proves**:

- ValidationAgent correctly accepts/rejects proposals based on:
  - Confidence threshold
  - Content validation (non-empty, max length)
  - Forbidden terms filter
  - Provenance requirement
- Rejection diagnostics are recorded in context

---

## Running All Proof Tests

```bash
# All proof tests (quiet)
cargo test -p converge-core

# All proof tests with output
cargo test -p converge-core -- --nocapture

# Specific verbose tests
cargo test -p converge-core --test engine_convergence_axioms -- --nocapture
cargo test -p converge-core --test engine_convergence_bones -- --nocapture
cargo test -p converge-core --test validation_verbose -- --nocapture
```

---

## The Verdict

When all proof tests pass, the following is mathematically proven:

```
✓ Agent execution is a pure function of context evolution.
✓ No agent can be starved (except by violating the dependency contract).
✓ No agent can run twice.
✓ Convergence is deterministic and explainable.
✓ Execution can be paused, serialized, and resumed safely.
✓ Human decisions are just facts — no special HITL machinery needed.
✓ Context snapshots are complete — no hidden state.
✓ Resuming on different engines produces identical results.

THE ENGINE IS NOT BUILT ON JELLY.
HUMANS ARE JUST ANOTHER SOURCE OF FACTS.
THE ENGINE IS STATELESS. CONTEXT IS EVERYTHING.
```

---

## Related Documentation

- [ENGINE_EXECUTION_MODEL.md](./ENGINE_EXECUTION_MODEL.md) — Architectural invariants
- [ROOT_INTENT_SCHEMA.md](./ROOT_INTENT_SCHEMA.md) — Intent specification
- [LLM_CONTAINMENT_MODEL.md](./LLM_CONTAINMENT_MODEL.md) — LLM output validation
