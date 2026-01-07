// Copyright 2024-2025 Aprio One AB, Sweden
// Author: Kenneth Pernyer, kenneth@aprio.one
// SPDX-License-Identifier: MIT
// See LICENSE file in the project root for full license information.

//! OpenAPI schema definitions for Converge Runtime.

use utoipa::OpenApi;

use crate::error::RuntimeErrorResponse;
use crate::handlers::{JobRequest, JobResponse, JobMetadata, ContextSummary};

/// OpenAPI schema for Converge Runtime API.
#[derive(OpenApi)]
#[openapi(
    paths(
        crate::handlers::health,
        crate::handlers::ready,
        crate::handlers::handle_job,
    ),
    components(schemas(
        JobRequest,
        JobResponse,
        JobMetadata,
        ContextSummary,
        RuntimeErrorResponse,
    )),
    tags(
        (name = "health", description = "Health check endpoints"),
        (name = "jobs", description = "Job execution endpoints"),
    ),
    info(
        title = "Converge Runtime API",
        description = "HTTP API for the Converge Agent OS",
        version = "0.1.0",
        contact(
            name = "Converge",
            url = "https://github.com/converge",
        ),
    ),
    servers(
        (url = "http://localhost:8080", description = "Local development server"),
    ),
)]
pub struct ApiDoc;

