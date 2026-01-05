# Plan: Explicit Authority & Lifecycle Semantics

## Phase 1: Readiness (Types & State Machine)
- [ ] Task: Define `HaltReason` enum (Converged, AwaitingAuthority, InvariantViolation, BudgetExhausted) in `engine.rs`.
- [ ] Task: Define `AuthorityBarrier` struct to represent *what* the engine is waiting for (e.g., "Approval for Fact X").
- [ ] Task: Refactor `ConvergeResult` to include `HaltReason` instead of just a boolean.
- [ ] Task: Conductor - User Manual Verification 'Readiness' (Protocol in workflow.md)

## Phase 2: Execution (Halt Logic & Barriers)
- [ ] Task: Implement `Engine::run_until_halt` that checks for `AuthorityBarrier`s in the context or agent signals.
- [ ] Task: Create a `GatekeeperAgent` pattern that emits `AuthorityBarrier` effects when proposals meet certain criteria.
- [ ] Task: Implement the logic to halt the loop when an `AuthorityBarrier` is detected and return the correct state.
- [ ] Task: Conductor - User Manual Verification 'Execution' (Protocol in workflow.md)

## Phase 3: Persistence (Snapshotting & Resume)
- [ ] Task: Add `serde` derivation to `Context`, `Fact`, `ProposedFact`, and `ContextKey` for full serialization.
- [ ] Task: Implement `Context::snapshot()` and `Context::from_snapshot()`.
- [ ] Task: Create an `Engine::resume()` method that takes a `Context` and "warms up" the dirty keys (or re-scans) to ensure correct next-cycle execution.
- [ ] Task: Conductor - User Manual Verification 'Persistence' (Protocol in workflow.md)

## Phase 4: Finalizing (Verification & Integration)
- [ ] Task: Write an integration test: Run -> Pause for Authority -> Save -> Load -> Resume -> Converge.
- [ ] Task: Verify that invariants hold correctly across the save/load boundary.
- [ ] Task: Conductor - User Manual Verification 'Finalizing' (Protocol in workflow.md)
