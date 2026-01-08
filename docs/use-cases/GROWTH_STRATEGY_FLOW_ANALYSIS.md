# Growth Strategy Use Case - Flow Analysis

## Current Problem: Underspecified Root Intent

The current test setup is too minimal. We're just providing:
- Market: "Nordic B2B SaaS"
- Product: "Converge platform"

But there's **no actual business question** or **strategic objective**. This makes it impossible for the system to generate meaningful, actionable strategies.

## What a Proper Root Intent Should Include

### 1. Business Objective (The "Why")
```
Objective: "Increase annual recurring revenue (ARR) by 50% in the next 12 months"
```

### 2. Constraints (The "What We Can't Do")
```
Constraints:
  - Budget: $500K marketing spend
  - Timeline: 12 months
  - Brand safety: No aggressive tactics, GDPR compliant
  - Geographic: Focus on Nordic region initially
  - Product: Current product capabilities (no new features in timeline)
```

### 3. Success Criteria (The "How We Measure")
```
Success Criteria:
  - At least 3 distinct, viable growth strategies
  - Each strategy must have:
    - Clear target customer segment
    - Estimated CAC (Customer Acquisition Cost)
    - Estimated LTV (Lifetime Value)
    - Risk assessment
    - Implementation timeline
  - Strategies must be ranked by:
    - Revenue potential
    - Feasibility
    - Risk-adjusted ROI
```

### 4. Current State (The "Where We Are")
```
Current State:
  - Product: Converge - multi-agent runtime system
  - Market position: Early stage, niche product
  - Current customers: 10 enterprise customers
  - Current ARR: $500K
  - Target ARR: $750K (50% growth)
  - Primary channel: Direct sales
  - Brand: Technical, correctness-focused
```

## Expected Flow Graph

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                           ROOT INTENT                                       │
│                                                                             │
│  Objective: Grow ARR by 50% ($500K → $750K) in 12 months                    │
│  Constraints: $500K budget, Nordic focus, GDPR compliant, brand safety      │
│  Success: 3+ ranked strategies with CAC/LTV/risk analysis                  │
└─────────────────────────────────────────────────────────────────────────────┘
                                    │
                                    ▼
┌─────────────────────────────────────────────────────────────────────────────┐
│                         CYCLE 1: DISCOVERY                                  │
└─────────────────────────────────────────────────────────────────────────────┘
                                    │
        ┌───────────────────────────┼───────────────────────────┐
        │                           │                           │
        ▼                           ▼                           ▼
┌──────────────────┐      ┌──────────────────┐      ┌──────────────────┐
│ MarketSignalAgent│      │ CompetitorAgent  │      │ ConstraintAgent │
│ (Gemini Flash)   │      │ (Perplexity)     │      │ (Deterministic)  │
│                  │      │                  │      │                  │
│ Extracts:        │      │ Finds:           │      │ Enforces:        │
│ - Market trends  │      │ - Competitors    │      │ - Budget limits  │
│ - Customer needs │      │ - Positioning    │      │ - Brand rules    │
│ - Pain points    │      │ - Pricing        │      │ - Timeline       │
│ - Opportunities  │      │ - Channels       │      │ - Geography      │
└──────────────────┘      └──────────────────┘      └──────────────────┘
        │                           │                           │
        └───────────────────────────┼───────────────────────────┘
                                    │
                                    ▼
                        ┌───────────────────────┐
                        │ ValidationAgent       │
                        │ (Validates proposals) │
                        └───────────────────────┘
                                    │
                                    ▼
┌─────────────────────────────────────────────────────────────────────────────┐
│                         CONTEXT STATE (After Cycle 1)                       │
│                                                                             │
│  Signals:                                                                    │
│    - "Nordic B2B SaaS market growing 15% YoY"                              │
│    - "Enterprise customers value correctness & reliability"                 │
│    - "Multi-agent systems emerging trend"                                   │
│    - "LinkedIn most effective B2B channel in Nordics"                       │
│                                                                             │
│  Competitors:                                                                │
│    - "LangChain: Focus on developer tools, strong brand"                   │
│    - "AutoGPT: Consumer-focused, viral growth"                              │
│    - "Local Nordic players: Small, relationship-based"                      │
│                                                                             │
│  Constraints:                                                                │
│    - Budget: $500K remaining                                                │
│    - Timeline: 12 months                                                     │
│    - Brand: Technical, trustworthy                                          │
└─────────────────────────────────────────────────────────────────────────────┘
                                    │
                                    ▼
┌─────────────────────────────────────────────────────────────────────────────┐
│                         CYCLE 2: SYNTHESIS                                  │
└─────────────────────────────────────────────────────────────────────────────┘
                                    │
                                    ▼
                        ┌───────────────────────┐
                        │ StrategyAgent        │
                        │ (Claude Sonnet)      │
                        │                      │
                        │ Synthesizes:         │
                        │ - Channel strategies │
                        │ - Positioning        │
                        │ - Pricing            │
                        │ - Messaging          │
                        └───────────────────────┘
                                    │
                                    ▼
                        ┌───────────────────────┐
                        │ ValidationAgent       │
                        │ (Validates strategies)│
                        └───────────────────────┘
                                    │
                                    ▼
┌─────────────────────────────────────────────────────────────────────────────┐
│                         CONTEXT STATE (After Cycle 2)                       │
│                                                                             │
│  Strategies:                                                                 │
│                                                                             │
│    1. "Enterprise Direct Sales Expansion"                                   │
│       - Target: Nordic enterprises (500+ employees)                         │
│       - Channel: Direct sales + LinkedIn                                    │
│       - CAC: $15K, LTV: $150K, ROI: 10x                                    │
│       - Risk: Medium (long sales cycles)                                    │
│       - Timeline: 6-9 months to first deals                                 │
│                                                                             │
│    2. "Developer Community Building"                                        │
│       - Target: Technical decision makers                                  │
│       - Channel: GitHub, technical content, conferences                     │
│       - CAC: $2K, LTV: $50K, ROI: 25x                                       │
│       - Risk: Low (organic growth)                                          │
│       - Timeline: 3-6 months to build momentum                               │
│                                                                             │
│    3. "Partner Channel Program"                                            │
│       - Target: System integrators, consultants                             │
│       - Channel: Partner network                                           │
│       - CAC: $5K, LTV: $75K, ROI: 15x                                      │
│       - Risk: Medium (partner dependency)                                   │
│       - Timeline: 4-8 months to establish partnerships                      │
└─────────────────────────────────────────────────────────────────────────────┘
                                    │
                                    ▼
┌─────────────────────────────────────────────────────────────────────────────┐
│                         CYCLE 3: EVALUATION                                 │
└─────────────────────────────────────────────────────────────────────────────┘
                                    │
                                    ▼
                        ┌───────────────────────┐
                        │ EvaluationAgent       │
                        │ (Perplexity 70b)     │
                        │                      │
                        │ Evaluates:            │
                        │ - Revenue potential   │
                        │ - Feasibility         │
                        │ - Risk assessment     │
                        │ - Resource needs      │
                        │ - Competitive fit     │
                        └───────────────────────┘
                                    │
                                    ▼
                        ┌───────────────────────┐
                        │ ValidationAgent       │
                        │ (Validates evaluations)│
                        └───────────────────────┘
                                    │
                                    ▼
┌─────────────────────────────────────────────────────────────────────────────┐
│                         CONTEXT STATE (After Cycle 3)                       │
│                                                                             │
│  Evaluations:                                                                │
│                                                                             │
│    Rank 1: "Developer Community Building"                                   │
│      - Score: 8.5/10                                                        │
│      - Rationale: Highest ROI (25x), lowest risk, fastest to start         │
│      - Revenue potential: $300K ARR (40% of target)                         │
│      - Feasibility: High (team has technical content expertise)             │
│                                                                             │
│    Rank 2: "Enterprise Direct Sales Expansion"                              │
│      - Score: 7.5/10                                                        │
│      - Rationale: Strong LTV, but longer sales cycles                       │
│      - Revenue potential: $400K ARR (53% of target)                        │
│      - Feasibility: Medium (need to hire sales team)                        │
│                                                                             │
│    Rank 3: "Partner Channel Program"                                        │
│      - Score: 6.5/10                                                        │
│      - Rationale: Good balance, but requires partner management             │
│      - Revenue potential: $200K ARR (27% of target)                         │
│      - Feasibility: Medium (need partner program infrastructure)            │
│                                                                             │
│  Convergence: ✓ Achieved                                                    │
│    - 3 strategies generated                                                 │
│    - All strategies evaluated and ranked                                    │
│    - Combined revenue potential: $900K (exceeds $750K target)                │
│    - All constraints satisfied                                              │
└─────────────────────────────────────────────────────────────────────────────┘
```

## Key Insights

### 1. Root Intent Must Be Specific
Without a clear objective, constraints, and success criteria, the LLM agents can't generate actionable strategies. They need:
- **What** we're trying to achieve (objective)
- **Why** it matters (context)
- **What** we can't do (constraints)
- **How** we'll measure success (criteria)

### 2. Multi-Cycle Flow is Essential
- **Cycle 1**: Discovery (market signals, competitors, constraints)
- **Cycle 2**: Synthesis (strategies from discovered information)
- **Cycle 3**: Evaluation (rank and assess strategies)

Each cycle builds on the previous one.

### 3. Validation is Critical
Every LLM output must be validated before becoming a fact:
- MarketSignalAgent → Proposals → ValidationAgent → Signals
- StrategyAgent → Proposals → ValidationAgent → Strategies
- EvaluationAgent → Proposals → ValidationAgent → Evaluations

### 4. Provider Selection Matters
- **Fast/Cheap** (Gemini Flash) for high-volume signal extraction
- **Web Search** (Perplexity) for competitor intelligence and evaluation
- **Reasoning** (Claude Sonnet) for strategy synthesis

## What the Test Should Actually Test

Instead of just "generate strategies", the test should verify:

1. **Discovery Phase**:
   - MarketSignalAgent extracts relevant market signals
   - CompetitorAgent finds actual competitors (via web search)
   - Signals are validated and promoted

2. **Synthesis Phase**:
   - StrategyAgent generates strategies that:
     - Reference specific market signals
     - Address competitive landscape
     - Respect constraints (budget, timeline, brand)
     - Include measurable metrics (CAC, LTV, ROI)

3. **Evaluation Phase**:
   - EvaluationAgent ranks strategies with:
     - Revenue potential analysis
     - Feasibility assessment
     - Risk evaluation
     - Resource requirements

4. **Convergence**:
   - System converges with 3+ strategies
   - All strategies evaluated
   - Success criteria met

## Recommended Test Setup

```rust
// Root Intent (properly specified)
let root_intent = RootIntent {
    objective: "Grow ARR by 50% ($500K → $750K) in 12 months",
    constraints: Constraints {
        budget: 500_000,
        timeline_months: 12,
        brand_safety: true,
        geographic_focus: vec!["Nordic".to_string()],
        gdpr_compliant: true,
    },
    success_criteria: SuccessCriteria {
        min_strategies: 3,
        require_metrics: true, // CAC, LTV, ROI
        require_risk_assessment: true,
    },
    current_state: CurrentState {
        product: "Converge - multi-agent runtime system",
        current_arr: 500_000,
        target_arr: 750_000,
        current_customers: 10,
        primary_channel: "Direct sales",
    },
};

// Seeds from Root Intent
engine.register(SeedAgent::new(
    "objective",
    "Grow ARR by 50% ($500K → $750K) in 12 months"
));
engine.register(SeedAgent::new(
    "constraints",
    "Budget: $500K, Timeline: 12 months, Nordic focus, GDPR compliant"
));
engine.register(SeedAgent::new(
    "current_state",
    "Product: Converge, Current ARR: $500K, Customers: 10 enterprise"
));
```

This gives the LLM agents enough context to generate **meaningful, actionable strategies** rather than generic suggestions.
