# Spec: Engine Execution Loop & Convergence Detection

## Overview
This track completes the core "Heartbeat" of Converge: the convergence engine. It transitions the engine from a sequential skeleton to a high-performance, parallel, and provably correct runtime.

## Goals
1. **System Excellence:** Parallelize agent execution while maintaining serial, deterministic commit.
2. **Safety by Construction:** Enforce strict validation of `ProposedFact` during the merge phase.
3. **Transparent Determinism:** Enhance observability with `tracing` and causal provenance.
4. **Provable Convergence:** Refine dirty-key tracking and invariant-gated halting.

## Technical Requirements
- Use `rayon` or `tokio` for parallel agent execution.
- Implement explicit `ProposedFact` -> `Fact` promotion logic in `Engine::merge_effects`.
- Ensure initial facts in `Context` trigger relevant agents in the first cycle.
- Add `tracing` spans to the engine loop.
- Implement "Diagnostic Facts" for invariant violations.

## Constraints
- No changes to the `Agent` trait's core contract (read-only access).
- Deterministic merge order (by `AgentId`) must be preserved.
- Monotonicity of `Context` must not be violated.
