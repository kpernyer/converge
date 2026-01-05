# Architecture Documentation

This directory contains detailed architectural specifications for Converge.

## Core Architecture

### [ARCHITECTURE.md](./ARCHITECTURE.md)
High-level system architecture, layers, and core axioms.

**Read this first** to understand the overall system design.

### [ENGINE_EXECUTION_MODEL.md](./ENGINE_EXECUTION_MODEL.md)
Detailed execution model: how agents run, how effects merge, how convergence is detected.

**Key concepts:**
- Parallel compute, serialized commit
- Eligibility phase
- Effect buffering
- Merge phase
- Convergence detection

### [CONVERGENCE_SEMANTICS.md](./CONVERGENCE_SEMANTICS.md)
Deep dive into convergence: how it's guaranteed, why infinite loops are impossible.

**Key guarantees:**
- Monotonicity of context
- Bounded fact space
- Budget enforcement
- Data-driven eligibility

### [CONVERGENCE_PROOFS.md](./CONVERGENCE_PROOFS.md)
Formal proofs and arguments for convergence guarantees.

## Data Models

### [ROOT_INTENT_SCHEMA.md](./ROOT_INTENT_SCHEMA.md)
The Root Intent schema — the constitution of every Converge job.

**Key fields:**
- IntentKind
- Objective
- Scope
- Constraints
- SuccessCriteria
- Budgets

### [GHERKIN_MODEL.md](./GHERKIN_MODEL.md)
How Gherkin expresses invariants (not workflows).

**Key classes:**
- Structural invariants (continuous)
- Semantic invariants (per-cycle)
- Acceptance invariants (convergence gates)

### [LLM_INTEGRATION.md](./LLM_INTEGRATION.md)
How LLMs are integrated without sacrificing correctness.

**Key principle:** "LLMs may suggest. The engine decides."

**Key mechanism:** ProposedFact → Fact via explicit validation

## Agent Model

### [AGENTS.md](./AGENTS.md)
Agent taxonomy and capabilities.

**Agent types:**
- Deterministic agents
- Retrieval agents
- LLM agents
- Solver agents
- Governance agents

### [AGENT_LIFECYCLE.md](./AGENT_LIFECYCLE.md)
Agent lifecycle and execution model.

### [COMMUNICATION_MODEL.md](./COMMUNICATION_MODEL.md)
How agents communicate (via context, not direct calls).

---

## Reading Order

**For understanding the system:**
1. ARCHITECTURE.md
2. ENGINE_EXECUTION_MODEL.md
3. CONVERGENCE_SEMANTICS.md

**For implementing agents:**
1. AGENTS.md
2. AGENT_LIFECYCLE.md
3. COMMUNICATION_MODEL.md

**For implementing use-cases:**
1. ROOT_INTENT_SCHEMA.md
2. GHERKIN_MODEL.md
3. LLM_INTEGRATION.md (if using LLMs)

---

## For AI Agents

These documents are **authoritative specifications**. When implementing:
- Follow the execution model exactly
- Respect the agent model constraints
- Ensure convergence semantics are preserved
- Use Root Intent schema as defined

