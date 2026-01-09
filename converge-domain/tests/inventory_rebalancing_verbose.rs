// Copyright 2024-2025 Aprio One AB, Sweden
// Author: Kenneth Pernyer, kenneth@aprio.one
// SPDX-License-Identifier: MIT
// See LICENSE file in the project root for full license information.

//! Verbose Inventory Rebalancing Integration Test
//!
//! This test demonstrates the complete Converge execution for the
//! Inventory Rebalancing use case with detailed output at each stage.

use converge_core::agents::SeedAgent;
use converge_core::{Budget, Context, ContextKey, Engine};
use converge_domain::inventory_rebalancing::{
    CapacityConstraintAgent, FinancialImpactAgent, ForecastAgent, InventoryAgent,
    RebalanceDecisionAgent, RequireBudgetCompliance, RequireCompleteForecasts, RequireSafetyStock,
    SalesVelocityAgent, TransferOptimizationAgent,
};

#[test]
fn verbose_inventory_rebalancing_execution() {
    println!("\n");
    println!("╔══════════════════════════════════════════════════════════════════════════════╗");
    println!("║       CONVERGE INVENTORY REBALANCING - VERBOSE EXECUTION TRACE               ║");
    println!("╚══════════════════════════════════════════════════════════════════════════════╝");

    let mut engine = Engine::with_budget(Budget {
        max_cycles: 100,
        max_facts: 1000,
    });

    println!("\n┌──────────────────────────────────────────────────────────────────────────────┐");
    println!("│ PHASE 1: ENGINE SETUP                                                        │");
    println!("└──────────────────────────────────────────────────────────────────────────────┘");
    println!("\n  Budget Configuration:");
    println!("    • max_cycles: 100");
    println!("    • max_facts: 1000");

    println!("\n┌──────────────────────────────────────────────────────────────────────────────┐");
    println!("│ PHASE 2: AGENT REGISTRATION                                                  │");
    println!("└──────────────────────────────────────────────────────────────────────────────┘");

    let seed1_id = engine.register(SeedAgent::new("regions", "North, South, East, West"));
    println!("\n  [{seed1_id}] SeedAgent 'regions'");

    let sales_id = engine.register(SalesVelocityAgent);
    println!("  [{sales_id}] SalesVelocityAgent → Signals (sales velocity)");

    let inv_id = engine.register(InventoryAgent);
    println!("  [{inv_id}] InventoryAgent → Signals (stock levels)");

    let forecast_id = engine.register(ForecastAgent);
    println!("  [{forecast_id}] ForecastAgent → Signals (demand forecasts)");

    let transfer_id = engine.register(TransferOptimizationAgent);
    println!("  [{transfer_id}] TransferOptimizationAgent → Strategies (transfer plans)");

    let capacity_id = engine.register(CapacityConstraintAgent);
    println!("  [{capacity_id}] CapacityConstraintAgent → Constraints (capacity limits)");

    let financial_id = engine.register(FinancialImpactAgent);
    println!("  [{financial_id}] FinancialImpactAgent → Strategies (cost analysis)");

    let decision_id = engine.register(RebalanceDecisionAgent);
    println!("  [{decision_id}] RebalanceDecisionAgent → Evaluations (ranked plans)");

    println!("\n  Total Agents: {}", engine.agent_count());

    println!("\n┌──────────────────────────────────────────────────────────────────────────────┐");
    println!("│ PHASE 3: INVARIANT REGISTRATION                                              │");
    println!("└──────────────────────────────────────────────────────────────────────────────┘");

    engine.register_invariant(RequireCompleteForecasts);
    println!("\n  ✓ RequireCompleteForecasts");

    engine.register_invariant(RequireSafetyStock);
    println!("  ✓ RequireSafetyStock");

    engine.register_invariant(RequireBudgetCompliance);
    println!("  ✓ RequireBudgetCompliance");

    let context = Context::new();
    println!("\n┌──────────────────────────────────────────────────────────────────────────────┐");
    println!("│ PHASE 4: INITIAL CONTEXT                                                     │");
    println!("└──────────────────────────────────────────────────────────────────────────────┘");
    println!("    • Version: {}", context.version());

    println!("\n┌──────────────────────────────────────────────────────────────────────────────┐");
    println!("│ PHASE 5: CONVERGENCE LOOP EXECUTION                                          │");
    println!("└──────────────────────────────────────────────────────────────────────────────┘");

    let result = engine.run(context).expect("should converge");

    println!("\n┌──────────────────────────────────────────────────────────────────────────────┐");
    println!("│ PHASE 6: CONVERGENCE RESULTS                                                 │");
    println!("└──────────────────────────────────────────────────────────────────────────────┘");

    println!("\n  Convergence Status:");
    println!("    • Converged: {}", result.converged);
    println!("    • Cycles: {}", result.cycles);

    println!("\n  📦 SEEDS:");
    for fact in result.context.get(ContextKey::Seeds) {
        println!("    [{:30}] {}", fact.id, fact.content);
    }

    println!("\n  📡 SIGNALS:");
    for fact in result.context.get(ContextKey::Signals) {
        println!("    [{:30}] {}", fact.id, fact.content);
    }

    println!("\n  🎯 STRATEGIES:");
    for fact in result.context.get(ContextKey::Strategies) {
        println!("    [{}] {}", fact.id, fact.content);
    }

    println!("\n  📊 EVALUATIONS:");
    for fact in result.context.get(ContextKey::Evaluations) {
        println!("    [{}] {}", fact.id, fact.content);
    }

    println!("\n╔══════════════════════════════════════════════════════════════════════════════╗");
    println!("║                              EXECUTION SUMMARY                                ║");
    println!("╠══════════════════════════════════════════════════════════════════════════════╣");
    println!("║  Agents Registered:    8                                                     ║");
    println!("║  Invariants Enforced:  3                                                     ║");
    println!(
        "║  Cycles Executed:      {}                                                     ║",
        result.cycles
    );
    println!("║  Convergence:          ✓ ACHIEVED                                            ║");
    println!("╚══════════════════════════════════════════════════════════════════════════════╝");
    println!();

    assert!(result.converged);
}
