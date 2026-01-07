# Provider-Specific Prompt Structuring

This document describes the LLM prompt structuring implementation in `converge-provider`, which provides provider-specific optimizations on top of the base Converge Prompt DSL.

---

## Overview

The `converge-provider` crate extends the prompt DSL from `converge-core` with:

1. **Provider-specific optimizations** — XML for Claude, JSON for OpenAI
2. **Structured response parsing** — XML/JSON parsers for reliable extraction
3. **Helper functions** — Convenient builders for common use cases

---

## Architecture

```
converge-core::prompt (EDN DSL)
    ↓
converge-provider::prompt (Provider optimizations)
    ├── ProviderPromptBuilder (wraps EDN with provider hints)
    ├── StructuredResponseParser (parses XML/JSON responses)
    └── Helper functions (build_claude_prompt, build_openai_prompt)
```

---

## Components

### 1. ProviderPromptBuilder

Enhances base `AgentPrompt` with provider-specific format hints.

**Features:**
- Wraps EDN prompts in XML tags for Claude (better instruction following)
- Adds JSON output instructions for OpenAI
- Maintains token efficiency of base EDN format

**Example:**
```rust
use converge_provider::ProviderPromptBuilder;
use converge_core::prompt::{AgentPrompt, AgentRole, OutputContract, PromptContext};

let base = AgentPrompt::new(
    AgentRole::Proposer,
    "extract-competitors",
    context,
    OutputContract::new("proposed-fact", ContextKey::Competitors),
);

let builder = ProviderPromptBuilder::new(base)
    .with_output_format("xml");

let claude_prompt = builder.build_for_claude();
```

### 2. StructuredResponseParser

Parses structured LLM responses into `ProposedFact`s.

**Supported formats:**
- **Claude XML** — `<response><proposals><proposal id="..." confidence="0.85">content</proposal></proposals></response>`
- **OpenAI JSON** — `{"proposals": [{"id": "...", "content": "...", "confidence": 0.85}]}`
- **Generic fallback** — Treats entire response as single proposal

**Example:**
```rust
use converge_provider::StructuredResponseParser;
use converge_core::llm::LlmResponse;

let proposals = StructuredResponseParser::parse_claude_xml(
    &response,
    ContextKey::Competitors,
    "anthropic",
);
```

### 3. Helper Functions

Convenient builders for common use cases.

**`build_claude_prompt`** — Builds optimized Claude prompt with XML structure
**`build_openai_prompt`** — Builds optimized OpenAI prompt with JSON instructions

**Example:**
```rust
use converge_provider::build_claude_prompt;
use converge_core::prompt::{AgentRole, OutputContract, PromptContext};
use converge_core::context::ContextKey;

let prompt = build_claude_prompt(
    AgentRole::Proposer,
    "extract-competitors",
    PromptContext::new(),
    OutputContract::new("proposed-fact", ContextKey::Competitors),
    vec![Constraint::NoInvent, Constraint::NoHallucinate],
);
```

---

## Provider-Specific Optimizations

### Claude (Anthropic)

**Input format:**
- EDN prompt wrapped in `<prompt>` tags
- XML output instructions added
- Benefits from Claude's XML tag awareness

**Output format:**
- XML with structured proposals
- Confidence scores as attributes
- Easy to parse reliably

**Example prompt:**
```xml
<prompt>
{:r :proposer
 :o :extract-competitors
 :c {:signals [{:id "s1" :c "Revenue +15% Q3"}]}
 :k #{:no-invent :no-contradict}
 :out {:emit :proposed-fact :key :competitors}}
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

### OpenAI (GPT-4, GPT-3.5)

**Input format:**
- EDN prompt with JSON output instructions
- Explicit JSON schema in prompt
- Compatible with JSON mode

**Output format:**
- JSON object with proposals array
- Structured fields (id, content, confidence)
- Reliable parsing with serde_json

**Example prompt:**
```
Prompt (EDN format):
{:r :proposer :o :extract-competitors ...}

Respond with a JSON object containing an array of proposals:
{
  "proposals": [
    {"id": "...", "content": "...", "confidence": 0.0-1.0}
  ]
}
```

---

## Usage Patterns

### Pattern 1: Direct Provider Integration

```rust
use converge_provider::{AnthropicProvider, build_claude_prompt, StructuredResponseParser};
use converge_core::llm::{LlmProvider, LlmRequest};
use converge_core::prompt::{AgentRole, OutputContract, PromptContext};
use converge_core::context::ContextKey;

// Build prompt
let prompt = build_claude_prompt(
    AgentRole::Proposer,
    "analyze-market",
    PromptContext::from_context(&ctx, &[ContextKey::Signals]),
    OutputContract::new("proposed-fact", ContextKey::Strategies),
    vec![Constraint::NoHallucinate],
);

// Send to provider
let provider = AnthropicProvider::from_env("claude-3-5-sonnet-20241022")?;
let response = provider.complete(&LlmRequest::new(prompt))?;

// Parse structured response
let proposals = StructuredResponseParser::parse_claude_xml(
    &response,
    ContextKey::Strategies,
    "anthropic",
);
```

### Pattern 2: Custom Builder

```rust
use converge_provider::ProviderPromptBuilder;
use converge_core::prompt::{AgentPrompt, AgentRole, OutputContract, PromptContext};

let base = AgentPrompt::new(
    AgentRole::Synthesizer,
    "synthesize-strategies",
    context,
    OutputContract::new("proposed-fact", ContextKey::Strategies)
        .with_format("xml"),
);

let builder = ProviderPromptBuilder::new(base)
    .with_output_format("xml");

let prompt = builder.build_for_claude();
```

---

## Token Efficiency

### Comparison

| Format | Character Count | Token Estimate | Use Case |
|--------|----------------|----------------|----------|
| Plain Markdown | ~200 | ~50 | Baseline |
| EDN (base) | ~120 | ~30 | All providers |
| EDN + XML (Claude) | ~180 | ~35 | Claude (better parsing) |
| EDN + JSON (OpenAI) | ~160 | ~32 | OpenAI (JSON mode) |

**Savings:** 30-40% token reduction vs plain Markdown, with better structure.

---

## Error Handling

### Parsing Errors

**XML parsing:**
- Falls back to simple line-by-line extraction
- Handles malformed XML gracefully
- Returns empty vector if no proposals found

**JSON parsing:**
- Returns `Result<Vec<ProposedFact>, String>` for error handling
- Validates required fields (id, content)
- Defaults confidence to 0.7 if missing

**Generic parsing:**
- Always succeeds (treats entire response as single proposal)
- Generates ID from timestamp
- Uses default confidence (0.7)

---

## Testing

All components are tested:

- ✅ `test_claude_prompt_building` — Verifies XML wrapping
- ✅ `test_openai_prompt_building` — Verifies JSON instructions
- ✅ `test_claude_xml_parsing` — Parses XML responses
- ✅ `test_openai_json_parsing` — Parses JSON responses

Run tests:
```bash
cargo test --package converge-provider prompt::tests
```

---

## Future Enhancements

### Phase 1: Enhanced Parsing
- [ ] Use proper XML parser (e.g., `quick-xml`) for robustness
- [ ] Support nested structures in responses
- [ ] Validate confidence ranges

### Phase 2: More Providers
- [ ] Google Gemini support
- [ ] Perplexity support
- [ ] Local model support (Ollama, etc.)

### Phase 3: Advanced Features
- [ ] Streaming response parsing
- [ ] Multi-format fallback (try XML, then JSON, then plain)
- [ ] Prompt caching for repeated patterns

---

## Alignment with Converge Principles

### Correctness First ✅
- Structured formats → deterministic parsing
- Type-safe contracts → easier validation
- Explicit schemas → fewer errors

### LLMs as Tools ✅
- Compact contracts → predictable outputs
- Format validation → catch errors early
- Provenance tracking → format in prompt hash

### Token Efficiency ✅
- 30-40% savings → lower costs
- Faster responses → better UX
- Scalability → handle larger contexts

---

## References

- [Converge Prompt DSL](../05-development/LLM_PROMPT_STRUCTURING.md)
- [Prompt DSL Implementation](../05-development/PROMPT_DSL_IMPLEMENTATION.md)
- [Claude XML Tags](https://docs.anthropic.com/claude/docs/use-xml-tags)
- [OpenAI JSON Mode](https://platform.openai.com/docs/guides/text-generation/json-mode)

