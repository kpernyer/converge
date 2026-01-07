# Error Handling Pattern Refactoring

This document explains what "error handling pattern refactoring" would mean and how it could further reduce code duplication.

## Current State

After the initial refactoring, **4 providers still have duplicated error handling code**:

- MinMax (~25 lines of error handling)
- Grok (~25 lines of error handling)
- Perplexity (~25 lines of error handling)
- OpenRouter (~25 lines of error handling)

**Total duplication**: ~100 lines of nearly identical error handling code.

## The Pattern

All these providers follow the same error handling pattern:

```rust
if !status.is_success() {
    #[derive(Deserialize)]
    struct ProviderError {
        error: ProviderErrorDetail,
    }
    #[derive(Deserialize)]
    struct ProviderErrorDetail {
        message: String,
        #[serde(rename = "type")]
        error_type: Option<String>,
    }

    let error_body: ProviderError = http_response
        .json()
        .map_err(|e| LlmError::parse(format!("Failed to parse error: {}", e)))?;

    let error_type = error_body.error.error_type.as_deref().unwrap_or("unknown");
    return match error_type {
        "invalid_request_error" | "authentication_error" => {
            Err(LlmError::auth(error_body.error.message))
        }
        "rate_limit_error" => Err(LlmError::rate_limit(error_body.error.message)),
        _ => Err(LlmError::provider(error_body.error.message)),
    };
}
```

## Error Format Patterns

### Pattern 1: OpenAI-Compatible (Most Common)

Used by: OpenAI, MinMax, Grok, Perplexity, OpenRouter

```json
{
  "error": {
    "message": "Invalid API key",
    "type": "authentication_error"
  }
}
```

**Structure**:
```rust
struct Error {
    error: ErrorDetail
}
struct ErrorDetail {
    message: String,
    #[serde(rename = "type")]
    error_type: Option<String>,
}
```

**Mapping**:
- `"invalid_request_error" | "authentication_error"` → `LlmError::auth()`
- `"rate_limit_error"` → `LlmError::rate_limit()`
- Everything else → `LlmError::provider()`

### Pattern 2: Anthropic

```json
{
  "error": {
    "type": "authentication_error",
    "message": "Invalid API key"
  }
}
```

**Structure**: Same as OpenAI, but `error_type` is required (not `Option`)

**Mapping**: Same as OpenAI

### Pattern 3: Gemini

```json
{
  "error": {
    "message": "Invalid API key",
    "status": "UNAUTHENTICATED"
  }
}
```

**Structure**:
```rust
struct Error {
    error: ErrorDetail
}
struct ErrorDetail {
    message: String,
    status: Option<String>,  // Different field name
}
```

**Mapping**:
- `"UNAUTHENTICATED" | "PERMISSION_DENIED"` → `LlmError::auth()`
- `"RESOURCE_EXHAUSTED"` → `LlmError::rate_limit()`
- Everything else → `LlmError::provider()`

### Pattern 4: Qwen

```json
{
  "code": "InvalidApiKey",
  "message": "Invalid API key"
}
```

**Structure**: Flat structure, no nested `error` object

**Mapping**:
- `"InvalidApiKey" | "InvalidParameter"` → `LlmError::auth()`
- `"Throttling"` → `LlmError::rate_limit()`
- Everything else → `LlmError::provider()`

## Proposed Refactoring

### Step 1: Generic Error Response Types

```rust
// OpenAI-compatible error format
#[derive(Deserialize)]
pub struct OpenAiStyleError {
    pub error: OpenAiStyleErrorDetail,
}

#[derive(Deserialize)]
pub struct OpenAiStyleErrorDetail {
    pub message: String,
    #[serde(rename = "type")]
    pub error_type: Option<String>,
}

// Gemini error format
#[derive(Deserialize)]
pub struct GeminiStyleError {
    pub error: GeminiStyleErrorDetail,
}

#[derive(Deserialize)]
pub struct GeminiStyleErrorDetail {
    pub message: String,
    pub status: Option<String>,
}

// Qwen error format
#[derive(Deserialize)]
pub struct QwenStyleError {
    pub code: Option<String>,
    pub message: String,
}
```

### Step 2: Error Type Mapper Trait

```rust
/// Maps provider-specific error types to LlmError kinds.
pub trait ErrorTypeMapper {
    /// Extracts the error type identifier from the error response.
    fn error_type(&self) -> Option<&str>;
    
    /// Extracts the error message.
    fn message(&self) -> &str;
    
    /// Maps the error type to an LlmError.
    fn map_to_llm_error(&self) -> LlmError {
        match self.error_type() {
            Some(typ) if self.is_auth_error(typ) => {
                LlmError::auth(self.message().to_string())
            }
            Some(typ) if self.is_rate_limit_error(typ) => {
                LlmError::rate_limit(self.message().to_string())
            }
            _ => LlmError::provider(self.message().to_string()),
        }
    }
    
    /// Checks if error type indicates authentication error.
    fn is_auth_error(&self, error_type: &str) -> bool;
    
    /// Checks if error type indicates rate limit error.
    fn is_rate_limit_error(&self, error_type: &str) -> bool;
}

impl ErrorTypeMapper for OpenAiStyleErrorDetail {
    fn error_type(&self) -> Option<&str> {
        self.error_type.as_deref()
    }
    
    fn message(&self) -> &str {
        &self.message
    }
    
    fn is_auth_error(&self, error_type: &str) -> bool {
        matches!(error_type, "invalid_request_error" | "authentication_error")
    }
    
    fn is_rate_limit_error(&self, error_type: &str) -> bool {
        error_type == "rate_limit_error"
    }
}
```

### Step 3: Generic Error Handler

```rust
/// Handles HTTP error responses for OpenAI-compatible providers.
pub fn handle_openai_style_error(
    http_response: reqwest::blocking::Response,
) -> Result<(), LlmError> {
    let error_body: OpenAiStyleError = http_response
        .json()
        .map_err(|e| LlmError::parse(format!("Failed to parse error: {}", e)))?;
    
    Err(error_body.error.map_to_llm_error())
}
```

### Step 4: Updated Provider Implementation

**Before** (25 lines):
```rust
if !status.is_success() {
    #[derive(Deserialize)]
    struct MinMaxError {
        error: MinMaxErrorDetail,
    }
    #[derive(Deserialize)]
    struct MinMaxErrorDetail {
        message: String,
        #[serde(rename = "type")]
        error_type: Option<String>,
    }

    let error_body: MinMaxError = http_response
        .json()
        .map_err(|e| LlmError::parse(format!("Failed to parse error: {}", e)))?;

    let error_type = error_body.error.error_type.as_deref().unwrap_or("unknown");
    return match error_type {
        "invalid_request_error" | "authentication_error" => {
            Err(LlmError::auth(error_body.error.message))
        }
        "rate_limit_error" => Err(LlmError::rate_limit(error_body.error.message)),
        _ => Err(LlmError::provider(error_body.error.message)),
    };
}
```

**After** (3 lines):
```rust
if !status.is_success() {
    return handle_openai_style_error(http_response);
}
```

## Benefits

1. **Further Code Reduction**: ~22 lines saved per provider (4 providers = ~88 lines)
2. **Consistency**: All providers handle errors the same way
3. **Maintainability**: Fix error mapping once, applies everywhere
4. **Extensibility**: Easy to add new error types or providers

## Implementation Complexity

### Low Complexity (Recommended)
- Extract OpenAI-compatible error handling (used by 5 providers)
- Keep custom handlers for Anthropic, Gemini, Qwen (they're different enough)

**Savings**: ~88 lines across 4 providers

### Medium Complexity
- Create generic error handler with provider-specific mappers
- Support all error formats through trait system

**Savings**: ~150 lines across all providers

### High Complexity (Not Recommended)
- Try to unify all error formats into one
- Risk: Over-abstraction, harder to maintain

## Example: Refactored MinMax Provider

```rust
impl LlmProvider for MinMaxProvider {
    fn complete(&self, request: &LlmRequest) -> Result<LlmResponse, LlmError> {
        let chat_request = ChatCompletionRequest::from_llm_request(
            self.config.model.clone(),
            request,
        );
        let url = format!("{}{}", self.config.base_url, self.endpoint());

        let http_response = self
            .config
            .client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.config.api_key))
            .header("Content-Type", "application/json")
            .json(&chat_request)
            .send()
            .map_err(|e| LlmError::network(format!("Request failed: {}", e)))?;

        // Error handling: 3 lines instead of 25
        if !http_response.status().is_success() {
            return handle_openai_style_error(http_response);
        }

        let api_response: ChatCompletionResponse = http_response
            .json()
            .map_err(|e| LlmError::parse(format!("Failed to parse response: {}", e)))?;

        chat_response_to_llm_response(api_response)
    }
}
```

## Potential Additional Patterns

### Retry Logic

Many providers could benefit from automatic retry on rate limits:

```rust
pub fn complete_with_retry(
    provider: &dyn LlmProvider,
    request: &LlmRequest,
    max_retries: u32,
) -> Result<LlmResponse, LlmError> {
    for attempt in 0..=max_retries {
        match provider.complete(request) {
            Ok(response) => return Ok(response),
            Err(e) if e.is_rate_limit() && attempt < max_retries => {
                // Exponential backoff
                std::thread::sleep(Duration::from_secs(2_u64.pow(attempt)));
                continue;
            }
            Err(e) => return Err(e),
        }
    }
    unreachable!()
}
```

### Error Classification

Helper to classify errors for logging/monitoring:

```rust
pub fn classify_error(error: &LlmError) -> ErrorCategory {
    match error.kind {
        LlmErrorKind::Auth => ErrorCategory::Configuration,
        LlmErrorKind::RateLimit => ErrorCategory::Temporary,
        LlmErrorKind::ProviderError => ErrorCategory::Permanent,
        _ => ErrorCategory::Unknown,
    }
}
```

## Recommendation

**Start with OpenAI-compatible error handling**:
- ✅ Low complexity
- ✅ High impact (4 providers, ~88 lines saved)
- ✅ Low risk (well-understood pattern)
- ✅ Easy to test

**Defer custom error formats**:
- Anthropic, Gemini, Qwen have different structures
- Less duplication (only 1 provider each)
- Higher complexity to abstract

## See Also

- [`common.rs`](../../converge-provider/src/common.rs) - Current common abstractions
- Provider implementations for current error handling patterns

