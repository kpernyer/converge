# Use Case Review Process

**Purpose**: Systematic review process to ensure use cases meet Converge's standards for correctness, completeness, and alignment with architectural principles.

**Status**: Draft - To be refined through practice

---

## Overview

This process ensures every use case:
- Aligns with Converge's core principles
- Has clear convergence criteria
- Demonstrates provable outcomes
- Maps correctly to the context model
- Includes appropriate invariants
- Handles HITL (Human-in-the-Loop) scenarios
- Can be implemented deterministically first, then enhanced with LLMs

---

## Review Checklist

### Phase 1: Problem Statement & Root Intent

- [ ] **Problem is convergence-based, not workflow-based**
  - Does this require reaching a fixed point?
  - Or is it just a linear sequence of steps?
  - ❌ Reject if: Simple workflow, event-driven, or actor system pattern

- [ ] **Root Intent is clear and bounded**
  - Single, explicit objective
  - Clear scope boundaries
  - Success criteria are measurable
  - Budgets/constraints defined

- [ ] **Why ERP/workflow/chatbot can't solve it**
  - Demonstrates semantic convergence need
  - Requires multi-agent collaboration
  - Needs explainable outcomes

### Phase 2: Context Model

- [ ] **Context keys map correctly**
  - Uses existing `ContextKey` enum appropriately
  - Or justifies new keys (rare)
  - Facts are typed and append-only

- [ ] **No hidden state**
  - All state in context
  - No agent-internal state across executions
  - Idempotency via context checks

- [ ] **Context evolution is monotonic**
  - Facts only added, never mutated
  - Invalidations are explicit facts
  - History preserved

### Phase 3: Agent Design

- [ ] **Agents follow the pattern**
  - `accepts()` is pure (no side effects)
  - `execute()` is read-only (emits effects only)
  - Dependencies declared correctly
  - Never call other agents

- [ ] **Idempotency is context-based**
  - Checks context for existing contributions
  - Uses agent name prefix pattern
  - LLM agents check both `Proposals` and `target_key`

- [ ] **Agent pipeline is clear**
  - Dependency graph is acyclic
  - Each agent has clear role
  - Parallel execution possible where dependencies allow

### Phase 4: Invariants (Gherkin)

- [ ] **Structural invariants** (checked on every merge)
  - Data integrity rules
  - Schema validation
  - Immediate failure on violation

- [ ] **Semantic invariants** (checked at end of cycle)
  - Business logic rules
  - Convergence blockers
  - Remediation possible

- [ ] **Acceptance invariants** (checked when convergence claimed)
  - Final quality gates
  - Reject convergence if violated
  - Must be satisfied for success

- [ ] **Invariants are testable**
  - Can be compiled to Rust predicates
  - No runtime string interpretation
  - Clear violation messages

### Phase 5: Human-in-the-Loop (HITL)

- [ ] **Authority model is explicit**
  - Who has authority? (managers, HR, executives)
  - What decisions require human input?
  - How does HITL work? (halt → snapshot → inject fact → resume)

- [ ] **HITL doesn't bypass engine**
  - Human input becomes facts in context
  - Engine processes facts normally
  - No special "human override" paths

- [ ] **Waiting is a valid state**
  - `HaltReason::AwaitingAuthority` is expected
  - Context snapshot preserved
  - Resume from snapshot possible

### Phase 6: Convergence Criteria

- [ ] **Fixed point is well-defined**
  - Clear condition: `Contextₙ₊₁ == Contextₙ`
  - All required facts present
  - All invariants satisfied

- [ ] **Convergence is provable**
  - Can explain why convergence occurred
  - Can reconstruct execution path
  - Deterministic (same inputs → same outputs)

- [ ] **Failure modes are explicit**
  - Budget exhaustion
  - Invariant violations
  - External dependency failures
  - HITL timeouts

### Phase 7: Implementation Plan

- [ ] **Deterministic version first**
  - All agents implementable without LLMs
  - Testable with mock data
  - Demonstrates convergence

- [ ] **LLM enhancement plan**
  - Which agents benefit from LLMs?
  - What are the requirements? (cost, latency, capabilities)
  - How are proposals validated?

- [ ] **Integration points**
  - External systems (HRIS, CRM, etc.)
  - Data sources
  - Notification channels

---

## Review Workflow

### Step 1: Self-Review (Author)

1. Fill out checklist above
2. Write test scenarios (happy path, edge cases, failures)
3. Document assumptions and out-of-scope items
4. Create agent dependency diagram

### Step 2: Architecture Review

1. Verify alignment with Converge principles
2. Check context model correctness
3. Validate agent design patterns
4. Review invariant classes

**Reviewer**: Architecture team / Tech lead

**Questions to ask**:
- Does this violate any core principles?
- Is the context model correct?
- Are agents following the pattern?
- Are invariants at the right class?

### Step 3: Domain Review

1. Verify problem statement accuracy
2. Check business logic correctness
3. Validate convergence criteria
4. Review HITL authority model

**Reviewer**: Domain expert / Product owner

**Questions to ask**:
- Is this a real problem?
- Are convergence criteria correct?
- Who has authority?
- What are the failure modes?

### Step 4: Implementation Review

1. Verify deterministic implementation plan
2. Check LLM integration points
3. Review test coverage
4. Validate external integrations

**Reviewer**: Engineering team

**Questions to ask**:
- Can this be implemented deterministically?
- Are LLM agents properly scoped?
- Are tests comprehensive?
- Are integrations feasible?

### Step 5: Final Approval

1. All checkboxes completed
2. All reviewers approved
3. Test scenarios documented
4. Ready for implementation

**Approver**: Product + Engineering leads

---

## Review Template

```markdown
# Use Case Review: [USE_CASE_NAME]

**Use Case ID**: [ID]
**Author**: [Name]
**Date**: [Date]
**Status**: [Draft | In Review | Approved | Rejected]

## Phase 1: Problem Statement & Root Intent
- [ ] Problem is convergence-based
- [ ] Root Intent is clear
- [ ] Why ERP/workflow can't solve it

**Notes**: [Any issues or concerns]

## Phase 2: Context Model
- [ ] Context keys map correctly
- [ ] No hidden state
- [ ] Context evolution is monotonic

**Notes**: [Any issues or concerns]

## Phase 3: Agent Design
- [ ] Agents follow the pattern
- [ ] Idempotency is context-based
- [ ] Agent pipeline is clear

**Notes**: [Any issues or concerns]

## Phase 4: Invariants
- [ ] Structural invariants defined
- [ ] Semantic invariants defined
- [ ] Acceptance invariants defined
- [ ] Invariants are testable

**Notes**: [Any issues or concerns]

## Phase 5: HITL
- [ ] Authority model is explicit
- [ ] HITL doesn't bypass engine
- [ ] Waiting is a valid state

**Notes**: [Any issues or concerns]

## Phase 6: Convergence Criteria
- [ ] Fixed point is well-defined
- [ ] Convergence is provable
- [ ] Failure modes are explicit

**Notes**: [Any issues or concerns]

## Phase 7: Implementation Plan
- [ ] Deterministic version first
- [ ] LLM enhancement plan
- [ ] Integration points

**Notes**: [Any issues or concerns]

## Reviewers

### Architecture Review
- **Reviewer**: [Name]
- **Date**: [Date]
- **Status**: [Approved | Needs Changes | Rejected]
- **Comments**: [Comments]

### Domain Review
- **Reviewer**: [Name]
- **Date**: [Date]
- **Status**: [Approved | Needs Changes | Rejected]
- **Comments**: [Comments]

### Implementation Review
- **Reviewer**: [Name]
- **Date**: [Date]
- **Status**: [Approved | Needs Changes | Rejected]
- **Comments**: [Comments]

## Final Approval
- **Approver**: [Name]
- **Date**: [Date]
- **Status**: [Approved | Rejected]
```

---

## Common Issues & Solutions

### Issue: "This is just a workflow"
**Solution**: Reject or reframe. Converge is for convergence problems, not linear workflows.

### Issue: "Agents need to call each other"
**Solution**: Refactor to use context. Agents read context, emit effects, engine merges.

### Issue: "We need to mutate existing facts"
**Solution**: Emit invalidation facts. Context is append-only in meaning.

### Issue: "LLM outputs should be facts directly"
**Solution**: No. LLMs emit `ProposedFact`, validation promotes to `Fact`.

### Issue: "Convergence criteria are unclear"
**Solution**: Define fixed point explicitly. What does "done" mean?

### Issue: "HITL needs to bypass the engine"
**Solution**: No. Human input becomes facts, engine processes normally.

---

## Next Steps

1. **Author**: Complete self-review using checklist
2. **Schedule**: Book review sessions with reviewers
3. **Iterate**: Address feedback, update use case
4. **Approve**: Get final sign-off
5. **Implement**: Start with deterministic version

---

**Last Updated**: [Date]
**Version**: 1.0
