# Use Case Iteration Guide

**Purpose**: Guide for iterating on existing use cases to improve them using the Jobs to be Done framework, review process, and business solutions catalog.

**Status**: Draft - To be refined through practice

---

## Overview

This guide helps you:
1. Review existing use cases through the Jobs to be Done lens
2. Identify gaps and improvements
3. Enhance use cases with better problem statements
4. Align use cases with business solutions catalog
5. Prepare use cases for implementation

---

## Iteration Process

### Phase 1: Current State Assessment

For each existing use case:

1. **Read the use case document**
   - Understand the problem statement
   - Review the agent pipeline
   - Check convergence criteria
   - Review invariants

2. **Map to Jobs to be Done**
   - Identify functional jobs
   - Identify emotional jobs
   - Identify relational jobs
   - Document gaps

3. **Review against Business Solutions Catalog**
   - Find matching solutions
   - Identify related solutions
   - Note missing solutions
   - Document opportunities

4. **Apply Review Process**
   - Go through review checklist
   - Identify issues
   - Document improvements needed

### Phase 2: Gap Analysis

1. **Functional Gaps**
   - Missing jobs
   - Incomplete jobs
   - Unclear success criteria

2. **Emotional Gaps**
   - Missing emotional jobs
   - Unaddressed feelings
   - Trust factors missing

3. **Relational Gaps**
   - Missing identity goals
   - Unclear reputation factors
   - Relationship goals not addressed

4. **Technical Gaps**
   - Missing agents
   - Incomplete invariants
   - Unclear convergence criteria
   - HITL not addressed

### Phase 3: Enhancement

1. **Enhance Problem Statement**
   - Make it more specific
   - Add Jobs to be Done context
   - Clarify why ERP/workflow can't solve it

2. **Enhance Agent Pipeline**
   - Add missing agents
   - Clarify agent roles
   - Improve dependencies

3. **Enhance Invariants**
   - Add missing invariants
   - Clarify invariant classes
   - Improve violation messages

4. **Enhance HITL**
   - Clarify authority model
   - Document HITL scenarios
   - Define halt conditions

5. **Enhance Convergence Criteria**
   - Make fixed point explicit
   - Add provability requirements
   - Document failure modes

### Phase 4: Documentation

1. **Update Use Case Document**
   - Add Jobs to be Done section
   - Update problem statement
   - Enhance agent descriptions
   - Improve convergence criteria

2. **Create Jobs to be Done Document**
   - Document all jobs
   - Map to stakeholders
   - Define metrics

3. **Update Review Checklist**
   - Complete review checklist
   - Document decisions
   - Note improvements made

---

## Template for Iterated Use Case

```markdown
# [USE_CASE_NAME] - Iterated

## Version History
- **v1.0**: [Date] - Initial version
- **v2.0**: [Date] - Iterated with Jobs to be Done framework

## Changes from Previous Version
- [ ] Enhanced problem statement
- [ ] Added Jobs to be Done analysis
- [ ] Improved agent pipeline
- [ ] Enhanced invariants
- [ ] Clarified HITL
- [ ] Improved convergence criteria

## 1. Problem Statement

### Original Problem
[Original problem statement]

### Enhanced Problem Statement
[Enhanced problem statement with Jobs to be Done context]

### Why This Cannot Be Solved by ERP/Workflow/Chatbot
[Clear explanation of why convergence is needed]

## 2. Jobs to be Done

### Functional Jobs
[Document functional jobs]

### Emotional Jobs
[Document emotional jobs]

### Relational Jobs
[Document relational jobs]

### How This Use Case Addresses Jobs
[Map use case capabilities to jobs]

## 3. Root Intent

[Enhanced root intent with Jobs to be Done context]

## 4. Scope & Assumptions

[Updated scope and assumptions]

## 5. Context Model

[Context model with any enhancements]

## 6. Agents

[Agent descriptions with Jobs to be Done mapping]

## 7. Invariants

[Invariants with enhanced descriptions]

## 8. HITL (Human-in-the-Loop)

[Enhanced HITL model]

## 9. Convergence Criteria

[Enhanced convergence criteria]

## 10. Failure & Halt States

[Failure modes and halt states]

## 11. Metrics

### Functional Metrics
[Metrics for functional jobs]

### Emotional Metrics
[Metrics for emotional jobs]

### Relational Metrics
[Metrics for relational jobs]

## 12. Competitive Alternatives

[Current solutions and why Converge is better]

## 13. Implementation Plan

[Implementation plan with Jobs to be Done considerations]
```

---

## Example: HR Policy Alignment Iteration

### Current State Assessment

**Original Problem Statement**: "HR must ensure that a critical policy, change, or directive is not merely sent, but understood, acknowledged, and acted upon across the organization."

**Jobs to be Done Analysis**:
- **Functional**: Ensure policy understanding and compliance
- **Emotional**: Feel confident in compliance, relieved about auditability
- **Relational**: Be seen as competent HR professional, compliance expert

**Gaps Identified**:
- Missing emotional jobs documentation
- Missing relational jobs documentation
- Could enhance problem statement with Jobs to be Done context

### Enhancement

**Enhanced Problem Statement**: "HR must ensure that a critical policy, change, or directive is not merely sent, but understood, acknowledged, and acted upon across the organization. This addresses the functional job of ensuring compliance, the emotional job of feeling confident in auditability, and the relational job of being seen as a competent HR professional who ensures organizational alignment."

**Jobs to be Done Section Added**:
- Full Jobs to be Done analysis
- Metrics for each job type
- Competitive alternatives analysis

---

## Checklist for Each Use Case

### Assessment
- [ ] Read use case document
- [ ] Map to Jobs to be Done
- [ ] Review against Business Solutions Catalog
- [ ] Apply review process checklist
- [ ] Document gaps

### Enhancement
- [ ] Enhance problem statement
- [ ] Add Jobs to be Done section
- [ ] Improve agent pipeline
- [ ] Enhance invariants
- [ ] Clarify HITL
- [ ] Improve convergence criteria

### Documentation
- [ ] Update use case document
- [ ] Create Jobs to be Done document
- [ ] Complete review checklist
- [ ] Document changes

---

## Priority Order for Iteration

1. **High Priority** (Core use cases)
   - Growth Strategy
   - HR Policy Alignment
   - Compliance Monitoring
   - CRM Account Health

2. **Medium Priority** (Important use cases)
   - Strategic Sourcing
   - Inventory Rebalancing
   - Supply Chain
   - Release Readiness

3. **Lower Priority** (Supporting use cases)
   - Meeting Scheduler
   - Resource Routing
   - Catalog Enrichment
   - SDR Sales

---

## Iteration Schedule

### Week 1: Assessment
- Assess all use cases
- Document gaps
- Prioritize enhancements

### Week 2-3: Enhancement
- Enhance high-priority use cases
- Add Jobs to be Done analysis
- Improve documentation

### Week 4: Review
- Review enhanced use cases
- Get stakeholder feedback
- Iterate based on feedback

---

## Success Criteria

### For Each Iterated Use Case
- [ ] Jobs to be Done fully documented
- [ ] Problem statement enhanced
- [ ] Review checklist completed
- [ ] Stakeholder feedback incorporated
- [ ] Ready for implementation

---

## Next Steps

1. **Start with High-Priority Use Cases**
   - Begin with Growth Strategy or HR Policy Alignment
   - Complete full iteration
   - Document learnings

2. **Refine Process**
   - Adjust process based on learnings
   - Update templates
   - Share best practices

3. **Scale to All Use Cases**
   - Apply to remaining use cases
   - Maintain consistency
   - Build library of patterns

---

**Last Updated**: [Date]
**Version**: 1.0
