# Runtime Model Selection

This document explains how model selection works at runtime when users provide API keys dynamically.

## The Problem

Model selection has two aspects:

1. **Static Metadata** (doesn't change):
   - Capabilities (reasoning, web search, multilingual)
   - Data sovereignty regions
   - Compliance levels
   - Quality characteristics

2. **Dynamic Metadata** (changes over time):
   - Pricing (cost classes can shift)
   - Latency (varies with load)
   - Provider availability (depends on API keys)

3. **Runtime Availability**:
   - Users provide API keys at runtime
   - Only providers with keys should be considered
   - Selection must work even if keys change

## Solution: Two-Layer Selection

### Layer 1: Static Selection (`ModelSelector`)

The `ModelSelector` contains **all possible models** with static metadata. This is defined at compile-time and includes:

- All providers (even if keys aren't set)
- Static capabilities (reasoning, multilingual, etc.)
- Default pricing/latency estimates

**Use case**: Design-time, testing, documentation.

```rust
use converge_core::{ModelSelector, AgentRequirements};

let selector = ModelSelector::new();
let reqs = AgentRequirements::fast_cheap();
let (provider, model) = selector.select(&reqs)?;
// Returns best model regardless of API key availability
```

### Layer 2: Runtime Selection (`ProviderRegistry`)

The `ProviderRegistry` filters by **available providers** (those with API keys) and allows dynamic metadata updates.

**Use case**: Production runtime, when users provide keys.

```rust
use converge_core::{ProviderRegistry, AgentRequirements};
use converge_provider::create_provider;

// 1. Create registry (checks environment for API keys)
let registry = ProviderRegistry::from_env();

// 2. Select model (only from available providers)
let reqs = AgentRequirements::fast_cheap();
let (provider_name, model_id) = registry.select(&reqs)?;

// 3. Create actual provider instance
let provider = create_provider(&provider_name, &model_id)?;
```

## How It Works

### Step 1: Check Available Providers

The registry checks which providers have API keys:

```rust
let registry = ProviderRegistry::from_env();
// Only considers providers where API keys are set:
// - ANTHROPIC_API_KEY → "anthropic" available
// - OPENAI_API_KEY → "openai" available
// - etc.
```

### Step 2: Filter by Availability

When selecting, the registry:
1. Gets all models that satisfy requirements (from `ModelSelector`)
2. Filters to only those with available providers
3. Applies dynamic metadata overrides (if any)
4. Returns the best match

```rust
let reqs = AgentRequirements::fast_cheap();
let (provider, model) = registry.select(&reqs)?;
// Only returns models from providers with API keys
```

### Step 3: Create Provider Instance

Use the factory to create the actual provider:

```rust
use converge_provider::create_provider;

let provider = create_provider(&provider, &model)?;
// provider is Arc<dyn LlmProvider>
```

## Dynamic Metadata Updates

You can update pricing/latency at runtime:

```rust
use converge_core::{ProviderRegistry, ModelMetadata, CostClass};

let mut registry = ProviderRegistry::from_env();

// Update pricing for a model (e.g., after price change)
let updated = ModelMetadata::new(
    "anthropic",
    "claude-3-5-haiku-20241022",
    CostClass::Low,  // Updated from VeryLow
    1500,
    0.75,
);
registry.update_metadata("anthropic", "claude-3-5-haiku-20241022", updated);

// Future selections will use updated metadata
let (provider, model) = registry.select(&reqs)?;
```

## Complete Example

```rust
use converge_core::{
    AgentRequirements, ProviderRegistry, CostClass, DataSovereignty,
};
use converge_provider::create_provider;
use converge_core::llm::LlmRequest;

// 1. User provides API keys (via environment or config)
std::env::set_var("ANTHROPIC_API_KEY", "sk-ant-...");
std::env::set_var("OPENAI_API_KEY", "sk-...");

// 2. Create registry (checks which providers are available)
let registry = ProviderRegistry::from_env();
println!("Available providers: {:?}", registry.available_providers());
// → ["anthropic", "openai"]

// 3. Define requirements
let reqs = AgentRequirements::fast_cheap()
    .with_data_sovereignty(DataSovereignty::Any);

// 4. Select model (only from available providers)
let (provider_name, model_id) = registry.select(&reqs)?;
println!("Selected: {} / {}", provider_name, model_id);
// → "anthropic" / "claude-3-5-haiku-20241022"

// 5. Create provider instance
let provider = create_provider(&provider_name, &model_id)?;

// 6. Use provider
let response = provider.complete(&LlmRequest::new("Hello!"))?;
```

## Explicit Provider Control

You can also specify which providers are available programmatically:

```rust
// User provides keys via config file or UI
let user_providers = vec!["anthropic", "openai", "gemini"];

let registry = ProviderRegistry::with_providers(&user_providers);
// Only these providers will be considered
```

## When to Use Each

### Use `ModelSelector` (Static) When:
- ✅ Writing tests (don't need real API keys)
- ✅ Documentation examples
- ✅ Design-time analysis
- ✅ Comparing all possible models

### Use `ProviderRegistry` (Runtime) When:
- ✅ Production runtime
- ✅ Users provide API keys dynamically
- ✅ Need to respect actual availability
- ✅ Want to update pricing/latency dynamically

## Integration with Agents

```rust
use converge_core::{
    LlmAgent, LlmAgentConfig, AgentRequirements, ProviderRegistry,
};
use converge_provider::create_provider;

// At runtime, when agent needs to execute:
fn create_agent_for_requirements(
    reqs: AgentRequirements,
) -> Result<LlmAgent, Box<dyn std::error::Error>> {
    // 1. Select model from available providers
    let registry = ProviderRegistry::from_env();
    let (provider_name, model_id) = registry.select(&reqs)?;
    
    // 2. Create provider
    let provider = create_provider(&provider_name, &model_id)?;
    
    // 3. Create agent with requirements
    let config = LlmAgentConfig {
        requirements: Some(reqs),
        ..Default::default()
    };
    
    Ok(LlmAgent::new("agent", provider, config))
}
```

## Pricing Updates

To handle dynamic pricing, you can:

1. **Periodic Updates**: Update metadata from pricing API
2. **User Configuration**: Allow users to override pricing
3. **Monitoring**: Track actual costs and update estimates

```rust
// Example: Update pricing from external source
async fn update_pricing(registry: &mut ProviderRegistry) {
    let pricing = fetch_pricing_from_api().await?;
    
    for (provider, model, cost_class) in pricing {
        let mut metadata = get_base_metadata(provider, model);
        metadata.cost_class = cost_class;
        registry.update_metadata(provider, model, metadata);
    }
}
```

## Error Handling

The registry provides clear errors when no providers are available:

```rust
match registry.select(&reqs) {
    Ok((provider, model)) => {
        // Success
    }
    Err(e) => {
        // Error message includes available providers:
        // "No available model found satisfying requirements. 
        //  Available providers: [anthropic, openai]"
        // or
        // "No available model found satisfying requirements. 
        //  Available providers: [none (set API keys)]"
    }
}
```

## Summary

- **Static Selection** (`ModelSelector`): All models, compile-time metadata
- **Runtime Selection** (`ProviderRegistry`): Only available providers, dynamic updates
- **Factory** (`create_provider`): Creates provider instances from selection
- **Best Practice**: Use `ProviderRegistry` in production, `ModelSelector` for design/testing

This two-layer approach ensures:
- ✅ Selection works at runtime with user-provided keys
- ✅ Static metadata (capabilities) doesn't need updates
- ✅ Dynamic metadata (pricing) can be updated
- ✅ Clear errors when providers aren't available

