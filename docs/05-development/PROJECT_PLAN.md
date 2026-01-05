# Converge — Updated Project Plan

This document reflects the **current, locked-in architecture** of Converge and
updates the original project plan to align with the agreed execution-first approach.

The guiding principle is:
> Prove convergence and correctness early, then layer complexity.

---

## Phase 0 — Architecture Lock (DONE)

Artifacts completed:
- ARCHITECTURE.md
- TERMINOLOGY.md
- DISTRIBUTED_SYSTEMS.md
- SCALING_MODEL.md
- FAILURE_MODES.md
- CONVERGENCE_PROOFS.md
- CONVERGENCE_SEMANTICS.md
- GHERKIN_MODEL.md
- LLM_INTEGRATION.md
- OTP_MODEL.md
- TEMPORAL_MODEL.md
- WHEN_TO_USE_CONVERGE.md
- REFERENCE_ARCHITECTURES.md

Outcome:
- All major architectural objections addressed
- No unresolved design ambiguities remain

---

## Phase 1 — Engine Skeleton (CURRENT PRIORITY)

**Goal:** Prove convergence, eligibility, and merge semantics in Rust.

### Deliverables
- Rust crate skeleton
- Context + ContextKey
- AgentEffect
- Agent trait (minimal)
- Engine loop (`run_until_converged`)
- Deterministic convergence test

### Exit Criteria
- Engine converges deterministically
- No infinite loops possible
- No shared mutable state outside engine
- All mutations flow through AgentEffect

---

## Phase 2 — Eligibility Index & Budgets

**Goal:** Make execution scale with data changes, not agent count.

### Deliverables
- Agent dependency declaration
- ContextKey → Agent index
- Budget enforcement (cycles, facts)
- Minimal pruning hooks

### Exit Criteria
- Agents only re-run when relevant context changes
- Budget exhaustion halts execution safely

---

## Phase 3 — Gherkin Runtime Enforcement

**Goal:** Turn Gherkin from documentation into runtime law.

### Deliverables
- Gherkin compiler (to Rust predicates)
- Structural vs semantic vs acceptance invariant classes
- Invariant execution points wired into engine
- Failure modes exercised by tests

### Exit Criteria
- Invalid convergence cannot be emitted
- Invariant violations are explicit and traceable

---

## Phase 4 — Growth Strategy v1 (No LLMs)

**Goal:** Validate the engine with a real but deterministic domain.

### Deliverables
- Growth Strategy context schema
- 3–5 deterministic agents
- Strategy convergence test
- Ranked outputs with rationale

### Exit Criteria
- End-to-end strategy run converges
- Output is explainable
- No LLM dependency

---

## Phase 5 — LLM Integration (Governed)

**Goal:** Add LLM power without sacrificing correctness.

### Deliverables
- ProposedFact vs Fact
- Validation agents
- LLM tool abstraction
- Trace capture of LLM outputs

### Exit Criteria
- LLM hallucinations cannot corrupt context
- LLM outputs are auditable and bounded

---

## Phase 6 — Integration & Packaging

**Goal:** Make Converge usable in real systems.

### Deliverables
- MCP adapter
- API surface for job execution
- Optional persistence adapters
- Example reference deployments

---

## Phase 7 — Hardening & Expansion

**Goal:** Prepare for broader use.

### Deliverables
- Performance tuning
- Additional use cases
- Security & audit docs
- Public README & examples

---

## Summary Timeline

Phase 1–2: Core engine proof  
Phase 3: Correctness enforcement  
Phase 4–5: Intelligence & usefulness  
Phase 6–7: Adoption & scale
