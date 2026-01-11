// Copyright 2024-2025 Aprio One AB, Sweden
// Author: Kenneth Pernyer, kenneth@aprio.one
// SPDX-License-Identifier: MIT
// See LICENSE file in the project root for full license information.

//! HTTP request handlers for Converge Runtime.

use axum::{
    Router,
    extract::{Json, Path, State},
    routing::{delete, get, post},
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
use crate::state::AppState;

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

// =============================================================================
// Firestore-backed Job API (when gcp feature is enabled)
// =============================================================================

/// Request to create a persisted job.
#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateJobRequest {
    /// User ID (will be authenticated in production).
    #[schema(example = "user-123")]
    pub user_id: String,
    /// Optional seed facts as JSON.
    #[schema(example = json!({"key": "value"}))]
    pub seeds: Option<serde_json::Value>,
}

/// Response with created job details.
#[derive(Debug, Serialize, ToSchema)]
pub struct CreateJobResponse {
    /// The created job ID.
    pub id: String,
    /// Job status.
    pub status: String,
    /// Created timestamp.
    pub created_at: String,
}

/// Response with job details.
#[derive(Debug, Serialize, ToSchema)]
pub struct GetJobResponse {
    /// Job ID.
    pub id: String,
    /// User ID.
    pub user_id: String,
    /// Job status.
    pub status: String,
    /// Number of cycles executed.
    pub cycles: u32,
    /// Seed facts.
    pub seeds: Option<serde_json::Value>,
    /// Final context (if converged).
    pub context: Option<serde_json::Value>,
    /// Error message (if failed).
    pub error: Option<String>,
    /// Created timestamp.
    pub created_at: String,
    /// Updated timestamp.
    pub updated_at: String,
}

/// Create a new job (persisted to Firestore).
///
/// Creates a job record in Firestore and returns the job ID.
#[utoipa::path(
    post,
    path = "/api/v1/store/jobs",
    tag = "store",
    request_body = CreateJobRequest,
    responses(
        (status = 201, description = "Job created successfully", body = CreateJobResponse),
        (status = 503, description = "Database not available", body = RuntimeError),
        (status = 500, description = "Internal server error", body = RuntimeError)
    )
)]
#[axum::debug_handler]
pub async fn create_job(
    State(state): State<AppState>,
    Json(request): Json<CreateJobRequest>,
) -> Result<(axum::http::StatusCode, Json<CreateJobResponse>), RuntimeError> {
    info!(user_id = %request.user_id, "Creating new job");

    #[cfg(feature = "gcp")]
    {
        use crate::db::{Job, JobStatus};

        let db = state.db.as_ref().ok_or_else(|| {
            RuntimeError::Config("Database not available".to_string())
        })?;

        // Create job record
        let mut job = Job::new(&request.user_id);
        job.seeds = request.seeds;

        let job_id = db.jobs.create(&job).await.map_err(|e| {
            RuntimeError::Config(format!("Failed to create job: {e}"))
        })?;

        info!(job_id = %job_id, "Job created in Firestore");

        Ok((
            axum::http::StatusCode::CREATED,
            Json(CreateJobResponse {
                id: job_id,
                status: format!("{:?}", JobStatus::Pending).to_lowercase(),
                created_at: job.created_at.to_rfc3339(),
            }),
        ))
    }

    #[cfg(not(feature = "gcp"))]
    {
        let _ = state;
        let _ = request;
        Err(RuntimeError::Config(
            "Firestore not available (compile with --features gcp)".to_string(),
        ))
    }
}

/// Get a job by ID.
///
/// Retrieves job details from Firestore.
#[utoipa::path(
    get,
    path = "/api/v1/store/jobs/{job_id}",
    tag = "store",
    params(
        ("job_id" = String, Path, description = "Job ID")
    ),
    responses(
        (status = 200, description = "Job found", body = GetJobResponse),
        (status = 404, description = "Job not found", body = RuntimeError),
        (status = 503, description = "Database not available", body = RuntimeError),
        (status = 500, description = "Internal server error", body = RuntimeError)
    )
)]
#[axum::debug_handler]
pub async fn get_job(
    State(state): State<AppState>,
    Path(job_id): Path<String>,
) -> Result<Json<GetJobResponse>, RuntimeError> {
    info!(job_id = %job_id, "Getting job");

    #[cfg(feature = "gcp")]
    {
        let db = state.db.as_ref().ok_or_else(|| {
            RuntimeError::Config("Database not available".to_string())
        })?;

        let job = db.jobs.get(&job_id).await.map_err(|e| {
            RuntimeError::Config(format!("Failed to get job: {e}"))
        })?;

        let job = job.ok_or_else(|| RuntimeError::NotFound(format!("Job {job_id} not found")))?;

        Ok(Json(GetJobResponse {
            id: job.id.unwrap_or_default(),
            user_id: job.user_id,
            status: format!("{:?}", job.status).to_lowercase(),
            cycles: job.cycles,
            seeds: job.seeds,
            context: job.context,
            error: job.error,
            created_at: job.created_at.to_rfc3339(),
            updated_at: job.updated_at.to_rfc3339(),
        }))
    }

    #[cfg(not(feature = "gcp"))]
    {
        let _ = state;
        let _ = job_id;
        Err(RuntimeError::Config(
            "Firestore not available (compile with --features gcp)".to_string(),
        ))
    }
}

/// Response from running a job.
#[derive(Debug, Serialize, ToSchema)]
pub struct RunJobResponse {
    /// Job ID.
    pub id: String,
    /// Final job status.
    pub status: String,
    /// Number of cycles executed.
    pub cycles: u32,
    /// Whether convergence was reached.
    pub converged: bool,
    /// Execution duration in milliseconds.
    pub duration_ms: Option<u64>,
    /// Error message (if failed).
    pub error: Option<String>,
}

/// Run a pending job.
///
/// Executes a pending job through the Converge engine and updates its status.
#[utoipa::path(
    post,
    path = "/api/v1/store/jobs/{job_id}/run",
    tag = "store",
    params(
        ("job_id" = String, Path, description = "Job ID to run")
    ),
    responses(
        (status = 200, description = "Job completed", body = RunJobResponse),
        (status = 404, description = "Job not found", body = RuntimeError),
        (status = 409, description = "Job not in pending state", body = RuntimeError),
        (status = 503, description = "Database not available", body = RuntimeError),
        (status = 500, description = "Internal server error", body = RuntimeError)
    )
)]
#[axum::debug_handler]
pub async fn run_job(
    State(state): State<AppState>,
    Path(job_id): Path<String>,
) -> Result<Json<RunJobResponse>, RuntimeError> {
    info!(job_id = %job_id, "Running job");

    #[cfg(feature = "gcp")]
    {
        use crate::db::JobStatus;

        let db = state.db.as_ref().ok_or_else(|| {
            RuntimeError::Config("Database not available".to_string())
        })?;

        // Get the job
        let mut job = db.jobs.get(&job_id).await.map_err(|e| {
            RuntimeError::Config(format!("Failed to get job: {e}"))
        })?.ok_or_else(|| RuntimeError::NotFound(format!("Job {job_id} not found")))?;

        // Check job is pending
        if job.status != JobStatus::Pending {
            return Err(RuntimeError::Conflict(format!(
                "Job {} is not pending (status: {:?})",
                job_id, job.status
            )));
        }

        // Mark as running
        job.start();
        db.jobs.update(&job_id, &job).await.map_err(|e| {
            RuntimeError::Config(format!("Failed to update job: {e}"))
        })?;

        info!(job_id = %job_id, "Job started");

        // Run the engine
        let seeds = job.seeds.clone();
        let max_cycles = job.max_cycles;

        let result = task::spawn_blocking(move || {
            use converge_core::Budget;

            let budget = Budget {
                max_cycles,
                ..Budget::default()
            };
            let mut engine = Engine::with_budget(budget);

            // TODO: Register agents based on job configuration
            // For now, create a minimal engine

            // Create context from seeds
            let _seeds = seeds;
            let context = Context::new();

            // Run engine
            engine.run(context)
        })
        .await
        .map_err(|e| RuntimeError::Config(format!("Task join error: {e}")))?;

        // Update job based on result
        match result {
            Ok(run_result) => {
                // Build context summary for storage
                let context_summary: std::collections::HashMap<String, usize> = ContextKey::iter()
                    .map(|key| {
                        let count = run_result.context.get(key).len();
                        (format!("{key:?}"), count)
                    })
                    .collect();

                job.complete(serde_json::to_value(&context_summary).unwrap_or_default(), run_result.cycles);
                db.jobs.update(&job_id, &job).await.map_err(|e| {
                    RuntimeError::Config(format!("Failed to update job: {e}"))
                })?;

                info!(job_id = %job_id, cycles = run_result.cycles, "Job converged");

                Ok(Json(RunJobResponse {
                    id: job_id,
                    status: "converged".to_string(),
                    cycles: run_result.cycles,
                    converged: run_result.converged,
                    duration_ms: job.duration_ms,
                    error: None,
                }))
            }
            Err(e) => {
                let error_msg = format!("{e}");
                job.fail(&error_msg);
                db.jobs.update(&job_id, &job).await.map_err(|e| {
                    RuntimeError::Config(format!("Failed to update job: {e}"))
                })?;

                warn!(job_id = %job_id, error = %error_msg, "Job failed");

                Ok(Json(RunJobResponse {
                    id: job_id,
                    status: "failed".to_string(),
                    cycles: job.cycles,
                    converged: false,
                    duration_ms: job.duration_ms,
                    error: Some(error_msg),
                }))
            }
        }
    }

    #[cfg(not(feature = "gcp"))]
    {
        let _ = state;
        let _ = job_id;
        Err(RuntimeError::Config(
            "Firestore not available (compile with --features gcp)".to_string(),
        ))
    }
}

/// Response from cancelling a job.
#[derive(Debug, Serialize, ToSchema)]
pub struct CancelJobResponse {
    /// Job ID.
    pub id: String,
    /// Final job status.
    pub status: String,
    /// Cancelled timestamp.
    pub cancelled_at: String,
}

/// Cancel a pending or running job.
///
/// Cancels a job that hasn't completed yet.
#[utoipa::path(
    post,
    path = "/api/v1/store/jobs/{job_id}/cancel",
    tag = "store",
    params(
        ("job_id" = String, Path, description = "Job ID to cancel")
    ),
    responses(
        (status = 200, description = "Job cancelled", body = CancelJobResponse),
        (status = 404, description = "Job not found", body = RuntimeError),
        (status = 409, description = "Job already completed", body = RuntimeError),
        (status = 503, description = "Database not available", body = RuntimeError),
        (status = 500, description = "Internal server error", body = RuntimeError)
    )
)]
#[axum::debug_handler]
pub async fn cancel_job(
    State(state): State<AppState>,
    Path(job_id): Path<String>,
) -> Result<Json<CancelJobResponse>, RuntimeError> {
    info!(job_id = %job_id, "Cancelling job");

    #[cfg(feature = "gcp")]
    {
        use crate::db::JobStatus;

        let db = state.db.as_ref().ok_or_else(|| {
            RuntimeError::Config("Database not available".to_string())
        })?;

        // Get the job
        let mut job = db.jobs.get(&job_id).await.map_err(|e| {
            RuntimeError::Config(format!("Failed to get job: {e}"))
        })?.ok_or_else(|| RuntimeError::NotFound(format!("Job {job_id} not found")))?;

        // Check job can be cancelled (pending or running)
        match job.status {
            JobStatus::Pending | JobStatus::Running => {}
            _ => {
                return Err(RuntimeError::Conflict(format!(
                    "Job {} cannot be cancelled (status: {:?})",
                    job_id, job.status
                )));
            }
        }

        // Cancel the job
        job.cancel();
        db.jobs.update(&job_id, &job).await.map_err(|e| {
            RuntimeError::Config(format!("Failed to update job: {e}"))
        })?;

        info!(job_id = %job_id, "Job cancelled");

        Ok(Json(CancelJobResponse {
            id: job_id,
            status: "cancelled".to_string(),
            cancelled_at: job.completed_at.map(|t| t.to_rfc3339()).unwrap_or_default(),
        }))
    }

    #[cfg(not(feature = "gcp"))]
    {
        let _ = state;
        let _ = job_id;
        Err(RuntimeError::Config(
            "Firestore not available (compile with --features gcp)".to_string(),
        ))
    }
}

/// Delete a job.
///
/// Permanently deletes a job from Firestore.
#[utoipa::path(
    delete,
    path = "/api/v1/store/jobs/{job_id}",
    tag = "store",
    params(
        ("job_id" = String, Path, description = "Job ID to delete")
    ),
    responses(
        (status = 204, description = "Job deleted"),
        (status = 404, description = "Job not found", body = RuntimeError),
        (status = 503, description = "Database not available", body = RuntimeError),
        (status = 500, description = "Internal server error", body = RuntimeError)
    )
)]
#[axum::debug_handler]
pub async fn delete_job(
    State(state): State<AppState>,
    Path(job_id): Path<String>,
) -> Result<axum::http::StatusCode, RuntimeError> {
    info!(job_id = %job_id, "Deleting job");

    #[cfg(feature = "gcp")]
    {
        let db = state.db.as_ref().ok_or_else(|| {
            RuntimeError::Config("Database not available".to_string())
        })?;

        // Check job exists
        let job = db.jobs.get(&job_id).await.map_err(|e| {
            RuntimeError::Config(format!("Failed to get job: {e}"))
        })?;

        if job.is_none() {
            return Err(RuntimeError::NotFound(format!("Job {job_id} not found")));
        }

        // Delete the job
        db.jobs.delete(&job_id).await.map_err(|e| {
            RuntimeError::Config(format!("Failed to delete job: {e}"))
        })?;

        info!(job_id = %job_id, "Job deleted");

        Ok(axum::http::StatusCode::NO_CONTENT)
    }

    #[cfg(not(feature = "gcp"))]
    {
        let _ = state;
        let _ = job_id;
        Err(RuntimeError::Config(
            "Firestore not available (compile with --features gcp)".to_string(),
        ))
    }
}

/// List jobs for a user.
///
/// Lists recent jobs for a user from Firestore.
#[utoipa::path(
    get,
    path = "/api/v1/store/users/{user_id}/jobs",
    tag = "store",
    params(
        ("user_id" = String, Path, description = "User ID")
    ),
    responses(
        (status = 200, description = "Jobs list", body = Vec<GetJobResponse>),
        (status = 503, description = "Database not available", body = RuntimeError),
        (status = 500, description = "Internal server error", body = RuntimeError)
    )
)]
#[axum::debug_handler]
pub async fn list_user_jobs(
    State(state): State<AppState>,
    Path(user_id): Path<String>,
) -> Result<Json<Vec<GetJobResponse>>, RuntimeError> {
    info!(user_id = %user_id, "Listing jobs for user");

    #[cfg(feature = "gcp")]
    {
        let db = state.db.as_ref().ok_or_else(|| {
            RuntimeError::Config("Database not available".to_string())
        })?;

        let jobs = db.jobs.list_by_user(&user_id, 50).await.map_err(|e| {
            RuntimeError::Config(format!("Failed to list jobs: {e}"))
        })?;

        let response: Vec<GetJobResponse> = jobs
            .into_iter()
            .map(|job| GetJobResponse {
                id: job.id.unwrap_or_default(),
                user_id: job.user_id,
                status: format!("{:?}", job.status).to_lowercase(),
                cycles: job.cycles,
                seeds: job.seeds,
                context: job.context,
                error: job.error,
                created_at: job.created_at.to_rfc3339(),
                updated_at: job.updated_at.to_rfc3339(),
            })
            .collect();

        Ok(Json(response))
    }

    #[cfg(not(feature = "gcp"))]
    {
        let _ = state;
        let _ = user_id;
        Err(RuntimeError::Config(
            "Firestore not available (compile with --features gcp)".to_string(),
        ))
    }
}

/// Build the HTTP router.
pub fn router(state: AppState) -> Router<()> {
    Router::new()
        .route("/health", get(health))
        .route("/ready", get(ready))
        .route("/api/v1/jobs", post(handle_job))
        .route("/api/v1/validate-rules", post(validate_rules))
        // Firestore-backed endpoints
        .route("/api/v1/store/jobs", post(create_job))
        .route("/api/v1/store/jobs/:job_id", get(get_job).delete(delete_job))
        .route("/api/v1/store/jobs/:job_id/run", post(run_job))
        .route("/api/v1/store/jobs/:job_id/cancel", post(cancel_job))
        .route("/api/v1/store/users/:user_id/jobs", get(list_user_jobs))
        .with_state(state)
}
