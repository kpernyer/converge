# Use Cases

This directory contains concrete use-case implementations showing how Converge solves real problems.

## Use Cases

### [CONVERGE_GROWTH_STRATEGY_USECASE.md](./CONVERGE_GROWTH_STRATEGY_USECASE.md)
Growth Strategy Runtime — exploring and converging on strategic options.

**Domain:** Business strategy, go-to-market planning
**Complexity:** High (multi-agent, LLM-involved)
**Key agents:** Discovery, Structuring, Relationship/Graph, Strategy Synthesis, Governance

### [CONVERGE_MEETING_SCHEDULER_USECASE.md](./CONVERGE_MEETING_SCHEDULER_USECASE.md)
Meeting Scheduler Runtime — scheduling meetings under constraints.

**Domain:** Calendar coordination
**Complexity:** Medium (constraint-based, deterministic)
**Key agents:** Retrieval, Normalization, Constraint, Optimization

### [CONVERGE_RESOURCE_ROUTING_USECASE.md](./CONVERGE_RESOURCE_ROUTING_USECASE.md)
Resource Allocation & Routing Runtime — optimizing resource assignment.

**Domain:** Logistics, resource allocation
**Complexity:** Medium (solver-based, deterministic)
**Key agents:** Retrieval, Constraint, Solver, Aggregation

## Context Schemas

### [CONTEXT_SCHEMA_GROWTH.md](./CONTEXT_SCHEMA_GROWTH.md)
Typed context schema for the Growth Strategy runtime.

**Shows:**
- ContextKey definitions
- Fact types
- Monotonic evolution rules

---

## Using These Documents

### For Domain Experts
These documents show:
- How to express business problems as Root Intents
- What questions the runtime answers
- What outputs to expect
- How convergence works in practice

### For Implementers
These documents provide:
- Templates for new use-cases
- Examples of context schema design
- Patterns for agent organization
- Gherkin invariant examples

### For AI Agents
When implementing a use-case:
1. Start with the use-case document (business intent)
2. Define domain-specific ContextKeys
3. Create Root Intent struct
4. Implement deterministic agents first
5. Add Gherkin invariants
6. Validate convergence criteria

---

## Pattern: Use-Case Structure

Each use-case document follows this structure:

1. **Business Problem** — What problem are we solving?
2. **Root Intent** — Natural language + Gherkin declaration
3. **Questions the Runtime Must Answer** — What does it explore?
4. **Context (High-Level View)** — What data evolves?
5. **Classes of Agents Involved** — What capabilities are needed?
6. **Execution Model** — How does it converge?
7. **Progressive Convergence** — Early → Primary → Extended
8. **Outputs of the Runtime** — What artifacts are produced?
9. **Gherkin Invariants** — What correctness rules apply?
10. **One-Sentence Summary** — What is this runtime?

---

## Creating New Use-Cases

To create a new use-case:

1. Copy a similar use-case document as a template
2. Fill in the business problem and Root Intent
3. Define domain-specific ContextKeys
4. Identify agent types needed
5. Write Gherkin invariants
6. Ensure convergence criteria are clear and measurable

See `../06-assistant-guides/cursor-use-case-owner-coder.md` for the use-case owner role.

