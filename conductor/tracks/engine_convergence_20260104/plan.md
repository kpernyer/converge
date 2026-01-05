# Plan: Engine Execution Loop & Convergence Detection

## Phase 1: Readiness (Observability & Safety)
- [~] Task: Integrate `tracing` spans into `Engine::run` to record cycle starts, agent eligibility, and merge events.
- [ ] Task: Refactor `Engine::run` to treat initial facts in the provided `Context` as "dirty" for Cycle 1.
- [ ] Task: Refactor `count_facts` to use an iterator over `ContextKey` variants instead of a hardcoded list.
- [ ] Task: Conductor - User Manual Verification 'Readiness' (Protocol in workflow.md)

## Phase 2: Execution (Parallel Compute)
- [ ] Task: Parallelize `Engine::execute_agents` using `rayon` (preferred for CPU-bound agent logic) or scoped threads.
- [ ] Task: Implement validation and promotion for `AgentEffect` containing `ProposedFact`s in the merge phase.
- [ ] Task: Ensure that `ProposedFact` failures are handled without crashing the engine (Transparent Determinism).
- [ ] Task: Conductor - User Manual Verification 'Execution' (Protocol in workflow.md)

## Phase 3: Convergence (System Integrity)
- [ ] Task: Implement conflict detection in `Context::add_fact` (e.g., same ID but different content).
- [ ] Task: Implement the "Diagnostic Fact" pattern: when an invariant is violated, emit a special fact to the `Context` for auditability before returning the error.
- [ ] Task: Refine `Engine::find_eligible` to be more efficient with large numbers of dirty keys.
- [ ] Task: Conductor - User Manual Verification 'Convergence' (Protocol in workflow.md)

## Phase 4: Finalizing (Verification & Proof)
- [ ] Task: Create an integration test in `tests/convergence.rs` that uses a complex dependency graph and verifies deterministic output across multiple runs.
- [ ] Task: Stress test the engine with a high volume of agents and facts to verify sub-linear latency growth (System Scalability North Star).
- [ ] Task: Conductor - User Manual Verification 'Finalizing' (Protocol in workflow.md)
