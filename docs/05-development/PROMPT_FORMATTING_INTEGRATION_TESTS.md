# Prompt Formatting Integration Tests

This document describes the integration tests that verify prompt formatting effectiveness with real LLM providers.

---

## Overview

The integration tests in `converge-provider/tests/integration_prompt_formatting.rs` verify:

1. **Correctness**: Different formats produce similar quality answers
2. **Format Preferences**: Providers perform better with their preferred formats
3. **Response Times**: Compact formats reduce latency
4. **Token Efficiency**: EDN format saves tokens vs plain text/Markdown

---

## Test Suite

### 1. Format Correctness Test

**Test**: `test_format_correctness_edn_vs_plain`

**Purpose**: Verify that EDN and plain text formats produce similar quality answers.

**What it tests**:
- Sends same context to LLM in both EDN and plain formats
- Compares response similarity using Jaccard similarity of key terms
- Measures token usage for both formats
- Verifies both produce non-empty, reasonable responses

**Expected Results**:
- Similarity > 30% (responses should cover similar topics)
- EDN uses fewer input tokens
- Both responses are non-empty and relevant

**Metrics**:
- Response similarity percentage
- Token usage (input, output, total)
- Token savings percentage

---

### 2. Claude XML Preference Test

**Test**: `test_claude_xml_preference`

**Purpose**: Verify Claude performs better with XML-structured prompts.

**What it tests**:
- Compares XML-wrapped EDN prompts vs plain EDN prompts
- Checks if XML responses are more structured
- Measures response times for both formats

**Expected Results**:
- XML responses may contain structured tags (`<response>`, `<proposal>`)
- Both formats produce valid responses
- Response times are similar (XML might be slightly faster due to better instruction following)

**Metrics**:
- Response structure (presence of XML tags)
- Response time (milliseconds)
- Token usage

---

### 3. Response Time Comparison

**Test**: `test_response_time_comparison`

**Purpose**: Measure response time differences between formats over multiple iterations.

**What it tests**:
- Runs 3 iterations of each format
- Calculates average response times
- Measures token usage
- Compares performance

**Expected Results**:
- EDN format similar or faster response times
- Consistent token savings with EDN
- Low variance across iterations

**Metrics**:
- Average response time (milliseconds)
- Average token usage
- Token savings percentage
- Performance difference

---

### 4. Token Compaction Test

**Test**: `test_token_compaction`

**Purpose**: Verify EDN format saves tokens vs Markdown format.

**What it tests**:
- Compares EDN format vs verbose Markdown format
- Measures actual token usage from API responses
- Calculates token savings

**Expected Results**:
- EDN uses fewer input tokens (10%+ savings expected)
- Total token savings (input + output)
- Character count savings

**Metrics**:
- Input token savings
- Total token savings
- Character count comparison

---

### 5. Structured Response Parsing Test

**Test**: `test_structured_response_parsing`

**Purpose**: Verify XML response parsing works correctly.

**What it tests**:
- Sends XML-optimized prompt to Claude
- Attempts to parse response as XML
- Validates proposal structure (ID, content, confidence)

**Expected Results**:
- If response is XML, parser extracts proposals correctly
- Proposals have valid structure (ID, content, confidence 0-1)
- If response is plain text, parser handles gracefully

**Metrics**:
- Number of parsed proposals
- Proposal structure validity
- Parsing success rate

---

## Running the Tests

### Prerequisites

1. Set `ANTHROPIC_API_KEY` environment variable:
   ```bash
   export ANTHROPIC_API_KEY="your-api-key-here"
   ```

2. Ensure you have access to `claude-3-5-haiku-20241022` (cheapest model for testing)

### Run All Tests

```bash
# From project root
cd converge-provider
cargo test --test integration_prompt_formatting -- --ignored --nocapture
```

Or use the helper script:
```bash
./converge-provider/tests/run_integration_tests.sh
```

### Run Specific Test

```bash
cargo test --test integration_prompt_formatting test_format_correctness_edn_vs_plain -- --ignored --nocapture
```

---

## Expected Results Summary

### Token Efficiency

| Format Comparison | Expected Savings |
|-------------------|------------------|
| EDN vs Plain Text | 10-30% |
| EDN vs Markdown   | 20-40% |
| EDN + XML vs Plain | 15-35% |

### Response Quality

- **Similarity**: EDN and plain formats should produce >30% similar responses
- **Structure**: XML prompts should produce more structured outputs
- **Completeness**: All formats should produce non-empty, relevant responses

### Performance

- **Response Time**: EDN format similar or faster (fewer tokens = faster processing)
- **Variance**: Network latency causes variance; tests run 3 iterations for stability

---

## Cost Considerations

### Per Test Run

- **Model**: `claude-3-5-haiku-20241022` (~$0.25 per 1M input tokens, ~$1.25 per 1M output tokens)
- **Estimated cost per test**: $0.01-0.05
- **Full test suite**: $0.05-0.25

### Cost Optimization

- Tests use cheapest Claude model
- Limited token counts (200-500 tokens)
- Multiple iterations only for response time test

---

## CI/CD Integration

### Default Behavior

Tests are marked with `#[ignore]` and **do not run in CI by default** because:
- They require API keys (security)
- They incur costs
- They depend on external services (network)

### Manual Execution

Run before:
- Releases
- Major prompt format changes
- Performance optimizations

### Enabling in CI (Optional)

If you want to run in CI:

1. Store `ANTHROPIC_API_KEY` in CI secrets
2. Uncomment `#[ignore]` attributes (or use feature flag)
3. Add to CI pipeline:
   ```yaml
   - name: Run integration tests
     env:
       ANTHROPIC_API_KEY: ${{ secrets.ANTHROPIC_API_KEY }}
     run: cargo test --test integration_prompt_formatting -- --ignored
   ```

---

## Troubleshooting

### "ANTHROPIC_API_KEY environment variable not set"

**Solution**: Set the environment variable:
```bash
export ANTHROPIC_API_KEY="your-key"
```

### Rate Limiting

**Symptoms**: Tests fail with rate limit errors

**Solution**:
- Wait a few minutes and retry
- Tests use cheapest model to minimize rate limits
- Consider running tests sequentially instead of in parallel

### Network Timeouts

**Symptoms**: Tests timeout or fail with network errors

**Solution**:
- Check internet connectivity
- Tests require stable connection to Anthropic API
- Consider increasing timeout if needed

### Parsing Failures

**Symptoms**: Structured parsing test returns no proposals

**Solution**:
- This is OK - Claude may return plain text instead of XML
- Test verifies parser handles both cases gracefully
- Check raw response to see actual format

---

## Interpreting Results

### Token Savings

- **>20%**: Excellent - significant cost savings
- **10-20%**: Good - meaningful savings
- **5-10%**: Acceptable - some savings
- **<5%**: May need optimization or different format

### Response Similarity

- **>50%**: Excellent - formats produce very similar results
- **30-50%**: Good - formats cover similar topics
- **<30%**: May indicate format-specific behavior

### Response Times

- **Similar**: Expected - format shouldn't dramatically change latency
- **EDN faster**: Bonus - fewer tokens = faster processing
- **EDN slower**: May indicate parsing overhead (investigate)

---

## Future Enhancements

### Additional Tests

- [ ] OpenAI provider tests (when implemented)
- [ ] Multi-model comparison (Claude vs GPT)
- [ ] Large context tests (10K+ tokens)
- [ ] Streaming response tests

### Metrics

- [ ] Cost per request tracking
- [ ] Response quality scoring (semantic similarity)
- [ ] Parsing accuracy metrics
- [ ] Format preference scoring

### Automation

- [ ] Automated benchmark reports
- [ ] Regression detection
- [ ] Performance trend tracking

---

## References

- [Prompt Structuring Guide](./LLM_PROMPT_STRUCTURING.md)
- [Prompt DSL Implementation](./PROMPT_DSL_IMPLEMENTATION.md)
- [Provider Prompt Structuring](./PROVIDER_PROMPT_STRUCTURING.md)
- [Anthropic API Documentation](https://docs.anthropic.com/claude/reference/messages_post)

