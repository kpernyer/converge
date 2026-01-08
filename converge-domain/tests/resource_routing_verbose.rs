// Copyright 2024-2025 Aprio One AB, Sweden
// Author: Kenneth Pernyer, kenneth@aprio.one
// SPDX-License-Identifier: MIT
// See LICENSE file in the project root for full license information.

//! Verbose Resource Routing Integration Test
//!
//! This test demonstrates the complete Converge execution for the
//! Resource Routing use case with detailed output at each stage.

use converge_core::agents::SeedAgent;
use converge_core::{Budget, Context, ContextKey, Engine};
use converge_domain::resource_routing::{
    ConstraintValidationAgent, FeasibilityAgent, RequireAllTasksAssigned, RequireCapacityRespected,
    RequireValidDefinitions, ResourceRetrievalAgent, SolverAgent, TaskRetrievalAgent,
};

#[test]
fn verbose_resource_routing_execution() {
    println!("\n");
    println!("╔══════════════════════════════════════════════════════════════════════════════╗");
    println!("║         CONVERGE RESOURCE ROUTING - VERBOSE EXECUTION TRACE                  ║");
    println!("╚══════════════════════════════════════════════════════════════════════════════╝");

    // =========================================================================
    // PHASE 1: ENGINE SETUP
    // =========================================================================
    println!("\n┌──────────────────────────────────────────────────────────────────────────────┐");
    println!("│ PHASE 1: ENGINE SETUP                                                        │");
    println!("└──────────────────────────────────────────────────────────────────────────────┘");

    let mut engine = Engine::with_budget(Budget {
        max_cycles: 100,
        max_facts: 1000,
    });

    println!("\n  Budget Configuration:");
    println!("    • max_cycles: 100");
    println!("    • max_facts: 1000");

    // =========================================================================
    // PHASE 2: AGENT REGISTRATION
    // =========================================================================
    println!("\n┌──────────────────────────────────────────────────────────────────────────────┐");
    println!("│ PHASE 2: AGENT REGISTRATION                                                  │");
    println!("└──────────────────────────────────────────────────────────────────────────────┘");

    println!("\n  Registering Seed Agents (provide initial context):");

    let seed1_id = engine.register(SeedAgent::new(
        "tasks",
        "Delivery A, Delivery B, Delivery C",
    ));
    println!("    [{}] SeedAgent 'tasks'", seed1_id);
    println!("         → Dependencies: [] (runs first cycle)");
    println!("         → Emits: Seeds");

    let seed2_id = engine.register(SeedAgent::new("resources", "Vehicle 1, Vehicle 2"));
    println!("    [{}] SeedAgent 'resources'", seed2_id);
    println!("         → Dependencies: [] (runs first cycle)");
    println!("         → Emits: Seeds");

    println!("\n  Registering Resource Routing Pipeline:");

    let task_id = engine.register(TaskRetrievalAgent);
    println!("    [{}] TaskRetrievalAgent", task_id);
    println!("         → Dependencies: [Seeds]");
    println!("         → Emits: Signals (task definitions)");

    let resource_id = engine.register(ResourceRetrievalAgent);
    println!("    [{}] ResourceRetrievalAgent", resource_id);
    println!("         → Dependencies: [Seeds]");
    println!("         → Emits: Signals (resource definitions)");

    let constraint_id = engine.register(ConstraintValidationAgent);
    println!("    [{}] ConstraintValidationAgent", constraint_id);
    println!("         → Dependencies: [Signals]");
    println!("         → Emits: Constraints (capacity, time windows)");

    let solver_id = engine.register(SolverAgent);
    println!("    [{}] SolverAgent", solver_id);
    println!("         → Dependencies: [Signals, Constraints]");
    println!("         → Emits: Strategies (candidate assignments)");

    let feasibility_id = engine.register(FeasibilityAgent);
    println!("    [{}] FeasibilityAgent", feasibility_id);
    println!("         → Dependencies: [Strategies]");
    println!("         → Emits: Evaluations (valid assignments ranked)");

    println!("\n  Total Agents: {}", engine.agent_count());

    // =========================================================================
    // PHASE 3: INVARIANT REGISTRATION
    // =========================================================================
    println!("\n┌──────────────────────────────────────────────────────────────────────────────┐");
    println!("│ PHASE 3: INVARIANT REGISTRATION (Gherkin → Runtime Law)                      │");
    println!("└──────────────────────────────────────────────────────────────────────────────┘");

    println!("\n  Structural Invariants (checked after every merge):");
    let valid_id = engine.register_invariant(RequireValidDefinitions);
    println!("    [{}] RequireValidDefinitions", valid_id);
    println!("         → Tasks and resources must be valid");
    println!("         → Violation = immediate failure");

    println!("\n  Semantic Invariants (checked at end of each cycle):");
    let capacity_id = engine.register_invariant(RequireCapacityRespected);
    println!("    [{}] RequireCapacityRespected", capacity_id);
    println!("         → Capacity constraints must be respected");
    println!("         → Violation = blocks convergence");

    println!("\n  Acceptance Invariants (checked before declaring convergence):");
    let assigned_id = engine.register_invariant(RequireAllTasksAssigned);
    println!("    [{}] RequireAllTasksAssigned", assigned_id);
    println!("         → All tasks must be assigned");
    println!("         → Violation = rejects result");

    // =========================================================================
    // PHASE 4: INITIAL CONTEXT
    // =========================================================================
    println!("\n┌──────────────────────────────────────────────────────────────────────────────┐");
    println!("│ PHASE 4: INITIAL CONTEXT                                                     │");
    println!("└──────────────────────────────────────────────────────────────────────────────┘");

    let context = Context::new();
    println!("\n  Context₀ (empty):");
    println!("    • Seeds: []");
    println!("    • Signals: []");
    println!("    • Constraints: []");
    println!("    • Strategies: []");
    println!("    • Evaluations: []");
    println!("    • Version: {}", context.version());

    // =========================================================================
    // PHASE 5: EXECUTION
    // =========================================================================
    println!("\n┌──────────────────────────────────────────────────────────────────────────────┐");
    println!("│ PHASE 5: CONVERGENCE LOOP EXECUTION                                          │");
    println!("└──────────────────────────────────────────────────────────────────────────────┘");

    println!("\n  Starting engine.run()...");
    println!("  ─────────────────────────────────────────────────────────────────────────────");

    let result = engine.run(context).expect("should converge");

    println!("\n  ─────────────────────────────────────────────────────────────────────────────");
    println!("  Execution complete.");

    // =========================================================================
    // PHASE 6: RESULTS
    // =========================================================================
    println!("\n┌──────────────────────────────────────────────────────────────────────────────┐");
    println!("│ PHASE 6: CONVERGENCE RESULTS                                                 │");
    println!("└──────────────────────────────────────────────────────────────────────────────┘");

    println!("\n  Convergence Status:");
    println!("    • Converged: {}", result.converged);
    println!("    • Cycles: {}", result.cycles);
    println!("    • Final Version: {}", result.context.version());

    println!("\n  ═══════════════════════════════════════════════════════════════════════════");
    println!("  CONTEXT EVOLUTION SUMMARY");
    println!("  ═══════════════════════════════════════════════════════════════════════════");

    // Seeds
    println!("\n  📦 SEEDS (Tasks & Resources):");
    println!("  ───────────────────────────────────────────────────────────────────────────");
    for fact in result.context.get(ContextKey::Seeds) {
        println!("    [{:30}] {}", fact.id, fact.content);
    }

    // Signals
    println!("\n  📡 SIGNALS (Task & Resource Definitions):");
    println!("  ───────────────────────────────────────────────────────────────────────────");
    for fact in result.context.get(ContextKey::Signals) {
        println!("    [{:30}] {}", fact.id, fact.content);
    }

    // Constraints
    println!("\n  🔒 CONSTRAINTS (Capacity & Time Windows):");
    println!("  ───────────────────────────────────────────────────────────────────────────");
    for fact in result.context.get(ContextKey::Constraints) {
        println!("    [{:30}] {}", fact.id, fact.content);
    }

    // Strategies
    println!("\n  🎯 STRATEGIES (Candidate Assignments):");
    println!("  ───────────────────────────────────────────────────────────────────────────");
    for fact in result.context.get(ContextKey::Strategies) {
        println!("    [{}]", fact.id);
        println!("       {}", fact.content);
        println!();
    }

    // Evaluations
    println!("  📊 EVALUATIONS (Valid Assignments Ranked):");
    println!("  ───────────────────────────────────────────────────────────────────────────");
    for fact in result.context.get(ContextKey::Evaluations) {
        println!("    [{}]", fact.id);
        println!("       {}", fact.content);
        println!();
    }

    // =========================================================================
    // PHASE 7: INVARIANT VERIFICATION
    // =========================================================================
    println!("┌──────────────────────────────────────────────────────────────────────────────┐");
    println!("│ PHASE 7: INVARIANT VERIFICATION (All Passed)                                 │");
    println!("└──────────────────────────────────────────────────────────────────────────────┘");

    let evaluations = result.context.get(ContextKey::Evaluations);

    println!("\n  ✓ RequireValidDefinitions: Tasks and resources validated");
    println!("  ✓ RequireCapacityRespected: Capacity constraints respected");
    println!(
        "  ✓ RequireAllTasksAssigned: All tasks assigned in {} evaluations",
        evaluations.len()
    );

    // =========================================================================
    // SUMMARY
    // =========================================================================
    println!("\n╔══════════════════════════════════════════════════════════════════════════════╗");
    println!("║                              EXECUTION SUMMARY                                ║");
    println!("╠══════════════════════════════════════════════════════════════════════════════╣");
    println!("║  Agents Registered:    7                                                     ║");
    println!("║  Invariants Enforced:  3                                                     ║");
    println!(
        "║  Cycles Executed:      {}                                                     ║",
        result.cycles
    );
    println!(
        "║  Facts Generated:      {}                                                   ║",
        result.context.version()
    );
    println!("║  Convergence:          ✓ ACHIEVED                                            ║");
    println!("╚══════════════════════════════════════════════════════════════════════════════╝");
    println!();

    // Assertions
    assert!(result.converged);
    assert!(!evaluations.is_empty());
}
