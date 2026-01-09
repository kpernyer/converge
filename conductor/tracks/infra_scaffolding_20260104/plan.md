# Plan: Infrastructure Scaffolding

## Phase 1: Containerization [checkpoint: 0135faa]
- [x] Task: Create `docker/Dockerfile` using multi-stage build (cargo-chef + distroless) for `converge-runtime`. 0135faa
- [x] Task: Create `.dockerignore` to exclude target, git, and conductor artifacts. 0135faa
- [x] Task: Conductor - User Manual Verification 'Containerization' (Protocol in workflow.md)

## Phase 2: Terraform Setup [checkpoint: e67ad55]
- [x] Task: Create `terraform/main.tf`, `terraform/variables.tf`, and `terraform/outputs.tf` scaffolding. e67ad55
- [x] Task: Configure the `google` provider in `terraform/provider.tf`. e67ad55
- [x] Task: Create `terraform/modules/storage` to provision a GCS bucket for snapshots. e67ad55
- [x] Task: Conductor - User Manual Verification 'Terraform Setup' (Protocol in workflow.md)

## Phase 3: Cloud Run Deployment [checkpoint: 3de6ecf]
- [x] Task: Create `terraform/modules/cloud_run` to define the Cloud Run service resource. 3de6ecf
- [x] Task: Integrate `cloud_run` module into `main.tf`. 3de6ecf
- [x] Task: Conductor - User Manual Verification 'Cloud Run' (Protocol in workflow.md)

## Phase 4: GKE Placeholder & Documentation [checkpoint: d64770b]
- [x] Task: Create `terraform/modules/gke_placeholder` directory with a `README.md` explaining future usage. 3de6ecf
- [x] Task: Create `README.md` for `storage` and `cloud_run` modules. d64770b
- [x] Task: Conductor - User Manual Verification 'Finalizing' (Protocol in workflow.md)
