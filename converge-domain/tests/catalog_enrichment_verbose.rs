// Copyright 2024-2025 Aprio One AB, Sweden
// Author: Kenneth Pernyer, kenneth@aprio.one
// SPDX-License-Identifier: MIT
// See LICENSE file in the project root for full license information.

//! Verbose Catalog Enrichment Integration Test
//!
//! This test demonstrates the complete Converge execution for the
//! Catalog Enrichment use case with detailed output at each stage.

use converge_core::agents::SeedAgent;
use converge_core::{Budget, Context, ContextKey, Engine};
use converge_domain::catalog_enrichment::{
    AttributeNormalizationAgent, CategoryInferenceAgent, DeduplicationAgent, FeedIngestionAgent,
    PricingValidationAgent, ProductReadyAgent, RequireNoDuplicates, RequireRequiredAttributes,
    RequireValidPrices, SchemaInvariantAgent,
};

#[test]
fn verbose_catalog_enrichment_execution() {
    println!("\n");
    println!("╔══════════════════════════════════════════════════════════════════════════════╗");
    println!("║         CONVERGE CATALOG ENRICHMENT - VERBOSE EXECUTION TRACE                ║");
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

    let seed1_id = engine.register(SeedAgent::new(
        "feeds",
        "ProductA:Widget:99.99|ProductB:Gadget:149.99",
    ));
    println!("\n  [{seed1_id}] SeedAgent 'feeds'");

    let feed_id = engine.register(FeedIngestionAgent);
    println!("  [{feed_id}] FeedIngestionAgent → Signals (raw products)");

    let dedup_id = engine.register(DeduplicationAgent);
    println!("  [{dedup_id}] DeduplicationAgent → Signals (deduplicated)");

    let norm_id = engine.register(AttributeNormalizationAgent);
    println!("  [{norm_id}] AttributeNormalizationAgent → Signals (normalized)");

    let cat_id = engine.register(CategoryInferenceAgent);
    println!("  [{cat_id}] CategoryInferenceAgent → Signals (categories)");

    let price_id = engine.register(PricingValidationAgent);
    println!("  [{price_id}] PricingValidationAgent → Signals (validated prices)");

    let schema_id = engine.register(SchemaInvariantAgent);
    println!("  [{schema_id}] SchemaInvariantAgent → Constraints (schema rules)");

    let ready_id = engine.register(ProductReadyAgent);
    println!("  [{ready_id}] ProductReadyAgent → Evaluations (ready products)");

    println!("\n  Total Agents: {}", engine.agent_count());

    println!("\n┌──────────────────────────────────────────────────────────────────────────────┐");
    println!("│ PHASE 3: INVARIANT REGISTRATION                                              │");
    println!("└──────────────────────────────────────────────────────────────────────────────┘");

    engine.register_invariant(RequireNoDuplicates);
    println!("\n  ✓ RequireNoDuplicates");

    engine.register_invariant(RequireRequiredAttributes);
    println!("  ✓ RequireRequiredAttributes");

    engine.register_invariant(RequireValidPrices);
    println!("  ✓ RequireValidPrices");

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
