# Provider Implementation Refactoring

This document describes the common abstractions available to reduce code duplication across provider implementations.

## Repeated Patterns

Most providers share these patterns:

1. **HTTP Client Setup**: `api_key`, `model`, `client`, `base_url`
2. **OpenAI-Compatible API**: Many providers use the same request/response format
3. **Message Building**: System + user message construction
4. **Response Parsing**: Extract choice, map finish_reason, build `LlmResponse`
5. **Error Handling**: HTTP status checks, error parsing

## Common Abstractions

### 1. `HttpProviderConfig`

Base configuration for HTTP-based providers:

```rust
use converge_provider::common::HttpProviderConfig;

pub struct MyProvider {
    config: HttpProviderConfig,
}

impl MyProvider {
    pub fn new(api_key: impl Into<String>, model: impl Into<String>) -> Self {
        Self {
            config: HttpProviderConfig::new(api_key, model, "https://api.example.com"),
        }
    }
}
```

### 2. OpenAI-Compatible Types

For providers using OpenAI's API format:

```rust
use converge_provider::common::{
    ChatCompletionRequest, ChatMessage, make_chat_completion_request,
};

// Build request from LlmRequest
let chat_request = ChatCompletionRequest::from_llm_request(
    self.config.model.clone(),
    request,
);

// Make HTTP request
let response = make_chat_completion_request(
    &self.config,
    "/v1/chat/completions",
    chat_request,
)?;
```

### 3. `OpenAiCompatibleProvider` Trait

For providers that fully match OpenAI's API:

```rust
use converge_provider::common::OpenAiCompatibleProvider;

impl OpenAiCompatibleProvider for MyProvider {
    fn config(&self) -> &HttpProviderConfig {
        &self.config
    }

    fn endpoint(&self) -> &str {
        "/v1/chat/completions"
    }
}

impl LlmProvider for MyProvider {
    fn complete(&self, request: &LlmRequest) -> Result<LlmResponse, LlmError> {
        self.complete_openai_compatible(request)
    }
}
```

## Refactoring Example: MistralProvider

### Before (185 lines)

```rust
pub struct MistralProvider {
    api_key: String,
    model: String,
    client: reqwest::blocking::Client,
    base_url: String,
}

impl MistralProvider {
    pub fn new(api_key: impl Into<String>, model: impl Into<String>) -> Self {
        Self {
            api_key: api_key.into(),
            model: model.into(),
            client: reqwest::blocking::Client::new(),
            base_url: "https://api.mistral.ai".into(),
        }
    }
}

impl LlmProvider for MistralProvider {
    fn complete(&self, request: &LlmRequest) -> Result<LlmResponse, LlmError> {
        let mut messages = Vec::new();
        if let Some(system) = &request.system {
            messages.push(MistralMessage { role: "system", content: system });
        }
        messages.push(MistralMessage { role: "user", content: &request.prompt });

        let api_request = MistralRequest {
            model: &self.model,
            messages,
            max_tokens: request.max_tokens,
            temperature: request.temperature,
            stop: request.stop_sequences.clone(),
        };

        let response = self.client
            .post(format!("{}/v1/chat/completions", self.base_url))
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&api_request)
            .send()
            .map_err(|e| LlmError::network(format!("Request failed: {}", e)))?;

        // ... error handling and parsing ...
    }
}
```

### After (50 lines)

```rust
use converge_provider::common::{HttpProviderConfig, OpenAiCompatibleProvider};

pub struct MistralProvider {
    config: HttpProviderConfig,
}

impl MistralProvider {
    pub fn new(api_key: impl Into<String>, model: impl Into<String>) -> Self {
        Self {
            config: HttpProviderConfig::new(
                api_key,
                model,
                "https://api.mistral.ai",
            ),
        }
    }

    pub fn from_env(model: impl Into<String>) -> Result<Self, LlmError> {
        let api_key = std::env::var("MISTRAL_API_KEY")
            .map_err(|_| LlmError::auth("MISTRAL_API_KEY not set"))?;
        Ok(Self::new(api_key, model))
    }
}

impl OpenAiCompatibleProvider for MistralProvider {
    fn config(&self) -> &HttpProviderConfig {
        &self.config
    }

    fn endpoint(&self) -> &str {
        "/v1/chat/completions"
    }
}

impl LlmProvider for MistralProvider {
    fn name(&self) -> &'static str {
        "mistral"
    }

    fn model(&self) -> &str {
        &self.config.model
    }

    fn complete(&self, request: &LlmRequest) -> Result<LlmResponse, LlmError> {
        self.complete_openai_compatible(request)
    }

    fn provenance(&self, request_id: &str) -> String {
        format!("mistral:{}:{}", self.config.model, request_id)
    }
}
```

**Reduction**: ~135 lines → ~50 lines (73% reduction)

## Providers That Can Use Common Abstractions

### Full OpenAI-Compatible (Can use `OpenAiCompatibleProvider`)

- ✅ Mistral
- ✅ DeepSeek
- ✅ Kimi
- ✅ Zhipu
- ✅ Apertus
- ✅ MinMax
- ✅ Grok
- ✅ Perplexity
- ✅ OpenRouter

### Partial Compatibility (Can use `ChatCompletionRequest`, `make_chat_completion_request`)

- ✅ OpenAI (has custom error handling)
- ✅ Qwen (different API structure)

### Custom APIs (Need custom implementation)

- ❌ Anthropic (Messages API, different format)
- ❌ Gemini (GenerateContent API)
- ❌ Baidu (ERNIE API, OAuth flow)

## Migration Strategy

1. **Phase 1**: Use common types for new providers
2. **Phase 2**: Refactor existing OpenAI-compatible providers
3. **Phase 3**: Extract more patterns (error handling, retries)

## Benefits

- **Less Code**: ~70% reduction for OpenAI-compatible providers
- **Consistency**: Same error handling, same patterns
- **Maintainability**: Fix bugs once, applies to all
- **Type Safety**: Shared types ensure compatibility

## Future Abstractions

Potential additional abstractions:

1. **Retry Logic**: Common retry patterns for rate limits
2. **Streaming**: Common streaming response handling
3. **Error Mapping**: Provider-specific error → `LlmError` mapping
4. **Token Counting**: Common token counting utilities
5. **Request Batching**: Batch request handling

## See Also

- [`common.rs`](../../converge-provider/src/common.rs) - Implementation
- Provider implementations for examples

