// Copyright 2024-2025 Aprio One AB, Sweden
// Author: Kenneth Pernyer, kenneth@aprio.one
// SPDX-License-Identifier: MIT
// See LICENSE file in the project root for full license information.

//! Verbose Growth Strategy Integration Test
//!
//! This test demonstrates the complete Converge execution for the
//! Growth Strategy use case with detailed output at each stage.

use converge_core::agents::SeedAgent;
use converge_core::{Budget, Context, ContextKey, Engine};
use converge_domain::growth_strategy::{
    BrandSafetyInvariant, CompetitorAgent, EvaluationAgent, MarketSignalAgent,
    RequireEvaluationRationale, RequireMultipleStrategies, RequireStrategyEvaluations,
    StrategyAgent,
};

#[test]
fn verbose_growth_strategy_execution() {
    println!("\n");
    println!("╔══════════════════════════════════════════════════════════════════════════════╗");
    println!("║           CONVERGE GROWTH STRATEGY - VERBOSE EXECUTION TRACE                 ║");
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

    let seed1_id = engine.register(SeedAgent::new("market:nordic-b2b", "Nordic B2B market"));
    println!("    [{}] SeedAgent 'market:nordic-b2b'", seed1_id);
    println!("         → Dependencies: [] (runs first cycle)");
    println!("         → Emits: Seeds");

    let seed2_id = engine.register(SeedAgent::new("product:product-x", "Product X - SaaS platform"));
    println!("    [{}] SeedAgent 'product:product-x'", seed2_id);
    println!("         → Dependencies: [] (runs first cycle)");
    println!("         → Emits: Seeds");

    println!("\n  Registering Growth Strategy Pipeline:");

    let market_id = engine.register(MarketSignalAgent);
    println!("    [{}] MarketSignalAgent", market_id);
    println!("         → Dependencies: [Seeds]");
    println!("         → Emits: Signals (market observations)");

    let competitor_id = engine.register(CompetitorAgent);
    println!("    [{}] CompetitorAgent", competitor_id);
    println!("         → Dependencies: [Signals]");
    println!("         → Emits: Competitors (competitor profiles)");

    let strategy_id = engine.register(StrategyAgent);
    println!("    [{}] StrategyAgent", strategy_id);
    println!("         → Dependencies: [Competitors, Signals]");
    println!("         → Emits: Strategies (growth strategies)");

    let eval_id = engine.register(EvaluationAgent);
    println!("    [{}] EvaluationAgent", eval_id);
    println!("         → Dependencies: [Strategies]");
    println!("         → Emits: Evaluations (ranked scores)");

    println!("\n  Total Agents: {}", engine.agent_count());

    // =========================================================================
    // PHASE 3: INVARIANT REGISTRATION
    // =========================================================================
    println!("\n┌──────────────────────────────────────────────────────────────────────────────┐");
    println!("│ PHASE 3: INVARIANT REGISTRATION (Gherkin → Runtime Law)                      │");
    println!("└──────────────────────────────────────────────────────────────────────────────┘");

    println!("\n  Structural Invariants (checked after every merge):");
    let brand_id = engine.register_invariant(BrandSafetyInvariant::default());
    println!("    [{}] BrandSafetyInvariant", brand_id);
    println!("         → Forbids: spam, misleading, aggressive, deceptive, unethical");
    println!("         → Violation = immediate failure");

    println!("\n  Semantic Invariants (checked at end of each cycle):");
    let rationale_id = engine.register_invariant(RequireEvaluationRationale);
    println!("    [{}] RequireEvaluationRationale", rationale_id);
    println!("         → All evaluations must contain 'Rationale:'");
    println!("         → Violation = blocks convergence");

    println!("\n  Acceptance Invariants (checked before declaring convergence):");
    let multi_id = engine.register_invariant(RequireMultipleStrategies);
    println!("    [{}] RequireMultipleStrategies", multi_id);
    println!("         → At least 2 distinct strategies required");
    println!("         → Violation = rejects result");

    let eval_inv_id = engine.register_invariant(RequireStrategyEvaluations);
    println!("    [{}] RequireStrategyEvaluations", eval_inv_id);
    println!("         → Every strategy must have an evaluation");
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
    println!("    • Competitors: []");
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
    println!("\n  📦 SEEDS (Root Intent):");
    println!("  ───────────────────────────────────────────────────────────────────────────");
    for fact in result.context.get(ContextKey::Seeds) {
        println!("    [{:30}] {}", fact.id, fact.content);
    }

    // Signals
    println!("\n  📡 SIGNALS (Market Observations):");
    println!("  ───────────────────────────────────────────────────────────────────────────");
    for fact in result.context.get(ContextKey::Signals) {
        println!("    [{:30}] {}", fact.id, fact.content);
    }

    // Competitors
    println!("\n  🏢 COMPETITORS (Competitive Analysis):");
    println!("  ───────────────────────────────────────────────────────────────────────────");
    for fact in result.context.get(ContextKey::Competitors) {
        println!("    [{:30}] {}", fact.id, fact.content);
    }

    // Strategies
    println!("\n  🎯 STRATEGIES (Growth Proposals):");
    println!("  ───────────────────────────────────────────────────────────────────────────");
    for fact in result.context.get(ContextKey::Strategies) {
        println!("    [{}]", fact.id);
        println!("       {}", fact.content);
        println!();
    }

    // Evaluations
    println!("  📊 EVALUATIONS (Ranked Output):");
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

    let strategies = result.context.get(ContextKey::Strategies);
    let evaluations = result.context.get(ContextKey::Evaluations);

    println!("\n  ✓ BrandSafetyInvariant: No forbidden terms in {} strategies", strategies.len());
    println!("  ✓ RequireEvaluationRationale: All {} evaluations have rationale", evaluations.len());
    println!("  ✓ RequireMultipleStrategies: {} strategies >= 2 required", strategies.len());
    println!("  ✓ RequireStrategyEvaluations: All {} strategies have evaluations", strategies.len());

    // =========================================================================
    // SUMMARY
    // =========================================================================
    println!("\n╔══════════════════════════════════════════════════════════════════════════════╗");
    println!("║                              EXECUTION SUMMARY                                ║");
    println!("╠══════════════════════════════════════════════════════════════════════════════╣");
    println!("║  Agents Registered:    6                                                     ║");
    println!("║  Invariants Enforced:  4                                                     ║");
    println!("║  Cycles Executed:      {}                                                     ║", result.cycles);
    println!("║  Facts Generated:      {}                                                   ║", result.context.version());
    println!("║  Convergence:          ✓ ACHIEVED                                            ║");
    println!("╚══════════════════════════════════════════════════════════════════════════════╝");
    println!();

    // Assertions
    assert!(result.converged);
    assert!(strategies.len() >= 2);
    assert_eq!(strategies.len(), evaluations.len());
}
