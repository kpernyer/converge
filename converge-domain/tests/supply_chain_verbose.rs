// Copyright 2024-2025 Aprio One AB, Sweden
// Author: Kenneth Pernyer, kenneth@aprio.one
// SPDX-License-Identifier: MIT
// See LICENSE file in the project root for full license information.

//! Verbose Supply Chain Re-planning Integration Test
//!
//! This test demonstrates the complete Converge execution for the
//! Supply Chain Re-planning use case with detailed output at each stage.

use converge_core::agents::SeedAgent;
use converge_core::{Budget, Context, ContextKey, Engine};
use converge_domain::supply_chain::{
    ConsolidationAgent, CostEstimationAgent, DemandSnapshotAgent, InventoryStateAgent,
    RequireCompleteAssessments, RequireFeasiblePlan, RequireSLACompliance, RiskAssessmentAgent,
    RouteGenerationAgent, SLAValidationAgent, SupplierStatusAgent,
};

#[test]
fn verbose_supply_chain_execution() {
    println!("\n");
    println!("╔══════════════════════════════════════════════════════════════════════════════╗");
    println!("║       CONVERGE SUPPLY CHAIN RE-PLANNING - VERBOSE EXECUTION TRACE           ║");
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

    let seed1_id = engine.register(SeedAgent::new("orders", "Order A, Order B, Order C"));
    println!("    [{seed1_id}] SeedAgent 'orders'");
    println!("         → Dependencies: [] (runs first cycle)");
    println!("         → Emits: Seeds");

    let seed2_id = engine.register(SeedAgent::new(
        "supplier:delay",
        "Supplier X delayed 3 days",
    ));
    println!("    [{seed2_id}] SeedAgent 'supplier:delay'");
    println!("         → Dependencies: [] (runs first cycle)");
    println!("         → Emits: Seeds");

    println!("\n  Registering Data Collection Agents (Parallel Track 1):");

    let demand_id = engine.register(DemandSnapshotAgent);
    println!("    [{demand_id}] DemandSnapshotAgent");
    println!("         → Dependencies: [Seeds]");
    println!("         → Emits: Signals (order requirements)");

    let inventory_id = engine.register(InventoryStateAgent);
    println!("    [{inventory_id}] InventoryStateAgent");
    println!("         → Dependencies: [Seeds]");
    println!("         → Emits: Signals (stock levels)");

    let supplier_id = engine.register(SupplierStatusAgent);
    println!("    [{supplier_id}] SupplierStatusAgent");
    println!("         → Dependencies: [Seeds]");
    println!("         → Emits: Signals (supplier availability)");

    println!("\n  Registering Optimization Agents (Parallel Track 2):");

    let route_id = engine.register(RouteGenerationAgent);
    println!("    [{route_id}] RouteGenerationAgent");
    println!("         → Dependencies: [Signals]");
    println!("         → Emits: Strategies (alternative routes)");

    let cost_id = engine.register(CostEstimationAgent);
    println!("    [{cost_id}] CostEstimationAgent");
    println!("         → Dependencies: [Signals]");
    println!("         → Emits: Strategies (cost analysis)");

    let risk_id = engine.register(RiskAssessmentAgent);
    println!("    [{risk_id}] RiskAssessmentAgent");
    println!("         → Dependencies: [Signals]");
    println!("         → Emits: Strategies (risk scores)");

    let sla_id = engine.register(SLAValidationAgent);
    println!("    [{sla_id}] SLAValidationAgent");
    println!("         → Dependencies: [Signals]");
    println!("         → Emits: Constraints (SLA requirements)");

    println!("\n  Registering Consolidation Agent:");

    let consolidation_id = engine.register(ConsolidationAgent);
    println!("    [{consolidation_id}] ConsolidationAgent");
    println!("         → Dependencies: [Strategies, Constraints]");
    println!("         → Emits: Evaluations (feasible plans ranked)");

    println!("\n  Total Agents: {}", engine.agent_count());

    // =========================================================================
    // PHASE 3: INVARIANT REGISTRATION
    // =========================================================================
    println!("\n┌──────────────────────────────────────────────────────────────────────────────┐");
    println!("│ PHASE 3: INVARIANT REGISTRATION (Gherkin → Runtime Law)                      │");
    println!("└──────────────────────────────────────────────────────────────────────────────┘");

    println!("\n  Structural Invariants (checked after every merge):");
    let complete_id = engine.register_invariant(RequireCompleteAssessments);
    println!("    [{complete_id}] RequireCompleteAssessments");
    println!("         → All required assessments must be complete");
    println!("         → Violation = immediate failure");

    println!("\n  Semantic Invariants (checked at end of each cycle):");
    let sla_inv_id = engine.register_invariant(RequireSLACompliance);
    println!("    [{sla_inv_id}] RequireSLACompliance");
    println!("         → Plans must satisfy SLA requirements");
    println!("         → Violation = blocks convergence");

    println!("\n  Acceptance Invariants (checked before declaring convergence):");
    let feasible_id = engine.register_invariant(RequireFeasiblePlan);
    println!("    [{feasible_id}] RequireFeasiblePlan");
    println!("         → At least one feasible plan must exist");
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
    println!("    • Strategies: []");
    println!("    • Constraints: []");
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
    println!("\n  📦 SEEDS (Orders & Disruption):");
    println!("  ───────────────────────────────────────────────────────────────────────────");
    for fact in result.context.get(ContextKey::Seeds) {
        println!("    [{:30}] {}", fact.id, fact.content);
    }

    // Signals
    println!("\n  📡 SIGNALS (Data Collection):");
    println!("  ───────────────────────────────────────────────────────────────────────────");
    for fact in result.context.get(ContextKey::Signals) {
        println!("    [{:30}] {}", fact.id, fact.content);
    }

    // Constraints
    println!("\n  🔒 CONSTRAINTS (SLA Requirements):");
    println!("  ───────────────────────────────────────────────────────────────────────────");
    for fact in result.context.get(ContextKey::Constraints) {
        println!("    [{:30}] {}", fact.id, fact.content);
    }

    // Strategies
    println!("\n  🎯 STRATEGIES (Optimization Plans):");
    println!("  ───────────────────────────────────────────────────────────────────────────");
    for fact in result.context.get(ContextKey::Strategies) {
        println!("    [{}]", fact.id);
        println!("       {}", fact.content);
        println!();
    }

    // Evaluations
    println!("  📊 EVALUATIONS (Feasible Plans Ranked):");
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

    println!("\n  ✓ RequireCompleteAssessments: All assessments complete");
    println!("  ✓ RequireSLACompliance: SLA requirements satisfied");
    println!(
        "  ✓ RequireFeasiblePlan: {} feasible plans found",
        evaluations.len()
    );

    // =========================================================================
    // SUMMARY
    // =========================================================================
    println!("\n╔══════════════════════════════════════════════════════════════════════════════╗");
    println!("║                              EXECUTION SUMMARY                                ║");
    println!("╠══════════════════════════════════════════════════════════════════════════════╣");
    println!("║  Agents Registered:    10                                                    ║");
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
