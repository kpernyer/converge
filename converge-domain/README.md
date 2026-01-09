# converge-domain

Domain-specific agent implementations and reference architectures for the [Converge](https://crates.io/crates/converge-core) runtime.

## Use Cases

### Business Strategy
- **Growth Strategy** - Market signal analysis, competitor intelligence, strategy evaluation
- **Strategic Sourcing** - Vendor assessment, risk scoring, negotiation recommendations

### Operations
- **Supply Chain Optimization** - Multi-warehouse routing, demand forecasting, cost optimization
- **Inventory Rebalancing** - Cross-region transfers, financial impact analysis
- **Resource Routing** - Task-resource matching, constraint satisfaction

### Enterprise
- **Meeting Scheduler** - Multi-participant availability, timezone normalization, conflict resolution
- **Release Readiness** - Quality gates, risk assessment, go/no-go decisions
- **Compliance Monitoring** - Regulation parsing, violation detection, remediation proposals

### Data & CRM
- **Catalog Enrichment** - Product deduplication, schema validation, feed ingestion
- **CRM Account Health** - Churn risk scoring, upsell identification, action prioritization

## Architecture Patterns

Each use case demonstrates:
- **Fan-out/fan-in** - Parallel data collection → consolidated analysis
- **Pipeline stages** - Seeds → Signals → Hypotheses → Strategies → Evaluations
- **Invariant enforcement** - Domain-specific quality gates
- **LLM integration** - Optional LLM-powered variants (`*_llm` modules)

## Installation

```toml
[dependencies]
converge-domain = "0.2"
```

## Example

```rust
use converge_core::{Engine, Context, ContextKey, Fact};
use converge_domain::growth_strategy::{
    MarketSignalAgent, CompetitorAgent, StrategyAgent, EvaluationAgent
};

// Create engine with domain agents
let mut engine = Engine::new();
engine.register(MarketSignalAgent);
engine.register(CompetitorAgent);
engine.register(StrategyAgent);
engine.register(EvaluationAgent);

// Seed with market data
let mut ctx = Context::new();
ctx.add_fact(Fact::new(
    ContextKey::Seeds,
    "market-data",
    "Q4 revenue: $2.3M, growth: 15% YoY"
))?;

// Run to convergence
let result = engine.run(ctx)?;

// Extract evaluated strategies
for fact in result.context.get(ContextKey::Evaluations) {
    println!("Strategy evaluation: {}", fact.content);
}
```

## License

MIT
