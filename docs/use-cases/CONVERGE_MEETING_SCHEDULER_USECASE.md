# Converge — Use Case: Meeting Scheduling Runtime

## Purpose of this document

This document describes a **Meeting Scheduling** use case implemented on the
**Converge Agent OS**, at a **high architectural and semantic level**.

It explains:
- the business intent
- the role of context and agents
- how convergence is reached
- how Gherkin expresses correctness

This is **not** a calendar UI.
This is a **decision runtime** for scheduling under constraints.

---

## 1. Business Problem

Teams need to schedule meetings that:
- respect participant availability
- respect working hours and policies
- account for time zones
- provide alternatives when conflicts exist

The difficulty lies in:
- overlapping constraints
- incomplete or changing availability
- competing preferences
- the need for fast answers with better options later

---

## 2. Root Intent (Operational Scope)

Everything starts with a Root Intent.

### Natural language intent

> “Schedule a 60-minute meeting with Alice, Bob, and Carol next week.”

### Gherkin — Root Intent Declaration

```gherkin
Feature: Meeting scheduling

Scenario: Define scheduling intent
  Given the meeting duration is 60 minutes
  And the participants are Alice, Bob, and Carol
  And the time window is next week
  Then the system searches for valid meeting times
```

---

## 3. Questions the Runtime Must Answer

- When are all participants available?
- Which times respect working-hour policies?
- Are rooms or resources required?
- Are there acceptable alternatives if no perfect slot exists?

These questions are explored **in parallel**, not sequentially.

---

## 4. Context (High-Level View)

Initial context:

```
Context₀
├─ Participants: Alice, Bob, Carol
├─ Duration: 60 minutes
├─ Window: Next week
├─ Constraints:
│   ├─ Working hours
│   └─ Time zones
└─ Availability: ∅
```

Context evolves as availability, policies, and candidates are added.

---

## 5. Classes of Agents Involved

### Retrieval Agents
- Calendar lookup agents
- Room/resource availability agents

### Normalization Agents
- Time-zone normalization
- Availability window alignment

### Constraint Agents
- Working-hour enforcement
- Conflict detection

### Optimization Agents
- Slot optimization (earliest, least disruption)

### Explanation Agents
- Explain why a slot was chosen or rejected

---

## 6. Execution Model

The runtime executes in cycles:
1. Availability is retrieved
2. Constraints are applied
3. Candidate slots are generated
4. Invalid slots are pruned
5. Remaining slots are ranked

Execution continues until no new valid slots emerge.

---

## 7. Progressive Convergence

### Early convergence
> “Tuesday 10:00–11:00 works for all participants.”

### Primary convergence
> “Tuesday 10:00–11:00 is optimal. Two alternatives exist later in the week.”

The user may accept early, while refinement continues.

---

## 8. Outputs of the Runtime

- Selected meeting time
- Ranked alternative times
- Explanation of constraints and tradeoffs

---

## 9. Gherkin — Scheduling Invariants

```gherkin
Scenario: Valid meeting time
  When the system converges
  Then the selected time works for all participants
  And no participant is scheduled outside working hours
```

---

## 10. End-Value and Proof Points

### What This Use-Case Proves

The Meeting Scheduler runtime demonstrates that **Converge is the ultimate way to handle reliable business agents** by proving:

#### 1. **Constraint Satisfaction Under Uncertainty**

**The Problem:** Traditional scheduling systems use rigid rules or sequential workflows that cannot handle incomplete information or competing constraints.

**Converge's Solution:**
- Multiple constraint agents run in parallel (availability, working hours, time zones)
- Constraints are explicit facts, not hidden in code
- Agents prune invalid options as constraints are discovered
- Convergence occurs when valid slots are found and ranked

**Why It Matters:** Real-world scheduling involves uncertainty (incomplete calendars, changing availability). Converge handles this gracefully by exploring constraints in parallel and converging when valid solutions emerge.

#### 2. **Progressive Convergence for Time-Sensitive Decisions**

**The Problem:** Users need scheduling answers quickly, but better options may exist if given more time.

**Converge's Solution:**
- **Early convergence:** First valid slot found → immediate answer
- **Primary convergence:** Optimal slot with alternatives → refined answer
- **Extended convergence:** Background refinement → better options if time allows
- Results are always valid and explainable, even if interrupted

**Why It Matters:** Scheduling decisions are time-sensitive. Converge delivers fast answers while continuing to improve, enabling users to act immediately without sacrificing quality.

#### 3. **Deterministic Optimization Without Hidden State**

**The Problem:** Traditional optimization systems use black-box solvers or heuristics that produce unpredictable results.

**Converge's Solution:**
- Optimization agents are deterministic — same inputs produce same outputs
- All optimization criteria are explicit facts in context
- Ranking logic is transparent and explainable
- Full provenance for every slot evaluation

**Why It Matters:** Users need to trust scheduling decisions. Converge provides deterministic, explainable optimization that users can understand and verify.

#### 4. **Parallel Constraint Evaluation**

**The Problem:** Sequential constraint checking is slow and cannot leverage parallelism.

**Converge's Solution:**
- Availability retrieval, timezone normalization, and constraint checking run in parallel
- Agents declare dependencies — execution order is data-driven, not predefined
- Context evolution triggers eligible agents automatically
- No hidden control flow — everything is explicit in context

**Why It Matters:** Performance matters for user experience. Converge's parallel execution delivers fast results while maintaining correctness guarantees.

#### 5. **Explainable Tradeoffs**

**The Problem:** Traditional systems provide answers without explaining why alternatives were rejected.

**Converge's Solution:**
- Every slot evaluation includes rationale
- Alternative slots are ranked with explanations
- Constraint violations are explicit facts, not silent failures
- Users understand tradeoffs (earliest vs. least disruption vs. most participants)

**Why It Matters:** Users need to understand scheduling decisions to make informed choices. Converge provides transparent explanations for every recommendation.

### End-Value Delivered

**For End Users:**
- **Fast scheduling answers** with immediate valid options
- **Better options over time** as the system continues to refine
- **Transparent explanations** for why slots were chosen or rejected
- **Reliable results** — same inputs produce same outputs

**For Technical Teams:**
- **Deterministic constraint satisfaction** — no flaky or unpredictable behavior
- **Parallel performance** without sacrificing correctness
- **Maintainable architecture** — agents are independent and testable
- **Extensible design** — new constraints can be added without breaking existing logic

**For the Industry:**
- **Proof that constraint satisfaction can be deterministic** and explainable
- **Demonstration that anytime algorithms deliver business value** for time-sensitive decisions
- **Evidence that parallel execution can improve performance** without compromising correctness
- **Foundation for reliable scheduling systems** that users can trust

### Why This Matters to Proving Converge

The Meeting Scheduler use-case proves that Converge's model works for **operational decision-making**:

1. **It requires fast answers** — proving that progressive convergence delivers business value
2. **It involves multiple constraints** — proving that parallel constraint evaluation works
3. **It needs deterministic results** — proving that correctness guarantees matter for user trust
4. **It demands explainability** — proving that transparent systems are more valuable
5. **It solves a common problem** — proving that Converge is practical, not just theoretical

**Traditional scheduling systems use workflows or heuristics that cannot guarantee correctness or explainability.** Converge provides both, proving that the convergence model is superior for operational decisions.

---

## 11. One-Sentence Summary

> The Meeting Scheduling runtime converges on valid and optimal meeting times under overlapping constraints, while remaining explainable and interruptible.
