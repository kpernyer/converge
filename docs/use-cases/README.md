# Use Cases

This directory contains concrete use-case implementations showing how Converge solves real problems.

## Why These Use-Cases Prove Converge

These use-cases demonstrate that **Converge is the ultimate way to handle reliable business agents** by proving:

### 1. **Correctness-First Architecture**
Every use-case enforces correctness through:
- **Type-safe fact boundaries** — LLM outputs are `ProposedFact`, never `Fact` without validation
- **Compile-time invariants** — Gherkin specs compile to Rust predicates, not runtime strings
- **Monotonic context evolution** — Facts are only added, never mutated, preserving full history
- **Deterministic convergence** — Same inputs produce same outputs, every time

### 2. **Context-Driven Collaboration**
Agents never call each other. They collaborate through shared context:
- **No hidden control flow** — Eligibility is data-driven via dependency declarations
- **Parallel execution** — Agents run concurrently when dependencies are satisfied
- **Serialized commit** — Effects merge deterministically, preserving order
- **Transparent state** — Every decision is traceable through context evolution

### 3. **Progressive Convergence**
Business decisions need answers now, but better answers later:
- **Early convergence** — Fast, good-enough answers for immediate action
- **Primary convergence** — Refined answers with alternatives and rationale
- **Extended convergence** — Background refinement that never invalidates prior results
- **Anytime algorithms** — Interruptible at any point with valid, explainable results

### 4. **Explainable Determinism**
Every outcome is provably correct and explainable:
- **Full provenance** — Every fact includes source, timestamp, and validation status
- **Rationale generation** — Agents explain *why* they made decisions, not just *what*
- **Invariant violations** — Failures are explicit facts, not silent errors
- **Reconstructible execution** — Complete context history enables full audit trails

### 5. **Real-World Applicability**
These use-cases solve actual business problems:
- **Growth Strategy** — Strategic planning under uncertainty with LLM integration
- **Meeting Scheduler** — Constraint satisfaction with progressive refinement
- **Resource Routing** — Deterministic optimization with solver integration
- **Release Readiness** — Parallel quality gates with consolidation patterns
- **Supply Chain** — Multi-track coordination with fan-out/fan-in patterns
- **Inventory Rebalancing** — Multi-region optimization with financial constraints
- **Strategic Sourcing** — Multi-criteria vendor evaluation with wide fan-out
- **Catalog Enrichment** — Data quality at scale with many small decisions
- **CRM Account Health** — Reactive monitoring with multi-signal analysis
- **Compliance Monitoring** — Regulatory compliance with evidence collection

### 6. **What Traditional Systems Cannot Do**

**Workflow engines** fail because:
- ❌ Predefined steps cannot adapt to uncertainty
- ❌ Sequential execution cannot leverage parallelism
- ❌ No convergence guarantees — workflows can loop or deadlock

**Actor systems** fail because:
- ❌ Message passing creates hidden dependencies
- ❌ No shared truth — each actor has isolated state
- ❌ Eventual consistency cannot guarantee correctness

**LLM orchestration frameworks** fail because:
- ❌ No fact validation — LLM outputs treated as truth
- ❌ No convergence model — execution continues indefinitely
- ❌ No explainability — black-box decisions with no provenance

**Converge succeeds** because:
- ✅ **Convergence is mandatory** — Fixed-point detection guarantees termination
- ✅ **Context is the API** — Shared truth enables parallel collaboration
- ✅ **Facts are validated** — Type system enforces ProposedFact → Fact boundary
- ✅ **Determinism is transparent** — Every outcome is explainable and reconstructible

---

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

### [CONVERGE_RELEASE_READINESS_USECASE.md](./CONVERGE_RELEASE_READINESS_USECASE.md)
Release Readiness Runtime — parallel quality gates with consolidation.

**Domain:** Engineering, CI/CD, release management
**Complexity:** Medium (parallel checks, deterministic)
**Key agents:** Parallel check agents, RiskSummaryAgent, ReleaseReadyAgent

### [CONVERGE_SUPPLY_CHAIN_USECASE.md](./CONVERGE_SUPPLY_CHAIN_USECASE.md)
Supply Chain Re-planning Runtime — multi-track optimization with fan-out/fan-in.

**Domain:** Supply chain, logistics, operations
**Complexity:** High (multiple parallel tracks, optimization)
**Key agents:** Data collection, Route generation, Cost estimation, Risk assessment, Consolidation

### [CONVERGE_INVENTORY_REBALANCING_USECASE.md](./CONVERGE_INVENTORY_REBALANCING_USECASE.md)
Inventory Rebalancing Runtime — multi-region optimization with financial constraints.

**Domain:** Inventory management, operations
**Complexity:** High (parallel forecasting, optimization)
**Key agents:** Sales velocity, Inventory, Forecast, Transfer optimization, Financial impact

### [CONVERGE_STRATEGIC_SOURCING_USECASE.md](./CONVERGE_STRATEGIC_SOURCING_USECASE.md)
Strategic Sourcing Runtime — multi-criteria vendor evaluation with wide fan-out.

**Domain:** Procurement, vendor selection
**Complexity:** High (wide fan-out, narrow fan-in)
**Key agents:** Supplier discovery, Compliance, ESG scoring, Price benchmarking, Risk modeling, Vendor ranking

### [CONVERGE_CATALOG_ENRICHMENT_USECASE.md](./CONVERGE_CATALOG_ENRICHMENT_USECASE.md)
Catalog Enrichment Runtime — data quality at scale with many small decisions.

**Domain:** Product catalog, data quality
**Complexity:** Medium (many parallel decisions, strong invariants)
**Key agents:** Feed ingestion, Deduplication, Attribute normalization, Category inference, Pricing validation

### [CONVERGE_CRM_ACCOUNT_HEALTH_USECASE.md](./CONVERGE_CRM_ACCOUNT_HEALTH_USECASE.md)
CRM Account Health Runtime — reactive monitoring with multi-signal analysis.

**Domain:** CRM, customer success, sales
**Complexity:** Medium (reactive agents, continuous monitoring)
**Key agents:** Usage signals, Support tickets, Revenue trends, Churn risk, Upsell opportunities

### [CONVERGE_COMPLIANCE_MONITORING_USECASE.md](./CONVERGE_COMPLIANCE_MONITORING_USECASE.md)
Compliance Monitoring Runtime — regulatory compliance with evidence collection.

**Domain:** Compliance, regulatory, governance
**Complexity:** Medium (evidence collection, violation detection)
**Key agents:** Regulation parsing, Policy rules, Evidence collection, Violation detection, Remediation planning

### [CONVERGE_SDR_SALES_USECASE.md](./CONVERGE_SDR_SALES_USECASE.md)
SDR Sales Runtime — qualification and outreach with evidence accumulation.

**Domain:** Sales development, lead qualification, outreach
**Complexity:** High (converging funnel, cost-aware decisions, learning loops, human-in-the-loop)
**Key agents:** Market scanning, Signal extraction, Qualification evidence, Message strategy, Channel decisions, Human approval gates

## Context Schemas

### [CONTEXT_SCHEMA_GROWTH.md](./CONTEXT_SCHEMA_GROWTH.md)
Typed context schema for the Growth Strategy runtime.

**Shows:**
- ContextKey definitions
- Fact types
- Monotonic evolution rules

## Gherkin Validation Tests

### [GHERKIN_VALIDATION_TESTS.md](./GHERKIN_VALIDATION_TESTS.md)
Comprehensive documentation of Gherkin validation integration tests.

**Shows:**
- All 21 test cases with Gherkin code
- LLM prompts (system and user) for each test
- Request parameters (temperature, max_tokens)
- Expected and actual LLM responses
- Test objectives and validation results

**Purpose**: These tests verify that the Gherkin validator provides early feedback to business developers before compiling bad Gherkin into the system. The validator uses Anthropic Claude to check:
- Business sense (semantic validity)
- Compilability to Rust invariants (technical feasibility)
- Convention compliance (style adherence)

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
10. **End-Value and Proof Points** — What this use-case proves about Converge
11. **One-Sentence Summary** — What is this runtime?

---

## Creating New Use-Cases

To create a new use-case:

1. Copy a similar use-case document as a template
2. Fill in the business problem and Root Intent
3. Define domain-specific ContextKeys
4. Identify agent types needed
5. Write Gherkin invariants
6. Ensure convergence criteria are clear and measurable

See `../assistant-guides/cursor-use-case-owner-coder.md` for the use-case owner role.

