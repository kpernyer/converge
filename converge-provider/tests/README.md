# Integration Tests for Prompt Formatting

These tests verify the effectiveness of different prompt formats with real LLM providers.

## Prerequisites

1. **API Key**: Set the `ANTHROPIC_API_KEY` environment variable:
   ```bash
   export ANTHROPIC_API_KEY="your-api-key-here"
   ```

2. **Model Access**: Tests use `claude-3-5-haiku-20241022` (fastest/cheapest model for testing)

## Running Tests

### Run All Integration Tests

```bash
cargo test --test integration_prompt_formatting -- --ignored
```

### Run Specific Test

```bash
# Test format correctness
cargo test --test integration_prompt_formatting test_format_correctness_edn_vs_plain -- --ignored

# Test Claude XML preference
cargo test --test integration_prompt_formatting test_claude_xml_preference -- --ignored

# Test response times
cargo test --test integration_prompt_formatting test_response_time_comparison -- --ignored

# Test token compaction
cargo test --test integration_prompt_formatting test_token_compaction -- --ignored

# Test structured parsing
cargo test --test integration_prompt_formatting test_structured_response_parsing -- --ignored
```

## Test Coverage

### 1. Format Correctness (`test_format_correctness_edn_vs_plain`)
- **Purpose**: Verify EDN and plain formats produce similar quality answers
- **Metrics**: Response similarity, token usage, response time
- **Expected**: Similarity > 30%, EDN uses fewer tokens

### 2. Claude XML Preference (`test_claude_xml_preference`)
- **Purpose**: Verify Claude performs better with XML-structured prompts
- **Metrics**: Response structure, response time
- **Expected**: XML responses are more structured, similar or faster response times

### 3. Response Time Comparison (`test_response_time_comparison`)
- **Purpose**: Measure response time differences between formats
- **Metrics**: Average response time over 3 iterations
- **Expected**: EDN format is similar or faster (due to fewer tokens)

### 4. Token Compaction (`test_token_compaction`)
- **Purpose**: Verify EDN format saves tokens vs Markdown
- **Metrics**: Input tokens, total tokens, savings percentage
- **Expected**: 10%+ token savings with EDN

### 5. Structured Response Parsing (`test_structured_response_parsing`)
- **Purpose**: Verify XML response parsing works correctly
- **Metrics**: Number of parsed proposals, proposal structure
- **Expected**: Proposals have valid IDs, content, and confidence scores

## Expected Results

### Token Savings
- **EDN vs Plain**: 10-30% token savings expected
- **EDN vs Markdown**: 20-40% token savings expected

### Response Times
- **EDN**: Similar or faster (fewer tokens = faster processing)
- **Variance**: Network latency can cause variance, tests run 3 iterations

### Response Quality
- **Similarity**: EDN and plain formats should produce similar quality (>30% similarity)
- **Structure**: XML prompts should produce more structured responses

## Cost Considerations

These tests make real API calls and will incur costs:
- **Model**: `claude-3-5-haiku-20241022` (cheapest Claude model)
- **Estimated cost per test run**: ~$0.01-0.05 (depending on prompt size)
- **Full test suite**: ~$0.05-0.25

## Troubleshooting

### "ANTHROPIC_API_KEY environment variable not set"
- Set the environment variable before running tests
- Or use: `ANTHROPIC_API_KEY=your-key cargo test --test integration_prompt_formatting -- --ignored`

### Rate Limiting
- If you hit rate limits, wait a few minutes and retry
- Tests use the cheapest model to minimize rate limit issues

### Network Issues
- Tests require internet connectivity
- Timeouts may occur with slow connections

## Continuous Integration

These tests are marked with `#[ignore]` and should:
- **NOT** run in CI by default (costs money)
- Be run manually before releases
- Be run when making prompt format changes

To enable in CI, uncomment the `#[ignore]` attributes and set API keys in CI secrets.

