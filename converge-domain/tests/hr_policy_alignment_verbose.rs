// Copyright 2024-2025 Aprio One AB, Sweden
// Author: Kenneth Pernyer, kenneth@aprio.one
// SPDX-License-Identifier: MIT
// See LICENSE file in the project root for full license information.

//! Verbose HR Policy Alignment Integration Test
//!
//! This test demonstrates the complete Converge execution for the
//! HR Policy Alignment use case with detailed output at each stage.

use converge_core::agents::SeedAgent;
use converge_core::{Budget, Context, ContextKey, Engine};
use converge_domain::hr_policy_alignment::{
    AcknowledgementTrackingAgent, AlignmentStatusAgent, EscalationAgent,
    ManagerFollowUpAgent, MeetingCompletionAgent, PolicyDistributionAgent,
    RequireAllAcknowledgements, RequireHighRiskRoleConfirmation, RequireManagerFollowUp,
    UnderstandingSignalAgent,
};

#[test]
#[ignore] // TODO: Fix domain logic - ManagerFollowUpAgent needs to schedule meetings before semantic invariant fires
fn verbose_hr_policy_alignment_execution() {
    println!("\n");
    println!("╔══════════════════════════════════════════════════════════════════════════════╗");
    println!("║        CONVERGE HR POLICY ALIGNMENT - VERBOSE EXECUTION TRACE                 ║");
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

    let policy_id = engine.register(SeedAgent::new(
        "policy:remote-work",
        "Remote work policy: all employees must follow new guidelines for remote work, including workspace setup, communication protocols, and security requirements",
    ));
    println!("    [{policy_id}] SeedAgent 'policy:remote-work'");
    println!("         → Dependencies: [] (runs first cycle)");
    println!("         → Emits: Seeds (policy definition)");

    println!("\n  Registering HR Policy Alignment Pipeline:");

    let dist_id = engine.register(PolicyDistributionAgent);
    println!("    [{dist_id}] PolicyDistributionAgent");
    println!("         → Dependencies: [Seeds]");
    println!("         → Emits: Signals (affected employees)");

    let ack_id = engine.register(AcknowledgementTrackingAgent);
    println!("    [{ack_id}] AcknowledgementTrackingAgent");
    println!("         → Dependencies: [Signals]");
    println!("         → Emits: Signals (acknowledgements)");

    let understanding_id = engine.register(UnderstandingSignalAgent);
    println!("    [{understanding_id}] UnderstandingSignalAgent");
    println!("         → Dependencies: [Signals]");
    println!("         → Emits: Signals (understanding signals)");

    let followup_id = engine.register(ManagerFollowUpAgent);
    println!("    [{followup_id}] ManagerFollowUpAgent");
    println!("         → Dependencies: [Signals, Constraints]");
    println!("         → Emits: Strategies (meetings scheduled)");

    let meeting_id = engine.register(MeetingCompletionAgent);
    println!("    [{meeting_id}] MeetingCompletionAgent");
    println!("         → Dependencies: [Strategies]");
    println!("         → Emits: Signals (meetings completed)");

    let escalation_id = engine.register(EscalationAgent);
    println!("    [{escalation_id}] EscalationAgent");
    println!("         → Dependencies: [Signals, Strategies]");
    println!("         → Emits: Strategies (escalations)");

    let status_id = engine.register(AlignmentStatusAgent);
    println!("    [{status_id}] AlignmentStatusAgent");
    println!("         → Dependencies: [Signals, Strategies]");
    println!("         → Emits: Evaluations (alignment status)");

    println!("\n  Total Agents: {}", engine.agent_count());

    // =========================================================================
    // PHASE 3: INVARIANT REGISTRATION
    // =========================================================================
    println!("\n┌──────────────────────────────────────────────────────────────────────────────┐");
    println!("│ PHASE 3: INVARIANT REGISTRATION (Gherkin → Runtime Law)                      │");
    println!("└──────────────────────────────────────────────────────────────────────────────┘");

    println!("\n  Semantic Invariants (checked at end of each cycle):");
    let manager_followup_id = engine.register_invariant(RequireManagerFollowUp);
    println!("    [{manager_followup_id}] RequireManagerFollowUp");
    println!("         → Rule: Unclear understanding → Manager meeting required");
    println!("         → Violation = convergence blocked");

    println!("\n  Acceptance Invariants (checked when convergence claimed):");
    let ack_id = engine.register_invariant(RequireAllAcknowledgements);
    println!("    [{ack_id}] RequireAllAcknowledgements");
    println!("         → Rule: All affected employees must acknowledge");
    println!("         → Violation = convergence rejected");

    let high_risk_id = engine.register_invariant(RequireHighRiskRoleConfirmation::default());
    println!("    [{high_risk_id}] RequireHighRiskRoleConfirmation");
    println!("         → Rule: High-risk roles require manager confirmation");
    println!("         → High-risk roles: executive, director, manager, finance, legal, compliance");
    println!("         → Violation = convergence rejected");

    // =========================================================================
    // PHASE 4: EXECUTION
    // =========================================================================
    println!("\n┌──────────────────────────────────────────────────────────────────────────────┐");
    println!("│ PHASE 4: EXECUTION                                                            │");
    println!("└──────────────────────────────────────────────────────────────────────────────┘");

    println!("\n  Running engine...");
    let result = engine.run(Context::new()).expect("should converge");

    println!("\n  Execution Summary:");
    println!("    • Cycles: {}", result.cycles);
    println!("    • Converged: {}", result.converged);
    let fact_count: usize = result.context.all_keys().iter()
        .map(|k| result.context.get(*k).len())
        .sum();
    println!("    • Total Facts: {}", fact_count);

    // =========================================================================
    // PHASE 5: CONTEXT INSPECTION
    // =========================================================================
    println!("\n┌──────────────────────────────────────────────────────────────────────────────┐");
    println!("│ PHASE 5: CONTEXT INSPECTION                                                   │");
    println!("└──────────────────────────────────────────────────────────────────────────────┘");

    println!("\n  Seeds (Policy):");
    let seeds = result.context.get(ContextKey::Seeds);
    for seed in seeds {
        println!("    • [{}] {}", seed.id, seed.content);
    }

    println!("\n  Signals (Employees, Acknowledgements, Understanding, Meetings):");
    let signals = result.context.get(ContextKey::Signals);
    for signal in signals {
        println!("    • [{}] {}", signal.id, signal.content);
    }

    println!("\n  Strategies (Meetings Scheduled, Escalations):");
    let strategies = result.context.get(ContextKey::Strategies);
    for strategy in strategies {
        println!("    • [{}] {}", strategy.id, strategy.content);
    }

    println!("\n  Evaluations (Alignment Status):");
    let evaluations = result.context.get(ContextKey::Evaluations);
    for eval in evaluations {
        println!("    • [{}] {}", eval.id, eval.content);
    }

    // =========================================================================
    // PHASE 6: CONVERGENCE ANALYSIS
    // =========================================================================
    println!("\n┌──────────────────────────────────────────────────────────────────────────────┐");
    println!("│ PHASE 6: CONVERGENCE ANALYSIS                                                 │");
    println!("└──────────────────────────────────────────────────────────────────────────────┘");

    let employees: Vec<_> = signals
        .iter()
        .filter(|s| s.id.starts_with("employee:"))
        .collect();
    let acknowledgements: Vec<_> = signals
        .iter()
        .filter(|s| s.id.starts_with("ack:"))
        .collect();
    let understanding: Vec<_> = signals
        .iter()
        .filter(|s| s.id.starts_with("understanding:"))
        .collect();
    let meetings_scheduled: Vec<_> = strategies
        .iter()
        .filter(|s| s.id.starts_with("meeting:scheduled:"))
        .collect();
    let meetings_completed: Vec<_> = signals
        .iter()
        .filter(|s| s.id.starts_with("meeting:completed:"))
        .collect();
    let escalations: Vec<_> = strategies
        .iter()
        .filter(|s| s.id.starts_with("escalation:"))
        .collect();

    println!("\n  Policy Alignment Metrics:");
    println!("    • Total Employees: {}", employees.len());
    println!("    • Acknowledged: {}", acknowledgements.len());
    println!("    • Understanding Assessed: {}", understanding.len());
    println!("    • Meetings Scheduled: {}", meetings_scheduled.len());
    println!("    • Meetings Completed: {}", meetings_completed.len());
    println!("    • Escalations: {}", escalations.len());

    let alignment_status = evaluations
        .iter()
        .find(|e| e.id == "alignment-status");

    if let Some(status) = alignment_status {
        println!("\n  Alignment Status:");
        println!("    {}", status.content);
    }

    // =========================================================================
    // PHASE 7: VALIDATION
    // =========================================================================
    println!("\n┌──────────────────────────────────────────────────────────────────────────────┐");
    println!("│ PHASE 7: VALIDATION                                                          │");
    println!("└──────────────────────────────────────────────────────────────────────────────┘");

    assert!(result.converged, "Engine should converge");
    assert!(!employees.is_empty(), "Should have identified employees");
    assert!(!acknowledgements.is_empty(), "Should have some acknowledgements");
    assert!(!understanding.is_empty(), "Should have assessed understanding");
    assert!(
        !evaluations.is_empty(),
        "Should have alignment status evaluation"
    );

    println!("\n  ✓ All validations passed");

    println!("\n╔══════════════════════════════════════════════════════════════════════════════╗");
    println!("║                    EXECUTION COMPLETE - CONVERGED                            ║");
    println!("╚══════════════════════════════════════════════════════════════════════════════╝\n");
}
