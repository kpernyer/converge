// Copyright 2024-2025 Aprio One AB, Sweden
// Author: Kenneth Pernyer, kenneth@aprio.one
// SPDX-License-Identifier: MIT
// See LICENSE file in the project root for full license information.

//! HTTP request handlers for Converge Runtime.

use axum::{
    Router,
    extract::Json,
    routing::{get, post},
};
use converge_core::{Context, ContextKey, Engine};
use converge_provider::AnthropicProvider;
use converge_tool::gherkin::{GherkinValidator, IssueCategory, Severity, ValidationConfig};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use strum::IntoEnumIterator;
use tokio::task;
use tracing::{info, info_span, warn};
use utoipa::ToSchema;

use crate::error::RuntimeError;

/// Request to create and run a job.
#[derive(Debug, Deserialize, ToSchema)]
pub struct JobRequest {
    /// Optional initial context data (for now, simplified).
    /// TODO: Replace with proper `RootIntent` when implemented.
    #[schema(example = json!({}))]
    pub context: Option<serde_json::Value>,
}

/// Response from a job execution.
#[derive(Debug, Serialize, ToSchema)]
pub struct JobResponse {
    /// Execution metadata.
    pub metadata: JobMetadata,
    /// Number of cycles executed.
    pub cycles: u32,
    /// Whether convergence was reached.
    pub converged: bool,
    /// Final context summary (simplified for now).
    pub context_summary: ContextSummary,
}

/// Simplified context summary for API responses.
#[derive(Debug, Serialize, ToSchema)]
pub struct ContextSummary {
    /// Number of facts by key.
    pub fact_counts: std::collections::HashMap<String, usize>,
    /// Context version.
    pub version: u64,
}

/// Execution metadata.
#[derive(Debug, Serialize, ToSchema)]
pub struct JobMetadata {
    /// Number of cycles executed.
    pub cycles: u32,
    /// Whether convergence was reached.
    pub converged: bool,
    /// Execution duration (milliseconds).
    pub duration_ms: u64,
}

/// Health check endpoint.
///
/// Returns "ok" if the server is running.
#[utoipa::path(
    get,
    path = "/health",
    tag = "health",
    responses(
        (status = 200, description = "Server is healthy", body = String)
    )
)]
pub async fn health() -> &'static str {
    "ok"
}

/// Readiness check endpoint.
///
/// Returns readiness status and service health.
#[utoipa::path(
    get,
    path = "/ready",
    tag = "health",
    responses(
        (status = 200, description = "Server is ready", body = serde_json::Value),
        (status = 503, description = "Server is not ready")
    )
)]
pub async fn ready() -> Result<Json<serde_json::Value>, RuntimeError> {
    // TODO: Check dependencies (SurrealDB, etc.) when added
    Ok(Json(serde_json::json!({
        "status": "ready",
        "services": {
            "engine": "ok"
        }
    })))
}

/// Handle job submission.
///
/// Submits a new job to the Converge engine and runs it until convergence.
#[utoipa::path(
    post,
    path = "/api/v1/jobs",
    tag = "jobs",
    request_body = JobRequest,
    responses(
        (status = 200, description = "Job completed successfully", body = JobResponse),
        (status = 400, description = "Invalid request", body = RuntimeError),
        (status = 422, description = "Invariant violation", body = RuntimeError),
        (status = 413, description = "Budget exhausted", body = RuntimeError),
        (status = 409, description = "Conflict detected", body = RuntimeError),
        (status = 500, description = "Internal server error", body = RuntimeError)
    )
)]
#[axum::debug_handler]
pub async fn handle_job(
    Json(request): Json<JobRequest>,
) -> Result<Json<JobResponse>, RuntimeError> {
    let _span = info_span!("handle_job");
    let _guard = _span.enter();
    info!("Received job request");

    let start = std::time::Instant::now();

    // Extract request data
    let context_data = request.context.clone();

    // Drop the span guard before await (it's not Send)
    drop(_guard);

    // Spawn blocking task to run engine (CPU-bound work)
    let result = task::spawn_blocking(move || {
        let mut engine = Engine::new();

        // TODO: Register agents based on request or configuration
        // For now, create a minimal engine

        // Create context from request or use empty
        // TODO: Properly deserialize RootIntent and create Context
        // For now, use empty context
        let _context_data = context_data;
        let context = Context::new();

        // Run engine until convergence
        engine.run(context)
    })
    .await
    .map_err(|e| RuntimeError::Config(format!("Task join error: {e}")))?
    .map_err(RuntimeError::Converge)?;

    let duration = start.elapsed();

    // Build context summary
    let fact_counts: std::collections::HashMap<String, usize> = ContextKey::iter()
        .map(|key| {
            let count = result.context.get(key).len();
            (format!("{key:?}"), count)
        })
        .collect();

    let context_summary = ContextSummary {
        fact_counts,
        version: result.context.version(),
    };

    info!(
        cycles = result.cycles,
        converged = result.converged,
        duration_ms = duration.as_millis(),
        "Job completed"
    );

    Ok(Json(JobResponse {
        metadata: JobMetadata {
            cycles: result.cycles,
            converged: result.converged,
            duration_ms: duration.as_millis() as u64,
        },
        cycles: result.cycles,
        converged: result.converged,
        context_summary,
    }))
}

/// Request to validate Converge Rules.
#[derive(Debug, Deserialize, ToSchema)]
pub struct ValidateRulesRequest {
    /// The Converge Rules content (Gherkin format).
    #[schema(example = "Feature: Test\n  Scenario: Example\n    When something\n    Then result")]
    pub content: String,
    /// Optional file name for reporting.
    #[schema(example = "rules.feature")]
    pub file_name: Option<String>,
    /// Whether to use LLM for deep validation.
    #[serde(default)]
    #[schema(example = false)]
    pub use_llm: bool,
}

/// A single validation issue.
#[derive(Debug, Serialize, ToSchema)]
pub struct ValidationIssueResponse {
    /// Location of the issue.
    pub location: String,
    /// Issue category.
    pub category: String,
    /// Severity level.
    pub severity: String,
    /// Issue message.
    pub message: String,
    /// Suggested fix.
    pub suggestion: Option<String>,
}

/// Response from rules validation.
#[derive(Debug, Serialize, ToSchema)]
pub struct ValidateRulesResponse {
    /// Whether the rules are valid.
    pub is_valid: bool,
    /// Number of scenarios checked.
    pub scenario_count: usize,
    /// Validation issues found.
    pub issues: Vec<ValidationIssueResponse>,
    /// Confidence score (0.0 - 1.0).
    pub confidence: f64,
}

/// Validate Converge Rules.
///
/// Validates Gherkin-format business rules for convention compliance,
/// compilability, and business sense.
#[utoipa::path(
    post,
    path = "/api/v1/validate-rules",
    tag = "validation",
    request_body = ValidateRulesRequest,
    responses(
        (status = 200, description = "Validation completed", body = ValidateRulesResponse),
        (status = 400, description = "Invalid request", body = RuntimeError),
        (status = 500, description = "Internal server error", body = RuntimeError)
    )
)]
#[axum::debug_handler]
pub async fn validate_rules(
    Json(request): Json<ValidateRulesRequest>,
) -> Result<Json<ValidateRulesResponse>, RuntimeError> {
    let _span = info_span!("validate_rules");
    let _guard = _span.enter();
    info!(use_llm = request.use_llm, "Validating Converge Rules");

    let content = request.content.clone();
    let file_name = request.file_name.clone().unwrap_or_else(|| "rules.feature".to_string());
    let use_llm = request.use_llm;

    // Drop the span guard before await
    drop(_guard);

    let result = task::spawn_blocking(move || {
        // Create validation config
        let config = ValidationConfig {
            check_business_sense: use_llm,
            check_compilability: use_llm,
            check_conventions: true,
            min_confidence: 0.7,
        };

        // Create provider if LLM validation is requested
        let provider: Arc<dyn converge_core::llm::LlmProvider> = if use_llm {
            match AnthropicProvider::from_env("claude-3-5-haiku-20241022") {
                Ok(p) => Arc::new(p),
                Err(e) => {
                    warn!("Failed to create LLM provider, falling back to convention-only: {e}");
                    Arc::new(converge_core::llm::MockProvider::new(vec![]))
                }
            }
        } else {
            // Use mock provider for convention-only validation
            Arc::new(converge_core::llm::MockProvider::new(vec![]))
        };

        let validator = GherkinValidator::new(provider, config);
        validator.validate(&content, &file_name)
    })
    .await
    .map_err(|e| RuntimeError::Config(format!("Task join error: {e}")))?
    .map_err(|e| RuntimeError::Config(format!("Validation error: {e}")))?;

    let issues: Vec<ValidationIssueResponse> = result
        .issues
        .into_iter()
        .map(|i| ValidationIssueResponse {
            location: i.location,
            category: match i.category {
                IssueCategory::BusinessSense => "business_sense",
                IssueCategory::Compilability => "compilability",
                IssueCategory::Convention => "convention",
                IssueCategory::Syntax => "syntax",
                IssueCategory::NotRelatedError => "internal_error",
            }
            .to_string(),
            severity: match i.severity {
                Severity::Info => "info",
                Severity::Warning => "warning",
                Severity::Error => "error",
            }
            .to_string(),
            message: i.message,
            suggestion: i.suggestion,
        })
        .collect();

    info!(
        is_valid = result.is_valid,
        issue_count = issues.len(),
        "Validation completed"
    );

    Ok(Json(ValidateRulesResponse {
        is_valid: result.is_valid,
        scenario_count: result.scenario_count,
        issues,
        confidence: result.confidence,
    }))
}

/// Build the HTTP router.
pub fn router() -> Router<()> {
    Router::new()
        .route("/health", get(health))
        .route("/ready", get(ready))
        .route("/api/v1/jobs", post(handle_job))
        .route("/api/v1/validate-rules", post(validate_rules))
}
