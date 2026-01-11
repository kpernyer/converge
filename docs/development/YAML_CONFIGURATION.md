# YAML Configuration Standards

In Converge, YAML is parsed via Serde into strongly typed Rust structures with unknown-field denial and schema validation, making configuration a reliable, versioned contract rather than a suggestion.

## The Canonical Stack

### 1. Serde (foundation)

```rust
serde = { version = "1.0", features = ["derive"] }
```

Serde enforces compile-time structural correctness.

### 2. serde_yaml (YAML → Rust types)

```rust
serde_yaml = "0.9"
```

At parse time you get:
- Missing fields → error
- Wrong types → error
- Invalid enum values → error

This is already stronger than most systems.

### 3. `#[serde(deny_unknown_fields)]` (non-negotiable)

```rust
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct Config {
    timeout_ms: u64,
    mode: Mode,
}
```

Without this:
- YAML silently accepts extra fields
- Config drift goes unnoticed
- Systems rot

With this:
- Unknown fields → hard failure
- Config stays honest

**Rule: If you don't deny unknown fields, you do not have a reliable system.**

### 4. Enums instead of strings (semantic safety)

```rust
#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
enum Mode {
    Development,
    Production,
}
```

This guarantees:
- No typo configs
- No "magic strings"
- No undocumented states

### 5. schemars (schema as contract)

```rust
schemars = "0.8"

use schemars::JsonSchema;

#[derive(Debug, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
struct Config {
    timeout_ms: u64,
    mode: Mode,
}
```

Now you can:
- Generate JSON Schema
- Validate configs before runtime
- Publish a machine-readable contract

This is how you turn config into API, not "just YAML".

## The Rules

### Rule 1: YAML is a transport, never a logic layer

- No conditionals
- No expressions
- No polymorphic structures

YAML describes data, Rust defines meaning.

### Rule 2: Every YAML file maps to one Rust root type

```rust
// No loose structures
struct RootConfig { ... }
```

If it can't deserialize into a root type → reject it.

### Rule 3: Deny unknown fields everywhere

No exceptions.

```rust
#[serde(deny_unknown_fields)]
```

This prevents:
- Version skew
- Silent misconfiguration
- "But it worked before" bugs

### Rule 4: Use enums for all categorical values

Never accept:
```yaml
strategy: "fast"
```

Accept only:
```rust
enum Strategy { Fast, Safe, Balanced }
```

This enforces semantic correctness at parse time.

### Rule 5: Separate parsing from validation

**Parsing:**
- structure
- types

**Validation:**
- cross-field rules
- invariants
- business constraints

Example:
```rust
impl Config {
    fn validate(&self) -> Result<(), ConfigError> {
        if self.timeout_ms == 0 {
            return Err(ConfigError::InvalidTimeout);
        }
        Ok(())
    }
}
```

This mirrors Converge's `ProposedFact` → `Fact` model exactly.

### Rule 6: Fail fast, never auto-correct

- No defaults that hide errors
- No silent coercion
- No "best effort"

If YAML is wrong → system refuses to start.

## Implementation: Model Registry

The model registry (`converge-provider/config/models.yaml`) demonstrates these principles:

### Type-Safe Enums

```rust
/// Cost class for model pricing tier.
#[derive(Debug, Deserialize, JsonSchema)]
pub enum CostClassYaml {
    VeryLow,
    Low,
    Medium,
    High,
    VeryHigh,
}

/// Region enum - type-safe parsing.
#[derive(Debug, Deserialize, JsonSchema)]
pub enum RegionYaml {
    US, EU, EEA, CH, CN, JP, UK, LOCAL,
}

/// Model capability flags.
#[derive(Debug, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum CapabilityYaml {
    ToolUse,
    Vision,
    StructuredOutput,
    Code,
    Reasoning,
    Multilingual,
    WebSearch,
}
```

### Schema with Unknown Field Denial

```rust
#[derive(Debug, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct ModelYaml {
    pub cost_class: CostClassYaml,
    pub typical_latency_ms: u32,
    pub quality: f64,
    #[serde(default)]
    pub capabilities: Vec<CapabilityYaml>,
}
```

### Validation After Parsing

```rust
fn validate_model(provider_id: &str, model_id: &str, model: &ModelYaml) -> Result<(), String> {
    // Quality must be 0.0-1.0
    if !(0.0..=1.0).contains(&model.quality) {
        return Err(format!(
            "Model '{provider_id}/{model_id}': quality must be 0.0-1.0, got {}",
            model.quality
        ));
    }

    // Latency must be positive
    if model.typical_latency_ms == 0 {
        return Err(format!(
            "Model '{provider_id}/{model_id}': typical_latency_ms must be > 0"
        ));
    }

    // Embedding models require dimensions
    if model.model_type == ModelTypeYaml::Embedding && model.dimensions.is_none() {
        return Err(format!(
            "Model '{provider_id}/{model_id}': embedding models must specify dimensions"
        ));
    }

    Ok(())
}
```

### JSON Schema Generation

```rust
use converge_provider::registry_loader::generate_schema;

let schema = generate_schema();
println!("{}", serde_json::to_string_pretty(&schema).unwrap());
```

This can be used for:
- IDE autocompletion in YAML files
- Pre-runtime validation
- Documentation generation

## Error Messages

Invalid configs fail with clear, actionable messages:

```
# Invalid cost class
"unknown variant `SuperLow`, expected one of `VeryLow`, `Low`, `Medium`, `High`, `VeryHigh`"

# Invalid capability
"unknown variant `telepathy`, expected one of `tool_use`, `vision`, `structured_output`, ..."

# Quality out of range
"Model 'anthropic/claude-3': quality must be 0.0-1.0, got 1.5"

# Unknown field (typo)
"unknown field `capabilties`, expected one of `cost_class`, `typical_latency_ms`, ..."

# Missing required field for embedding
"Model 'ollama/nomic-embed': embedding models must specify dimensions"
```

## Alignment with Converge

This model fits Converge perfectly:

| YAML Concept | Converge Concept |
|-------------|------------------|
| YAML file | Intent declaration |
| Rust types | Semantic authority |
| Parsing | ProposedFact creation |
| Validation | Invariant checking |
| Loaded config | Fact (trusted) |

**No runtime surprises. No jelly.**

## Claim

> In Converge, configuration errors are compile-time or startup-time failures — never runtime surprises.

This is a serious system claim, and we back it with:
- Type-safe enum parsing
- Unknown field denial
- Explicit validation
- Schema generation
- Comprehensive test coverage

## See Also

- `converge-provider/src/registry_loader.rs` - Implementation
- `converge-provider/config/models.yaml` - Model registry
- `docs/assistant-guides/Rust-Best-Practices-v2.md` - Rust guidelines
