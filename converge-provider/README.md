# converge-provider

Multi-provider LLM abstraction layer for the [Converge](https://crates.io/crates/converge-core) runtime.

**Website:** [converge.zone](https://converge.zone) | **Docs:** [docs.rs](https://docs.rs/converge-provider)

## Supported Providers

| Provider | Models |
|----------|--------|
| **Anthropic** | Claude 3.5 Sonnet, Haiku, Opus |
| **OpenAI** | GPT-4o, GPT-4o-mini |
| **Google Gemini** | Gemini Pro, Flash |
| **Qwen** | Qwen-Max, Qwen-Plus |
| **DeepSeek** | DeepSeek Chat, Coder |
| **Mistral** | Mistral Large, Medium |
| **Grok** | xAI models |
| **Perplexity** | Online models |
| **OpenRouter** | Multi-provider gateway |
| **Baidu** | ERNIE |
| **Zhipu** | GLM-4 |
| **Kimi** | Moonshot |

## Features

### Model Selection
- Cost-aware selection (`CostClass`: VeryLow → VeryHigh)
- Latency requirements (tokens/second thresholds)
- Quality requirements (capability levels)
- `AgentRequirements` builder for declarative selection

### Prompt Engineering
- **EDN format** for token-efficient prompts (~40% reduction)
- XML format for Claude models
- JSON format for OpenAI models
- Structured response parsing

## Installation

```toml
[dependencies]
converge-provider = "0.2"
```

## Example

```rust
use converge_provider::{AnthropicProvider, LlmProvider, LlmRequest};

// Create provider from environment
let provider = AnthropicProvider::from_env("claude-3-5-sonnet-20241022")?;

// Make a request
let request = LlmRequest {
    prompt: "Analyze market trends for Q4".to_string(),
    max_tokens: Some(1000),
    temperature: Some(0.7),
};

let response = provider.complete(&request)?;
println!("Response: {}", response.content);
```

### Model Selection

```rust
use converge_provider::{ModelSelector, AgentRequirements, CostClass};

let selector = ModelSelector::default();

let requirements = AgentRequirements::fast_and_cheap()
    .with_max_cost(CostClass::Low);

let (provider, model) = selector.select(&requirements)?;
```

## License

MIT
