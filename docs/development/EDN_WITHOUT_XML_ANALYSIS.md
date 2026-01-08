# EDN Without XML Wrapping - Analysis

This document analyzes what happens when we remove XML wrapping from EDN prompts.

---

## Current Implementation

Currently, `build_claude_prompt()` wraps EDN in XML tags:

```rust
<prompt>
{:r :proposer
 :o :analyze-market
 :c {:signals [...]}
 :k #{:no-hallucinate}
 :out {:emit :proposed-fact :key :strategies}}
</prompt>

<instructions>
Respond in XML format with the following structure:
<response>
  <proposals>
    <proposal id="..." confidence="0.0-1.0">content</proposal>
  </proposals>
</response>
</instructions>
```

**Overhead**: ~140-200 characters of XML tags and instructions

---

## Pure EDN Format

The new `build_edn_only()` method returns pure EDN:

```edn
{:r :proposer
 :o :analyze-market
 :c {:signals [{:id "s1" :c "Nordic B2B SaaS market growing 15% annually"}]}
 :k #{:no-hallucinate}
 :out {:emit :proposed-fact :key :strategies}}
```

**No overhead**: Just the compact EDN structure

---

## Expected Differences

### Token Usage

**With XML Wrapping:**
- Input tokens: ~195 tokens (EDN + XML tags + instructions)
- Output tokens: ~141 tokens (structured XML response)
- Total: ~336 tokens

**Without XML Wrapping (Pure EDN):**
- Input tokens: ~80-100 tokens (just EDN)
- Output tokens: ~200-300 tokens (may be less structured)
- Total: ~280-400 tokens (depends on response structure)

**Expected Savings:**
- Input tokens: **50-60% reduction** (removing XML overhead)
- Output tokens: **May increase** (less structured = more verbose)
- Total tokens: **Variable** (depends on response verbosity)

### Response Quality

**With XML:**
- ✅ Highly structured responses (XML format)
- ✅ Easy to parse programmatically
- ✅ Better instruction following (Claude respects XML tags)
- ✅ Consistent format

**Without XML:**
- ⚠️ May be less structured (free-form text or EDN)
- ⚠️ Harder to parse reliably
- ⚠️ May be more verbose
- ✅ More token-efficient input

### Response Time

**Expected**: Similar or slightly faster without XML (fewer input tokens = faster processing)

---

## Test: `test_edn_with_vs_without_xml`

A new integration test compares both approaches:

```rust
// XML-wrapped (current)
let xml_prompt = ProviderPromptBuilder::new(base.clone())
    .with_output_format("xml")
    .build_for_claude();

// Pure EDN (new)
let edn_only_prompt = ProviderPromptBuilder::new(base)
    .build_edn_only();
```

**Metrics measured:**
- Character count difference
- Input/output/total token usage
- Response time
- Response structure (XML vs free-form)
- Parsing success rate

---

## When to Use Each Format

### Use XML-Wrapped EDN When:
- ✅ You need structured, parseable responses
- ✅ Response quality and consistency are critical
- ✅ You're building production systems
- ✅ You can afford the input token overhead
- ✅ You want guaranteed XML output format

### Use Pure EDN When:
- ✅ Token costs are a primary concern
- ✅ You're doing exploratory testing
- ✅ Response structure is less important
- ✅ You're using a provider that doesn't benefit from XML
- ✅ You want maximum input token efficiency

---

## Trade-offs Summary

| Aspect | XML-Wrapped | Pure EDN |
|--------|-------------|----------|
| Input Tokens | Higher (~195) | Lower (~80-100) |
| Output Tokens | Lower (~141) | Higher (~200-300) |
| Total Tokens | Similar/Lower | Similar/Higher |
| Response Structure | Excellent (XML) | Variable |
| Parsing Reliability | High | Medium |
| Instruction Following | Excellent | Good |
| Token Efficiency | Good (output savings) | Good (input savings) |

---

## Recommendation

**For Production**: Use XML-wrapped EDN
- The input token overhead is worth it for:
  - Structured, parseable responses
  - Better instruction following
  - Consistent output format
  - Lower output tokens (often offsets input overhead)

**For Testing/Exploration**: Use pure EDN
- When you want to measure base EDN efficiency
- When testing different providers
- When token costs are critical

**Hybrid Approach**: Make it configurable
- Allow users to choose based on their needs
- Default to XML-wrapped for production
- Provide pure EDN as an option

---

## Implementation

Both methods are now available:

```rust
use converge_provider::ProviderPromptBuilder;

// XML-wrapped (default for Claude)
let xml_prompt = builder.build_for_claude();

// Pure EDN (no XML overhead)
let edn_prompt = builder.build_edn_only();
```

The test `test_edn_with_vs_without_xml` will provide empirical data on the actual differences when run with API credits.

---

## Next Steps

1. Run the test with API credits to get real metrics
2. Analyze the results to determine optimal default
3. Consider making XML wrapping optional/configurable
4. Document findings in this analysis

