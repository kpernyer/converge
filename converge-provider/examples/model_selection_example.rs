// Copyright 2024-2025 Aprio One AB, Sweden
// Author: Kenneth Pernyer, kenneth@aprio.one
// SPDX-License-Identifier: MIT
// See LICENSE file in the project root for full license information.

//! Example: Using requirements-based model selection.
//!
//! This example demonstrates how agents specify their requirements
//! and how the system automatically selects appropriate models.

use converge_core::{AgentRequirements, CostClass, ModelSelectorTrait};
use converge_provider::{ModelMetadata, ModelSelector};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a model selector with default models
    let selector = ModelSelector::new();

    // Example 1: Fast, cheap agent (many instances)
    println!("=== Fast, Cheap Agent ===");
    let fast_reqs = AgentRequirements::fast_cheap();
    println!("Requirements: {fast_reqs:?}");

    match selector.select(&fast_reqs) {
        Ok((provider, model)) => {
            println!("Selected: {provider} / {model}");
        }
        Err(e) => {
            println!("Error: {e}");
        }
    }

    // Example 2: Deep research agent
    println!("\n=== Deep Research Agent ===");
    let research_reqs = AgentRequirements::deep_research();
    println!("Requirements: {research_reqs:?}");

    match selector.select(&research_reqs) {
        Ok((provider, model)) => {
            println!("Selected: {provider} / {model}");
        }
        Err(e) => {
            println!("Error: {e}");
        }
    }

    // Example 3: Balanced agent with web search
    println!("\n=== Balanced Agent with Web Search ===");
    let balanced_reqs = AgentRequirements::balanced().with_web_search(true);
    println!("Requirements: {balanced_reqs:?}");

    match selector.select(&balanced_reqs) {
        Ok((provider, model)) => {
            println!("Selected: {provider} / {model}");
        }
        Err(e) => {
            println!("Error: {e}");
        }
    }

    // Example 4: Custom requirements
    println!("\n=== Custom Requirements ===");
    let custom_reqs = AgentRequirements::new(
        CostClass::Low,
        3000,
        true, // requires reasoning
    )
    .with_min_quality(0.85);
    println!("Requirements: {custom_reqs:?}");

    match selector.select(&custom_reqs) {
        Ok((provider, model)) => {
            println!("Selected: {provider} / {model}");
        }
        Err(e) => {
            println!("Error: {e}");
        }
    }

    // Example 5: List all satisfying models
    println!("\n=== All Models Satisfying Fast/Cheap ===");
    let satisfying = selector.list_satisfying(&fast_reqs);
    for model in satisfying {
        println!(
            "  {} / {} (cost: {:?}, latency: {}ms, quality: {:.2})",
            model.provider, model.model, model.cost_class, model.typical_latency_ms, model.quality
        );
    }

    // Example 6: Custom model registry
    println!("\n=== Custom Model Registry ===");
    let custom_selector = ModelSelector::empty()
        .with_model(ModelMetadata::new(
            "anthropic",
            "claude-3-5-haiku-20241022",
            CostClass::VeryLow,
            1500,
            0.75,
        ))
        .with_model(
            ModelMetadata::new(
                "anthropic",
                "claude-3-5-sonnet-20241022",
                CostClass::Low,
                3000,
                0.85,
            )
            .with_reasoning(true),
        );

    let reqs = AgentRequirements::new(CostClass::Low, 5000, false);
    match custom_selector.select(&reqs) {
        Ok((provider, model)) => {
            println!("Selected: {provider} / {model}");
        }
        Err(e) => {
            println!("Error: {e}");
        }
    }

    Ok(())
}
