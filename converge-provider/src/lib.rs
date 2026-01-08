// Copyright 2024-2025 Aprio One AB, Sweden
// Author: Kenneth Pernyer, kenneth@aprio.one
// SPDX-License-Identifier: MIT
// See LICENSE file in the project root for full license information.

//! LLM provider implementations for Converge.
//!
//! This crate provides concrete LLM provider implementations that connect
//! to actual API services. The core `LlmProvider` trait is defined in
//! `converge-core`; this crate provides the implementations.
//!
//! # Available Providers
//!
//! - [`AnthropicProvider`] - Claude API (Anthropic)
//! - [`OpenAiProvider`] - GPT-4, GPT-3.5 (OpenAI)
//! - [`GeminiProvider`] - Gemini Pro (Google)
//! - [`PerplexityProvider`] - Perplexity AI
//! - [`QwenProvider`] - Qwen models (Alibaba Cloud)
//! - [`OpenRouterProvider`] - Multi-provider aggregator
//! - [`MinMaxProvider`] - MinMax AI
//! - [`GrokProvider`] - Grok (xAI)
//! - [`MistralProvider`] - Mistral AI
//! - [`DeepSeekProvider`] - DeepSeek AI
//! - [`BaiduProvider`] - Baidu ERNIE
//! - [`ZhipuProvider`] - Zhipu GLM
//! - [`KimiProvider`] - Kimi (Moonshot AI)
//! - [`ApertusProvider`] - Apertus (Switzerland, EU digital sovereignty)
//!
//! # Prompt Structuring
//!
//! This crate provides provider-specific prompt structuring and optimization:
//!
//! - [`ProviderPromptBuilder`]: Builds prompts optimized for specific providers
//! - [`StructuredResponseParser`]: Parses structured responses (XML/JSON)
//! - Helper functions: [`build_claude_prompt`], [`build_openai_prompt`]
//!
//! # Examples
//!
//! ## Using Anthropic (Claude)
//!
//! ```ignore
//! use converge_provider::{AnthropicProvider, build_claude_prompt, StructuredResponseParser};
//! use converge_core::llm::{LlmProvider, LlmRequest};
//! use converge_core::prompt::{AgentRole, OutputContract, PromptContext};
//! use converge_core::context::ContextKey;
//!
//! let provider = AnthropicProvider::from_env("claude-3-5-sonnet-20241022")?;
//!
//! // Build optimized prompt with XML structure
//! let prompt = build_claude_prompt(
//!     AgentRole::Proposer,
//!     "extract-competitors",
//!     PromptContext::new(),
//!     OutputContract::new("proposed-fact", ContextKey::Competitors),
//!     vec![],
//! );
//!
//! let response = provider.complete(&LlmRequest::new(prompt))?;
//!
//! // Parse structured XML response
//! let proposals = StructuredResponseParser::parse_claude_xml(
//!     &response,
//!     ContextKey::Competitors,
//!     "anthropic",
//! );
//! ```
//!
//! ## Using `OpenAI`
//!
//! ```ignore
//! use converge_provider::OpenAiProvider;
//! use converge_core::llm::{LlmProvider, LlmRequest};
//!
//! let provider = OpenAiProvider::from_env("gpt-4")?;
//! let response = provider.complete(&LlmRequest::new("Hello!"))?;
//! ```
//!
//! ## Using `OpenRouter` (Multi-Provider)
//!
//! ```ignore
//! use converge_provider::OpenRouterProvider;
//! use converge_core::llm::{LlmProvider, LlmRequest};
//!
//! // Access any provider through OpenRouter
//! let provider = OpenRouterProvider::from_env("anthropic/claude-3-opus")?;
//! let response = provider.complete(&LlmRequest::new("Hello!"))?;
//! ```

mod anthropic;
mod apertus;
mod baidu;
mod common;
mod deepseek;
mod factory;
mod gemini;
mod grok;
mod kimi;
mod minmax;
mod mistral;
mod model_selection;
mod openai;
mod openrouter;
mod perplexity;
mod prompt;
mod qwen;
mod zhipu;

pub use anthropic::AnthropicProvider;
pub use apertus::ApertusProvider;
pub use baidu::BaiduProvider;
pub use common::{
    ChatCompletionRequest, ChatCompletionResponse, ChatMessage, ChatUsage, HttpProviderConfig,
    OpenAiCompatibleProvider, OpenAiStyleError, OpenAiStyleErrorDetail,
    chat_response_to_llm_response, handle_openai_style_error, make_chat_completion_request,
    parse_finish_reason,
};
pub use deepseek::DeepSeekProvider;
pub use factory::{can_create_provider, create_provider};
pub use gemini::GeminiProvider;
pub use grok::GrokProvider;
pub use kimi::KimiProvider;
pub use minmax::MinMaxProvider;
pub use mistral::MistralProvider;
pub use model_selection::{ModelMetadata, ModelSelector, ProviderRegistry, is_provider_available};
pub use openai::OpenAiProvider;
pub use openrouter::OpenRouterProvider;
pub use perplexity::PerplexityProvider;
pub use prompt::{
    ProviderPromptBuilder, StructuredResponseParser, build_claude_prompt, build_openai_prompt,
};
pub use qwen::QwenProvider;
pub use zhipu::ZhipuProvider;