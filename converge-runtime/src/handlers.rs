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
use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;
use tokio::task;
use tracing::{info, info_span};
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

/// Build the HTTP router.
pub fn router() -> Router<()> {
    Router::new()
        .route("/health", get(health))
        .route("/ready", get(ready))
        .route("/api/v1/jobs", post(handle_job))
}
