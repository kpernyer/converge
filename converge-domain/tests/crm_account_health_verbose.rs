// Copyright 2024-2025 Aprio One AB, Sweden
// Author: Kenneth Pernyer, kenneth@aprio.one
// SPDX-License-Identifier: MIT
// See LICENSE file in the project root for full license information.

//! Verbose CRM Account Health Integration Test
//!
//! This test demonstrates the complete Converge execution for the
//! CRM Account Health use case with detailed output at each stage.

use converge_core::agents::SeedAgent;
use converge_core::{Budget, Context, ContextKey, Engine};
use converge_domain::crm_account_health::{
    ActionPrioritizationAgent, ChurnRiskAgent, RequireChurnActionPlan, RequireCompleteAnalysis,
    RevenueTrendAgent, SupportTicketAgent, UpsellOpportunityAgent, UsageSignalAgent,
};

#[test]
fn verbose_crm_account_health_execution() {
    println!("\n");
    println!("╔══════════════════════════════════════════════════════════════════════════════╗");
    println!("║         CONVERGE CRM ACCOUNT HEALTH - VERBOSE EXECUTION TRACE                  ║");
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

    let seed1_id = engine.register(SeedAgent::new("account", "Account123"));
    println!("\n  [{}] SeedAgent 'account'", seed1_id);

    let usage_id = engine.register(UsageSignalAgent);
    println!(
        "  [{}] UsageSignalAgent → Signals (usage metrics)",
        usage_id
    );

    let support_id = engine.register(SupportTicketAgent);
    println!(
        "  [{}] SupportTicketAgent → Signals (support activity)",
        support_id
    );

    let revenue_id = engine.register(RevenueTrendAgent);
    println!(
        "  [{}] RevenueTrendAgent → Signals (revenue trends)",
        revenue_id
    );

    let churn_id = engine.register(ChurnRiskAgent);
    println!(
        "  [{}] ChurnRiskAgent → Strategies (risk assessments)",
        churn_id
    );

    let upsell_id = engine.register(UpsellOpportunityAgent);
    println!(
        "  [{}] UpsellOpportunityAgent → Strategies (opportunities)",
        upsell_id
    );

    let action_id = engine.register(ActionPrioritizationAgent);
    println!(
        "  [{}] ActionPrioritizationAgent → Evaluations (ranked actions)",
        action_id
    );

    println!("\n  Total Agents: {}", engine.agent_count());

    println!("\n┌──────────────────────────────────────────────────────────────────────────────┐");
    println!("│ PHASE 3: INVARIANT REGISTRATION                                              │");
    println!("└──────────────────────────────────────────────────────────────────────────────┘");

    engine.register_invariant(RequireCompleteAnalysis);
    println!("\n  ✓ RequireCompleteAnalysis");

    engine.register_invariant(RequireChurnActionPlan);
    println!("  ✓ RequireChurnActionPlan");

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
    println!("║  Agents Registered:    7                                                     ║");
    println!("║  Invariants Enforced:  2                                                     ║");
    println!(
        "║  Cycles Executed:      {}                                                     ║",
        result.cycles
    );
    println!("║  Convergence:          ✓ ACHIEVED                                            ║");
    println!("╚══════════════════════════════════════════════════════════════════════════════╝");
    println!();

    assert!(result.converged);
}
