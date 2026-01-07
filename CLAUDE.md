# CLAUDE.md - Converge Project Guide

## Project Overview

Converge is an open-source semantic convergence engine written in Rust. It's a foundational runtime library for building correctness-first, context-driven multi-agent systems that provably converge.

**Core Philosophy**: Replace configuration with intent, workflows with convergence, and automation with explainable decisions. This is NOT an agent framework, NOT a workflow engine, and NOT event-driven—it's a system for alignment.

## Quick Reference

### Build & Test Commands

```bash
# Building
just build              # Build all crates (release)
just build-core         # Build converge-core only
just build-runtime      # Build runtime server

# Testing
just test               # Run all tests
just test-core          # Test core only
just test-runtime       # Test runtime only

# Running
just run-server         # Start HTTP server
just run-server-trace   # Start with RUST_LOG=info

# Code Quality
just fmt                # Format code
just lint               # Run clippy with -D warnings
```

### Workspace Structure

```
converge-core/      # Core engine (PRIVATE - compiled library only)
converge-provider/  # LLM providers (Anthropic implemented)
converge-domain/    # Domain-specific agents and examples
converge-runtime/   # HTTP/gRPC/TUI server
converge-tool/      # Development utilities (Gherkin parsing)
```

## Critical Rules

### Semantic Guarantees (Non-Negotiable)

1. **Determinism**: Same input must produce same output
2. **Single authority**: One semantic authority per intent
3. **No hidden control flow**: No message buses, no background execution
4. **Agents suggest, engine decides**: Agents never call each other
5. **Append-only truth**: Context is immutable once written

### Code Style Requirements

**Error Handling**:
- NO `unwrap()`, `expect()`, or `panic!()` in production code
- Use `thiserror` for domain errors
- Use `anyhow` only at boundaries (CLI, HTTP handlers)

**Ownership**:
- Default to `let`, not `let mut`
- Prefer iterators over mutating loops
- Avoid unnecessary `.clone()` (document when required)
- No global state or singletons

**Data Modeling**:
- Use newtypes for IDs (e.g., `AgentId(u32)`)
- Prefer enums over boolean flags
- Use `#[non_exhaustive]` for public enums

**Async**:
- Never hold locks across `.await`
- Add timeouts around external calls
- Use `tokio::sync` primitives in async code

### Testing Requirements

- TDD: Red → Green → Refactor
- Target >80% code coverage for new code
- Use `proptest` for property-based testing
- All changes must pass `cargo clippy -- -D warnings`

## Architecture Decisions

### Core Patterns

**Agent Trait**:
```rust
pub trait Agent: Send + Sync {
    fn name(&self) -> &str;
    fn dependencies(&self) -> &[ContextKey];
    fn accepts(&self, ctx: &Context) -> bool;  // Pure, no side effects
    fn execute(&self, ctx: &Context) -> AgentEffect;  // Read-only
}
```

**Context Keys**: Seeds, Hypotheses, Strategies, Constraints, Signals, Competitors, Evaluations, Proposals, Diagnostic

**Type Boundary**: `ProposedFact` is separate from `Fact` - LLMs can only emit ProposedFact, which must be validated before becoming a Fact.

### Engine Execution Model

1. Register agents (builds dependency index)
2. Initialize context from RootIntent
3. Loop until convergence:
   - Find eligible agents (dirty keys + dependencies)
   - Execute in parallel (Rayon)
   - Merge effects serially (deterministic by AgentId order)
   - Detect fixed point (no new facts = converged)
4. Return ConvergeResult

### Deliberate Non-Choices

These are **rejected by design**:
- Message buses (Kafka, NATS, Pub/Sub)
- Workflow engines (Temporal, Cadence)
- Actor systems
- Event-driven orchestration
- Distributed consensus

These violate semantic guarantees by introducing implicit control flow.

## File Locations

| Need | Location |
|------|----------|
| Design principles | `DESIGN_TENETS.md` |
| Contribution rules | `CONTRIBUTOR_GUIDE.md` |
| Architecture docs | `docs/02-architecture/` |
| Development status | `docs/05-development/STATUS.md` |
| Locked decisions | `docs/05-development/DECISIONS.md` |
| Task workflow | `conductor/workflow.md` |
| Rust style guide | `conductor/code_styleguides/rust.md` |
| Tech stack | `conductor/tech-stack.md` |

## Contribution Guidelines

**Welcome**:
- Deterministic agents
- Domain capability packages
- Gherkin invariants
- Validators and promotion logic
- Tests and documentation

**Rejected**:
- Anything introducing hidden control flow
- Background execution mechanisms
- Eventual consistency patterns
- Message queues or actor frameworks

**converge-core is PRIVATE**: The core engine source is not open for direct contribution. Open API issues for discussion.

## Git Commit Format

```
domain(subsystem): description

Example: chore(engine): Update converge-core with eligibility efficiency
```

## Key Technologies

- **Language**: Rust 1.85+ (Edition 2024)
- **Async**: Tokio
- **HTTP**: Axum + utoipa (OpenAPI)
- **Parallelism**: Rayon
- **Serialization**: Serde
- **Errors**: thiserror
- **Observability**: tracing
