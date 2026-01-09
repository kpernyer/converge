# Cloud Run Module

This module defines the serverless compute environment for the Converge semantic runtime.

## Architecture Role
This provisions the "Phase 1: Stateless Execution" substrate. It hosts the `converge-runtime` container, which executes bounded convergence cycles in response to HTTP/gRPC requests.

## Configuration
- **Ingress**: Open to all traffic (configurable via Terraform).
- **Environment Variables**:
    - `SNAPSHOT_BUCKET`: Injected from the `storage` module to tell the runtime where to persist state.
- **Port**: Defaults to `8080`.

## Scaling
Managed automatically by Cloud Run (Scale-to-Zero supported), reinforcing the request-driven nature of the current execution model.
