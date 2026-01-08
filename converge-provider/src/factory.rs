// Copyright 2024-2025 Aprio One AB, Sweden
// Author: Kenneth Pernyer, kenneth@aprio.one
// SPDX-License-Identifier: MIT
// See LICENSE file in the project root for full license information.

//! Provider factory for creating providers from runtime selection.

use crate::{
    AnthropicProvider, ApertusProvider, BaiduProvider, DeepSeekProvider, GeminiProvider,
    GrokProvider, KimiProvider, MinMaxProvider, MistralProvider, OpenAiProvider,
    OpenRouterProvider, PerplexityProvider, QwenProvider, ZhipuProvider,
};
use converge_core::llm::{LlmError, LlmProvider};
use std::sync::Arc;

/// Creates a provider instance from provider name and model ID.
///
/// This factory is used after runtime model selection to instantiate
/// the actual provider.
///
/// # Errors
///
/// Returns error if:
/// - Provider name is unknown
/// - Required environment variables are not set
/// - Provider creation fails
pub fn create_provider(
    provider_name: &str,
    model_id: &str,
) -> Result<Arc<dyn LlmProvider>, LlmError> {
    match provider_name {
        "anthropic" => {
            let provider = AnthropicProvider::from_env(model_id)?;
            Ok(Arc::new(provider))
        }
        "openai" => {
            let provider = OpenAiProvider::from_env(model_id)?;
            Ok(Arc::new(provider))
        }
        "gemini" => {
            let provider = GeminiProvider::from_env(model_id)?;
            Ok(Arc::new(provider))
        }
        "perplexity" => {
            let provider = PerplexityProvider::from_env(model_id)?;
            Ok(Arc::new(provider))
        }
        "openrouter" => {
            let provider = OpenRouterProvider::from_env(model_id)?;
            Ok(Arc::new(provider))
        }
        "qwen" => {
            let provider = QwenProvider::from_env(model_id)?;
            Ok(Arc::new(provider))
        }
        "minmax" => {
            let provider = MinMaxProvider::from_env(model_id)?;
            Ok(Arc::new(provider))
        }
        "grok" => {
            let provider = GrokProvider::from_env(model_id)?;
            Ok(Arc::new(provider))
        }
        "mistral" => {
            let provider = MistralProvider::from_env(model_id)?;
            Ok(Arc::new(provider))
        }
        "deepseek" => {
            let provider = DeepSeekProvider::from_env(model_id)?;
            Ok(Arc::new(provider))
        }
        "baidu" => {
            let provider = BaiduProvider::from_env(model_id)?;
            Ok(Arc::new(provider))
        }
        "zhipu" => {
            let provider = ZhipuProvider::from_env(model_id)?;
            Ok(Arc::new(provider))
        }
        "kimi" => {
            let provider = KimiProvider::from_env(model_id)?;
            Ok(Arc::new(provider))
        }
        "apertus" => {
            let provider = ApertusProvider::from_env(model_id)?;
            Ok(Arc::new(provider))
        }
        _ => Err(LlmError::provider(format!(
            "Unknown provider: {provider_name}"
        ))),
    }
}

/// Checks if a provider can be created (has required API keys).
///
/// Returns `true` if the provider can be instantiated.
#[must_use]
pub fn can_create_provider(provider_name: &str) -> bool {
    match provider_name {
        "anthropic" => std::env::var("ANTHROPIC_API_KEY").is_ok(),
        "openai" => std::env::var("OPENAI_API_KEY").is_ok(),
        "gemini" => std::env::var("GEMINI_API_KEY").is_ok(),
        "perplexity" => std::env::var("PERPLEXITY_API_KEY").is_ok(),
        "openrouter" => std::env::var("OPENROUTER_API_KEY").is_ok(),
        "qwen" => std::env::var("QWEN_API_KEY").is_ok(),
        "minmax" => std::env::var("MINMAX_API_KEY").is_ok(),
        "grok" => std::env::var("GROK_API_KEY").is_ok(),
        "mistral" => std::env::var("MISTRAL_API_KEY").is_ok(),
        "deepseek" => std::env::var("DEEPSEEK_API_KEY").is_ok(),
        "baidu" => {
            std::env::var("BAIDU_API_KEY").is_ok() && std::env::var("BAIDU_SECRET_KEY").is_ok()
        }
        "zhipu" => std::env::var("ZHIPU_API_KEY").is_ok(),
        "kimi" => std::env::var("KIMI_API_KEY").is_ok(),
        "apertus" => std::env::var("APERTUS_API_KEY").is_ok(),
        _ => false,
    }
}
