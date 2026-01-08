# Plan: Infrastructure Scaffolding

## Phase 1: Containerization
- [ ] Task: Create `docker/Dockerfile` using multi-stage build (cargo-chef + distroless) for `converge-runtime`.
- [ ] Task: Create `.dockerignore` to exclude target, git, and conductor artifacts.
- [ ] Task: Conductor - User Manual Verification 'Containerization' (Protocol in workflow.md)

## Phase 2: Terraform Setup
- [ ] Task: Create `terraform/main.tf`, `terraform/variables.tf`, and `terraform/outputs.tf` scaffolding.
- [ ] Task: Configure the `google` provider in `terraform/provider.tf`.
- [ ] Task: Create `terraform/modules/storage` to provision a GCS bucket for snapshots.
- [ ] Task: Conductor - User Manual Verification 'Terraform Setup' (Protocol in workflow.md)

## Phase 3: Cloud Run Deployment
- [ ] Task: Create `terraform/modules/cloud_run` to define the Cloud Run service resource.
- [ ] Task: Integrate `cloud_run` module into `main.tf`.
- [ ] Task: Conductor - User Manual Verification 'Cloud Run' (Protocol in workflow.md)

## Phase 4: GKE Placeholder
- [ ] Task: Create `terraform/modules/gke_placeholder` directory with a `README.md` explaining future usage.
- [ ] Task: Conductor - User Manual Verification 'Finalizing' (Protocol in workflow.md)
