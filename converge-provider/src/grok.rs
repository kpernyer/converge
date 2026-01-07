// Copyright 2024-2025 Aprio One AB, Sweden
// Author: Kenneth Pernyer, kenneth@aprio.one
// SPDX-License-Identifier: MIT
// See LICENSE file in the project root for full license information.

//! Grok (xAI) API provider.

use crate::common::{chat_response_to_llm_response, ChatCompletionRequest, ChatCompletionResponse, HttpProviderConfig, OpenAiCompatibleProvider};
use converge_core::llm::{LlmError, LlmProvider, LlmRequest, LlmResponse};
use serde::Deserialize;

/// Grok (xAI) API provider.
///
/// # Example
///
/// ```ignore
/// use converge_provider::GrokProvider;
/// use converge_core::llm::{LlmProvider, LlmRequest};
///
/// let provider = GrokProvider::new(
///     "your-api-key",
///     "grok-beta"
/// );
///
/// let request = LlmRequest::new("What is 2+2?");
/// let response = provider.complete(&request)?;
/// println!("{}", response.content);
/// ```
pub struct GrokProvider {
    config: HttpProviderConfig,
}

impl GrokProvider {
    /// Creates a new Grok provider.
    #[must_use]
    pub fn new(api_key: impl Into<String>, model: impl Into<String>) -> Self {
        Self {
            config: HttpProviderConfig::new(api_key, model, "https://api.x.ai"),
        }
    }

    /// Creates a provider using the `GROK_API_KEY` environment variable.
    ///
    /// # Errors
    ///
    /// Returns error if the environment variable is not set.
    pub fn from_env(model: impl Into<String>) -> Result<Self, LlmError> {
        let api_key = std::env::var("GROK_API_KEY")
            .map_err(|_| LlmError::auth("GROK_API_KEY environment variable not set"))?;
        Ok(Self::new(api_key, model))
    }

    /// Uses a custom base URL (for testing or proxies).
    #[must_use]
    pub fn with_base_url(mut self, url: impl Into<String>) -> Self {
        self.config.base_url = url.into();
        self
    }
}

impl OpenAiCompatibleProvider for GrokProvider {
    fn config(&self) -> &HttpProviderConfig {
        &self.config
    }

    fn endpoint(&self) -> &str {
        "/v1/chat/completions"
    }
}

impl LlmProvider for GrokProvider {
    fn name(&self) -> &'static str {
        "grok"
    }

    fn model(&self) -> &str {
        &self.config.model
    }

    fn complete(&self, request: &LlmRequest) -> Result<LlmResponse, LlmError> {
        // Grok has custom error handling
        let chat_request = ChatCompletionRequest::from_llm_request(
            self.config.model.clone(),
            request,
        );
        let url = format!("{}{}", self.config.base_url, self.endpoint());

        let http_response = self
            .config
            .client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.config.api_key))
            .header("Content-Type", "application/json")
            .json(&chat_request)
            .send()
            .map_err(|e| LlmError::network(format!("Request failed: {}", e)))?;

        let status = http_response.status();

        if !status.is_success() {
            #[derive(Deserialize)]
            struct GrokError {
                error: GrokErrorDetail,
            }
            #[derive(Deserialize)]
            struct GrokErrorDetail {
                message: String,
                #[serde(rename = "type")]
                error_type: Option<String>,
            }

            let error_body: GrokError = http_response
                .json()
                .map_err(|e| LlmError::parse(format!("Failed to parse error: {}", e)))?;

            let error_type = error_body.error.error_type.as_deref().unwrap_or("unknown");
            return match error_type {
                "invalid_request_error" | "authentication_error" => {
                    Err(LlmError::auth(error_body.error.message))
                }
                "rate_limit_error" => Err(LlmError::rate_limit(error_body.error.message)),
                _ => Err(LlmError::provider(error_body.error.message)),
            };
        }

        let api_response: ChatCompletionResponse = http_response
            .json()
            .map_err(|e| LlmError::parse(format!("Failed to parse response: {}", e)))?;

        chat_response_to_llm_response(api_response)
    }

    fn provenance(&self, request_id: &str) -> String {
        format!("grok:{}:{}", self.config.model, request_id)
    }
}
