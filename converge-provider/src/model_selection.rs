// Copyright 2024-2025 Aprio One AB, Sweden
// Author: Kenneth Pernyer, kenneth@aprio.one
// SPDX-License-Identifier: MIT
// See LICENSE file in the project root for full license information.

//! Model selection implementation with provider-specific metadata.
//!
//! This module provides concrete implementations of model selection
//! with hardcoded knowledge of all providers. The abstract interface
//! (`ModelSelectorTrait`, `AgentRequirements`) is in converge-core.

use converge_core::llm::LlmError;
use converge_core::{
    AgentRequirements, ComplianceLevel, CostClass, DataSovereignty, ModelSelectorTrait,
};

/// Breakdown of fitness score components.
#[derive(Debug, Clone, PartialEq)]
pub struct FitnessBreakdown {
    /// Cost efficiency score (0.0-1.0, higher = cheaper).
    /// VeryLow=1.0, Low=0.8, Medium=0.6, High=0.4, VeryHigh=0.2
    pub cost_score: f64,
    /// Latency efficiency score (0.0-1.0, higher = faster).
    /// Calculated as: 1.0 - (typical_latency / max_allowed_latency)
    pub latency_score: f64,
    /// Quality score (0.0-1.0, model's quality rating).
    pub quality_score: f64,
    /// Total weighted score: 40% cost + 30% latency + 30% quality.
    pub total: f64,
}

impl std::fmt::Display for FitnessBreakdown {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:.3} = 40%×cost({:.2}) + 30%×latency({:.2}) + 30%×quality({:.2})",
            self.total, self.cost_score, self.latency_score, self.quality_score
        )
    }
}

/// Result of model selection with detailed information.
#[derive(Debug, Clone)]
pub struct SelectionResult {
    /// The selected model's metadata.
    pub selected: ModelMetadata,
    /// Fitness breakdown for the selected model.
    pub fitness: FitnessBreakdown,
    /// All candidates that were considered, with their fitness scores.
    /// Sorted by fitness (best first).
    pub candidates: Vec<(ModelMetadata, FitnessBreakdown)>,
    /// Models that were rejected and why.
    pub rejected: Vec<(ModelMetadata, RejectionReason)>,
}

/// Reason why a model was rejected during selection.
#[derive(Debug, Clone, PartialEq)]
pub enum RejectionReason {
    /// Provider not available (no API key).
    ProviderUnavailable,
    /// Cost class exceeds budget.
    CostTooHigh { model_cost: CostClass, max_allowed: CostClass },
    /// Latency exceeds limit.
    LatencyTooHigh { model_latency_ms: u32, max_allowed_ms: u32 },
    /// Quality below threshold.
    QualityTooLow { model_quality: f64, min_required: f64 },
    /// Reasoning required but not supported.
    ReasoningRequired,
    /// Web search required but not supported.
    WebSearchRequired,
    /// Data sovereignty mismatch.
    DataSovereigntyMismatch { required: DataSovereignty, model_has: DataSovereignty },
    /// Compliance level mismatch.
    ComplianceMismatch { required: ComplianceLevel, model_has: ComplianceLevel },
    /// Multilingual required but not supported.
    MultilingualRequired,
}

impl std::fmt::Display for RejectionReason {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ProviderUnavailable => write!(f, "provider unavailable (no API key)"),
            Self::CostTooHigh { model_cost, max_allowed } => {
                write!(f, "cost {:?} exceeds max {:?}", model_cost, max_allowed)
            }
            Self::LatencyTooHigh { model_latency_ms, max_allowed_ms } => {
                write!(f, "latency {}ms exceeds max {}ms", model_latency_ms, max_allowed_ms)
            }
            Self::QualityTooLow { model_quality, min_required } => {
                write!(f, "quality {:.2} below min {:.2}", model_quality, min_required)
            }
            Self::ReasoningRequired => write!(f, "reasoning required but not supported"),
            Self::WebSearchRequired => write!(f, "web search required but not supported"),
            Self::DataSovereigntyMismatch { required, model_has } => {
                write!(f, "data sovereignty {:?} != required {:?}", model_has, required)
            }
            Self::ComplianceMismatch { required, model_has } => {
                write!(f, "compliance {:?} != required {:?}", model_has, required)
            }
            Self::MultilingualRequired => write!(f, "multilingual required but not supported"),
        }
    }
}

/// Model metadata for selection.
#[derive(Debug, Clone, PartialEq)]
pub struct ModelMetadata {
    /// Provider name (e.g., "anthropic", "openai").
    pub provider: String,
    /// Model identifier (e.g., "claude-3-5-haiku-20241022").
    pub model: String,
    /// Cost class of this model.
    pub cost_class: CostClass,
    /// Typical latency in milliseconds.
    pub typical_latency_ms: u32,
    /// Quality score (0.0-1.0).
    pub quality: f64,
    /// Whether this model has strong reasoning capabilities.
    pub has_reasoning: bool,
    /// Whether this model supports web search.
    pub supports_web_search: bool,
    /// Data sovereignty region.
    pub data_sovereignty: DataSovereignty,
    /// Compliance level.
    pub compliance: ComplianceLevel,
    /// Whether this model supports multiple languages.
    pub supports_multilingual: bool,
}

impl ModelMetadata {
    /// Creates new model metadata.
    #[must_use]
    pub fn new(
        provider: impl Into<String>,
        model: impl Into<String>,
        cost_class: CostClass,
        typical_latency_ms: u32,
        quality: f64,
    ) -> Self {
        Self {
            provider: provider.into(),
            model: model.into(),
            cost_class,
            typical_latency_ms,
            quality: quality.clamp(0.0, 1.0),
            has_reasoning: false,
            supports_web_search: false,
            data_sovereignty: DataSovereignty::Any,
            compliance: ComplianceLevel::None,
            supports_multilingual: false,
        }
    }

    /// Sets reasoning capability.
    #[must_use]
    pub fn with_reasoning(mut self, has: bool) -> Self {
        self.has_reasoning = has;
        self
    }

    /// Sets web search support.
    #[must_use]
    pub fn with_web_search(mut self, supports: bool) -> Self {
        self.supports_web_search = supports;
        self
    }

    /// Sets data sovereignty.
    #[must_use]
    pub fn with_data_sovereignty(mut self, sovereignty: DataSovereignty) -> Self {
        self.data_sovereignty = sovereignty;
        self
    }

    /// Sets compliance level.
    #[must_use]
    pub fn with_compliance(mut self, compliance: ComplianceLevel) -> Self {
        self.compliance = compliance;
        self
    }

    /// Sets multilingual support.
    #[must_use]
    pub fn with_multilingual(mut self, supports: bool) -> Self {
        self.supports_multilingual = supports;
        self
    }

    /// Checks if this model satisfies the given requirements.
    #[must_use]
    pub fn satisfies(&self, requirements: &AgentRequirements) -> bool {
        // Cost check
        if !requirements
            .max_cost_class
            .allowed_classes()
            .contains(&self.cost_class)
        {
            return false;
        }

        // Latency check
        if self.typical_latency_ms > requirements.max_latency_ms {
            return false;
        }

        // Reasoning check
        if requirements.requires_reasoning && !self.has_reasoning {
            return false;
        }

        // Web search check
        if requirements.requires_web_search && !self.supports_web_search {
            return false;
        }

        // Quality check
        if self.quality < requirements.min_quality {
            return false;
        }

        // Data sovereignty check
        if requirements.data_sovereignty != DataSovereignty::Any
            && self.data_sovereignty != requirements.data_sovereignty
        {
            return false;
        }

        // Compliance check
        if requirements.compliance != ComplianceLevel::None
            && self.compliance != requirements.compliance
        {
            return false;
        }

        // Multilingual check
        if requirements.requires_multilingual && !self.supports_multilingual {
            return false;
        }

        true
    }

    /// Calculates a fitness score for matching requirements.
    ///
    /// Higher score = better match. Considers:
    /// - Cost efficiency (lower cost within allowed range)
    /// - Latency efficiency (faster within allowed range)
    /// - Quality (higher is better)
    #[must_use]
    pub fn fitness_score(&self, requirements: &AgentRequirements) -> f64 {
        if !self.satisfies(requirements) {
            return 0.0;
        }

        // Cost efficiency: prefer lower cost (inverted, normalized)
        let cost_score = match self.cost_class {
            CostClass::VeryLow => 1.0,
            CostClass::Low => 0.8,
            CostClass::Medium => 0.6,
            CostClass::High => 0.4,
            CostClass::VeryHigh => 0.2,
        };

        // Latency efficiency: prefer faster (inverted, normalized)
        let latency_ratio =
            f64::from(self.typical_latency_ms) / f64::from(requirements.max_latency_ms);
        let latency_score = 1.0 - latency_ratio.min(1.0);

        // Quality score (already 0.0-1.0)
        let quality_score = self.quality;

        // Weighted combination
        // Cost: 40%, Latency: 30%, Quality: 30%
        0.4 * cost_score + 0.3 * latency_score + 0.3 * quality_score
    }

    /// Calculates a detailed fitness breakdown for matching requirements.
    ///
    /// Returns `None` if the model doesn't satisfy requirements.
    #[must_use]
    pub fn fitness_breakdown(&self, requirements: &AgentRequirements) -> Option<FitnessBreakdown> {
        if !self.satisfies(requirements) {
            return None;
        }

        let cost_score = match self.cost_class {
            CostClass::VeryLow => 1.0,
            CostClass::Low => 0.8,
            CostClass::Medium => 0.6,
            CostClass::High => 0.4,
            CostClass::VeryHigh => 0.2,
        };

        let latency_ratio =
            f64::from(self.typical_latency_ms) / f64::from(requirements.max_latency_ms);
        let latency_score = 1.0 - latency_ratio.min(1.0);

        let quality_score = self.quality;

        let total = 0.4 * cost_score + 0.3 * latency_score + 0.3 * quality_score;

        Some(FitnessBreakdown {
            cost_score,
            latency_score,
            quality_score,
            total,
        })
    }

    /// Determines why this model was rejected for the given requirements.
    ///
    /// Returns `None` if the model satisfies all requirements.
    #[must_use]
    pub fn rejection_reason(&self, requirements: &AgentRequirements) -> Option<RejectionReason> {
        // Cost check
        if !requirements
            .max_cost_class
            .allowed_classes()
            .contains(&self.cost_class)
        {
            return Some(RejectionReason::CostTooHigh {
                model_cost: self.cost_class,
                max_allowed: requirements.max_cost_class,
            });
        }

        // Latency check
        if self.typical_latency_ms > requirements.max_latency_ms {
            return Some(RejectionReason::LatencyTooHigh {
                model_latency_ms: self.typical_latency_ms,
                max_allowed_ms: requirements.max_latency_ms,
            });
        }

        // Reasoning check
        if requirements.requires_reasoning && !self.has_reasoning {
            return Some(RejectionReason::ReasoningRequired);
        }

        // Web search check
        if requirements.requires_web_search && !self.supports_web_search {
            return Some(RejectionReason::WebSearchRequired);
        }

        // Quality check
        if self.quality < requirements.min_quality {
            return Some(RejectionReason::QualityTooLow {
                model_quality: self.quality,
                min_required: requirements.min_quality,
            });
        }

        // Data sovereignty check
        if requirements.data_sovereignty != DataSovereignty::Any
            && self.data_sovereignty != requirements.data_sovereignty
        {
            return Some(RejectionReason::DataSovereigntyMismatch {
                required: requirements.data_sovereignty,
                model_has: self.data_sovereignty,
            });
        }

        // Compliance check
        if requirements.compliance != ComplianceLevel::None
            && self.compliance != requirements.compliance
        {
            return Some(RejectionReason::ComplianceMismatch {
                required: requirements.compliance,
                model_has: self.compliance,
            });
        }

        // Multilingual check
        if requirements.requires_multilingual && !self.supports_multilingual {
            return Some(RejectionReason::MultilingualRequired);
        }

        None
    }
}

/// Model selector that matches requirements to models.
#[derive(Debug, Clone)]
pub struct ModelSelector {
    /// Available models with metadata.
    models: Vec<ModelMetadata>,
}

impl ModelSelector {
    /// Creates a new model selector with default models.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates an empty selector (add models manually).
    #[must_use]
    pub fn empty() -> Self {
        Self { models: Vec::new() }
    }

    /// Adds a model to the selector.
    #[must_use]
    pub fn with_model(mut self, metadata: ModelMetadata) -> Self {
        self.models.push(metadata);
        self
    }

    /// Lists all models that satisfy the requirements.
    #[must_use]
    pub fn list_satisfying(&self, requirements: &AgentRequirements) -> Vec<&ModelMetadata> {
        self.models
            .iter()
            .filter(|m| m.satisfies(requirements))
            .collect()
    }
}

impl ModelSelectorTrait for ModelSelector {
    fn select(&self, requirements: &AgentRequirements) -> Result<(String, String), LlmError> {
        let mut candidates: Vec<(&ModelMetadata, f64)> = self
            .models
            .iter()
            .filter_map(|m| {
                if m.satisfies(requirements) {
                    Some((m, m.fitness_score(requirements)))
                } else {
                    None
                }
            })
            .collect();

        if candidates.is_empty() {
            return Err(LlmError::provider(format!(
                "No model found satisfying requirements: cost <= {:?}, latency <= {}ms, reasoning = {}, web_search = {}, quality >= {:.2}, data_sovereignty = {:?}, compliance = {:?}, multilingual = {}",
                requirements.max_cost_class,
                requirements.max_latency_ms,
                requirements.requires_reasoning,
                requirements.requires_web_search,
                requirements.min_quality,
                requirements.data_sovereignty,
                requirements.compliance,
                requirements.requires_multilingual
            )));
        }

        // Sort by fitness score (descending)
        candidates.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

        // Return best match
        let best = candidates[0].0;
        Ok((best.provider.clone(), best.model.clone()))
    }
}

impl Default for ModelSelector {
    fn default() -> Self {
        // Default models with realistic metadata
        Self {
            models: vec![
                // Anthropic
                ModelMetadata::new(
                    "anthropic",
                    "claude-3-5-haiku-20241022",
                    CostClass::VeryLow,
                    1500,
                    0.75,
                ),
                ModelMetadata::new(
                    "anthropic",
                    "claude-3-5-sonnet-20241022",
                    CostClass::Low,
                    3000,
                    0.85,
                )
                .with_reasoning(true),
                ModelMetadata::new(
                    "anthropic",
                    "claude-3-opus-20240229",
                    CostClass::High,
                    8000,
                    0.95,
                )
                .with_reasoning(true),
                // OpenAI
                ModelMetadata::new("openai", "gpt-3.5-turbo", CostClass::VeryLow, 1200, 0.70),
                ModelMetadata::new("openai", "gpt-4", CostClass::Medium, 5000, 0.90)
                    .with_reasoning(true),
                ModelMetadata::new("openai", "gpt-4-turbo", CostClass::Medium, 4000, 0.92)
                    .with_reasoning(true),
                // Google Gemini
                ModelMetadata::new("gemini", "gemini-pro", CostClass::Low, 2000, 0.80),
                ModelMetadata::new(
                    "gemini",
                    "gemini-2.0-flash-exp",
                    CostClass::VeryLow,
                    1000,
                    0.75,
                ),
                // Perplexity (web search)
                ModelMetadata::new(
                    "perplexity",
                    "pplx-70b-online",
                    CostClass::Medium,
                    4000,
                    0.90,
                )
                .with_reasoning(true)
                .with_web_search(true),
                ModelMetadata::new("perplexity", "pplx-7b-online", CostClass::Low, 2500, 0.75)
                    .with_web_search(true),
                // Qwen
                ModelMetadata::new("qwen", "qwen-turbo", CostClass::VeryLow, 1500, 0.70),
                ModelMetadata::new("qwen", "qwen-plus", CostClass::Low, 2500, 0.80),
                // OpenRouter (examples - actual models depend on routing)
                ModelMetadata::new(
                    "openrouter",
                    "anthropic/claude-3-5-haiku-20241022",
                    CostClass::VeryLow,
                    1500,
                    0.75,
                ),
                ModelMetadata::new("openrouter", "openai/gpt-4", CostClass::Medium, 5000, 0.90)
                    .with_reasoning(true),
                // MinMax
                ModelMetadata::new("minmax", "abab5.5-chat", CostClass::Low, 2000, 0.75),
                // Grok
                ModelMetadata::new("grok", "grok-beta", CostClass::Medium, 3000, 0.80),
                // Mistral
                ModelMetadata::new(
                    "mistral",
                    "mistral-large-latest",
                    CostClass::Low,
                    3000,
                    0.85,
                )
                .with_reasoning(true)
                .with_multilingual(true),
                ModelMetadata::new(
                    "mistral",
                    "mistral-medium-latest",
                    CostClass::Medium,
                    4000,
                    0.88,
                )
                .with_reasoning(true)
                .with_multilingual(true),
                // DeepSeek
                ModelMetadata::new("deepseek", "deepseek-chat", CostClass::VeryLow, 1500, 0.75)
                    .with_reasoning(true),
                ModelMetadata::new("deepseek", "deepseek-r1", CostClass::Low, 3000, 0.85)
                    .with_reasoning(true),
                // Baidu ERNIE (China)
                ModelMetadata::new("baidu", "ernie-bot", CostClass::Low, 2500, 0.80)
                    .with_data_sovereignty(DataSovereignty::China)
                    .with_multilingual(true),
                ModelMetadata::new("baidu", "ernie-bot-turbo", CostClass::VeryLow, 1500, 0.75)
                    .with_data_sovereignty(DataSovereignty::China)
                    .with_multilingual(true),
                // Zhipu GLM (China)
                ModelMetadata::new("zhipu", "glm-4", CostClass::Low, 2500, 0.82)
                    .with_data_sovereignty(DataSovereignty::China)
                    .with_multilingual(true),
                ModelMetadata::new("zhipu", "glm-4.5", CostClass::Medium, 3000, 0.88)
                    .with_data_sovereignty(DataSovereignty::China)
                    .with_reasoning(true)
                    .with_multilingual(true),
                // Kimi (Moonshot AI)
                ModelMetadata::new("kimi", "moonshot-v1-8k", CostClass::Low, 2000, 0.80)
                    .with_multilingual(true),
                ModelMetadata::new("kimi", "moonshot-v1-32k", CostClass::Medium, 3000, 0.85)
                    .with_reasoning(true)
                    .with_multilingual(true),
                // Apertus (Switzerland, EU digital sovereignty)
                ModelMetadata::new("apertus", "apertus-v1", CostClass::Medium, 4000, 0.85)
                    .with_data_sovereignty(DataSovereignty::Switzerland)
                    .with_compliance(ComplianceLevel::GDPR)
                    .with_multilingual(true),
            ],
        }
    }
}

/// Checks if a provider is available (has API key set).
///
/// Returns `true` if the environment variable for the provider is set.
#[must_use]
pub fn is_provider_available(provider: &str) -> bool {
    match provider {
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

/// Runtime provider registry that tracks available providers and allows
/// dynamic metadata updates.
///
/// This registry:
/// 1. Filters models by available providers (based on API keys)
/// 2. Allows dynamic updates to metadata (pricing, latency, etc.)
/// 3. Maintains requirements-based selection logic
#[derive(Debug, Clone)]
pub struct ProviderRegistry {
    /// Base selector with all models (static metadata).
    base_selector: ModelSelector,
    /// Available providers (checked at runtime).
    available_providers: std::collections::HashSet<String>,
    /// Dynamic metadata overrides (updates to pricing, latency, etc.).
    metadata_overrides: std::collections::HashMap<(String, String), ModelMetadata>,
}

impl ProviderRegistry {
    /// Creates a new registry that checks available providers from environment.
    ///
    /// Only providers with API keys set will be considered for selection.
    #[must_use]
    pub fn from_env() -> Self {
        let base_selector = ModelSelector::new();

        // Check all known providers
        let known_providers = vec![
            "anthropic",
            "openai",
            "gemini",
            "perplexity",
            "openrouter",
            "qwen",
            "minmax",
            "grok",
            "mistral",
            "deepseek",
            "baidu",
            "zhipu",
            "kimi",
            "apertus",
        ];

        let available_providers: std::collections::HashSet<String> = known_providers
            .into_iter()
            .filter(|p| is_provider_available(p))
            .map(std::string::ToString::to_string)
            .collect();

        Self {
            base_selector,
            available_providers,
            metadata_overrides: std::collections::HashMap::new(),
        }
    }

    /// Creates a registry with explicit provider availability.
    ///
    /// Use this when you want to control which providers are available
    /// programmatically (e.g., from a config file or user input).
    #[must_use]
    pub fn with_providers(providers: &[&str]) -> Self {
        let base_selector = ModelSelector::new();
        let available_providers: std::collections::HashSet<String> = providers
            .iter()
            .map(std::string::ToString::to_string)
            .collect();

        Self {
            base_selector,
            available_providers,
            metadata_overrides: std::collections::HashMap::new(),
        }
    }

    /// Updates metadata for a specific model (e.g., pricing, latency).
    ///
    /// This allows dynamic updates to model characteristics without
    /// rebuilding the entire registry.
    pub fn update_metadata(
        &mut self,
        provider: impl Into<String>,
        model: impl Into<String>,
        metadata: ModelMetadata,
    ) {
        self.metadata_overrides
            .insert((provider.into(), model.into()), metadata);
    }

    /// Lists all available models that satisfy the requirements.
    #[must_use]
    pub fn list_available(&self, requirements: &AgentRequirements) -> Vec<&ModelMetadata> {
        self.base_selector
            .list_satisfying(requirements)
            .into_iter()
            .filter(|m| self.available_providers.contains(&m.provider))
            .collect()
    }

    /// Gets the list of available providers.
    #[must_use]
    pub fn available_providers(&self) -> Vec<&str> {
        self.available_providers
            .iter()
            .map(std::string::String::as_str)
            .collect()
    }

    /// Checks if a provider is available.
    #[must_use]
    pub fn is_available(&self, provider: &str) -> bool {
        self.available_providers.contains(provider)
    }

    /// Selects the best model with detailed information about the selection process.
    ///
    /// Returns a `SelectionResult` containing:
    /// - The selected model and its fitness breakdown
    /// - All candidates that were considered (sorted by fitness)
    /// - Models that were rejected and why
    ///
    /// # Errors
    ///
    /// Returns error if no model satisfies the requirements.
    pub fn select_with_details(
        &self,
        requirements: &AgentRequirements,
    ) -> Result<SelectionResult, LlmError> {
        let mut candidates: Vec<(ModelMetadata, FitnessBreakdown)> = Vec::new();
        let mut rejected: Vec<(ModelMetadata, RejectionReason)> = Vec::new();

        // Process all models in the base selector
        for model in &self.base_selector.models {
            // Check provider availability first
            if !self.available_providers.contains(&model.provider) {
                rejected.push((model.clone(), RejectionReason::ProviderUnavailable));
                continue;
            }

            // Use override if available
            let metadata = self
                .metadata_overrides
                .get(&(model.provider.clone(), model.model.clone()))
                .unwrap_or(model);

            // Check if model satisfies requirements
            if let Some(breakdown) = metadata.fitness_breakdown(requirements) {
                candidates.push((metadata.clone(), breakdown));
            } else if let Some(reason) = metadata.rejection_reason(requirements) {
                rejected.push((metadata.clone(), reason));
            }
        }

        if candidates.is_empty() {
            let available = self
                .available_providers
                .iter()
                .map(std::string::String::as_str)
                .collect::<Vec<_>>()
                .join(", ");
            return Err(LlmError::provider(format!(
                "No available model found satisfying requirements. Available providers: [{}]",
                if available.is_empty() {
                    "none (set API keys)".to_string()
                } else {
                    available
                }
            )));
        }

        // Sort by fitness score (descending)
        candidates.sort_by(|a, b| {
            b.1.total
                .partial_cmp(&a.1.total)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        // Extract the best
        let (selected, fitness) = candidates[0].clone();

        Ok(SelectionResult {
            selected,
            fitness,
            candidates,
            rejected,
        })
    }
}

impl ModelSelectorTrait for ProviderRegistry {
    fn select(&self, requirements: &AgentRequirements) -> Result<(String, String), LlmError> {
        // Get all models that satisfy requirements
        let all_candidates = self.base_selector.list_satisfying(requirements);

        // Filter by available providers and apply overrides
        let mut candidates: Vec<(&ModelMetadata, f64)> = all_candidates
            .iter()
            .filter(|m| self.available_providers.contains(&m.provider))
            .map(|m| {
                // Use override if available, otherwise use base metadata
                let metadata = self
                    .metadata_overrides
                    .get(&(m.provider.clone(), m.model.clone()))
                    .unwrap_or(m);
                (metadata, metadata.fitness_score(requirements))
            })
            .collect();

        if candidates.is_empty() {
            let available = self
                .available_providers
                .iter()
                .map(std::string::String::as_str)
                .collect::<Vec<_>>()
                .join(", ");
            return Err(LlmError::provider(format!(
                "No available model found satisfying requirements. Available providers: [{}]",
                if available.is_empty() {
                    "none (set API keys)".to_string()
                } else {
                    available
                }
            )));
        }

        // Sort by fitness score (descending)
        candidates.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

        // Return best match
        let best = candidates[0].0;
        Ok((best.provider.clone(), best.model.clone()))
    }
}

impl Default for ProviderRegistry {
    fn default() -> Self {
        Self::from_env()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use converge_core::model_selection::CostClass;

    #[test]
    fn test_provider_availability_check() {
        // This test depends on environment, so we just check the function exists
        let _ = is_provider_available("anthropic");
    }

    #[test]
    fn test_registry_with_explicit_providers() {
        let registry = ProviderRegistry::with_providers(&["anthropic", "openai"]);
        assert!(registry.is_available("anthropic"));
        assert!(registry.is_available("openai"));
        assert!(!registry.is_available("gemini"));
    }

    #[test]
    fn test_metadata_override() {
        let mut registry = ProviderRegistry::with_providers(&["anthropic"]);

        // Override latency for a model
        let updated = ModelMetadata::new(
            "anthropic",
            "claude-3-5-haiku-20241022",
            CostClass::VeryLow,
            1000, // Updated latency
            0.75,
        );
        registry.update_metadata("anthropic", "claude-3-5-haiku-20241022", updated);

        let reqs = AgentRequirements::fast_cheap();
        let result = registry.select(&reqs);
        assert!(result.is_ok());
    }

    #[test]
    fn test_model_selection() {
        let selector = ModelSelector::new();
        let reqs = AgentRequirements::fast_cheap();

        let (provider, model) = selector.select(&reqs).unwrap();
        // Should select a VeryLow cost, fast model
        assert!(
            provider == "anthropic"
                || provider == "openai"
                || provider == "gemini"
                || provider == "qwen"
        );
        assert!(
            model.contains("haiku")
                || model.contains("flash")
                || model.contains("turbo")
                || model.contains("qwen")
        );
    }
}
