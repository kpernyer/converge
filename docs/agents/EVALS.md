# Converge — Eval System

This document describes the **eval system** in Converge, which provides formal definitions of acceptable outcomes.

---

## Philosophy

In Converge, **evals are not tests of behavior — they are formal definitions of acceptable outcomes**.

This aligns with the principle that:
- Evals test whether a convergence outcome satisfies intent-level properties
- Evals are outcome-based, not path-based
- Evals are reusable across models, agents, and time
- Evals are stored as traceable artifacts
- Evals can be used in invariant checks

This makes evals a **competitive moat** because Converge's explicit semantics make robust evals possible at all.

---

## Eval Layers

Evals exist at three layers, each with a different purpose:

### 1. Engine-Level Evals (Foundational, Non-Negotiable)

These are axioms, like the tests in `converge-core/tests/`:
- No hidden state
- No starvation
- Deterministic convergence
- Idempotency via context
- Dependency completeness

These are not optional. They define whether Converge itself is correct.

**You already have this. This is your bedrock moat.**

### 2. Domain-Level Evals (The Competitive Moat)

This is the most important layer.

These evals answer questions like:
- Did the SDR funnel produce diverse, qualified leads?
- Did HR policy rollout reach organizational understanding, not just clicks?
- Did routing decisions respect cost and risk bounds?

These are:
- Business semantics
- Intent-relative
- Domain-specific
- Outcome-oriented

And they map directly to Gherkin.

**Example:**

```rust
struct StrategyDiversityEval;

impl Eval for StrategyDiversityEval {
    fn name(&self) -> &str { "strategy_diversity" }
    fn description(&self) -> &str {
        "Ensures at least 3 distinct strategies exist with no two targeting the same primary channel"
    }

    fn evaluate(&self, ctx: &Context) -> EvalResult {
        let strategies = ctx.get(ContextKey::Strategies);
        // ... evaluation logic ...
    }
}
```

This is not a test of agents. It's a test of meaningful convergence.

### 3. Agent-Level Evals (Useful, But Subordinate)

These are still useful:
- Prompt quality
- Output format
- Hallucination containment
- Safety constraints

But in Converge, these should be:
- Scoped
- Local
- Non-authoritative

They never override:
- Invariants
- Domain evals
- Human authority

Think of them as quality hints, not truth.

---

## Eval vs Invariant

| Aspect | Eval | Invariant |
|--------|------|-----------|
| **Purpose** | Evaluative (measure quality) | Prescriptive (must hold) |
| **Outcome** | Returns result (pass/fail/indeterminate) | Returns violation or Ok |
| **Execution** | Can be run on-demand | Checked at specific points |
| **Storage** | Results stored as facts | Violations block execution |
| **Usage** | Can be used in invariant checks | Blocks convergence if violated |

**Key Insight:** Evals define what "good" means. Invariants enforce what "must" be true.

---

## Eval API

### Eval Trait

```rust
pub trait Eval: Send + Sync {
    fn name(&self) -> &str;
    fn description(&self) -> &str;
    fn evaluate(&self, ctx: &Context) -> EvalResult;
    fn dependencies(&self) -> &[ContextKey] { &[] }
}
```

### EvalResult

```rust
pub struct EvalResult {
    pub eval_name: String,
    pub outcome: EvalOutcome,  // Pass, Fail, or Indeterminate
    pub score: f64,             // 0.0 - 1.0
    pub rationale: String,
    pub fact_ids: Vec<String>,
    pub metadata: Option<String>,
}
```

### EvalRegistry

```rust
pub struct EvalRegistry {
    // ...
}

impl EvalRegistry {
    pub fn register(&mut self, eval: impl Eval + 'static) -> EvalId;
    pub fn evaluate_all(&self, ctx: &Context) -> Vec<EvalResult>;
    pub fn evaluate_dependent(&self, ctx: &Context, dirty_keys: &[ContextKey]) -> Vec<EvalResult>;
}
```

---

## Usage Patterns

### Pattern 1: Agent-Based Eval Execution

Create an agent that runs evals and stores results:

```rust
use converge_core::{Agent, AgentEffect, Context, ContextKey, Eval, EvalRegistry, Fact};
use converge_domain::eval_agent::EvalExecutionAgent;

let mut agent = EvalExecutionAgent::new("domain_evals");
agent.register_eval(StrategyDiversityEval);
agent.register_eval(LeadQualificationQualityEval);

// Register agent with engine
engine.register(agent);
```

The agent:
- Runs when eval dependencies change
- Executes registered evals
- Stores results as facts in `ContextKey::Evaluations`
- Is idempotent (checks for existing eval results)

### Pattern 2: Invariant That Uses Eval Results

Create an invariant that checks eval results:

```rust
use converge_core::{Invariant, InvariantClass, InvariantResult, Violation};

struct RequirePassingEvals {
    eval_names: Vec<String>,
}

impl Invariant for RequirePassingEvals {
    fn name(&self) -> &str { "require_passing_evals" }
    fn class(&self) -> InvariantClass { InvariantClass::Acceptance }

    fn check(&self, ctx: &Context) -> InvariantResult {
        let evaluations = ctx.get(ContextKey::Evaluations);
        
        for eval_name in &self.eval_names {
            let has_passing = evaluations.iter().any(|e| {
                e.id.contains(eval_name) && e.content.contains("Pass")
            });
            
            if !has_passing {
                return InvariantResult::Violated(Violation::new(
                    format!("Eval '{}' did not pass", eval_name)
                ));
            }
        }
        
        InvariantResult::Ok
    }
}
```

### Pattern 3: Multiple Convergence Attempts with Eval Scoring

Run multiple convergence attempts, score them with evals, then converge over eval results:

```rust
// Try 3 model configs
let configs = vec![config1, config2, config3];
let mut results = Vec::new();

for config in configs {
    let mut engine = Engine::new();
    // ... setup agents with config ...
    let result = engine.run(context.clone())?;
    
    // Score with evals
    let eval_results = eval_registry.evaluate_all(&result.context);
    results.push((result, eval_results));
}

// Higher-level agent selects best based on eval scores
let best = results.iter()
    .max_by_key(|(_, evals)| {
        evals.iter()
            .filter(|e| e.outcome == EvalOutcome::Pass)
            .map(|e| e.score)
            .sum::<f64>()
    })
    .unwrap();
```

This is not trial-and-error. It's structured search over meaning.

---

## Gherkin Integration

Evals can be expressed in Gherkin as evaluative assertions:

```gherkin
Scenario: Strategy diversity
Given a growth strategy convergence
Then the final context must contain
  - at least 3 distinct strategies
  - no two strategies targeting the same primary channel
```

This is:
- Explainable
- Testable
- Model-agnostic
- Human-readable
- Machine-checkable

**Rule:** Gherkin never specifies how. Only what must be true.

---

## Best Practices

### Do

- ✅ Treat evals as contracts on context
- ✅ Version evals alongside domain specs
- ✅ Run evals:
  - At convergence
  - After resume
  - Across model swaps
- ✅ Store eval results as facts or diagnostic facts
- ✅ Use evals in invariant checks

### Don't

- ❌ Tie evals to specific agent paths
- ❌ Encode prompt logic in evals
- ❌ Let evals mutate context
- ❌ Auto-fix failures without authority
- ❌ Use evals as control flow

---

## Examples

See `converge-domain/src/evals.rs` for domain-level eval examples:
- `StrategyDiversityEval` — Growth strategy diversity
- `LeadQualificationQualityEval` — SDR lead quality
- `MeetingScheduleFeasibilityEval` — Meeting scheduler feasibility

See `converge-domain/src/eval_agent.rs` for the eval execution agent pattern.

---

## Competitive Advantage

Most companies:
- Can't define evals clearly
- Because their systems don't have explicit semantics
- And their agents mutate state implicitly

Converge has:
- Explicit context
- Append-only meaning
- Deterministic convergence points
- Named invariants
- Traceable provenance

That makes robust evals possible at all.

Without this foundation:
- Evals degrade into brittle prompt tests
- Noise overwhelms signal
- Teams thrash

With Converge:
- Evals become stable artifacts
- Reusable across models
- Reusable across agents
- Reusable across time

**That's the moat.**

---

## One-Sentence Summary

> In Converge, evals are not tests of behavior — they are formal definitions of acceptable outcomes.

---

## Related Documents

- [`INVARIANTS.md`](../testing/INVARIANTS.md) — Invariant system
- [`ENGINE_EXECUTION_MODEL.md`](../architecture/ENGINE_EXECUTION_MODEL.md) — Execution model
- [`CONVERGENCE_SEMANTICS.md`](../architecture/CONVERGENCE_SEMANTICS.md) — Convergence guarantees
