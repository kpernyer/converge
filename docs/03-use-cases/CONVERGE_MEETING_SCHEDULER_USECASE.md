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

## 10. One-Sentence Summary

> The Meeting Scheduling runtime converges on valid and optimal meeting times under overlapping constraints, while remaining explainable and interruptible.
