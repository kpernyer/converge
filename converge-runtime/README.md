# Converge Runtime

HTTP, gRPC, and TUI server for the Converge Agent OS.

## Features

- **HTTP Server** (Axum) - Fully implemented
  - REST API for job submission
  - Health and readiness endpoints
  - Structured logging and tracing

- **gRPC Server** (Tonic) - Prepared, not implemented
  - Structure in place for future implementation
  - Protobuf schema to be defined

- **TUI** (ratatui) - Prepared, not implemented
  - Structure in place for future terminal interface
  - Job monitoring and visualization to be added

## Building

```bash
# Build runtime
cargo build --release

# Run HTTP server
cargo run

# Run with tracing
RUST_LOG=info cargo run
```

## API Endpoints

- `GET /health` - Health check
- `GET /ready` - Readiness check
- `POST /api/v1/jobs` - Submit a job

## Configuration

Configuration is loaded from environment variables (to be expanded with config crate).

Default HTTP server binds to `0.0.0.0:8080`.

## Architecture

The runtime uses:
- **Tokio** for async I/O (HTTP server)
- **Rayon** (via `converge-core`) for parallel agent execution
- **Axum** for HTTP routing and middleware
- **tracing** for structured logging

See `docs/05-development/RAYON_TOKIO_INTEGRATION.md` for details on how Rayon and Tokio work together.

