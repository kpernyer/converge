# Build stage
FROM rust:1.85-slim-bookworm AS builder

WORKDIR /app

# Install build dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Copy workspace files
COPY Cargo.toml Cargo.lock ./
COPY converge-core ./converge-core
COPY converge-provider ./converge-provider
COPY converge-domain ./converge-domain
COPY converge-tool ./converge-tool
COPY converge-runtime ./converge-runtime

# Build release binary
RUN cargo build --release --package converge-runtime

# Runtime stage
FROM debian:bookworm-slim

WORKDIR /app

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Copy binary from builder
COPY --from=builder /app/target/release/converge-runtime /app/converge-runtime

# Create non-root user
RUN useradd -r -s /bin/false converge
USER converge

# Cloud Run expects port 8080
ENV PORT=8080
EXPOSE 8080

# Health check
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD curl -f http://localhost:8080/health || exit 1

ENTRYPOINT ["/app/converge-runtime"]
CMD ["serve"]
