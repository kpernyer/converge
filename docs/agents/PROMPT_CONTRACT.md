# LLM Prompt Structuring Guide

This document defines the **Converge Prompt DSL** — a compact, machine-to-machine contract format for agent prompts.

---

## Core Principle: Two Surfaces

**Critical distinction:**

1. **Agent Prompt Surface** (this document)
   - Never shown to humans
   - Optimized for: tokens, determinism, speed
   - Consumed by: LLMs, solvers, Rust agents
   - Format: Compact EDN-like DSL

2. **Outcome / Explanation Surface** (separate concern)
   - Human-readable
   - Narrative, visual, causal graphs, summaries
   - Derived from provenance + context, not from prompts
   - Generated downstream by explanation agents

**Key insight:** LLMs don't need friendly prompts. They need clear contracts.

---

## Current State

The current implementation uses plain text prompts with markdown-style formatting:

```rust
// Current approach in build_prompt()
writeln!(context_str, "\n## {key:?}");
writeln!(context_str, "- {}: {}", fact.id, fact.content);
```

**Problems:**
- Token-heavy (markdown overhead)
- Human-friendly but unnecessary for machines
- Ambiguous parsing
- No structural contracts

---

## Format Evaluation

### ❌ Formats to Avoid for Agent Prompts

**YAML**
- Indentation-sensitive
- Token-heavy
- Brittle
- LLMs mess it up under pressure

**JSON (as the whole prompt)**
- Too verbose
- Quotation overhead
- Poor instruction readability
- JSON is great for **outputs**, not inputs

**Markdown sections**
- Human-friendly
- Token-expensive
- Unnecessary if no human reads it

**Plain text with prose**
- Verbose
- Ambiguous
- No structure

---

### ✅ Recommended: EDN (Extensible Data Notation)

**Why EDN works extremely well for agent prompts:**

1. **More compact than YAML/JSON** — 30-50% token savings
2. **Fewer punctuation tokens** than JSON (no quotes for keywords)
3. **Very clear structure** — unambiguous parsing
4. **Maps cleanly to Rust data models** — serde-compatible
5. **LLMs handle it surprisingly well** — clear boundaries
6. **No indentation sensitivity** — robust

**Example:**
```edn
{:r :agent
 :o :market-analysis
 :c {:signals [{:id "s1" :content "Revenue +15% Q3"}]
     :competitors [{:id "c1" :content "Acme Corp"}]}
 :k #{:no-hallucinate :no-contradict}
 :out {:emit :proposed-fact :key :strategies}}
```

**Token comparison:**
- Markdown: ~150 tokens
- JSON: ~120 tokens
- EDN: ~80 tokens (47% savings vs JSON)

---

### Alternative: TOML (Second Choice)

**Pros:**
- Widely understood
- Easy to parse in Rust
- Stable spec

**Cons:**
- More tokens than EDN
- Section headers add overhead
- Less expressive for nested semantics

**Verdict:** Good, but not optimal for token-minimal prompts.

---

### Alternative: S-expressions (Research)

**Pros:**
- Extremely compact
- Unambiguous
- Great for formal semantics

**Cons:**
- Unfamiliar to many
- Slightly higher cognitive cost
- Some LLMs less consistent unless trained

**Verdict:** Very attractive for research systems, slightly risky for broad adoption.

---

## Converge Prompt DSL v0.1

### Canonical Internal Representation

```rust
pub struct AgentPrompt {
    pub role: AgentRole,
    pub objective: String,
    pub context: PromptContext,
    pub constraints: HashSet<Constraint>,
    pub output_contract: OutputContract,
}
```

### Serialization to Compact EDN

**Minimal keywords:**
- `:r` = role
- `:o` = objective
- `:c` = context
- `:k` = constraints (keywords)
- `:out` = output contract

**Stable ordering:** Deterministic serialization for hashability

**Example:**
```edn
{:r :proposer
 :o :extract-competitors
 :c {:signals [{:id "s1" :content "..."}]
     :competitors []}
 :k #{:no-invent :no-contradict}
 :out {:emit :proposed-fact :key :competitors}}
```

**Token savings:** ~50-60% fewer tokens than Markdown

---

## Implementation

### Phase 1: EDN Serialization

1. **Add EDN serialization** to `AgentPrompt`
2. **Update `build_prompt()`** to use EDN format
3. **Add format detection** (EDN for all, provider-specific output)

### Phase 2: Structured Outputs

1. **Request EDN/JSON outputs** based on provider
2. **Update parsers** to handle structured responses
3. **Validate structure** before parsing

### Phase 3: Token Optimization

1. **Benchmark token usage** vs current format
2. **Add compression** for large contexts
3. **Monitor TOON** adoption

---

## Best Practices

### 1. Use Compact Keywords

**Bad:**
```edn
{:role :proposer
 :objective :extract-competitors
 :context {...}}
```

**Good:**
```edn
{:r :proposer
 :o :extract-competitors
 :c {...}}
```

**Rationale:** Keywords are tokenized, shorter = fewer tokens.

### 2. Minimize Redundancy

**Bad:**
```edn
{:c {:market-data {:data [{:id "f1" :content "Revenue increased"}]}}}
```

**Good:**
```edn
{:c {:md [{:id "f1" :c "Revenue increased"}]}}
```

**Trade-off:** Clarity vs. token savings. Use abbreviations for repeated terms.

### 3. Use Sets for Constraints

**Good:**
```edn
:k #{:no-hallucinate :no-contradict :no-invent}
```

**Why:** Sets are compact and semantically clear.

### 4. Structure Output Contracts

**Good:**
```edn
:out {:emit :proposed-fact :key :strategies :format :edn}
```

**Why:** Explicit contracts reduce parsing errors.

---

## Model-Specific Considerations

### Claude (Anthropic)

- **EDN input** — works well, clear structure
- **XML output** — request XML tags for responses
- **Best practice:** EDN prompt, XML response

### OpenAI (GPT-4, GPT-3.5)

- **EDN input** — works, but less optimized
- **JSON output** — use JSON mode for responses
- **Best practice:** EDN prompt, JSON response

### Other Models (via OpenRouter)

- **EDN input** — universal fallback
- **Check provider docs** — format support varies
- **Default to EDN** — safest compact format

---

## Alignment with Converge Principles

### Correctness First

- **Structured formats improve parsing reliability** → fewer errors
- **Type-safe formats** → easier validation
- **Explicit schemas** → deterministic parsing

### LLMs as Tools

- **Structured prompts constrain outputs** → more predictable
- **Format validation** → catch errors early
- **Provenance tracking** → format is part of prompt hash

### Token Efficiency

- **Reduced costs** → more experiments possible
- **Faster responses** → better UX
- **Scalability** → handle larger contexts

---

## Migration Path

### Step 1: Add EDN Support (Non-Breaking)

```rust
pub enum PromptFormat {
    Plain,  // Default, backward compatible
    Edn,    // New compact format
}

impl LlmAgentConfig {
    pub fn with_format(mut self, format: PromptFormat) -> Self {
        self.format = format;
        self
    }
}
```

### Step 2: Update Prompt Builders

```rust
impl LlmAgent {
    fn build_prompt(&self, ctx: &Context) -> String {
        match self.config.format() {
            PromptFormat::Edn => self.build_prompt_edn(ctx),
            PromptFormat::Plain => self.build_prompt_plain(ctx),
        }
    }
    
    fn build_prompt_edn(&self, ctx: &Context) -> String {
        // Serialize to EDN format
    }
}
```

### Step 3: Make EDN Default

```rust
// Auto-detect format (EDN for all providers)
let format = PromptFormat::Edn;
```

---

## Token Efficiency Guidelines

### Target Savings

| Format | Token Savings | Use When |
|--------|---------------|----------|
| EDN | 50-60% vs Markdown | All agent prompts (default) |
| Plain | 0% (baseline) | Backward compatibility only |
| XML | 20-30% vs Markdown | Claude output responses |
| JSON | 0% (but enables JSON mode) | OpenAI output responses |

### When to Optimize

**Optimize when:**
- Context > 10,000 tokens
- Repeated patterns in data
- Cost-sensitive workloads
- Large batch operations

**Don't optimize when:**
- Context < 1,000 tokens
- Debugging/development
- One-off requests

---

## Summary

1. **Use EDN for agent prompts** — 50-60% token savings, deterministic
2. **Separate agent prompts from human explanations** — different surfaces
3. **Use structured outputs** — XML for Claude, JSON for OpenAI
4. **Keep plain text as fallback** — backward compatibility
5. **Monitor TOON** — future token-efficient format

**Priority:** Implement EDN format for all agent prompts (immediate win), add structured output parsing (next phase).

---

## References

- [EDN Specification](https://github.com/edn-format/edn)
- [Claude XML Tags](https://docs.anthropic.com/claude/docs/use-xml-tags)
- [OpenAI JSON Mode](https://platform.openai.com/docs/guides/text-generation/json-mode)
- [TOON Specification](https://en.wikipedia.org/wiki/Token-Oriented_Object_Notation)
