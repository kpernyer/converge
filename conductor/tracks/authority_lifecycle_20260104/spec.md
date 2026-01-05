# Spec: Explicit Authority & Lifecycle Semantics

## Overview
This track solidifies the Converge engine's lifecycle by formalizing halt states, explicit authority barriers, and snapshot persistence. It ensures that "Human-in-the-Loop" (HITL) is not just a concept but a typed, recoverable state where the engine safely pauses for authoritative input.

## Goals
1.  **Formalize Halt States:** Define and implement explicit states for `Converged`, `InvariantViolation`, `AwaitingAuthority`, and `BudgetExhausted`.
2.  **Explicit Authority:** Implement the mechanism for "Authority Barriers" where the engine halts execution until a specific `ProposedFact` is resolved or a `Fact` is injected by an authority (Human or System).
3.  **Persistence & Resume:** Enable the engine to serialize its state (Context + Dirty Keys) to a snapshot and resume execution deterministically.
4.  **Lifecycle Integrity:** Ensure that pausing and resuming does not violate convergence guarantees or monotonicity.

## Technical Requirements
-   **HaltReason Enum:** Create a comprehensive enum for why the engine stopped.
-   **Authority Trait:** Define how external authorities interact with the engine.
-   **Snapshotting:** Implement `serde` support for `Context` to allow save/load operations.
-   **Resume Logic:** Ensure `Engine::resume(snapshot)` reconstructs the exact state required to continue the next cycle.

## Constraints
-   Must not introduce "event loops" or background polling.
-   Snapshots must be mathematically complete (no hidden state).
-   Authority actions must be recorded as Facts with provenance.

## Success Criteria
-   The engine can run until it hits an "Authority Barrier" and halt with `HaltReason::AwaitingAuthority`.
-   The context can be saved to disk.
-   The engine can be restarted from disk and complete the convergence loop to a fixed point.
