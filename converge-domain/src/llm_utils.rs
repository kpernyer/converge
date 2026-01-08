// Copyright 2024-2025 Aprio One AB, Sweden
// Author: Kenneth Pernyer, kenneth@aprio.one
// SPDX-License-Identifier: MIT
// See LICENSE file in the project root for full license information.

//! Utilities for creating LLM-enabled agents with model selection.
//!
//! This module provides helpers for setting up LLM agents that follow
//! the Converge pattern for model selection based on agent requirements.

use converge_core::{
    ContextKey,
    llm::{LlmAgent, LlmAgentConfig, MockProvider, MockResponse, ResponseParser, SimpleParser},
    model_selection::{AgentRequirements, CostClass, ModelSelectorTrait},
    prompt::PromptFormat,
};
use converge_provider::{ProviderRegistry, create_provider};
use std::sync::Arc;

/// Creates an LLM agent with model selection.
///
/// This is the recommended way to create LLM agents:
/// 1. Specify requirements
/// 2. Select model based on requirements
/// 3. Create provider
/// 4. Create agent
///
/// # Errors
///
/// Returns error if model selection or provider creation fails.
pub fn create_llm_agent(
    name: impl Into<String>,
    system_prompt: impl Into<String>,
    prompt_template: impl Into<String>,
    target_key: ContextKey,
    dependencies: Vec<ContextKey>,
    requirements: AgentRequirements,
    registry: &ProviderRegistry,
) -> Result<LlmAgent, converge_core::llm::LlmError> {
    // Select model based on requirements
    let (provider_name, model_id) = registry.select(&requirements)?;

    // Create provider
    let provider = create_provider(&provider_name, &model_id)?;

    // Create agent config
    let config = LlmAgentConfig {
        system_prompt: system_prompt.into(),
        prompt_template: prompt_template.into(),
        prompt_format: PromptFormat::Edn,
        target_key,
        dependencies,
        default_confidence: 0.7,
        max_tokens: 1024,
        temperature: 0.7,
        requirements: Some(requirements),
    };

    let name_str = name.into();

    // Create parser
    let parser: Arc<dyn ResponseParser> = Arc::new(SimpleParser {
        id_prefix: format!("{}:", name_str.clone()),
        confidence: 0.7,
    });

    Ok(LlmAgent::new(name_str, provider, config).with_parser(parser))
}

/// Creates an LLM agent with a mock provider (for testing).
///
/// This bypasses model selection and uses a mock provider directly.
/// Returns both the agent and the mock provider so you can configure responses.
#[must_use]
pub fn create_mock_llm_agent(
    name: impl Into<String>,
    system_prompt: impl Into<String>,
    prompt_template: impl Into<String>,
    target_key: ContextKey,
    dependencies: Vec<ContextKey>,
    requirements: AgentRequirements,
    mock_responses: Vec<MockResponse>,
) -> (LlmAgent, Arc<MockProvider>) {
    // Create mock provider with responses
    let mock_provider = Arc::new(MockProvider::new(mock_responses));

    // Create agent config
    let config = LlmAgentConfig {
        system_prompt: system_prompt.into(),
        prompt_template: prompt_template.into(),
        prompt_format: PromptFormat::Edn,
        target_key,
        dependencies,
        default_confidence: 0.7,
        max_tokens: 1024,
        temperature: 0.7,
        requirements: Some(requirements),
    };

    let name_str = name.into();

    // Create parser
    let parser: Arc<dyn ResponseParser> = Arc::new(SimpleParser {
        id_prefix: format!("{}:", name_str.clone()),
        confidence: 0.7,
    });

    let agent = LlmAgent::new(name_str, mock_provider.clone(), config).with_parser(parser);
    (agent, mock_provider)
}

/// Common requirement presets for different agent types.
pub mod requirements {
    use super::{AgentRequirements, CostClass};

    /// Requirements for fast, high-volume agents (e.g., data extraction).
    #[must_use]
    pub fn fast_extraction() -> AgentRequirements {
        AgentRequirements::fast_cheap()
    }

    /// Requirements for analysis agents (e.g., market analysis, strategy synthesis).
    #[must_use]
    pub fn analysis() -> AgentRequirements {
        AgentRequirements::balanced().with_min_quality(0.75)
    }

    /// Requirements for deep research agents (e.g., competitor analysis, risk assessment).
    #[must_use]
    pub fn deep_research() -> AgentRequirements {
        AgentRequirements::deep_research()
    }

    /// Requirements for synthesis agents (e.g., strategy synthesis, consolidation).
    #[must_use]
    pub fn synthesis() -> AgentRequirements {
        AgentRequirements::new(CostClass::Medium, 10000, true).with_min_quality(0.8)
    }

    /// Requirements for validation agents (e.g., compliance checking, quality gates).
    #[must_use]
    pub fn validation() -> AgentRequirements {
        AgentRequirements::balanced().with_min_quality(0.85)
    }

    /// Requirements for categorization agents (e.g., category inference, classification).
    #[must_use]
    pub fn categorization() -> AgentRequirements {
        AgentRequirements::fast_cheap().with_min_quality(0.7)
    }
}
