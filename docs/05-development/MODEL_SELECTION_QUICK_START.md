# Model Selection Quick Start

Quick reference for using requirements-based model selection in Converge.

## Basic Usage

```rust
use converge_core::{AgentRequirements, ModelSelector, CostClass};

// 1. Create a selector (uses default models)
let selector = ModelSelector::new();

// 2. Define requirements
let reqs = AgentRequirements::fast_cheap();
// or
let reqs = AgentRequirements::new(
    CostClass::Low,
    2000,  // max latency in ms
    false, // requires reasoning
);

// 3. Select a model
let (provider, model) = selector.select(&reqs)?;
// e.g., ("anthropic", "claude-3-5-haiku-20241022")
```

## Common Presets

### Fast & Cheap (High Volume)
```rust
let reqs = AgentRequirements::fast_cheap();
// Cost: VeryLow, Latency: ≤2000ms, No reasoning
// Best for: Many agents running frequently
```

### Deep Research
```rust
let reqs = AgentRequirements::deep_research();
// Cost: High, Latency: ≤30000ms, Reasoning: Yes, Web Search: Yes
// Best for: Thorough analysis tasks
```

### Balanced
```rust
let reqs = AgentRequirements::balanced();
// Cost: Medium, Latency: ≤5000ms, No reasoning
// Best for: General-purpose agents
```

## Custom Requirements

```rust
let reqs = AgentRequirements::new(
    CostClass::Low,  // max cost class
    2000,            // max latency (ms)
    false,           // requires reasoning
)
.with_web_search(true)      // enable web search
.with_min_quality(0.85);    // minimum quality threshold
```

## Cost Classes

- `CostClass::VeryLow` - Haiku, GPT-3.5, Gemini Flash
- `CostClass::Low` - Sonnet, GPT-4 Turbo
- `CostClass::Medium` - Opus, GPT-4
- `CostClass::High` - Opus-4, GPT-4o
- `CostClass::VeryHigh` - Specialized models

## Integration with LlmAgent

```rust
use converge_core::{LlmAgent, LlmAgentConfig, AgentRequirements};

let config = LlmAgentConfig {
    requirements: Some(AgentRequirements::fast_cheap()),
    // ... other config
    ..Default::default()
};

// The agent will use requirements to select a provider
let agent = LlmAgent::new("my-agent", provider, config);
```

## See Also

- [`MODEL_SELECTION.md`](./MODEL_SELECTION.md) - Full documentation
- [`converge-core/examples/model_selection_example.rs`](../../converge-core/examples/model_selection_example.rs) - Complete example

