# Model Selection Based on Agent Requirements

This document describes how agents specify their LLM requirements and how the system automatically selects appropriate models.

## Overview

Converge uses **requirements-based model selection** to match agents with optimal LLM models based on their needs:

- **Cost constraints**: Agents specify maximum acceptable cost class
- **Latency requirements**: Agents specify maximum acceptable response time
- **Capability needs**: Agents specify whether they need reasoning or web search

The system automatically selects the best matching model from available providers.

## Agent Requirements

Agents specify their requirements using `AgentRequirements`:

```rust
use converge_core::{AgentRequirements, CostClass};

// Fast, cheap agent (many instances)
let reqs = AgentRequirements::fast_cheap();
// max_cost_class: VeryLow
// max_latency_ms: 2000
// requires_reasoning: false

// Deep research agent
let reqs = AgentRequirements::deep_research();
// max_cost_class: High
// max_latency_ms: 30000
// requires_reasoning: true
// requires_web_search: true

// Custom requirements
let reqs = AgentRequirements::new(
    CostClass::Low,
    2000,
    false,
)
.with_web_search(true)
.with_min_quality(0.8);
```

## Cost Classes

Models are classified by cost:

- **VeryLow**: Haiku, GPT-3.5 Turbo, Gemini Flash (~$0.10-0.50 per 1M tokens)
- **Low**: Sonnet, GPT-4 Turbo (~$1-3 per 1M tokens)
- **Medium**: Opus, GPT-4 (~$5-15 per 1M tokens)
- **High**: Opus-4, GPT-4o (~$15-30 per 1M tokens)
- **VeryHigh**: Specialized models (~$30+ per 1M tokens)

## Model Selection

The `ModelSelector` matches requirements to available models:

```rust
use converge_core::{ModelSelector, AgentRequirements};

let selector = ModelSelector::new(); // Uses default models
let reqs = AgentRequirements::fast_cheap();

match selector.select(&reqs) {
    Ok((provider, model)) => {
        println!("Selected: {} / {}", provider, model);
        // e.g., "anthropic" / "claude-3-5-haiku-20241022"
    }
    Err(e) => {
        eprintln!("No suitable model: {}", e);
    }
}
```

### Selection Algorithm

The selector:

1. **Filters** models that satisfy all requirements:
   - Cost ≤ max_cost_class
   - Latency ≤ max_latency_ms
   - Has reasoning (if required)
   - Supports web search (if required)
   - Quality ≥ min_quality

2. **Scores** candidates by:
   - Cost efficiency (40%): Prefer lower cost within allowed range
   - Latency efficiency (30%): Prefer faster within allowed range
   - Quality (30%): Prefer higher quality

3. **Returns** the highest-scoring model

## Using Requirements in Agents

### Option 1: Specify Requirements in Config

```rust
use converge_core::{
    LlmAgent, LlmAgentConfig, AgentRequirements, ContextKey,
    prompt::PromptFormat,
};

let config = LlmAgentConfig {
    system_prompt: "You are a market analyst".into(),
    prompt_template: "Analyze: {context}".into(),
    prompt_format: PromptFormat::Edn,
    target_key: ContextKey::Hypotheses,
    dependencies: vec![ContextKey::Seeds],
    requirements: Some(AgentRequirements::fast_cheap()),
    ..Default::default()
};

// The agent will use requirements to select a provider
let agent = LlmAgent::new("fast-analyzer", provider, config);
```

### Option 2: Use Provider Factory

```rust
use converge_core::{ModelSelector, AgentRequirements};
use converge_provider::ProviderFactory;

let selector = ModelSelector::new();
let reqs = AgentRequirements::fast_cheap();

let (provider_name, model_id) = selector.select(&reqs)?;
let provider = ProviderFactory::create(&provider_name, &model_id)?;

let config = LlmAgentConfig {
    requirements: Some(reqs),
    ..Default::default()
};

let agent = LlmAgent::new("analyzer", provider, config);
```

## Default Models

The default `ModelSelector` includes:

### Anthropic
- `claude-3-5-haiku-20241022` (VeryLow, 1500ms, quality 0.75)
- `claude-3-5-sonnet-20241022` (Low, 3000ms, quality 0.85, reasoning)
- `claude-3-opus-20240229` (High, 8000ms, quality 0.95, reasoning)

### OpenAI
- `gpt-3.5-turbo` (VeryLow, 1200ms, quality 0.70)
- `gpt-4` (Medium, 5000ms, quality 0.90, reasoning)
- `gpt-4-turbo` (Medium, 4000ms, quality 0.92, reasoning)

### Google Gemini
- `gemini-pro` (Low, 2000ms, quality 0.80)
- `gemini-2.0-flash-exp` (VeryLow, 1000ms, quality 0.75)

### Perplexity (Web Search)
- `pplx-70b-online` (Medium, 4000ms, quality 0.85, web_search)
- `pplx-7b-online` (Low, 2500ms, quality 0.75, web_search)

### Qwen
- `qwen-turbo` (VeryLow, 1500ms, quality 0.70)
- `qwen-plus` (Low, 2500ms, quality 0.80)

### OpenRouter
- Various models routed through OpenRouter

### MinMax
- `abab5.5-chat` (Low, 2000ms, quality 0.75)

### Grok
- `grok-beta` (Medium, 3000ms, quality 0.80)

## Custom Model Registry

You can create a custom selector with your own models:

```rust
use converge_core::{ModelSelector, ModelMetadata, CostClass};

let selector = ModelSelector::empty()
    .with_model(ModelMetadata::new(
        "anthropic",
        "claude-3-5-haiku-20241022",
        CostClass::VeryLow,
        1500,
        0.75,
    ))
    .with_model(ModelMetadata::new(
        "custom-provider",
        "my-model-v1",
        CostClass::Low,
        2000,
        0.80,
    )
    .with_reasoning(true)
    .with_web_search(false));
```

## Use Cases

### High-Volume Fast Agents

For agents that run frequently and need quick, cheap responses:

```rust
let reqs = AgentRequirements::fast_cheap();
// Selects: Haiku, GPT-3.5 Turbo, or Gemini Flash
```

### Deep Research Agents

For agents that need thorough analysis:

```rust
let reqs = AgentRequirements::deep_research();
// Selects: Opus, GPT-4, or similar high-quality models
```

### Web-Enabled Agents

For agents that need current information:

```rust
let reqs = AgentRequirements::balanced()
    .with_web_search(true);
// Selects: Perplexity models or similar
```

### Balanced Agents

For general-purpose agents:

```rust
let reqs = AgentRequirements::balanced();
// Selects: Sonnet, GPT-4 Turbo, or similar
```

## Integration with Existing Code

The requirements system is **optional** and **backward compatible**:

- If `requirements` is `None`, agents use the provider passed to them
- If `requirements` is `Some(...)`, the system selects a model automatically

This allows gradual migration:

```rust
// Old way (still works)
let agent = LlmAgent::new("agent", provider, config);

// New way (with requirements)
let config = LlmAgentConfig {
    requirements: Some(AgentRequirements::fast_cheap()),
    ..config
};
let agent = LlmAgent::new("agent", provider, config);
```

## Best Practices

1. **Specify realistic requirements**: Don't require High cost for simple tasks
2. **Use presets when possible**: `fast_cheap()`, `deep_research()`, `balanced()`
3. **Customize only when needed**: Use `.with_web_search()` or `.with_min_quality()` for specific needs
4. **Monitor actual costs**: Track which models are selected and adjust requirements
5. **Test selection**: Use `list_satisfying()` to see all candidates before selection

## Future Enhancements

- **Dynamic pricing**: Update cost classes based on real-time pricing
- **Latency monitoring**: Adjust typical_latency_ms based on observed performance
- **Quality metrics**: Update quality scores based on validation outcomes
- **Provider health**: Factor in provider availability and error rates
- **Cost budgets**: Enforce per-job or per-agent cost limits

