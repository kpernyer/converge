// Copyright 2024-2025 Aprio One AB, Sweden
// Author: Kenneth Pernyer, kenneth@aprio.one
// SPDX-License-Identifier: MIT
// See LICENSE file in the project root for full license information.

//! Verbose Meeting Scheduler Integration Test
//!
//! This test demonstrates the complete Converge execution for the
//! Meeting Scheduler use case with detailed output at each stage.

use converge_core::agents::SeedAgent;
use converge_core::{Budget, Context, ContextKey, Engine};
use converge_domain::meeting_scheduler::{
    AvailabilityRetrievalAgent, ConflictDetectionAgent, RequireParticipantAvailability,
    RequirePositiveDuration, RequireValidSlot, SlotOptimizationAgent, TimeZoneNormalizationAgent,
    WorkingHoursConstraintAgent,
};

#[test]
fn verbose_meeting_scheduler_execution() {
    println!("\n");
    println!("╔══════════════════════════════════════════════════════════════════════════════╗");
    println!("║        CONVERGE MEETING SCHEDULER - VERBOSE EXECUTION TRACE                  ║");
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

    let seed1_id = engine.register(SeedAgent::new("participants", "Alice, Bob, Carol"));
    println!("    [{seed1_id}] SeedAgent 'participants'");
    println!("         → Dependencies: [] (runs first cycle)");
    println!("         → Emits: Seeds");

    let seed2_id = engine.register(SeedAgent::new("duration", "60"));
    println!("    [{seed2_id}] SeedAgent 'duration'");
    println!("         → Dependencies: [] (runs first cycle)");
    println!("         → Emits: Seeds");

    let seed3_id = engine.register(SeedAgent::new("window", "next week"));
    println!("    [{seed3_id}] SeedAgent 'window'");
    println!("         → Dependencies: [] (runs first cycle)");
    println!("         → Emits: Seeds");

    println!("\n  Registering Meeting Scheduler Pipeline:");

    let avail_id = engine.register(AvailabilityRetrievalAgent);
    println!("    [{avail_id}] AvailabilityRetrievalAgent");
    println!("         → Dependencies: [Seeds]");
    println!("         → Emits: Signals (availability data)");

    let tz_id = engine.register(TimeZoneNormalizationAgent);
    println!("    [{tz_id}] TimeZoneNormalizationAgent");
    println!("         → Dependencies: [Signals]");
    println!("         → Emits: Signals (normalized times)");

    let hours_id = engine.register(WorkingHoursConstraintAgent);
    println!("    [{hours_id}] WorkingHoursConstraintAgent");
    println!("         → Dependencies: [Signals]");
    println!("         → Emits: Constraints (working hours)");

    let slot_id = engine.register(SlotOptimizationAgent);
    println!("    [{slot_id}] SlotOptimizationAgent");
    println!("         → Dependencies: [Signals, Constraints]");
    println!("         → Emits: Strategies (candidate slots)");

    let conflict_id = engine.register(ConflictDetectionAgent);
    println!("    [{conflict_id}] ConflictDetectionAgent");
    println!("         → Dependencies: [Strategies]");
    println!("         → Emits: Evaluations (valid slots ranked)");

    println!("\n  Total Agents: {}", engine.agent_count());

    // =========================================================================
    // PHASE 3: INVARIANT REGISTRATION
    // =========================================================================
    println!("\n┌──────────────────────────────────────────────────────────────────────────────┐");
    println!("│ PHASE 3: INVARIANT REGISTRATION (Gherkin → Runtime Law)                      │");
    println!("└──────────────────────────────────────────────────────────────────────────────┘");

    println!("\n  Structural Invariants (checked after every merge):");
    let duration_id = engine.register_invariant(RequirePositiveDuration);
    println!("    [{duration_id}] RequirePositiveDuration");
    println!("         → Duration must be > 0");
    println!("         → Violation = immediate failure");

    println!("\n  Semantic Invariants (checked at end of each cycle):");
    let avail_inv_id = engine.register_invariant(RequireParticipantAvailability);
    println!("    [{avail_inv_id}] RequireParticipantAvailability");
    println!("         → All participants must be available");
    println!("         → Violation = blocks convergence");

    println!("\n  Acceptance Invariants (checked before declaring convergence):");
    let slot_inv_id = engine.register_invariant(RequireValidSlot);
    println!("    [{slot_inv_id}] RequireValidSlot");
    println!("         → At least one valid slot must exist");
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
    println!("\n  📦 SEEDS (Meeting Requirements):");
    println!("  ───────────────────────────────────────────────────────────────────────────");
    for fact in result.context.get(ContextKey::Seeds) {
        println!("    [{:30}] {}", fact.id, fact.content);
    }

    // Signals
    println!("\n  📡 SIGNALS (Availability & Constraints):");
    println!("  ───────────────────────────────────────────────────────────────────────────");
    for fact in result.context.get(ContextKey::Signals) {
        println!("    [{:30}] {}", fact.id, fact.content);
    }

    // Constraints
    println!("\n  🔒 CONSTRAINTS (Working Hours):");
    println!("  ───────────────────────────────────────────────────────────────────────────");
    for fact in result.context.get(ContextKey::Constraints) {
        println!("    [{:30}] {}", fact.id, fact.content);
    }

    // Strategies
    println!("\n  🎯 STRATEGIES (Candidate Slots):");
    println!("  ───────────────────────────────────────────────────────────────────────────");
    for fact in result.context.get(ContextKey::Strategies) {
        println!("    [{}]", fact.id);
        println!("       {}", fact.content);
        println!();
    }

    // Evaluations
    println!("  📊 EVALUATIONS (Valid Slots Ranked):");
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

    println!("\n  ✓ RequirePositiveDuration: Duration > 0 validated");
    println!("  ✓ RequireParticipantAvailability: All participants available");
    println!(
        "  ✓ RequireValidSlot: {} valid slots found",
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
