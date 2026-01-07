# LLM Provider API Keys Reference

This document lists all supported LLM providers and how to obtain their API keys.

---

## Supported Providers

### 1. Anthropic (Claude)

**Key Format**: `sk-ant-api03-...`

**Get Key**: https://console.anthropic.com/settings/keys

**Usage**:
```rust
use converge_provider::AnthropicProvider;

let provider = AnthropicProvider::from_env("claude-3-5-sonnet-20241022")?;
```

**Environment Variable**: `ANTHROPIC_API_KEY`

---

### 2. OpenAI (GPT-4, GPT-3.5, etc.)

**Key Format**: `sk-...`

**Get Key**: https://platform.openai.com/api-keys

**Usage**:
```rust
use converge_provider::OpenAiProvider;

let provider = OpenAiProvider::from_env("gpt-4")?;
```

**Environment Variable**: `OPENAI_API_KEY`

---

### 3. Google Gemini

**Key Format**: `AIza...`

**Get Key**: https://makersuite.google.com/app/apikey

**Usage**:
```rust
use converge_provider::GeminiProvider;

let provider = GeminiProvider::from_env("gemini-pro")?;
```

**Environment Variable**: `GEMINI_API_KEY`

---

### 4. Perplexity AI

**Key Format**: `pplx-...`

**Get Key**: https://www.perplexity.ai/settings/api

**Usage**:
```rust
use converge_provider::PerplexityProvider;

let provider = PerplexityProvider::from_env("pplx-70b-online")?;
```

**Environment Variable**: `PERPLEXITY_API_KEY`

---

### 5. Qwen (Alibaba Cloud)

**Key Format**: `sk-...` (varies)

**Get Key**: https://dashscope.console.aliyun.com/

**Usage**:
```rust
use converge_provider::QwenProvider;

let provider = QwenProvider::from_env("qwen-turbo")?;
```

**Environment Variable**: `QWEN_API_KEY`

---

### 6. OpenRouter (Multi-Provider Aggregator)

**Key Format**: `sk-or-v1-...`

**Get Key**: https://openrouter.ai/keys

**Usage**:
```rust
use converge_provider::OpenRouterProvider;

// One key works for multiple providers
let provider = OpenRouterProvider::from_env("anthropic/claude-3-opus")?;
```

**Environment Variable**: `OPENROUTER_API_KEY`

**Benefits**:
- Single API key for multiple providers
- Unified interface
- Cost tracking across providers

---

### 7. MinMax AI

**Key Format**: Varies

**Get Key**: https://platform.minmax.ai/

**Usage**:
```rust
use converge_provider::MinMaxProvider;

let provider = MinMaxProvider::from_env("abab5.5-chat")?;
```

**Environment Variable**: `MINMAX_API_KEY`

---

### 8. Grok (xAI)

**Key Format**: `xai-...`

**Get Key**: https://console.x.ai/

**Usage**:
```rust
use converge_provider::GrokProvider;

let provider = GrokProvider::from_env("grok-beta")?;
```

**Environment Variable**: `GROK_API_KEY`

---

## Environment Variable Setup

### Quick Setup

1. **Copy example file**:
   ```bash
   cp .env.example .env
   ```

2. **Edit `.env`** and add your keys:
   ```bash
   ANTHROPIC_API_KEY=sk-ant-api03-...
   OPENAI_API_KEY=sk-...
   # ... etc
   ```

3. **Load environment**:
   ```bash
   export $(cat .env | xargs)
   ```

### Provider Priority

For integration tests, providers are checked in this order:
1. Provider-specific key (e.g., `ANTHROPIC_API_KEY`)
2. OpenRouter key (if provider available via OpenRouter)
3. Fallback to mock provider (in tests)

---

## Cost Considerations

| Provider | Cost per 1M Tokens (Input) | Cost per 1M Tokens (Output) | Notes |
|----------|---------------------------|----------------------------|-------|
| Anthropic Claude Haiku | ~$0.25 | ~$1.25 | Cheapest Claude model |
| Anthropic Claude Sonnet | ~$3.00 | ~$15.00 | Balanced |
| OpenAI GPT-4 | ~$30.00 | ~$60.00 | Most expensive |
| OpenAI GPT-3.5 Turbo | ~$0.50 | ~$1.50 | Cost-effective |
| Google Gemini Pro | ~$0.50 | ~$1.50 | Competitive |
| Perplexity | Varies | Varies | Includes web search |
| OpenRouter | Varies | Varies | Aggregates multiple providers |

**Recommendation for Testing**: Use Claude Haiku or GPT-3.5 Turbo for cost-effective testing.

---

## Security Best Practices

1. **Never commit keys to git**
   - `.env` is already in `.gitignore`
   - Use `.env.example` as template only

2. **Rotate keys regularly**
   - Especially if shared or exposed

3. **Use least privilege**
   - Create keys with minimal permissions
   - Use separate keys for dev/test/prod

4. **Monitor usage**
   - Set up billing alerts
   - Review API usage regularly

---

## Testing with Multiple Providers

To test with different providers:

```bash
# Test with Anthropic
export ANTHROPIC_API_KEY="sk-ant-api03-..."
cargo test --test integration_prompt_formatting -- --ignored

# Test with OpenAI (when implemented)
export OPENAI_API_KEY="sk-..."
cargo test --test integration_prompt_formatting -- --ignored

# Test with OpenRouter (accesses multiple providers)
export OPENROUTER_API_KEY="sk-or-v1-..."
cargo test --test integration_prompt_formatting -- --ignored
```

---

## References

- [Anthropic API Docs](https://docs.anthropic.com/claude/reference)
- [OpenAI API Docs](https://platform.openai.com/docs)
- [Google Gemini API Docs](https://ai.google.dev/docs)
- [Perplexity API Docs](https://docs.perplexity.ai/)
- [OpenRouter API Docs](https://openrouter.ai/docs)
- [Qwen API Docs](https://help.aliyun.com/zh/model-studio/)
- [MinMax API Docs](https://docs.minmax.ai/)
- [Grok API Docs](https://docs.x.ai/)

