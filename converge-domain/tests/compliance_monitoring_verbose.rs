// Copyright 2024-2025 Aprio One AB, Sweden
// Author: Kenneth Pernyer, kenneth@aprio.one
// SPDX-License-Identifier: MIT
// See LICENSE file in the project root for full license information.

//! Verbose Compliance Monitoring Integration Test
//!
//! This test demonstrates the complete Converge execution for the
//! Compliance Monitoring use case with detailed output at each stage.

use converge_core::agents::SeedAgent;
use converge_core::{Budget, Context, ContextKey, Engine};
use converge_domain::compliance_monitoring::{
    EvidenceCollectorAgent, PolicyRuleAgent, RegulationParserAgent, RemediationProposalAgent,
    RequireEvidenceForAllRegulations, RequireRemediationPlans, ViolationDetectorAgent,
};

#[test]
fn verbose_compliance_monitoring_execution() {
    println!("\n");
    println!("╔══════════════════════════════════════════════════════════════════════════════╗");
    println!("║         CONVERGE COMPLIANCE MONITORING - VERBOSE EXECUTION TRACE             ║");
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

    let seed1_id = engine.register(SeedAgent::new("regulations", "GDPR, SOC2, HIPAA"));
    println!("\n  [{seed1_id}] SeedAgent 'regulations'");

    let parser_id = engine.register(RegulationParserAgent);
    println!("  [{parser_id}] RegulationParserAgent → Signals (parsed regulations)");

    let policy_id = engine.register(PolicyRuleAgent);
    println!("  [{policy_id}] PolicyRuleAgent → Constraints (policy rules)");

    let evidence_id = engine.register(EvidenceCollectorAgent);
    println!("  [{evidence_id}] EvidenceCollectorAgent → Signals (evidence data)");

    let violation_id = engine.register(ViolationDetectorAgent);
    println!("  [{violation_id}] ViolationDetectorAgent → Strategies (violation reports)");

    let remediation_id = engine.register(RemediationProposalAgent);
    println!("  [{remediation_id}] RemediationProposalAgent → Evaluations (remediation plans)");

    println!("\n  Total Agents: {}", engine.agent_count());

    println!("\n┌──────────────────────────────────────────────────────────────────────────────┐");
    println!("│ PHASE 3: INVARIANT REGISTRATION                                              │");
    println!("└──────────────────────────────────────────────────────────────────────────────┘");

    engine.register_invariant(RequireEvidenceForAllRegulations);
    println!("\n  ✓ RequireEvidenceForAllRegulations");

    engine.register_invariant(RequireRemediationPlans);
    println!("  ✓ RequireRemediationPlans");

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

    println!("\n  🔒 CONSTRAINTS:");
    for fact in result.context.get(ContextKey::Constraints) {
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
    println!("║  Agents Registered:    6                                                     ║");
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
