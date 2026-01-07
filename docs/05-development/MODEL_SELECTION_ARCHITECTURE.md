# Model Selection Architecture

This document explains the architecture of model selection in Converge, addressing the question: **"How does selection work when users provide API keys at runtime, and pricing changes dynamically?"**

## The Core Question

> "Some aspects/criteria will not change (capabilities, data sovereignty), but e.g pricing will. How do we make a selection if in runtime a user has provided KEYS? Or is the selection always in code, no dynamic selection, or should it work?"

## Answer: Two-Layer Architecture

Converge uses a **two-layer selection architecture**:

1. **Static Layer** (`ModelSelector`): All models, compile-time metadata
2. **Runtime Layer** (`ProviderRegistry`): Available providers, dynamic metadata

This allows:
- ✅ **Static aspects** (capabilities, sovereignty) defined in code
- ✅ **Dynamic aspects** (pricing, latency) updated at runtime
- ✅ **Runtime availability** (API keys) checked at runtime
- ✅ **Selection always works** based on what's actually available

## Architecture Diagram

```
┌─────────────────────────────────────────────────────────┐
│  Static Layer: ModelSelector                           │
│  - All possible models (14 providers)                  │
│  - Static metadata (capabilities, sovereignty)          │
│  - Default pricing/latency estimates                   │
│  - Defined at compile-time                            │
└─────────────────────────────────────────────────────────┘
                          ↓
┌─────────────────────────────────────────────────────────┐
│  Runtime Layer: ProviderRegistry                       │
│  - Filters by available providers (API keys)           │
│  - Applies dynamic metadata overrides                  │
│  - Returns best match from available options           │
└─────────────────────────────────────────────────────────┘
                          ↓
┌─────────────────────────────────────────────────────────┐
│  Factory: create_provider()                            │
│  - Creates actual provider instance                    │
│  - Uses selected provider name and model ID            │
└─────────────────────────────────────────────────────────┘
```

## Static vs Dynamic Metadata

### Static (Defined in Code)

These don't change and are defined in `ModelSelector`:

- **Capabilities**: Reasoning, web search, multilingual
- **Data Sovereignty**: EU, Switzerland, China, etc.
- **Compliance**: GDPR, SOC2, HIPAA
- **Quality Characteristics**: Model quality scores

**Why static?** These are inherent to the model/provider, not pricing.

### Dynamic (Updated at Runtime)

These can change and are updated via `ProviderRegistry.update_metadata()`:

- **Pricing**: Cost classes (VeryLow → VeryHigh)
- **Latency**: Typical response times
- **Availability**: Which providers have API keys

**Why dynamic?** These change over time or depend on runtime configuration.

## Selection Flow

### 1. Design Time (Static)

```rust
// Define requirements
let reqs = AgentRequirements::fast_cheap()
    .with_data_sovereignty(DataSovereignty::Switzerland)
    .with_compliance(ComplianceLevel::GDPR);

// Select from all possible models
let selector = ModelSelector::new();
let (provider, model) = selector.select(&reqs)?;
// → "apertus" / "apertus-v1"
```

**Use case**: Testing, documentation, design.

### 2. Runtime (Dynamic)

```rust
// User provides API keys (via .env, config, or UI)
std::env::set_var("ANTHROPIC_API_KEY", "sk-ant-...");
std::env::set_var("APERTUS_API_KEY", "apertus-...");

// Create registry (checks which providers are available)
let registry = ProviderRegistry::from_env();
// Only considers providers with API keys

// Select from available providers
let reqs = AgentRequirements::fast_cheap()
    .with_data_sovereignty(DataSovereignty::Switzerland);
let (provider, model) = registry.select(&reqs)?;
// → Only returns models from providers with keys

// Create provider instance
let provider = create_provider(&provider, &model)?;
```

**Use case**: Production runtime.

## Handling Dynamic Pricing

### Option 1: Periodic Updates

```rust
// Background task updates pricing
async fn update_pricing_periodically(registry: &mut ProviderRegistry) {
    loop {
        let pricing = fetch_latest_pricing().await?;
        
        for (provider, model, new_cost_class) in pricing {
            let mut metadata = get_base_metadata(provider, model);
            metadata.cost_class = new_cost_class;
            registry.update_metadata(provider, model, metadata);
        }
        
        tokio::time::sleep(Duration::from_secs(3600)).await; // Every hour
    }
}
```

### Option 2: User Configuration

```rust
// User provides pricing overrides
let mut registry = ProviderRegistry::from_env();

// User says: "Anthropic Haiku is now Low cost (not VeryLow)"
let updated = ModelMetadata::new(
    "anthropic",
    "claude-3-5-haiku-20241022",
    CostClass::Low, // User override
    1500,
    0.75,
);
registry.update_metadata("anthropic", "claude-3-5-haiku-20241022", updated);
```

### Option 3: Monitoring-Based Updates

```rust
// Track actual costs and update estimates
fn update_from_actual_costs(registry: &mut ProviderRegistry, costs: &[CostRecord]) {
    for cost in costs {
        let cost_class = classify_cost(cost.actual_cost_per_token);
        let mut metadata = get_base_metadata(&cost.provider, &cost.model);
        metadata.cost_class = cost_class;
        registry.update_metadata(&cost.provider, &cost.model, metadata);
    }
}
```

## Selection Always Works

The key insight: **Selection always works** because:

1. **Static layer** has all models → Selection logic always has candidates
2. **Runtime layer** filters by availability → Only returns what's possible
3. **Clear errors** when nothing is available → User knows what to fix

```rust
match registry.select(&reqs) {
    Ok((provider, model)) => {
        // Success: Found a model from available providers
    }
    Err(e) => {
        // Clear error: "No available model. Available providers: [anthropic, openai]"
        // or "Available providers: [none (set API keys)]"
    }
}
```

## Code vs Runtime Selection

### Selection Logic: Always in Code

The **selection algorithm** is always in code:
- Requirements matching
- Fitness scoring
- Best match selection

This ensures consistency and correctness.

### Provider Availability: Runtime

The **provider availability** is checked at runtime:
- Which API keys are set
- Which providers can be instantiated
- Dynamic metadata overrides

This ensures flexibility and practicality.

## Best Practices

### 1. Use Static Selection for Design

```rust
// When designing agents, use static selector
let selector = ModelSelector::new();
let (provider, model) = selector.select(&reqs)?;
// Works even without API keys
```

### 2. Use Runtime Selection for Production

```rust
// In production, use runtime registry
let registry = ProviderRegistry::from_env();
let (provider, model) = registry.select(&reqs)?;
// Only considers available providers
```

### 3. Update Pricing Periodically

```rust
// Update pricing from external source
let mut registry = ProviderRegistry::from_env();
update_pricing(&mut registry).await?;
```

### 4. Handle Missing Providers Gracefully

```rust
match registry.select(&reqs) {
    Ok((provider, model)) => {
        // Use provider
    }
    Err(e) => {
        // Log error, suggest which keys to set
        tracing::warn!("Model selection failed: {}", e);
    }
}
```

## Summary

- **Static metadata** (capabilities, sovereignty) → Defined in code
- **Dynamic metadata** (pricing, latency) → Updated at runtime
- **Provider availability** → Checked at runtime (API keys)
- **Selection logic** → Always in code (consistent algorithm)
- **Selection result** → Always works (filters by availability)

This architecture ensures:
- ✅ Selection works at runtime with user-provided keys
- ✅ Pricing can be updated dynamically
- ✅ Static aspects don't need updates
- ✅ Clear errors when providers aren't available
- ✅ Consistent selection algorithm

