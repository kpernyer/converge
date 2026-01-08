// Copyright 2024-2025 Aprio One AB, Sweden
// Author: Kenneth Pernyer, kenneth@aprio.one
// SPDX-License-Identifier: MIT
// See LICENSE file in the project root for full license information.

//! Verbose Release Readiness Integration Test
//!
//! This test demonstrates the complete Converge execution for the
//! Release Readiness use case with detailed output at each stage.

use converge_core::agents::SeedAgent;
use converge_core::{Budget, Context, ContextKey, Engine};
use converge_domain::release_readiness::{
    DependencyGraphAgent, DocumentationAgent, PerformanceRegressionAgent, ReleaseReadyAgent,
    RequireAllChecksComplete, RequireMinimumCoverage, RequireNoCriticalVulnerabilities,
    RiskSummaryAgent, SecurityScanAgent, TestCoverageAgent,
};

#[test]
fn verbose_release_readiness_execution() {
    println!("\n");
    println!("╔══════════════════════════════════════════════════════════════════════════════╗");
    println!("║         CONVERGE RELEASE READINESS - VERBOSE EXECUTION TRACE                 ║");
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

    let seed1_id = engine.register(SeedAgent::new("release:v1.2.0", "Release candidate v1.2.0"));
    println!("    [{}] SeedAgent 'release:v1.2.0'", seed1_id);
    println!("         → Dependencies: [] (runs first cycle)");
    println!("         → Emits: Seeds");

    println!("\n  Registering Parallel Quality Gate Agents:");

    let dep_id = engine.register(DependencyGraphAgent);
    println!("    [{}] DependencyGraphAgent", dep_id);
    println!("         → Dependencies: [Seeds]");
    println!("         → Emits: Signals (dependency analysis)");

    let coverage_id = engine.register(TestCoverageAgent);
    println!("    [{}] TestCoverageAgent", coverage_id);
    println!("         → Dependencies: [Seeds]");
    println!("         → Emits: Signals (coverage metrics)");

    let security_id = engine.register(SecurityScanAgent);
    println!("    [{}] SecurityScanAgent", security_id);
    println!("         → Dependencies: [Seeds]");
    println!("         → Emits: Signals (vulnerability reports)");

    let perf_id = engine.register(PerformanceRegressionAgent);
    println!("    [{}] PerformanceRegressionAgent", perf_id);
    println!("         → Dependencies: [Seeds]");
    println!("         → Emits: Signals (performance metrics)");

    let docs_id = engine.register(DocumentationAgent);
    println!("    [{}] DocumentationAgent", docs_id);
    println!("         → Dependencies: [Seeds]");
    println!("         → Emits: Signals (docs status)");

    println!("\n  Registering Consolidation & Decision Agents:");

    let risk_id = engine.register(RiskSummaryAgent);
    println!("    [{}] RiskSummaryAgent", risk_id);
    println!("         → Dependencies: [Signals] (waits for all checks)");
    println!("         → Emits: Strategies (risk assessments)");

    let release_id = engine.register(ReleaseReadyAgent);
    println!("    [{}] ReleaseReadyAgent", release_id);
    println!("         → Dependencies: [Strategies]");
    println!("         → Emits: Evaluations (go/no-go decision)");

    println!("\n  Total Agents: {}", engine.agent_count());

    // =========================================================================
    // PHASE 3: INVARIANT REGISTRATION
    // =========================================================================
    println!("\n┌──────────────────────────────────────────────────────────────────────────────┐");
    println!("│ PHASE 3: INVARIANT REGISTRATION (Gherkin → Runtime Law)                      │");
    println!("└──────────────────────────────────────────────────────────────────────────────┘");

    println!("\n  Structural Invariants (checked after every merge):");
    let vuln_id = engine.register_invariant(RequireNoCriticalVulnerabilities);
    println!("    [{}] RequireNoCriticalVulnerabilities", vuln_id);
    println!("         → No critical vulnerabilities allowed");
    println!("         → Violation = immediate failure");

    println!("\n  Semantic Invariants (checked at end of each cycle):");
    let coverage_inv_id = engine.register_invariant(RequireMinimumCoverage);
    println!("    [{}] RequireMinimumCoverage", coverage_inv_id);
    println!("         → Minimum coverage threshold must be met");
    println!("         → Violation = blocks convergence");

    println!("\n  Acceptance Invariants (checked before declaring convergence):");
    let complete_id = engine.register_invariant(RequireAllChecksComplete);
    println!("    [{}] RequireAllChecksComplete", complete_id);
    println!("         → All quality gate checks must be complete");
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
    println!("\n  📦 SEEDS (Release Candidate):");
    println!("  ───────────────────────────────────────────────────────────────────────────");
    for fact in result.context.get(ContextKey::Seeds) {
        println!("    [{:30}] {}", fact.id, fact.content);
    }

    // Signals
    println!("\n  📡 SIGNALS (Quality Gate Results):");
    println!("  ───────────────────────────────────────────────────────────────────────────");
    for fact in result.context.get(ContextKey::Signals) {
        println!("    [{:30}] {}", fact.id, fact.content);
    }

    // Strategies
    println!("\n  🎯 STRATEGIES (Risk Assessments):");
    println!("  ───────────────────────────────────────────────────────────────────────────");
    for fact in result.context.get(ContextKey::Strategies) {
        println!("    [{}]", fact.id);
        println!("       {}", fact.content);
        println!();
    }

    // Evaluations
    println!("  📊 EVALUATIONS (Release Decision):");
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

    let signals = result.context.get(ContextKey::Signals);
    let evaluations = result.context.get(ContextKey::Evaluations);

    println!("\n  ✓ RequireNoCriticalVulnerabilities: No critical vulnerabilities found");
    println!("  ✓ RequireMinimumCoverage: Coverage threshold met");
    println!(
        "  ✓ RequireAllChecksComplete: All {} checks complete",
        signals.len()
    );
    println!(
        "  ✓ Release Decision: {} evaluations generated",
        evaluations.len()
    );

    // =========================================================================
    // SUMMARY
    // =========================================================================
    println!("\n╔══════════════════════════════════════════════════════════════════════════════╗");
    println!("║                              EXECUTION SUMMARY                                ║");
    println!("╠══════════════════════════════════════════════════════════════════════════════╣");
    println!("║  Agents Registered:    8                                                     ║");
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
