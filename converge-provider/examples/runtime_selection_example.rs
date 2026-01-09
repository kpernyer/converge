// Copyright 2024-2025 Aprio One AB, Sweden
// Author: Kenneth Pernyer, kenneth@aprio.one
// SPDX-License-Identifier: MIT
// See LICENSE file in the project root for full license information.

//! Example: Runtime model selection with API key availability.
//!
//! This example demonstrates how to select models at runtime
//! based on which providers have API keys available.

use converge_core::{AgentRequirements, ModelSelectorTrait};
use converge_provider::ProviderRegistry;
// Note: In a real application, you would use:
// use converge_provider::create_provider;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Runtime Model Selection Example ===\n");

    // Step 1: Create registry (checks environment for API keys)
    println!("1. Checking available providers from environment...");
    let registry = ProviderRegistry::from_env();
    let available = registry.available_providers();

    if available.is_empty() {
        println!("   ⚠️  No providers available (no API keys set)");
        println!("\n   To test this example, set at least one API key:");
        println!("   export ANTHROPIC_API_KEY='your-key'");
        println!("   export OPENAI_API_KEY='your-key'");
        return Ok(());
    }

    println!("   ✅ Available providers: {available:?}");

    // Step 2: Define requirements
    println!("\n2. Defining requirements...");
    let reqs = AgentRequirements::fast_cheap();
    println!("   Requirements: {reqs:?}");

    // Step 3: Select model (only from available providers)
    println!("\n3. Selecting model from available providers...");
    match registry.select(&reqs) {
        Ok((provider_name, model_id)) => {
            println!("   ✅ Selected: {provider_name} / {model_id}");

            // Step 4: Create provider instance
            println!("\n4. Creating provider instance...");
            println!("   (In real code, use: create_provider(&provider_name, &model_id))");
            println!("   Provider would be: {provider_name} / {model_id}");
        }
        Err(e) => {
            println!("   ❌ No suitable model: {e}");
        }
    }

    // Example: Explicit provider control
    println!("\n=== Explicit Provider Control ===");
    let explicit_registry = ProviderRegistry::with_providers(&["anthropic", "openai"]);
    println!(
        "Available providers: {:?}",
        explicit_registry.available_providers()
    );

    // Example: Dynamic metadata update
    println!("\n=== Dynamic Metadata Update ===");
    let mut registry_with_updates = ProviderRegistry::from_env();

    // Simulate a price change
    use converge_core::CostClass;
    use converge_provider::ModelMetadata;
    let updated = ModelMetadata::new(
        "anthropic",
        "claude-3-5-haiku-20241022",
        CostClass::Low, // Updated from VeryLow
        1500,
        0.75,
    );
    registry_with_updates.update_metadata("anthropic", "claude-3-5-haiku-20241022", updated);
    println!("Updated metadata for claude-3-5-haiku-20241022");

    Ok(())
}
