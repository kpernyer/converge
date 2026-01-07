# Spec: Finalize Converge Core

## Overview
This track solidifies `converge-core` as the foundational library for the Converge workspace. Following the recent restructuring, the core must be verified to contain *only* the essential mechanisms for the convergence engine, context management, and abstraction traits (e.g., `LlmProvider`), while stripping away any remaining domain-specific or provider-specific logic that belongs in the new crates.

## Goals
1.  **Architecture Verification:** Ensure `converge-core` has no circular dependencies or leaked abstractions from `converge-provider`, `converge-domain`, or `converge-runtime`.
2.  **Trait Stability:** Finalize the `LlmProvider` and `Agent` traits to support the external implementations in `converge-provider` and `converge-domain`.
3.  **Mock Completeness:** Ensure `MockProvider` in `converge-core` is sufficient for testing the core engine without external dependencies.
4.  **Core Test Suite:** Establish a baseline test suite for `converge-core` that verifies the engine's correctness, determinism, and safety properties in isolation.

## Technical Requirements
-   Audit `converge-core/Cargo.toml` to ensure only essential dependencies remain.
-   Verify that `LlmProvider` trait in `converge-core` is public and extensible.
-   Ensure `MockProvider` supports all features required by the `LlmProvider` trait.
-   Confirm that `Engine`, `Context`, and `Fact` types are robust and documented.

## Constraints
-   `converge-core` MUST NOT depend on `converge-provider`, `converge-domain`, or `converge-runtime`.
-   All domain-specific logic (e.g., growth strategies) MUST stay in `converge-domain`.
-   All real LLM implementations (e.g., Anthropic) MUST stay in `converge-provider`.
