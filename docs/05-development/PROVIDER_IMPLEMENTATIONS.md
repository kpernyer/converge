# LLM Provider Implementations

This document summarizes all implemented LLM providers in `converge-provider`.

---

## Implemented Providers

All providers follow the same pattern:
- Implement `LlmProvider` trait
- Support `from_env()` for environment variable configuration
- Support `with_base_url()` for custom endpoints
- Handle errors consistently (auth, rate limit, network, etc.)
- Return standardized `LlmResponse` with token usage

---

## Provider List

### ✅ 1. Anthropic (Claude)

**File**: `converge-provider/src/anthropic.rs`

**Status**: ✅ Implemented (was already present)

**Features**:
- Claude API v1
- System prompts
- Stop sequences
- Token usage tracking

**Example**:
```rust
use converge_provider::AnthropicProvider;
use converge_core::llm::{LlmProvider, LlmRequest};

let provider = AnthropicProvider::from_env("claude-3-5-sonnet-20241022")?;
let response = provider.complete(&LlmRequest::new("Hello!"))?;
```

---

### ✅ 2. OpenAI (GPT-4, GPT-3.5)

**File**: `converge-provider/src/openai.rs`

**Status**: ✅ Implemented

**Features**:
- Chat Completions API
- System prompts
- Stop sequences
- Token usage tracking

**Example**:
```rust
use converge_provider::OpenAiProvider;
use converge_core::llm::{LlmProvider, LlmRequest};

let provider = OpenAiProvider::from_env("gpt-4")?;
let response = provider.complete(&LlmRequest::new("Hello!"))?;
```

**Models**: `gpt-4`, `gpt-4-turbo`, `gpt-3.5-turbo`, etc.

---

### ✅ 3. Google Gemini

**File**: `converge-provider/src/gemini.rs`

**Status**: ✅ Implemented

**Features**:
- Gemini API v1beta
- System prompts (prepended to user message)
- Stop sequences
- Token usage tracking

**Example**:
```rust
use converge_provider::GeminiProvider;
use converge_core::llm::{LlmProvider, LlmRequest};

let provider = GeminiProvider::from_env("gemini-pro")?;
let response = provider.complete(&LlmRequest::new("Hello!"))?;
```

**Models**: `gemini-pro`, `gemini-pro-vision`, etc.

---

### ✅ 4. Perplexity AI

**File**: `converge-provider/src/perplexity.rs`

**Status**: ✅ Implemented

**Features**:
- Chat Completions API (OpenAI-compatible)
- System prompts
- Token usage tracking
- Web search capabilities (via model selection)

**Example**:
```rust
use converge_provider::PerplexityProvider;
use converge_core::llm::{LlmProvider, LlmRequest};

let provider = PerplexityProvider::from_env("pplx-70b-online")?;
let response = provider.complete(&LlmRequest::new("Hello!"))?;
```

**Models**: `pplx-70b-online`, `pplx-7b-online`, etc.

---

### ✅ 5. Qwen (Alibaba Cloud)

**File**: `converge-provider/src/qwen.rs`

**Status**: ✅ Implemented

**Features**:
- DashScope API
- System prompts
- Stop sequences
- Token usage tracking

**Example**:
```rust
use converge_provider::QwenProvider;
use converge_core::llm::{LlmProvider, LlmRequest};

let provider = QwenProvider::from_env("qwen-turbo")?;
let response = provider.complete(&LlmRequest::new("Hello!"))?;
```

**Models**: `qwen-turbo`, `qwen-plus`, `qwen-max`, etc.

---

### ✅ 6. OpenRouter (Multi-Provider Aggregator)

**File**: `converge-provider/src/openrouter.rs`

**Status**: ✅ Implemented

**Features**:
- Single API key for multiple providers
- OpenAI-compatible API
- System prompts
- Token usage tracking
- Analytics headers (optional)

**Example**:
```rust
use converge_provider::OpenRouterProvider;
use converge_core::llm::{LlmProvider, LlmRequest};

// Access any provider through OpenRouter
let provider = OpenRouterProvider::from_env("anthropic/claude-3-opus")?;
let response = provider.complete(&LlmRequest::new("Hello!"))?;
```

**Model Format**: `provider/model-name` (e.g., `anthropic/claude-3-opus`, `openai/gpt-4`)

**Benefits**:
- One key for all providers
- Unified interface
- Cost tracking across providers

---

### ✅ 7. MinMax AI

**File**: `converge-provider/src/minmax.rs`

**Status**: ✅ Implemented

**Features**:
- Chat Completions API (OpenAI-compatible)
- System prompts
- Stop sequences
- Token usage tracking

**Example**:
```rust
use converge_provider::MinMaxProvider;
use converge_core::llm::{LlmProvider, LlmRequest};

let provider = MinMaxProvider::from_env("abab5.5-chat")?;
let response = provider.complete(&LlmRequest::new("Hello!"))?;
```

**Models**: `abab5.5-chat`, `abab6.5-chat`, etc.

---

### ✅ 8. Grok (xAI)

**File**: `converge-provider/src/grok.rs`

**Status**: ✅ Implemented

**Features**:
- xAI Chat Completions API (OpenAI-compatible)
- System prompts
- Stop sequences
- Token usage tracking

**Example**:
```rust
use converge_provider::GrokProvider;
use converge_core::llm::{LlmProvider, LlmRequest};

let provider = GrokProvider::from_env("grok-beta")?;
let response = provider.complete(&LlmRequest::new("Hello!"))?;
```

**Models**: `grok-beta`, `grok-2`, etc.

---

## Common Interface

All providers implement the `LlmProvider` trait:

```rust
pub trait LlmProvider: Send + Sync {
    fn name(&self) -> &str;
    fn model(&self) -> &str;
    fn complete(&self, request: &LlmRequest) -> Result<LlmResponse, LlmError>;
    fn provenance(&self, request_id: &str) -> String;
}
```

### Standard Methods

Each provider has:

1. **`new(api_key, model)`** - Create with explicit key
2. **`from_env(model)`** - Create from environment variable
3. **`with_base_url(url)`** - Use custom endpoint (for testing/proxies)

### Error Handling

All providers handle:
- **Authentication errors** → `LlmError::auth()`
- **Rate limit errors** → `LlmError::rate_limit()`
- **Network errors** → `LlmError::network()`
- **Parse errors** → `LlmError::parse()`
- **Provider errors** → `LlmError::provider()`

---

## Testing

All providers have unit tests:

```bash
cargo test --package converge-provider --lib
```

**Test Coverage**:
- ✅ Provider name correctness
- ✅ Model name storage
- ✅ Error handling structure
- ✅ Response parsing

**Integration Tests**:
- See `converge-provider/tests/integration_prompt_formatting.rs`
- Requires API keys (marked with `#[ignore]`)

---

## Usage Patterns

### Pattern 1: Direct Provider Usage

```rust
use converge_provider::OpenAiProvider;
use converge_core::llm::{LlmProvider, LlmRequest};

let provider = OpenAiProvider::from_env("gpt-4")?;
let response = provider.complete(&LlmRequest::new("Hello!"))?;
```

### Pattern 2: Provider Selection

```rust
use converge_provider::{AnthropicProvider, OpenAiProvider};
use converge_core::llm::{LlmProvider, LlmRequest};

// Select provider based on configuration
let provider: Box<dyn LlmProvider> = if use_claude {
    Box::new(AnthropicProvider::from_env("claude-3-opus")?)
} else {
    Box::new(OpenAiProvider::from_env("gpt-4")?)
};

let response = provider.complete(&LlmRequest::new("Hello!"))?;
```

### Pattern 3: OpenRouter (Multi-Provider)

```rust
use converge_provider::OpenRouterProvider;
use converge_core::llm::{LlmProvider, LlmRequest};

// One key, multiple providers
let claude = OpenRouterProvider::from_env("anthropic/claude-3-opus")?;
let gpt = OpenRouterProvider::from_env("openai/gpt-4")?;
let gemini = OpenRouterProvider::from_env("google/gemini-pro")?;
```

---

## API Compatibility

### OpenAI-Compatible Providers

These providers use OpenAI's Chat Completions API format:
- ✅ OpenAI
- ✅ Perplexity
- ✅ OpenRouter
- ✅ MinMax
- ✅ Grok

**Benefits**:
- Same request/response format
- Easy to swap providers
- Consistent error handling

### Custom API Providers

These providers have custom APIs:
- ✅ Anthropic (Claude) - Messages API
- ✅ Google Gemini - GenerateContent API
- ✅ Qwen - DashScope API

**Note**: All are wrapped to provide the same `LlmProvider` interface.

---

## Environment Variables

All providers support environment variable configuration:

| Provider | Environment Variable | Example |
|----------|---------------------|---------|
| Anthropic | `ANTHROPIC_API_KEY` | `sk-ant-api03-...` |
| OpenAI | `OPENAI_API_KEY` | `sk-...` |
| Gemini | `GEMINI_API_KEY` | `AIza...` |
| Perplexity | `PERPLEXITY_API_KEY` | `pplx-...` |
| Qwen | `QWEN_API_KEY` | `sk-...` |
| OpenRouter | `OPENROUTER_API_KEY` | `sk-or-v1-...` |
| MinMax | `MINMAX_API_KEY` | varies |
| Grok | `GROK_API_KEY` | `xai-...` |

See `.env.example` for all keys.

---

## Next Steps

### Future Enhancements

- [ ] Streaming response support
- [ ] Function calling support (where available)
- [ ] Vision/image input support
- [ ] Provider-specific optimizations
- [ ] Cost tracking per provider
- [ ] Automatic failover between providers

### Provider-Specific Features

- [ ] Anthropic: Tool use support
- [ ] OpenAI: Function calling, JSON mode
- [ ] Gemini: Multimodal input
- [ ] Perplexity: Web search configuration
- [ ] OpenRouter: Model routing optimization

---

## References

- [Provider API Keys Guide](./LLM_PROVIDER_KEYS.md)
- [Secure API Key Setup](./SECURE_API_KEY_SETUP.md)
- [Prompt Structuring](./PROVIDER_PROMPT_STRUCTURING.md)

