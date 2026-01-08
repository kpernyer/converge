# Converge — Use Case: Strategic Sourcing Runtime

## Purpose of this document

This document describes a **Strategic Sourcing** use case implemented on the
**Converge Agent OS**, at a **high architectural and semantic level**.

It demonstrates:
- wide fan-out (many suppliers evaluated)
- narrow fan-in (shortlist consolidation)
- multi-criteria vendor evaluation
- deterministic ranking

This is **not** a workflow engine.
This is a **bounded vendor selection runtime**.

---

## 1. Business Problem

Organizations need to select vendors for strategic sourcing based on:
- cost competitiveness
- ESG (Environmental, Social, Governance) scores
- compliance status
- risk profiles
- quality and capacity

The challenge is not a lack of vendors, but:
- evaluating many suppliers across multiple criteria
- consolidating evaluations into a shortlist
- ranking vendors with transparent rationale
- ensuring compliance requirements are met

The system must:
- discover and profile suppliers in parallel
- evaluate multiple criteria simultaneously
- consolidate evaluations into shortlist
- rank vendors with explainable rationale

---

## 2. Root Intent (Operational Scope)

Everything starts with a Root Intent.

### Natural language intent

> "Select optimal vendors for component sourcing based on cost, ESG, compliance, and risk criteria."

### Gherkin — Root Intent Declaration

```gherkin
Feature: Strategic vendor selection

Scenario: Define sourcing intent
  Given sourcing requirements exist
  And multiple vendors are available
  Then the system evaluates and ranks vendors
```

---

## 3. Questions the Runtime Must Answer

- Which suppliers are available?
- What are compliance statuses?
- What are ESG scores?
- How do prices compare to benchmarks?
- What are risk profiles?
- Which vendors meet all requirements?
- How should vendors be ranked?
- What is the rationale for rankings?

These questions are explored **in parallel** across suppliers and criteria.

---

## 4. Context (High-Level View)

Initial context:

```
Context₀
├─ Requirements: Component sourcing
├─ Criteria:
│   ├─ Cost
│   ├─ ESG
│   ├─ Compliance
│   └─ Risk
└─ Suppliers: ∅
```

Context evolves as suppliers are discovered and evaluated.

---

## 5. Classes of Agents Involved

### Discovery and Evaluation Agents (Wide Fan-Out)
- **SupplierDiscoveryAgent** — Discovers and profiles suppliers
- **ComplianceAgent** — Checks compliance status
- **ESGScoringAgent** — Evaluates ESG scores
- **PriceBenchmarkAgent** — Compares prices to benchmarks
- **RiskModelAgent** — Assesses risk profiles

### Consolidation Agents (Narrow Fan-In)
- **SourcingStrategyAgent** — Creates shortlist from evaluations
- **VendorRankingAgent** — Ranks vendors with rationale

---

## 6. Execution Model

The runtime executes in cycles:

1. Supplier discovery runs (wide fan-out)
2. Evaluation agents run in parallel for each supplier
3. Multiple criteria evaluated simultaneously
4. SourcingStrategyAgent consolidates into shortlist
5. VendorRankingAgent ranks shortlisted vendors
6. Convergence occurs when ranking is complete

Execution continues until:
- all suppliers are evaluated
- shortlist is created
- vendors are ranked

---

## 7. Progressive Convergence

### Early convergence
> "Initial vendor evaluations complete. Shortlisting in progress."

### Primary convergence
> "Top 3 vendors ranked. Vendor A: Best overall, Vendor B: Best cost, Vendor C: Best ESG."

### Extended convergence
Background refinement may continue, but ranking is stable.

---

## 8. Outputs of the Runtime

- Supplier profiles and capabilities
- Compliance status per supplier
- ESG scores and assessments
- Price benchmark comparisons
- Risk profiles and assessments
- Shortlisted vendors
- Ranked vendor list with rationale
- Recommendation with tradeoff analysis

---

## 9. Gherkin — Sourcing Invariants

### Structural invariants

```gherkin
Scenario: Complete assessments
  When the system converges
  Then all suppliers have been evaluated
  And all criteria have been assessed
```

### Semantic invariants

```gherkin
Scenario: Compliant vendor
  When a vendor is recommended
  Then the vendor meets all compliance requirements
  And compliance violations are explicitly documented
```

### Acceptance invariants

```gherkin
Scenario: Shortlist compliance
  When the system converges
  Then all shortlisted vendors meet minimum requirements
  And rankings are explained with rationale
```

---

## 10. End-Value and Proof Points

### What This Use-Case Proves

The Strategic Sourcing runtime demonstrates that **Converge is the ultimate way to handle reliable business agents** by proving:

#### 1. **Wide Fan-Out Without Coordination Complexity**

**The Problem:** Traditional systems evaluate suppliers sequentially or use complex orchestration that creates bottlenecks.

**Converge's Solution:**
- Multiple evaluation agents run in parallel for each supplier
- No explicit coordination — agents declare dependencies on context keys
- Wide fan-out happens naturally through parallel execution
- Deterministic evaluation — same suppliers produce same assessments

**Why It Matters:** Strategic sourcing requires evaluating many suppliers simultaneously. Converge enables true parallel evaluation without coordination overhead.

#### 2. **Narrow Fan-In Consolidation**

**The Problem:** Traditional systems use message passing or event buses to consolidate parallel evaluations, creating hidden dependencies.

**Converge's Solution:**
- SourcingStrategyAgent declares dependencies on all evaluation signals
- Consolidation happens automatically when all dependencies are satisfied
- No explicit coordination code — data-driven eligibility
- Full transparency — all evaluations and consolidation logic are explicit

**Why It Matters:** Vendor selection requires consolidating many evaluations into a decision. Converge provides a clean fan-in pattern without message passing.

#### 3. **Multi-Criteria Evaluation**

**The Problem:** Traditional systems evaluate criteria sequentially or use black-box multi-criteria decision models.

**Converge's Solution:**
- Multiple criteria evaluated in parallel (cost, ESG, compliance, risk)
- Each criterion is an explicit fact in context
- Evaluation agents are independent and testable
- Rankings are transparent and explainable

**Why It Matters:** Vendor selection requires balancing multiple criteria. Converge enables transparent multi-criteria evaluation.

#### 4. **Deterministic Vendor Ranking**

**The Problem:** Traditional ranking systems produce unpredictable results or require complex configuration.

**Converge's Solution:**
- All evaluation agents are deterministic — same inputs produce same outputs
- Ranking logic is explicit and transparent
- Rationale is generated for every ranking decision
- Full reproducibility — same suppliers produce same rankings

**Why It Matters:** Vendor selection decisions require confidence. Converge provides deterministic ranking that produces the same results every time.

#### 5. **Compliance Enforcement**

**The Problem:** Traditional systems check compliance as a separate step, making it easy to miss violations.

**Converge's Solution:**
- Compliance is an explicit fact checked in parallel with other criteria
- Compliance violations block vendor selection automatically
- Violations are explicit facts with full provenance
- Shortlist only includes compliant vendors

**Why It Matters:** Vendor selection requires compliance guarantees. Converge enforces compliance through explicit facts and invariants.

### End-Value Delivered

**For Procurement Teams:**
- **Fast vendor evaluation** through parallel assessment
- **Optimal vendor selection** based on multiple criteria
- **Transparent rankings** with explainable rationale
- **Compliance guarantees** — only compliant vendors are considered

**For Technical Teams:**
- **Wide fan-out evaluation** without coordination complexity
- **Deterministic ranking** — reproducible decisions
- **Maintainable architecture** — agents are independent and testable
- **Extensible design** — new criteria can be added easily

**For the Industry:**
- **Proof that multi-criteria evaluation can be deterministic** and explainable
- **Demonstration that wide fan-out/narrow fan-in patterns work** without message passing
- **Evidence that compliance can be enforced** transparently
- **Foundation for vendor selection systems** that procurement teams can trust

### Why This Matters to Proving Converge

The Strategic Sourcing use-case proves that Converge's model works for **multi-criteria decision-making**:

1. **It requires wide fan-out** — proving that context-driven coordination enables parallel evaluation at scale
2. **It needs narrow fan-in** — proving that consolidation patterns work for decision-making
3. **It involves multi-criteria evaluation** — proving that transparent multi-objective decisions work
4. **It demands determinism** — proving that correctness guarantees matter for strategic decisions
5. **It solves real-world problems** — proving that Converge is practical for procurement

**Traditional vendor selection systems use sequential workflows or black-box ranking that cannot guarantee determinism or explainability.** Converge provides both, proving that the convergence model is superior for strategic sourcing.

---

## 11. One-Sentence Summary

> The Strategic Sourcing runtime converges on optimal vendor rankings through wide fan-out evaluation and narrow fan-in consolidation, providing transparent multi-criteria assessment with compliance guarantees.

