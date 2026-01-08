// Copyright 2024-2025 Aprio One AB, Sweden
// Author: Kenneth Pernyer, kenneth@aprio.one
// SPDX-License-Identifier: MIT
// See LICENSE file in the project root for full license information.

//! Verbose Strategic Sourcing Integration Test
//!
//! This test demonstrates the complete Converge execution for the
//! Strategic Sourcing use case with detailed output at each stage.

use converge_core::agents::SeedAgent;
use converge_core::{Budget, Context, ContextKey, Engine};
use converge_domain::strategic_sourcing::{
    ComplianceAgent, ESGScoringAgent, PriceBenchmarkAgent,
    RequireCompleteAssessments as RequireSourcingAssessments, RequireCompliantVendor,
    RequireShortlistCompliance, RiskModelAgent, SourcingStrategyAgent, SupplierDiscoveryAgent,
    VendorRankingAgent,
};

#[test]
fn verbose_strategic_sourcing_execution() {
    println!("\n");
    println!("╔══════════════════════════════════════════════════════════════════════════════╗");
    println!("║         CONVERGE STRATEGIC SOURCING - VERBOSE EXECUTION TRACE                 ║");
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

    let seed1_id = engine.register(SeedAgent::new("suppliers", "VendorA, VendorB, VendorC"));
    println!("\n  [{}] SeedAgent 'suppliers'", seed1_id);

    let discovery_id = engine.register(SupplierDiscoveryAgent);
    println!(
        "  [{}] SupplierDiscoveryAgent → Signals (supplier profiles)",
        discovery_id
    );

    let compliance_id = engine.register(ComplianceAgent);
    println!(
        "  [{}] ComplianceAgent → Signals (compliance status)",
        compliance_id
    );

    let esg_id = engine.register(ESGScoringAgent);
    println!("  [{}] ESGScoringAgent → Signals (ESG scores)", esg_id);

    let price_id = engine.register(PriceBenchmarkAgent);
    println!(
        "  [{}] PriceBenchmarkAgent → Signals (price comparisons)",
        price_id
    );

    let risk_id = engine.register(RiskModelAgent);
    println!(
        "  [{}] RiskModelAgent → Signals (risk assessments)",
        risk_id
    );

    let strategy_id = engine.register(SourcingStrategyAgent);
    println!(
        "  [{}] SourcingStrategyAgent → Strategies (shortlist)",
        strategy_id
    );

    let ranking_id = engine.register(VendorRankingAgent);
    println!(
        "  [{}] VendorRankingAgent → Evaluations (ranked vendors)",
        ranking_id
    );

    println!("\n  Total Agents: {}", engine.agent_count());

    println!("\n┌──────────────────────────────────────────────────────────────────────────────┐");
    println!("│ PHASE 3: INVARIANT REGISTRATION                                              │");
    println!("└──────────────────────────────────────────────────────────────────────────────┘");

    engine.register_invariant(RequireSourcingAssessments);
    println!("\n  ✓ RequireSourcingAssessments");

    engine.register_invariant(RequireCompliantVendor);
    println!("  ✓ RequireCompliantVendor");

    engine.register_invariant(RequireShortlistCompliance);
    println!("  ✓ RequireShortlistCompliance");

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
