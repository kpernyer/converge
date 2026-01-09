# Storage Module

This module provisions the Google Cloud Storage (GCS) resources required for Converge's snapshot persistence.

## Architecture Role
In Converge's "stateless semantic execution" model (Phase 1), the engine must externalize its state during HITL pauses or halts. This bucket serves as the durable storage for those context snapshots.

## Resources
- **Google Cloud Storage Bucket**: 
    - Naming convention: `converge-snapshots-${project_id}`.
    - **Versioning**: Enabled to allow auditability and recovery of previous context states.
    - **Lifecycle**: Auto-deletes snapshots older than 30 days to manage costs and data privacy.
    - **Access**: Uniform bucket-level access for security consistency.
