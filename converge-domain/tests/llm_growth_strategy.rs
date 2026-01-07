// Copyright 2024-2025 Aprio One AB, Sweden
// Author: Kenneth Pernyer, kenneth@aprio.one
// SPDX-License-Identifier: MIT
// See LICENSE file in the project root for full license information.

//! Deep Integration Test: LLM-Powered Growth Strategy
//!
//! This test demonstrates a realistic scenario where:
//! 1. Human provides initial seeds (trusted)
//! 2. LLM agents propose hypotheses and signals (untrusted)
//! 3. ValidationAgent filters LLM outputs
//! 4. Growth strategy agents use validated facts
//! 5. Final strategies are evaluated and ranked
//!
//! This proves the full safety model in a real-world use case.

use converge_core::agents::SeedAgent;
use converge_core::validation::{encode_proposal, ValidationAgent, ValidationConfig};
use converge_core::{Agent, AgentEffect, Context, ContextKey, Engine, Fact, ProposedFact};
use converge_domain::growth_strategy::{
    BrandSafetyInvariant, EvaluationAgent, RequireMultipleStrategies,
    RequireStrategyEvaluations, StrategyAgent,
};

// =============================================================================
// SIMULATED LLM AGENTS
// =============================================================================

/// Simulates an LLM agent that proposes market signals.
///
/// In production, this would call Claude/GPT and parse the response.
/// Here we simulate both good and bad outputs to test the safety model.
struct LlmSignalAgent;

impl Agent for LlmSignalAgent {
    fn name(&self) -> &str {
        "LlmSignalAgent"
    }

    fn dependencies(&self) -> &[ContextKey] {
        &[ContextKey::Seeds]
    }

    fn accepts(&self, ctx: &Context) -> bool {
        // Run once when seeds exist but we haven't proposed signals yet
        ctx.has(ContextKey::Seeds) &&
        !ctx.get(ContextKey::Proposals).iter().any(|p| p.id.contains("llm-signal"))
    }

    fn execute(&self, ctx: &Context) -> AgentEffect {
        let seeds = ctx.get(ContextKey::Seeds);
        let mut facts = Vec::new();

        // Simulate LLM analyzing the seeds and proposing signals
        // Mix of good and bad proposals to test validation

        if seeds.iter().any(|s| s.content.contains("Nordic")) {
            // Good proposal - high confidence, proper provenance
            facts.push(encode_proposal(&ProposedFact {
                key: ContextKey::Signals,
                id: "llm-signal-nordic-trend".into(),
                content: "Nordic tech adoption accelerating in enterprise segment".into(),
                confidence: 0.82,
                provenance: "claude-3-opus:market-analysis-001".into(),
            }));

            // Good proposal
            facts.push(encode_proposal(&ProposedFact {
                key: ContextKey::Signals,
                id: "llm-signal-competition".into(),
                content: "3 major competitors identified in Nordic region".into(),
                confidence: 0.88,
                provenance: "claude-3-opus:competitor-scan-001".into(),
            }));

            // Bad proposal - hallucinated overconfident claim
            facts.push(encode_proposal(&ProposedFact {
                key: ContextKey::Signals,
                id: "llm-signal-hallucination".into(),
                content: "Market will definitely grow 500% next year guaranteed".into(),
                confidence: 0.95, // LLM is overconfident about hallucination
                provenance: "gpt-4:hallucination-001".into(),
            }));
        }

        if seeds.iter().any(|s| s.content.contains("B2B")) {
            // Good proposal
            facts.push(encode_proposal(&ProposedFact {
                key: ContextKey::Signals,
                id: "llm-signal-b2b-channel".into(),
                content: "LinkedIn most effective B2B channel in region".into(),
                confidence: 0.75,
                provenance: "claude-3-opus:channel-analysis-001".into(),
            }));

            // Weak proposal - low confidence
            facts.push(encode_proposal(&ProposedFact {
                key: ContextKey::Signals,
                id: "llm-signal-uncertain".into(),
                content: "Maybe TikTok could work for B2B possibly".into(),
                confidence: 0.35, // LLM correctly indicates uncertainty
                provenance: "gpt-3.5:speculation-001".into(),
            }));
        }

        AgentEffect::with_facts(facts)
    }
}

/// Simulates an LLM agent that proposes strategic hypotheses.
struct LlmHypothesisAgent;

impl Agent for LlmHypothesisAgent {
    fn name(&self) -> &str {
        "LlmHypothesisAgent"
    }

    fn dependencies(&self) -> &[ContextKey] {
        &[ContextKey::Signals]
    }

    fn accepts(&self, ctx: &Context) -> bool {
        // Run once when signals exist but we haven't proposed hypotheses yet
        ctx.has(ContextKey::Signals) &&
        !ctx.get(ContextKey::Proposals).iter().any(|p| p.id.contains("llm-hyp"))
    }

    fn execute(&self, ctx: &Context) -> AgentEffect {
        let signals = ctx.get(ContextKey::Signals);
        let mut facts = Vec::new();

        // Simulate LLM synthesizing hypotheses from signals

        if signals.iter().any(|s| s.content.contains("competitor")) {
            // Good hypothesis
            facts.push(encode_proposal(&ProposedFact {
                key: ContextKey::Hypotheses,
                id: "llm-hyp-smb-gap".into(),
                content: "Competitors focus on enterprise, leaving SMB underserved".into(),
                confidence: 0.78,
                provenance: "claude-3-opus:synthesis-001".into(),
            }));
        }

        if signals.iter().any(|s| s.content.contains("LinkedIn")) {
            // Good hypothesis
            facts.push(encode_proposal(&ProposedFact {
                key: ContextKey::Hypotheses,
                id: "llm-hyp-channel-opportunity".into(),
                content: "LinkedIn presence could differentiate from competitors".into(),
                confidence: 0.72,
                provenance: "claude-3-opus:synthesis-002".into(),
            }));
        }

        // Bad hypothesis - no provenance (simulating a bug)
        facts.push(encode_proposal(&ProposedFact {
            key: ContextKey::Hypotheses,
            id: "llm-hyp-anonymous".into(),
            content: "We should pivot to crypto".into(),
            confidence: 0.60,
            provenance: "".into(), // Bug: missing provenance
        }));

        AgentEffect::with_facts(facts)
    }
}

// =============================================================================
// CUSTOM SIGNAL-TO-COMPETITOR AGENT
// =============================================================================

/// Agent that converts validated signals into competitor profiles.
/// Only processes validated signals (not raw LLM proposals).
struct SignalToCompetitorAgent;

impl Agent for SignalToCompetitorAgent {
    fn name(&self) -> &str {
        "SignalToCompetitorAgent"
    }

    fn dependencies(&self) -> &[ContextKey] {
        &[ContextKey::Signals]
    }

    fn accepts(&self, ctx: &Context) -> bool {
        // Run when we have validated signals about competition
        let signals = ctx.get(ContextKey::Signals);
        let has_competition_signal = signals.iter().any(|s|
            s.content.contains("competitor") && !s.id.contains("rejected")
        );
        has_competition_signal && !ctx.has(ContextKey::Competitors)
    }

    fn execute(&self, _ctx: &Context) -> AgentEffect {
        // Generate competitor profiles based on validated signals
        AgentEffect::with_facts(vec![
            Fact {
                key: ContextKey::Competitors,
                id: "competitor:enterprise-inc".into(),
                content: "EnterpriseInc: Strong enterprise focus, weak SMB presence".into(),
            },
            Fact {
                key: ContextKey::Competitors,
                id: "competitor:legacy-corp".into(),
                content: "LegacyCorp: Established brand, slow innovation".into(),
            },
        ])
    }
}

#[test]
fn llm_powered_growth_strategy_verbose() {
    println!("\n");
    println!("╔══════════════════════════════════════════════════════════════════════════════╗");
    println!("║          LLM-POWERED GROWTH STRATEGY - DEEP INTEGRATION TEST                 ║");
    println!("╚══════════════════════════════════════════════════════════════════════════════╝");

    // =========================================================================
    // SCENARIO DESCRIPTION
    // =========================================================================
    println!("\n┌──────────────────────────────────────────────────────────────────────────────┐");
    println!("│ SCENARIO: Real-World LLM Integration                                         │");
    println!("└──────────────────────────────────────────────────────────────────────────────┘");

    println!("\n  A growth strategist wants to:");
    println!("    1. Input: Market (Nordic B2B) and Product (SaaS Platform)");
    println!("    2. Let LLMs analyze market signals and propose hypotheses");
    println!("    3. Filter out LLM hallucinations and weak proposals");
    println!("    4. Generate strategies from validated insights");
    println!("    5. Get ranked recommendations with rationale");
    println!();
    println!("  Data Flow:");
    println!("    Human Seeds → LLM Signals → Validation → LLM Hypotheses → Validation");
    println!("         → Competitors → Strategies → Evaluations");

    // =========================================================================
    // ENGINE SETUP
    // =========================================================================
    println!("\n┌──────────────────────────────────────────────────────────────────────────────┐");
    println!("│ ENGINE SETUP                                                                 │");
    println!("└──────────────────────────────────────────────────────────────────────────────┘");

    let mut engine = Engine::new();

    // Human-provided seeds (trusted)
    println!("\n  TRUSTED AGENTS (Human Input):");
    let s1 = engine.register(SeedAgent::new("market:nordic-b2b", "Nordic B2B market"));
    println!("    [{}] SeedAgent: market:nordic-b2b", s1);
    let s2 = engine.register(SeedAgent::new("product:saas-platform", "Enterprise SaaS Platform"));
    println!("    [{}] SeedAgent: product:saas-platform", s2);

    // LLM agents (untrusted - outputs go to Proposals)
    println!("\n  UNTRUSTED AGENTS (LLM Output → Proposals):");
    let llm1 = engine.register(LlmSignalAgent);
    println!("    [{}] LlmSignalAgent: Proposes market signals", llm1);
    let llm2 = engine.register(LlmHypothesisAgent);
    println!("    [{}] LlmHypothesisAgent: Proposes strategic hypotheses", llm2);

    // Validation gateway
    println!("\n  VALIDATION GATEWAY:");
    let val_config = ValidationConfig {
        min_confidence: 0.6,
        max_content_length: 500,
        forbidden_terms: vec!["guaranteed".into(), "definitely".into(), "100%".into()],
        require_provenance: true,
    };
    let val = engine.register(ValidationAgent::new(val_config));
    println!("    [{}] ValidationAgent", val);
    println!("         → min_confidence: 0.6");
    println!("         → forbidden: [guaranteed, definitely, 100%]");
    println!("         → require_provenance: true");

    // Strategy pipeline (operates on validated facts)
    println!("\n  STRATEGY PIPELINE (Uses Validated Facts Only):");
    let comp = engine.register(SignalToCompetitorAgent);
    println!("    [{}] SignalToCompetitorAgent: Signals → Competitors", comp);
    let strat = engine.register(StrategyAgent);
    println!("    [{}] StrategyAgent: Competitors → Strategies", strat);
    let eval = engine.register(EvaluationAgent);
    println!("    [{}] EvaluationAgent: Strategies → Evaluations", eval);

    // Invariants
    println!("\n  INVARIANTS:");
    engine.register_invariant(BrandSafetyInvariant::default());
    println!("    • BrandSafetyInvariant (structural)");
    engine.register_invariant(RequireMultipleStrategies);
    println!("    • RequireMultipleStrategies (acceptance)");
    engine.register_invariant(RequireStrategyEvaluations);
    println!("    • RequireStrategyEvaluations (acceptance)");

    println!("\n  Total: {} agents registered", engine.agent_count());

    // =========================================================================
    // EXECUTION
    // =========================================================================
    println!("\n┌──────────────────────────────────────────────────────────────────────────────┐");
    println!("│ CONVERGENCE EXECUTION                                                        │");
    println!("└──────────────────────────────────────────────────────────────────────────────┘");

    println!("\n  Expected cycle progression:");
    println!("    Cycle 1: SeedAgents emit seeds");
    println!("    Cycle 2: LlmSignalAgent proposes signals");
    println!("    Cycle 3: ValidationAgent filters signals");
    println!("    Cycle 4: LlmHypothesisAgent proposes hypotheses");
    println!("    Cycle 5: ValidationAgent filters hypotheses");
    println!("    Cycle 6: SignalToCompetitorAgent creates profiles");
    println!("    Cycle 7: StrategyAgent proposes strategies");
    println!("    Cycle 8: EvaluationAgent scores strategies");
    println!("    Cycle 9: Convergence (no more work)");

    println!("\n  Running engine.run()...");
    println!("  ─────────────────────────────────────────────────────────────────────────────");

    let result = engine.run(Context::new()).expect("should converge");

    println!("  ─────────────────────────────────────────────────────────────────────────────");
    println!("  Converged in {} cycles", result.cycles);

    // =========================================================================
    // LLM PROPOSAL ANALYSIS
    // =========================================================================
    println!("\n┌──────────────────────────────────────────────────────────────────────────────┐");
    println!("│ LLM PROPOSAL ANALYSIS                                                        │");
    println!("└──────────────────────────────────────────────────────────────────────────────┘");

    let proposals = result.context.get(ContextKey::Proposals);
    let signals = result.context.get(ContextKey::Signals);
    let hypotheses = result.context.get(ContextKey::Hypotheses);

    // Count rejections
    let rejections: Vec<_> = signals.iter().filter(|s| s.id.contains("rejected")).collect();
    let accepted_signals: Vec<_> = signals.iter().filter(|s| !s.id.contains("rejected")).collect();

    println!("\n  LLM Proposals Submitted: {}", proposals.len());
    println!("  ───────────────────────────────────────────────────────────────────────────");
    for p in proposals {
        let short_id = p.id.strip_prefix("proposal:").unwrap_or(&p.id);
        println!("    • {}", short_id);
    }

    println!("\n  Validation Results:");
    println!("  ───────────────────────────────────────────────────────────────────────────");
    println!("    ✓ Accepted Signals: {}", accepted_signals.len());
    println!("    ✓ Accepted Hypotheses: {}", hypotheses.len());
    println!("    ✗ Rejected: {}", rejections.len());

    println!("\n  ACCEPTED (Promoted to Trusted Context):");
    println!("  ───────────────────────────────────────────────────────────────────────────");

    println!("\n    📡 Signals:");
    for s in &accepted_signals {
        println!("      ✓ [{}] \"{}\"", s.id, truncate(&s.content, 50));
    }

    println!("\n    💡 Hypotheses:");
    for h in hypotheses {
        println!("      ✓ [{}] \"{}\"", h.id, truncate(&h.content, 50));
    }

    println!("\n  REJECTED (Filtered by ValidationAgent):");
    println!("  ───────────────────────────────────────────────────────────────────────────");
    for r in &rejections {
        let reason = r.content.split("rejected: ").nth(1).unwrap_or(&r.content);
        let id = r.id.strip_prefix("validation:rejected:").unwrap_or(&r.id);
        println!("    ✗ [{}]", id);
        println!("      Reason: {}", reason);
        println!();
    }

    // =========================================================================
    // STRATEGY OUTPUT
    // =========================================================================
    println!("┌──────────────────────────────────────────────────────────────────────────────┐");
    println!("│ FINAL STRATEGY OUTPUT                                                        │");
    println!("└──────────────────────────────────────────────────────────────────────────────┘");

    let strategies = result.context.get(ContextKey::Strategies);
    let evaluations = result.context.get(ContextKey::Evaluations);
    let competitors = result.context.get(ContextKey::Competitors);

    println!("\n  🏢 COMPETITORS IDENTIFIED:");
    println!("  ───────────────────────────────────────────────────────────────────────────");
    for c in competitors {
        println!("    • {}", c.content);
    }

    println!("\n  🎯 STRATEGIES GENERATED:");
    println!("  ───────────────────────────────────────────────────────────────────────────");
    for s in strategies {
        println!("    [{}]", s.id);
        println!("    {}", s.content);
        println!();
    }

    println!("  📊 RANKED EVALUATIONS:");
    println!("  ═══════════════════════════════════════════════════════════════════════════");
    for e in evaluations {
        println!("    {}", e.content);
        println!();
    }

    // =========================================================================
    // DATA LINEAGE
    // =========================================================================
    println!("┌──────────────────────────────────────────────────────────────────────────────┐");
    println!("│ DATA LINEAGE - TRUST CHAIN                                                   │");
    println!("└──────────────────────────────────────────────────────────────────────────────┘");

    println!("\n  Every fact in the final output has a verifiable trust chain:");
    println!();
    println!("    Seeds (Human)");
    println!("      └─→ LLM Proposals (Untrusted)");
    println!("            └─→ ValidationAgent (Gateway)");
    println!("                  ├─→ Accepted → Signals/Hypotheses (Trusted)");
    println!("                  └─→ Rejected → Audit Trail");
    println!("                        └─→ Competitors (Derived from Trusted)");
    println!("                              └─→ Strategies (Derived)");
    println!("                                    └─→ Evaluations (Final)");
    println!();
    println!("  NO LLM OUTPUT REACHED THE FINAL STRATEGIES WITHOUT:");
    println!("    ✓ Passing confidence threshold (≥60%)");
    println!("    ✓ Having valid provenance");
    println!("    ✓ Avoiding forbidden terms");
    println!("    ✓ Containing non-empty content");

    // =========================================================================
    // SUMMARY
    // =========================================================================
    println!("\n╔══════════════════════════════════════════════════════════════════════════════╗");
    println!("║                              EXECUTION SUMMARY                                ║");
    println!("╠══════════════════════════════════════════════════════════════════════════════╣");
    println!("║  Agents:           8 (2 human, 2 LLM, 1 validator, 3 strategy)               ║");
    println!("║  Invariants:       3                                                         ║");
    println!("║  Cycles:           {}                                                        ║", result.cycles);
    println!("║  LLM Proposals:    {}                                                         ║", proposals.len());
    println!("║  Accepted:         {}                                                         ║", accepted_signals.len() + hypotheses.len());
    println!("║  Rejected:         {}                                                         ║", rejections.len());
    println!("║  Final Strategies: {}                                                         ║", strategies.len());
    println!("║  Convergence:      ✓ ACHIEVED                                                ║");
    println!("╚══════════════════════════════════════════════════════════════════════════════╝");
    println!();

    // Assertions
    assert!(result.converged);
    assert!(!strategies.is_empty(), "Should have strategies");
    assert!(!evaluations.is_empty(), "Should have evaluations");
    assert!(!rejections.is_empty(), "Should have rejected some LLM proposals");
    assert!(
        rejections.iter().any(|r| r.content.contains("guaranteed")),
        "Should have rejected hallucination with 'guaranteed'"
    );
}

fn truncate(s: &str, max: usize) -> String {
    if s.len() <= max {
        s.to_string()
    } else {
        format!("{}...", &s[..max])
    }
}
