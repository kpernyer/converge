# Use Case Development Guide

**Purpose**: Central guide for developing, reviewing, and iterating on Converge use cases.

**Last Updated**: 2024

---

## Quick Start

1. **New Use Case?** → Follow [USE_CASE_REVIEW_PROCESS.md](./USE_CASE_REVIEW_PROCESS.md)
2. **Iterating Existing?** → Follow [USE_CASE_ITERATION_GUIDE.md](./USE_CASE_ITERATION_GUIDE.md)
3. **Need Business Solutions?** → Check [BUSINESS_SOLUTIONS_CATALOG.md](./BUSINESS_SOLUTIONS_CATALOG.md)
4. **Jobs to be Done?** → Use [JOBS_TO_BE_DONE_FRAMEWORK.md](./JOBS_TO_BE_DONE_FRAMEWORK.md)

---

## Document Overview

### 1. Use Case Review Process
**File**: `USE_CASE_REVIEW_PROCESS.md`

Systematic review process to ensure use cases meet Converge's standards:
- 7-phase review checklist
- Review workflow (Self → Architecture → Domain → Implementation → Approval)
- Common issues & solutions
- Review template

**Use When**: Creating a new use case or reviewing an existing one

### 2. Jobs to be Done Framework
**File**: `JOBS_TO_BE_DONE_FRAMEWORK.md`

Framework for analyzing use cases through the Jobs to be Done lens:
- Functional Jobs (what needs to be accomplished)
- Emotional Jobs (how stakeholders want to feel)
- Relational Jobs (how stakeholders want to be perceived)
- Template and examples

**Use When**: Understanding stakeholder needs and designing solutions

### 3. Business Solutions Catalog
**File**: `BUSINESS_SOLUTIONS_CATALOG.md`

Comprehensive catalog of business problems and solutions across domains:
- HR, Sales, Marketing, Legal, IP, Contracts, R&D
- Relationship Capital, Partnership Development
- Technology & Innovation
- Cross-domain solutions

**Use When**: Identifying use case opportunities or understanding domain context

### 4. Use Case Iteration Guide
**File**: `USE_CASE_ITERATION_GUIDE.md`

Guide for iterating on existing use cases:
- Current state assessment
- Gap analysis
- Enhancement process
- Documentation updates

**Use When**: Improving existing use cases

---

## Workflow

### For New Use Cases

```
1. Identify Problem
   ↓
2. Check Business Solutions Catalog
   ↓
3. Apply Jobs to be Done Framework
   ↓
4. Write Use Case Specification
   ↓
5. Follow Review Process
   ↓
6. Implement (Deterministic → LLM)
```

### For Existing Use Cases

```
1. Read Current Use Case
   ↓
2. Apply Jobs to be Done Framework
   ↓
3. Review Against Business Solutions Catalog
   ↓
4. Follow Iteration Guide
   ↓
5. Update Documentation
   ↓
6. Re-review if Major Changes
```

---

## Key Principles

### 1. Convergence-Based, Not Workflow-Based
- Use cases must require reaching a fixed point
- Not just linear sequences of steps
- Must demonstrate semantic convergence

### 2. Jobs to be Done First
- Understand what stakeholders need to get done
- Address functional, emotional, and relational jobs
- Design solution to address all three

### 3. Provable Outcomes
- Convergence must be provable
- Outcomes must be explainable
- Evidence must be auditable

### 4. Human Authority First-Class
- HITL is a valid state
- Human input becomes facts
- No bypassing the engine

---

## Common Patterns

### Problem Statement Pattern
```
[Domain] must [action] to ensure [outcome],
with [requirements],
demonstrating [evidence],
while [constraints].
```

### Jobs to be Done Pattern
```
Functional: "When [situation], I need to [task] so I can [outcome]"
Emotional: "I want to feel [feeling] about [concern]"
Relational: "I want to be seen as [identity] in [context]"
```

### Convergence Pattern
```
The system converges when:
- All required facts are present
- All invariants are satisfied
- No new facts can be added
- Fixed point reached: Contextₙ₊₁ == Contextₙ
```

---

## Resources

### Existing Use Cases
- Growth Strategy
- HR Policy Alignment
- Compliance Monitoring
- CRM Account Health
- Strategic Sourcing
- Inventory Rebalancing
- Supply Chain
- Release Readiness
- Meeting Scheduler
- Resource Routing
- Catalog Enrichment
- SDR Sales

### Reference Documents
- `docs/architecture/ARCHITECTURE.md` - System architecture
- `docs/agents/AGENT_LIFECYCLE.md` - Agent patterns
- `docs/governance/GOVERNANCE.md` - Design principles
- `AGENTS.md` - AI assistant guide

---

## Getting Help

### Questions About Process?
- Review `USE_CASE_REVIEW_PROCESS.md`
- Check examples in existing use cases

### Questions About Jobs to be Done?
- Review `JOBS_TO_BE_DONE_FRAMEWORK.md`
- See example in HR Policy Alignment

### Questions About Business Solutions?
- Check `BUSINESS_SOLUTIONS_CATALOG.md`
- Search by domain

### Questions About Iteration?
- Follow `USE_CASE_ITERATION_GUIDE.md`
- Start with high-priority use cases

---

## Next Steps

1. **Review Process**: Walk through `USE_CASE_REVIEW_PROCESS.md` together
2. **Jobs to be Done**: Apply framework to existing use cases
3. **Business Solutions**: Review catalog and identify opportunities
4. **Iteration**: Enhance existing use cases using iteration guide

---

**Status**: Living document - updated as process evolves
