# Converge — Use Case: CRM Account Health Runtime

## Purpose of this document

This document describes a **CRM Account Health** use case implemented on the
**Converge Agent OS**, at a **high architectural and semantic level**.

It demonstrates:
- reactive agents
- continuous monitoring patterns
- multi-signal analysis
- action prioritization

This is **not** a workflow engine.
This is a **bounded account monitoring runtime**.

---

## 1. Business Problem

Sales and customer success teams need to monitor account health:
- usage patterns and trends
- support ticket activity
- revenue trends
- churn risk indicators
- upsell opportunities

The challenge is not a lack of data, but:
- multiple signals that must be analyzed together
- reactive monitoring as data changes
- prioritizing actions across accounts
- continuous assessment needs

The system must:
- monitor multiple signals in parallel
- analyze patterns and trends
- assess risks and opportunities
- prioritize actions with rationale

---

## 2. Root Intent (Operational Scope)

Everything starts with a Root Intent.

### Natural language intent

> "Assess account health for Account123, identify churn risks and upsell opportunities, and prioritize actions."

### Gherkin — Root Intent Declaration

```gherkin
Feature: Account health monitoring

Scenario: Define monitoring intent
  Given account data exists
  And monitoring requirements are defined
  Then the system assesses health and prioritizes actions
```

---

## 3. Questions the Runtime Must Answer

- What are usage patterns and trends?
- What is support ticket activity?
- What are revenue trends?
- What is churn risk?
- What are upsell opportunities?
- How should actions be prioritized?
- What is the rationale for priorities?

These questions are answered through **reactive monitoring** as data changes.

---

## 4. Context (High-Level View)

Initial context:

```
Context₀
├─ Account: Account123
├─ Signals:
│   ├─ Usage metrics
│   ├─ Support tickets
│   └─ Revenue data
└─ Analysis: ∅
```

Context evolves as signals are analyzed and actions are prioritized.

---

## 5. Classes of Agents Involved

### Signal Collection Agents (Reactive)
- **UsageSignalAgent** — Monitors usage metrics and trends
- **SupportTicketAgent** — Analyzes support ticket patterns
- **RevenueTrendAgent** — Tracks revenue trends

### Analysis Agents
- **ChurnRiskAgent** — Assesses churn risk
- **UpsellOpportunityAgent** — Identifies upsell opportunities

### Decision Agents
- **ActionPrioritizationAgent** — Prioritizes actions with rationale

---

## 6. Execution Model

The runtime executes in cycles:

1. Signal collection agents run reactively as data changes
2. Analysis agents run when signals are available
3. Risk and opportunity assessments are generated
4. ActionPrioritizationAgent ranks actions
5. Convergence occurs when actions are prioritized

Execution continues until:
- all signals are analyzed
- risks and opportunities are assessed
- actions are prioritized

---

## 7. Progressive Convergence

### Early convergence
> "Initial signals analyzed. Risk assessment in progress."

### Primary convergence
> "Account health assessed. High churn risk, 2 upsell opportunities. Top action: Schedule check-in call."

### Extended convergence
Continuous monitoring may continue, but assessment is stable.

---

## 8. Outputs of the Runtime

- Usage metrics and trends
- Support ticket analysis
- Revenue trend analysis
- Churn risk assessment
- Upsell opportunity identification
- Ranked action list with rationale
- Recommended actions with priorities

---

## 9. Gherkin — Account Health Invariants

### Structural invariants

```gherkin
Scenario: Complete analysis
  When the system converges
  Then all required signals have been analyzed
  And risk and opportunity assessments are complete
```

### Semantic invariants

```gherkin
Scenario: Churn action plan
  When churn risk is identified
  Then an action plan exists to address the risk
  And the plan includes specific recommendations
```

### Acceptance invariants

```gherkin
Scenario: Action prioritization
  When the system converges
  Then actions are prioritized with rationale
  And priorities reflect risk and opportunity assessments
```

---

## 10. End-Value and Proof Points

### What This Use-Case Proves

The CRM Account Health runtime demonstrates that **Converge is the ultimate way to handle reliable business agents** by proving:

#### 1. **Reactive Agents Without Event Loops**

**The Problem:** Traditional systems use event loops or polling that create latency and complexity.

**Converge's Solution:**
- Agents declare dependencies on context keys
- Agents run reactively when dependencies are satisfied
- No explicit event loops — data-driven eligibility
- Low latency — agents run as soon as data is available

**Why It Matters:** Account monitoring requires reactive responses to data changes. Converge enables reactive agents without event loop complexity.

#### 2. **Multi-Signal Analysis**

**The Problem:** Traditional systems analyze signals sequentially or use complex orchestration.

**Converge's Solution:**
- Multiple signal agents run in parallel
- Analysis agents run when signals are available
- No explicit coordination — data-driven eligibility
- Full transparency — all signals and analyses are explicit

**Why It Matters:** Account health requires analyzing multiple signals together. Converge enables parallel signal analysis without coordination overhead.

#### 3. **Continuous Monitoring Patterns**

**The Problem:** Traditional systems use polling or scheduled jobs that create latency.

**Converge's Solution:**
- Agents run reactively as context evolves
- Monitoring happens continuously through context updates
- No explicit polling — data-driven execution
- Low latency — immediate response to data changes

**Why It Matters:** Account monitoring requires continuous assessment. Converge enables continuous monitoring through reactive agents.

#### 4. **Deterministic Action Prioritization**

**The Problem:** Traditional prioritization systems produce unpredictable results or require complex configuration.

**Converge's Solution:**
- All analysis agents are deterministic — same inputs produce same outputs
- Prioritization logic is explicit and transparent
- Rationale is generated for every priority decision
- Full reproducibility — same account data produces same priorities

**Why It Matters:** Action prioritization requires confidence. Converge provides deterministic prioritization that produces the same results every time.

#### 5. **Explainable Risk Assessment**

**The Problem:** Traditional risk systems provide scores without explanations.

**Converge's Solution:**
- Risk assessments include explicit rationale
- Churn risk factors are transparent
- Opportunity identification is explainable
- Action plans reference specific risk factors

**Why It Matters:** Account management requires understanding why risks exist. Converge provides transparent risk assessment.

### End-Value Delivered

**For Sales Teams:**
- **Fast account health assessment** through reactive monitoring
- **Prioritized actions** with clear rationale
- **Transparent risk assessment** — understand why accounts are at risk
- **Reliable prioritization** — same data produces same priorities

**For Technical Teams:**
- **Reactive agents** without event loop complexity
- **Deterministic analysis** — reproducible assessments
- **Maintainable architecture** — agents are independent and testable
- **Extensible design** — new signals can be added easily

**For the Industry:**
- **Proof that reactive monitoring can be deterministic** and explainable
- **Demonstration that multi-signal analysis works** without coordination overhead
- **Evidence that continuous monitoring patterns work** through reactive agents
- **Foundation for account management systems** that sales teams can trust

### Why This Matters to Proving Converge

The CRM Account Health use-case proves that Converge's model works for **reactive monitoring**:

1. **It requires reactive agents** — proving that context-driven coordination enables reactive responses
2. **It needs multi-signal analysis** — proving that parallel signal analysis works transparently
3. **It involves continuous monitoring** — proving that reactive patterns work for ongoing assessment
4. **It demands determinism** — proving that correctness guarantees matter for account decisions
5. **It solves real-world problems** — proving that Converge is practical for CRM

**Traditional CRM systems use polling or event loops that cannot guarantee determinism or explainability.** Converge provides both, proving that the convergence model is superior for reactive monitoring.

---

## 11. One-Sentence Summary

> The CRM Account Health runtime converges on prioritized actions through reactive multi-signal analysis and deterministic risk assessment, providing transparent account health monitoring.

