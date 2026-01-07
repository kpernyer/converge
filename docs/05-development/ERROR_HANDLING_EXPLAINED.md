# Error Handling Pattern Refactoring - Explained

## What It Means

**Error handling pattern refactoring** means extracting the **duplicated error parsing and mapping logic** that appears across multiple providers into reusable abstractions.

## The Problem

Currently, **4 providers** (MinMax, Grok, Perplexity, OpenRouter) each have ~25 lines of nearly identical error handling code:

```rust
// This pattern is repeated 4 times with only the struct names changing
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
        .map_err(|e| LlmError::parse(...))?;

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

**Total duplication**: ~100 lines of identical logic.

## The Solution

### Step 1: Extract Common Error Types

Create shared error response types in `common.rs`:

```rust
#[derive(Deserialize, Debug)]
pub struct OpenAiStyleError {
    pub error: OpenAiStyleErrorDetail,
}

#[derive(Deserialize, Debug)]
pub struct OpenAiStyleErrorDetail {
    pub message: String,
    #[serde(rename = "type")]
    pub error_type: Option<String>,
}
```

### Step 2: Create Error Handler Function

Create a reusable error handler:

```rust
pub fn handle_openai_style_error(
    http_response: reqwest::blocking::Response,
) -> Result<LlmResponse, LlmError> {
    let error_body: OpenAiStyleError = http_response
        .json()
        .map_err(|e| LlmError::parse(format!("Failed to parse error: {}", e)))?;

    let error_type = error_body.error.error_type.as_deref().unwrap_or("unknown");
    let message = error_body.error.message;

    let llm_error = match error_type {
        "invalid_request_error" | "authentication_error" => LlmError::auth(message),
        "rate_limit_error" => LlmError::rate_limit(message),
        _ => LlmError::provider(message),
    };

    Err(llm_error)
}
```

### Step 3: Use in Providers

**Before** (25 lines):
```rust
if !status.is_success() {
    #[derive(Deserialize)]
    struct MinMaxError { error: MinMaxErrorDetail }
    #[derive(Deserialize)]
    struct MinMaxErrorDetail {
        message: String,
        #[serde(rename = "type")]
        error_type: Option<String>,
    }
    // ... 20 more lines of parsing and mapping
}
```

**After** (1 line):
```rust
if !status.is_success() {
    return handle_openai_style_error(http_response);
}
```

## Impact

### Code Reduction
- **Before**: ~25 lines per provider × 4 providers = **100 lines**
- **After**: 1 line per provider + ~20 lines in common = **24 lines total**
- **Savings**: **~76 lines** (76% reduction)

### Benefits
1. ✅ **Consistency**: All providers handle errors identically
2. ✅ **Maintainability**: Fix error mapping once, applies everywhere
3. ✅ **Testability**: Test error handling in one place
4. ✅ **Extensibility**: Easy to add new error types or providers

## Current Status

✅ **Implemented**: `handle_openai_style_error()` function in `common.rs`
✅ **Available**: Can be used by all OpenAI-compatible providers
⏳ **Pending**: Update providers to use it (optional, can be done incrementally)

## Example: Refactored Provider

Here's how a provider would look after using the error handler:

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

        // Error handling: 1 line instead of 25
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

## Why Not All Providers?

Some providers have **different error formats**:

- **Anthropic**: `error.type` is required (not `Option`)
- **Gemini**: Uses `error.status` instead of `error.type`
- **Qwen**: Flat structure with `code` field

These need custom handlers, but the pattern could still be abstracted further if needed.

## Future Enhancements

### 1. Retry Logic
```rust
pub fn complete_with_retry(
    provider: &dyn LlmProvider,
    request: &LlmRequest,
) -> Result<LlmResponse, LlmError> {
    // Automatically retry on rate limits with exponential backoff
}
```

### 2. Error Classification
```rust
pub fn classify_error(error: &LlmError) -> ErrorCategory {
    // Categorize errors for monitoring/alerting
}
```

### 3. Error Metrics
```rust
pub fn track_error(error: &LlmError, provider: &str) {
    // Track error rates per provider for observability
}
```

## Summary

**Error handling pattern refactoring** means:
- ✅ Extracting duplicated error parsing logic
- ✅ Creating reusable error handlers
- ✅ Reducing code from ~25 lines to ~1 line per provider
- ✅ Ensuring consistent error handling across providers

The infrastructure is now in place - providers can be updated incrementally to use it.

