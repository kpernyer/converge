# Converge — Use Case: SDR Sales Runtime

## Purpose of this document

This document describes a **Sales Development Representative (SDR) Sales** use case implemented on the
**Converge Agent OS**, at a **high architectural and semantic level**.

It demonstrates:
- **Converging funnel** — Not a linear workflow, but evidence accumulation
- **Cost-aware decisions** — Expensive actions require explicit justification
- **Learning loops** — Outcomes inform future convergence without corrupting truth
- **Human-in-the-loop** — Authority barriers before expensive/brand-critical actions
- **Explicit qualification** — Evidence-based, not opaque scoring

This is **not** a CRM workflow.
This is **not** a lead scoring system.
This is a **bounded qualification and outreach runtime**.

---

## 1. Business Problem

SDR teams need to:
- Identify high-potential prospects
- Qualify leads with evidence, not guesses
- Decide when to contact and what to say
- Learn from outcomes without drift
- Respect cost and brand constraints

The challenge is not a lack of tools, but:
- **Partial signals** — Incomplete information at every stage
- **Expensive actions** — Calls cost time and brand risk
- **Learning loops** — Must improve without corrupting past decisions
- **Human judgment** — Some decisions require human authority
- **Opaque scoring** — Traditional CRMs hide why leads are contacted

The system must:
- Accumulate evidence fragments (not assign scores)
- Converge on qualification through explicit criteria
- Generate message hypotheses before contact
- Make cost-aware channel decisions
- Halt for human approval before expensive actions
- Learn from outcomes without retroactive changes

---

## 2. Root Intent (Operational Scope)

Everything starts with a Root Intent.

### Natural language intent

> "Identify, qualify, and convert high-potential prospects into sales conversations with minimal waste and maximal learning."

### Gherkin — Root Intent Declaration

```gherkin
Feature: SDR sales qualification and outreach

Scenario: Define SDR intent
  Given an ICP definition exists
  And product value propositions are defined
  And constraints are specified (calls/day, emails/day, budget)
  Then the system identifies, qualifies, and prioritizes prospects
  And the system generates message strategies
  And the system requests human approval before expensive actions
  And the system learns from outcomes
```

---

## 3. Questions the Runtime Must Answer

- Who are candidate prospects? (Discovery)
- What evidence supports qualification? (Fit, timing, need, risk)
- What should we say to this prospect? (Message hypotheses)
- Which channel should we use? (Cost vs. value)
- When should we contact? (Timing optimization)
- Should we proceed or wait? (Human-in-the-loop gates)
- What did we learn from outcomes? (Learning without drift)

These questions are answered through **convergence**, not sequential steps.

---

## 4. Context (High-Level View)

Initial context (Ground Truth):

```
Context₀
├─ Seeds:
│   ├─ ICP definition (company size, industry, geography, tech stack)
│   ├─ Product value propositions
│   ├─ Constraints:
│   │   ├─ Calls/day limit
│   │   ├─ Emails/day limit
│   │   ├─ Budget limits
│   │   └─ Brand safety requirements
│   └─ Budget limits (experiments/week)
├─ Signals: ∅
├─ Hypotheses: ∅
├─ Strategies: ∅
└─ Evaluations: ∅
```

Context evolves through phases:

**Phase 1: Lead Discovery**
- `Signals` accumulate: "Company X hired Head of RevOps", "Company Y uses competitor Z"
- These are `ProposedFacts` with weak confidence

**Phase 2: Qualification**
- `Signals` → `Hypotheses`: Evidence fragments (fit, timing, need, risk)
- Qualification converges when evidence thresholds are met

**Phase 3: Message Strategy**
- `Hypotheses` → `Strategies`: Multiple message angles per lead
- Each strategy has rationale and confidence

**Phase 4: Channel & Timing**
- `Strategies` → `Evaluations`: Cost vs. value analysis
- Channel decision (call/email/LinkedIn/nothing)

**Phase 5: Human Approval**
- System halts, presents rationale
- Human decision becomes `Fact`

**Phase 6: Execution**
- Action taken (call/email/message)
- Outcome ingested as `Signal`

**Phase 7: Learning**
- Outcomes analyzed
- Patterns extracted (message effectiveness, timing, channel ROI)
- Future convergence influenced (not past decisions changed)

---

## 5. Classes of Agents Involved

### Discovery Agents (Phase 1)
- **MarketScanAgent** (LLM / scraper / API) — Scans market for candidate companies
- **SignalExtractionAgent** (pure Rust or LLM-assisted) — Extracts weak signals from data
- **DeduplicationAgent** (pure Rust) — Removes duplicates, normalizes identities

**Output**: Candidate leads as `ProposedFacts` in `Signals` with weak confidence.

### Qualification Agents (Phase 2)
- **FitEvidenceAgent** (LLM / rules) — Checks ICP match (company size, industry, geography, tech stack)
- **TimingEvidenceAgent** (LLM / scraper) — Finds recent events (hires, funding, job posts, content)
- **NeedEvidenceAgent** (LLM / API) — Identifies pain signals (public signals, job posts, content)
- **RiskEvidenceAgent** (LLM / rules) — Flags brand mismatches, compliance issues

**Output**: Evidence fragments in `Hypotheses`. No single agent assigns a final verdict.

### Message Strategy Agents (Phase 3)
- **MessageHypothesisAgent** (LLM) — Generates multiple message angles per lead
- **HistoricalOutcomeAgent** (pure Rust + DB) — Finds similar past examples
- **ComplianceAgent** (rules) — Validates brand safety and compliance

**Output**: Multiple message hypotheses in `Strategies` with rationale and confidence.

### Channel & Timing Agents (Phase 4)
- **ChannelDecisionAgent** (LLM / optimizer) — Evaluates cost vs. value per channel
- **TimingOptimizationAgent** (pure Rust + DB) — Considers time-of-day, time-of-week effects
- **SaturationAgent** (pure Rust) — Tracks fatigue and saturation limits

**Output**: Channel and timing decisions in `Evaluations`.

### Human-in-the-Loop Agent (Phase 5)
- **ApprovalGateAgent** (engine-level) — Halts execution, presents rationale
- Human decision becomes `Fact` with provenance

### Execution Agents (Phase 6)
- **CallExecutionAgent** (IO) — Places calls (dumb by design)
- **EmailExecutionAgent** (IO) — Sends emails (dumb by design)
- **MessageExecutionAgent** (IO) — Delivers messages (dumb by design)

**Output**: Actions executed, outcomes ingested as `Signals`.

### Learning Agents (Phase 7)
- **MessageEffectivenessAgent** (pure Rust + DB) — Analyzes message performance by segment
- **TimingEffectivenessAgent** (pure Rust + DB) — Analyzes timing patterns
- **ChannelROIAgent** (pure Rust + DB) — Analyzes channel ROI
- **FalsePositiveAgent** (pure Rust + DB) — Identifies false positive patterns

**Output**: Learning insights in `Signals` that influence future convergence (not past decisions).

---

## 6. Execution Model (Conceptual)

The runtime executes in **cycles**, not steps.

### Convergence Phases

**Phase 0: Ground Truth Initialization**
- Root Intent seeds context with ICP, value props, constraints
- No agents run yet

**Phase 1: Lead Discovery (Wide, Cheap, Parallel)**
- MarketScanAgent, SignalExtractionAgent, DeduplicationAgent run in parallel
- Context accumulates candidate leads as `ProposedFacts` in `Signals`
- No qualification yet — just evidence fragments

**Phase 2: Qualification (Evidence Accumulation)**
- FitEvidenceAgent, TimingEvidenceAgent, NeedEvidenceAgent, RiskEvidenceAgent run
- Each adds evidence fragments to `Hypotheses`
- Qualification converges when:
  - ≥N independent evidence categories present
  - No strong negative signals
  - Evidence threshold met

**Phase 3: Message Strategy (Before Contact)**
- MessageHypothesisAgent generates multiple angles
- HistoricalOutcomeAgent finds similar examples
- ComplianceAgent validates brand safety
- Multiple message hypotheses in `Strategies`

**Phase 4: Channel & Timing (Cost-Aware)**
- ChannelDecisionAgent evaluates cost vs. value
- TimingOptimizationAgent considers time effects
- SaturationAgent checks limits
- Channel decision in `Evaluations`

**Phase 5: Human Approval (Authority Barrier)**
- System halts before expensive/brand-critical actions
- Presents: why this lead, why now, what to say, alternatives rejected
- Human approves/modifies/rejects/defers
- Decision becomes `Fact`

**Phase 6: Execution (Dumb by Design)**
- CallExecutionAgent, EmailExecutionAgent, MessageExecutionAgent execute
- Outcomes ingested as `Signals`

**Phase 7: Learning (Without Drift)**
- MessageEffectivenessAgent, TimingEffectivenessAgent, ChannelROIAgent, FalsePositiveAgent analyze outcomes
- Patterns extracted, future convergence influenced
- Past decisions never retroactively changed

### Convergence Criteria

The system converges when:
- All eligible agents have run
- No new evidence can be added (or budgets exhausted)
- Qualification decisions are explicit (qualified, rejected, or stalled)
- Message strategies are generated for qualified leads
- Channel decisions are made
- Human approvals are obtained (if required)
- Actions are executed (if approved)

**Stalling is not failure** — some leads will stall due to insufficient evidence.

---

## 7. Progressive Convergence

### Early Convergence (Fast, Good-Enough)
- First qualified leads identified
- Basic message strategies generated
- Quick channel decisions
- **Use case**: Immediate outreach for high-confidence leads

### Primary Convergence (Refined)
- All evidence categories evaluated
- Multiple message angles per lead
- Cost-optimized channel decisions
- Human approvals obtained
- **Use case**: Full qualification and outreach pipeline

### Extended Convergence (Background Refinement)
- Learning agents analyze outcomes
- Patterns extracted for future use
- **Use case**: Continuous improvement without invalidating past decisions

---

## 8. Outputs of the Runtime

### Qualified Leads
- List of prospects with explicit evidence
- Qualification rationale (fit, timing, need, risk)
- No opaque scores — just evidence

### Message Strategies
- Multiple message angles per lead
- Rationale for each angle
- Historical examples
- Confidence bounds

### Channel & Timing Decisions
- Recommended channel (call/email/LinkedIn/nothing)
- Timing recommendation
- Cost vs. value analysis
- Rationale

### Human Approval Requests
- Why this lead
- Why now
- What to say
- Alternatives rejected
- Expected value vs. cost

### Execution Outcomes
- Actions taken (calls, emails, messages)
- Outcomes (answered, positive signal, rejection, conversion)
- Ground truth, not scores

### Learning Insights
- Message effectiveness by segment
- Timing effectiveness patterns
- Channel ROI analysis
- False positive patterns
- **Note**: These influence future convergence, not past decisions

---

## 9. Gherkin Invariants

### Structural Invariants (Checked on Every Merge)

```gherkin
Scenario: Require valid ICP definition
  Given a Root Intent exists
  Then the ICP definition must be non-empty
  And the ICP must specify at least company size or industry

Scenario: Require valid constraints
  Given a Root Intent exists
  Then calls/day limit must be > 0
  And emails/day limit must be > 0
  And budget limits must be specified
```

### Semantic Invariants (Checked at End of Cycle)

```gherkin
Scenario: Require evidence for qualification
  Given a lead is marked as "Sales Qualified"
  Then at least 3 independent evidence categories must exist
  And no strong negative signals must be present

Scenario: Require message strategy before contact
  Given a lead is approved for contact
  Then at least one message hypothesis must exist
  And the message hypothesis must have confidence ≥ 0.6

Scenario: Require human approval for expensive actions
  Given a cold call is proposed
  Then qualification threshold must be met
  And message hypothesis confidence must be ≥ 0.7
  And human approval must be obtained
  Otherwise the call is impossible to represent

Scenario: Require brand safety
  Given a message strategy is proposed
  Then it must pass compliance checks
  And it must not violate brand safety rules
```

### Acceptance Invariants (Checked When Convergence Claimed)

```gherkin
Scenario: Require explicit qualification decisions
  Given convergence is claimed
  Then all leads must have explicit status:
    - Qualified (with evidence)
    - Rejected (with reason)
    - Stalled (insufficient evidence)
  And no lead can be in an ambiguous state

Scenario: Require learning without drift
  Given learning insights are generated
  Then they must not retroactively change past decisions
  And they must only influence future convergence
```

---

## 10. End-Value and Proof Points

### What This Use Case Proves About Converge

1. **Convergence Over Workflows**
   - SDR work is a convergence problem, not a linear workflow
   - Evidence accumulates until qualification is explicit
   - Stalling is acceptable (insufficient evidence)

2. **Explicit Qualification**
   - No opaque scores — just evidence fragments
   - Qualification is transparent and explainable
   - "Why did we contact this lead?" is always answerable

3. **Cost-Aware Decisions**
   - Expensive actions require explicit justification
   - Channel decisions consider cost vs. value
   - Budget constraints are invariants, not tuning parameters

4. **Human Authority**
   - System halts before expensive/brand-critical actions
   - Human decisions are facts with provenance
   - No hidden overrides

5. **Learning Without Drift**
   - Outcomes inform future convergence
   - Past decisions never retroactively changed
   - Truth is preserved

6. **Agent Replaceability**
   - Any agent can be replaced (LLM → rules → optimizer → human)
   - System remains correct
   - No hidden coupling

### Why Agent Frameworks Fail Here

**Typical agent systems:**
- ❌ Let agents talk to each other (hidden dependencies)
- ❌ Learn implicitly (drift)
- ❌ Accumulate hidden state (opaque scoring)
- ❌ Optimize locally (no global view)
- ❌ Drift globally (past decisions change)

**Converge succeeds because:**
- ✅ Context is the API (explicit evidence)
- ✅ Convergence is mandatory (explicit qualification)
- ✅ Facts are validated (no opaque scores)
- ✅ Determinism is transparent (always explainable)
- ✅ Learning doesn't corrupt truth (past decisions preserved)

---

## 11. One-Sentence Summary

SDR work isn't a workflow — it's a convergence problem under cost, risk, and learning constraints, which is exactly what Converge is built for.

---

## 12. Implementation Notes

### Context Keys Used

- `Seeds` — ICP definition, value props, constraints
- `Signals` — Candidate leads, evidence fragments, outcomes
- `Hypotheses` — Qualification evidence (fit, timing, need, risk)
- `Strategies` — Message hypotheses
- `Evaluations` — Channel decisions, timing decisions
- `Constraints` — Budget limits, brand safety rules
- `Proposals` — LLM-generated suggestions (before validation)
- `Diagnostic` — Rejection reasons, stall reasons

### Agent Dependencies

```
MarketScanAgent: [] → Signals
SignalExtractionAgent: [Signals] → Signals
DeduplicationAgent: [Signals] → Signals

FitEvidenceAgent: [Signals] → Hypotheses
TimingEvidenceAgent: [Signals] → Hypotheses
NeedEvidenceAgent: [Signals] → Hypotheses
RiskEvidenceAgent: [Signals] → Hypotheses

MessageHypothesisAgent: [Hypotheses] → Strategies
HistoricalOutcomeAgent: [Hypotheses, Signals] → Strategies
ComplianceAgent: [Strategies] → Strategies

ChannelDecisionAgent: [Strategies, Constraints] → Evaluations
TimingOptimizationAgent: [Strategies, Signals] → Evaluations
SaturationAgent: [Evaluations, Signals] → Evaluations

CallExecutionAgent: [Evaluations] → Signals (outcomes)
EmailExecutionAgent: [Evaluations] → Signals (outcomes)
MessageExecutionAgent: [Evaluations] → Signals (outcomes)

MessageEffectivenessAgent: [Signals] → Signals (learning)
TimingEffectivenessAgent: [Signals] → Signals (learning)
ChannelROIAgent: [Signals] → Signals (learning)
FalsePositiveAgent: [Signals] → Signals (learning)
```

### LLM Integration

- **MarketScanAgent** — Uses web search (Perplexity) for market scanning
- **SignalExtractionAgent** — Uses fast extraction models (Gemini Flash) for signal extraction
- **FitEvidenceAgent** — Uses analysis models (Claude Sonnet) for ICP matching
- **TimingEvidenceAgent** — Uses web search (Perplexity) for recent events
- **NeedEvidenceAgent** — Uses analysis models (Claude Sonnet) for pain signal detection
- **RiskEvidenceAgent** — Uses analysis models (Claude Sonnet) for risk assessment
- **MessageHypothesisAgent** — Uses synthesis models (Claude Sonnet) for message generation
- **ChannelDecisionAgent** — Uses reasoning models (Claude Sonnet) for cost vs. value analysis

All LLM outputs are `ProposedFacts` requiring validation before promotion to `Facts`.

---

## 13. Comparison with Traditional CRMs

### Salesforce / HubSpot Approach
- Opaque lead scoring (hidden algorithms)
- Linear workflows (stage-based)
- Implicit learning (scores drift)
- No explicit qualification criteria
- No cost-aware decisions
- No human-in-the-loop gates

### Converge Approach
- Explicit evidence accumulation
- Convergence-based qualification
- Learning without drift (past decisions preserved)
- Explicit qualification criteria (invariants)
- Cost-aware channel decisions
- Human authority barriers before expensive actions

**Key Difference**: Converge answers "Why did we contact this lead?" every time. Traditional CRMs cannot.
