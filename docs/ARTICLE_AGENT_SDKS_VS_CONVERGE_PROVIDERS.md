# Agent SDKs vs Converge (and why Converge-Providers matters)

This article is written to work in two modes:

- **Marketing mode:** a clean story for founders, engineers, and AI-native startups.
- **Implementation mode:** concrete guidance for how to structure `converge-providers` without drifting into “yet another agent framework.”

---

## The short version

Agent SDKs (Anthropic’s **Claude Agent SDK**, OpenAI’s **Agents SDK**) help you build *agents that act*: read files, call tools, run commands, hand off tasks, and orchestrate multi-step loops. citeturn0search8turn0search11turn0search9turn0search12

**Converge** is not trying to be a better agent loop.

Converge is a **semantic convergence engine**:
- Agents are **pure contributors**: `Context → AgentEffect`
- The engine is the **single semantic authority**
- Execution halts at a **fixed point** (convergence)
- Safety comes from explicit boundaries (**ProposedFact → validated Fact**)
- Everything is traceable (provenance + deterministic merge + invariants)

Agent SDKs can still be extremely useful inside Converge—**as libraries used by specific agents**—but they should never become the thing that owns global execution.

---

## What an Agent SDK is optimized for

Agent SDKs generally solve the “agent harness” problem:

- Orchestrate **tool use** and **multi-step execution**
- Manage agent **context** and **state**
- Provide primitives for **handoffs**, **tracing**, and “agent-y” control flow citeturn0search11turn0search9turn0search12

Anthropic frames the Claude Agent SDK as tooling to build agents “on top of Claude Code,” i.e., a programmable agentic harness. citeturn0search11turn0search8turn0search13

OpenAI frames its Agents SDK as a lightweight framework with primitives to build agentic apps with tool use, handoffs, and full traces. citeturn0search9turn0search12turn0search17

This is great—but it creates a default gravity:
- **agents own control flow**
- **agents own loops**
- “success” often means **task completion**, not **system convergence**

That gravity is exactly what Converge is resisting.

---

## What Converge is optimized for

Converge is optimized for building **business-grade agent systems** where you can say:

- “We halt.”
- “We can explain every decision.”
- “We can restart safely.”
- “Agents can be wrong without breaking the system.”

This requires three things to hold at the same time:

1. **Safety by construction** (types enforce authority boundaries)
2. **Zero-trust agency** (agents suggest; engine validates)
3. **Transparent determinism** (traceable, reproducible, auditable)

Agent SDKs can help you write agents faster.  
Converge exists so you can **trust the outcome**.

---

# Where `converge-providers` fits

If `converge-core` is the kernel, then `converge-providers` is your **hardware abstraction layer**:

- LLMs (remote + local)
- retrieval (vector stores, rerankers, hybrid search)
- knowledge graphs
- SaaS connectors
- filesystems and “tools” (web, email, docs, etc.)

The key is: **providers are not orchestration.**

Providers should never:
- schedule work
- run background loops
- retry indefinitely
- “decide” completion
- mutate context

Providers should only:
- execute a **bounded request**
- return **data + metadata**
- allow Converge agents to turn that into **ProposedFacts** and **Evidence**

Think of providers as “syscalls,” not “daemons.”

---

## Your current provider drift (and how to turn it into a clean structure)

You described a natural evolution:

- a “smartness module” for various LLMs
- LanceDB and vector retrieval
- Ollama for local execution
- knowledge in graph databases

This is normal. The danger is that the provider layer becomes a junk drawer of mixed concerns.

A cleaner taxonomy (opinionated) is:

### 1) `converge-providers-llm`
**Goal:** unify “generate / classify / extract / embed / rerank” across vendors and local models.

- OpenAI
- Anthropic
- Gemini
- Local (Ollama)  
- Optional: Perplexity-style web-research provider (as a tool call)

Key design: **capability-based traits**, not vendor-based traits.

```rust
pub trait TextModel {
    async fn generate(&self, req: GenerateRequest) -> Result<GenerateResponse>;
}

pub trait EmbeddingModel {
    async fn embed(&self, req: EmbedRequest) -> Result<EmbedResponse>;
}

pub trait Reranker {
    async fn rerank(&self, req: RerankRequest) -> Result<RerankResponse>;
}
```

This prevents “LLM provider sprawl” inside agents.

### 2) `converge-providers-retrieval`
**Goal:** fetch candidate evidence; never assert truth.

- LanceDB / Qdrant / other vector DBs
- Hybrid retrieval (BM25 + vector)
- Reranker integration (if it’s a separate service)

Return payload should be explicit:

```rust
pub struct RetrievedChunk {
    pub source: SourceRef,        // URL / file / doc id
    pub text: String,
    pub score: f32,
    pub fingerprint: String,      // stable hash for auditability
    pub retrieved_at: SystemTime,
}
```

### 3) `converge-providers-kg`
**Goal:** graph queries and graph updates, but “updates” should usually be **derived** and auditable.

- Neo4j, SurrealDB graph mode, etc.

The engine should treat KG results as **evidence** or **candidate facts**, not authoritative truth.

### 4) `converge-providers-connectors`
**Goal:** “enterprise glue”—CRM, email, calendar, billing, ticketing.

This is where “SMB business OS” becomes real.

Each connector should expose:
- fetch operations (bounded, paged)
- write operations (explicit, idempotent, versioned)

No hidden sync loops. If you want sync, do it as a Converge job.

---

# Should you split `converge-providers` now?

You can split **by crate** immediately, and split **by repo** later.

### Recommended now (fast + clean)
- Keep a **single Git repo** (workspace)
- Split providers into crates with strict boundaries:
  - `converge-core` (no heavy deps)
  - `converge-providers-*` (feature-gated, optional)
  - `converge-domain` (use-cases)
  - `converge-tools` (CLI, local runners, dashboards)

Why: you want coherent versioning while the contracts are still moving.

### Recommended later (once contracts stabilize)
Split into repos if:
- you want separate release cadence
- providers attract external contributors
- you want smaller review surfaces
- you want “optional ecosystems” around the kernel

But: splitting repos too early often increases maintenance friction (CI, releases, cross-version breakage).

A good compromise is: **workspace first**, then **publish crates**, then **repo split** if the ecosystem actually demands it.

---

# Where Anthropic/OpenAI Agent SDKs fit in Converge

Converge can “use” these SDKs in two valid ways:

## Pattern A: As a provider behind a Converge agent
A Converge agent calls:
- `TextModel::generate()` (implemented using OpenAI/Anthropic SDKs under the hood)
- `Tool` operations (web/search, filesystem, etc.)

But the agent **does not** delegate control flow to the SDK.

## Pattern B: As an implementation helper inside a single agent
For example, you might use an Agent SDK for:
- codebase traversal
- tool invocation convenience
- prompt management utilities

Still: Converge owns the lifecycle.

This aligns with how these SDKs describe themselves as enabling tool-using agent behaviors and orchestration primitives. citeturn0search11turn0search9turn0search12

Converge simply says: “fine—inside an agent; never as the global runtime.”

---

# RAG in Converge: retrieval is evidence, not truth

RAG becomes dramatically safer in Converge if you adopt one rule:

> Retrieval results never become Facts directly. They become **Evidence** that supports (or falsifies) ProposedFacts.

### A Converge-native RAG flow

1) Retrieval agent queries LanceDB/Qdrant and emits:
- `EvidenceChunk` facts (or evidence records under a dedicated key)
- provenance for every chunk (source + fingerprint)

2) LLM agent uses evidence and emits **ProposedFacts**:
- include cited evidence fingerprints

3) Deterministic validator promotes **ProposedFact → Fact** only if:
- evidence exists
- invariant checks pass (e.g., “must have ≥2 independent sources”)
- optional: schema checks, unit normalization, guardrails

This lets you scale RAG without letting “retrieved garbage” pollute the long-term semantic context.

### Why this matters
Most RAG systems collapse:
- retrieval
- reasoning
- decision
into a single opaque agent loop.

Converge separates them into auditable phases.

---

# The implementation “golden contracts” for providers

If you want `converge-providers` to remain healthy, enforce these in code review:

1) **Bounded calls only**
- every provider call has timeouts
- every provider call returns structured metadata

2) **Deterministic envelopes**
Even if the provider is non-deterministic (LLMs), the envelope is stable:
- request id
- model id
- parameters
- timestamps
- provenance

3) **No implicit retries**
Retries belong in Converge agents (where budgets and policy live), not inside providers.

4) **No global state**
Providers can have caches, but they must be bounded and non-authoritative.

---

# A practical next step

If you want to start refactoring `converge-providers` without disrupting velocity:

1) Create a top-level `Provider` traits module:
- `llm::{TextModel, EmbeddingModel, Reranker}`
- `retrieval::{Retriever}`
- `graph::{GraphStore}`

2) Move LanceDB, Ollama, graph DB, and vendor SDK adapters behind those traits.

3) Add one end-to-end “provider contract test” per provider:
- deterministic request formatting
- stable provenance fields
- consistent error mapping

This gives you the “systems core” discipline *in the provider layer*, where projects most often drift into jelly.

---

## Closing: the relationship in one line

- **Agent SDKs** help you build agents that act.
- **Converge** helps you build systems you can trust.

And `converge-providers` is how Converge touches the messy real world—**without letting the mess into the kernel**.
