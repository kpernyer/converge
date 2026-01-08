# Plan: Finalize Converge Core

## Phase 1: Readiness (Dependency & Boundary Audit) [checkpoint: e78ed40]
- [x] Task: Audit `converge-core/Cargo.toml` and remove any unused or inappropriate dependencies (e.g., specific HTTP clients if not needed for traits). ddc056e
- [x] Task: Verify that `converge-core` does not import from or reference the other workspace crates. ddc056e
- [x] Task: Conductor - User Manual Verification 'Readiness' (Protocol in workflow.md)

## Phase 2: Execution (Trait & Mock Hardening) [checkpoint: 44ae027]
- [x] Task: Review `LlmProvider` and `Agent` traits for completeness and public visibility. 35cd6fd
- [x] Task: Enhance `MockProvider` to support streaming or other advanced features if defined in the trait, ensuring fully isolated testing. 35cd6fd
- [x] Task: Ensure all `Context` and `Fact` types have necessary derives (Debug, Clone, Serialize, Deserialize) for cross-crate usage. 0ec86f1
- [x] Task: Conductor - User Manual Verification 'Execution' (Protocol in workflow.md)

## Phase 3: Convergence (Core Test Suite) [checkpoint: 23454f5]
- [x] Task: Create a comprehensive integration test in `converge-core/tests/core_mechanics.rs` that simulates a generic agent loop using only core types and mocks. 7be1532
- [x] Task: Verify that existing property tests in `converge-core` still pass and cover the core invariants. 7be1532
- [x] Task: Conductor - User Manual Verification 'Convergence' (Protocol in workflow.md)

## Phase 4: Finalizing (Documentation & Release Prep) [checkpoint: 6aad766]
- [x] Task: Update `converge-core/README.md` (or lib.rs docs) to clearly define its scope and how to use it with the other crates. 44c3351
- [x] Task: Run `cargo doc -p converge-core` to ensure public API documentation is generated and correct. 44c3351
- [x] Task: Conductor - User Manual Verification 'Finalizing' (Protocol in workflow.md)
