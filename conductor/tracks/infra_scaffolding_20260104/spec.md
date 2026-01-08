# Spec: Infrastructure Scaffolding (Cloud Run & Terraform)

## Overview
This track establishes the deployment foundation for Converge. It focuses on "Phase 1: Cloud Run" for stateless, request-driven execution while preparing the ground for "Phase 2: GKE". It uses Docker for reproducible builds and Terraform for declarative infrastructure management.

## Goals
1.  **Containerization:** Create a hardened, multi-stage `Dockerfile` for `converge-runtime` using `cargo-chef` and `distroless`.
2.  **Infrastructure as Code:** Set up a Terraform project to provision Google Cloud resources without direct `gcloud` calls.
3.  **Compute:** Provision a Cloud Run service for the semantic runtime.
4.  **Persistence:** Provision a Google Cloud Storage (GCS) bucket for storing Context snapshots (supporting the "feasible managed storage" requirement).
5.  **Future-Proofing:** Establish a placeholder structure for future GKE clusters.

## Technical Requirements
-   **Docker:**
    -   Use `lukemathwalker/cargo-chef` for dependency caching.
    -   Use `gcr.io/distroless/cc-debian12` for the runtime image.
    -   Target `converge-runtime` binary.
-   **Terraform:**
    -   Use the `google` provider.
    -   State should be local for now (or GCS if configured, but local is fine for scaffolding).
    -   Modules: `cloud_run`, `storage`.
-   **Cloud Run:**
    -   Service name: `converge-runtime`.
    -   Allow unauthenticated invocations (for prototype ease, or secured if preferred - will default to secure but allow override).
-   **Persistence:**
    -   GCS Bucket: `converge-snapshots-<project-id>`.

## Constraints
-   No shell scripts for deployment; use Terraform.
-   No manual infrastructure creation.
-   Keep GKE config empty/minimal for now.
