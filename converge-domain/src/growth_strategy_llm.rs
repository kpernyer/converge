// Copyright 2024-2025 Aprio One AB, Sweden
// Author: Kenneth Pernyer, kenneth@aprio.one
// SPDX-License-Identifier: MIT
// See LICENSE file in the project root for full license information.

//! LLM-enabled Growth Strategy use case.
//!
//! This module demonstrates how to set up LLM agents with model selection
//! for the Growth Strategy use case. It follows the Converge pattern:
//!
//! 1. Agents specify requirements (cost, latency, capabilities)
//! 2. Model selector chooses appropriate models
//! 3. Providers are created from selected models
//! 4. LLM agents are instantiated with providers
//!
//! For testing, use `create_mock_llm_agent` from `llm_utils`.

use converge_core::{
    ContextKey, Engine,
    llm::{MockProvider, MockResponse},
};
use converge_provider::ProviderRegistry;
use std::sync::Arc;

use crate::llm_utils::{create_llm_agent, create_mock_llm_agent, requirements};

/// Sets up LLM-enabled Growth Strategy agents with model selection.
///
/// This function demonstrates the pattern:
/// 1. Create a provider registry (checks available API keys)
/// 2. Create LLM agents with appropriate requirements
/// 3. Register them with the engine
///
/// # Errors
///
/// Returns error if model selection or provider creation fails.
pub fn setup_llm_growth_strategy(
    engine: &mut Engine,
    registry: &ProviderRegistry,
) -> Result<(), converge_core::llm::LlmError> {
    // Market Signal Agent: Fast extraction of market signals
    let market_agent = create_llm_agent(
        "MarketSignalAgent",
        "You are a market analyst. Extract key market signals from the given context.",
        "Extract market signals from: {context}",
        ContextKey::Signals,
        vec![ContextKey::Seeds],
        requirements::fast_extraction(),
        registry,
    )?;
    engine.register(market_agent);

    // Competitor Agent: Analysis of competitor landscape (with web search for real-time data)
    // Note: Depends on Signals (after MarketSignalAgent runs) to get market context
    let mut competitor_reqs = requirements::analysis();
    competitor_reqs.requires_web_search = true; // Enable web search for competitor intelligence
    let competitor_agent = create_llm_agent(
        "CompetitorAgent",
        "You are a competitive intelligence analyst. Analyze competitor strategies and positioning. Use web search to find current information about competitors.",
        "Analyze competitors from: {context}",
        ContextKey::Competitors,
        vec![ContextKey::Signals], // Depends on Signals (validated market signals)
        competitor_reqs,
        registry,
    )?;
    engine.register(competitor_agent);

    // Strategy Agent: Deep synthesis of strategies
    // Depends on both Signals and Competitors for comprehensive strategy synthesis
    let strategy_agent = create_llm_agent(
        "StrategyAgent",
        "You are a strategic planner. Synthesize growth strategies from market signals and competitor analysis.",
        "Synthesize growth strategies from: {context}",
        ContextKey::Strategies,
        vec![ContextKey::Signals, ContextKey::Competitors], // Needs both signals and competitor intel
        requirements::synthesis(),
        registry,
    )?;
    engine.register(strategy_agent);

    // Evaluation Agent: Deep research for strategy evaluation
    let evaluation_agent = create_llm_agent(
        "EvaluationAgent",
        "You are a strategy evaluator. Evaluate growth strategies and provide rationale.",
        "Evaluate strategies from: {context}",
        ContextKey::Evaluations,
        vec![ContextKey::Strategies],
        requirements::deep_research(),
        registry,
    )?;
    engine.register(evaluation_agent);

    Ok(())
}

/// Sets up LLM-enabled Growth Strategy agents with mock providers (for testing).
///
/// Returns the mock providers so you can configure their responses.
#[must_use]
pub fn setup_mock_llm_growth_strategy(engine: &mut Engine) -> Vec<Arc<MockProvider>> {
    let mut providers = Vec::new();

    // Market Signal Agent
    let (agent, provider) = create_mock_llm_agent(
        "MarketSignalAgent",
        "You are a market analyst.",
        "Extract market signals: {context}",
        ContextKey::Signals,
        vec![ContextKey::Seeds],
        requirements::fast_extraction(),
        vec![MockResponse::success(
            "Market signal: Growing demand in Nordic B2B SaaS sector",
            0.8,
        )],
    );
    engine.register(agent);
    providers.push(provider);

    // Competitor Agent
    let (agent, provider) = create_mock_llm_agent(
        "CompetitorAgent",
        "You are a competitive intelligence analyst.",
        "Analyze competitors: {context}",
        ContextKey::Signals,
        vec![ContextKey::Seeds],
        requirements::analysis(),
        vec![MockResponse::success(
            "Competitor analysis: Major players focusing on enterprise segment",
            0.85,
        )],
    );
    engine.register(agent);
    providers.push(provider);

    // Strategy Agent
    let (agent, provider) = create_mock_llm_agent(
        "StrategyAgent",
        "You are a strategic planner.",
        "Synthesize strategies: {context}",
        ContextKey::Strategies,
        vec![ContextKey::Signals],
        requirements::synthesis(),
        vec![MockResponse::success(
            "Strategy 1: Expand into Nordic markets with localized offerings",
            0.9,
        )],
    );
    engine.register(agent);
    providers.push(provider);

    // Evaluation Agent
    let (agent, provider) = create_mock_llm_agent(
        "EvaluationAgent",
        "You are a strategy evaluator.",
        "Evaluate strategies: {context}",
        ContextKey::Evaluations,
        vec![ContextKey::Strategies],
        requirements::deep_research(),
        vec![MockResponse::success(
            "Evaluation: Strategy 1 shows high potential with moderate risk. Rationale: Strong market signals and competitive positioning.",
            0.88,
        )],
    );
    engine.register(agent);
    providers.push(provider);

    providers
}

#[cfg(test)]
mod tests {
    use super::*;
    use converge_core::agents::SeedAgent;
    use converge_core::Context;

    #[test]
    fn mock_llm_growth_strategy_converges() {
        let mut engine = Engine::new();
        engine.register(SeedAgent::new("market", "Nordic B2B SaaS"));

        let _providers = setup_mock_llm_growth_strategy(&mut engine);

        let result = engine.run(Context::new()).expect("should converge");

        assert!(result.converged);
        // LLM agents emit proposals to ContextKey::Proposals
        // At least the first agent (MarketSignalAgent) should run since it depends on Seeds
        let proposals = result.context.get(ContextKey::Proposals);
        assert!(
            !proposals.is_empty(),
            "At least one LLM agent should have produced proposals"
        );
    }

    #[test]
    fn llm_agents_use_appropriate_requirements() {
        // Verify that different agents use different requirements
        let reqs_fast = requirements::fast_extraction();
        let reqs_synthesis = requirements::synthesis();
        let reqs_deep = requirements::deep_research();

        // Fast extraction should be cheaper
        assert!(reqs_fast.max_cost_class < reqs_synthesis.max_cost_class);
        // Deep research should require reasoning
        assert!(reqs_deep.requires_reasoning);
        // Synthesis should have higher quality threshold
        assert!(reqs_synthesis.min_quality > reqs_fast.min_quality);
    }
}
