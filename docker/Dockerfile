# Stage 1: Planner
FROM rust:1.85-slim AS planner
WORKDIR /app
RUN cargo install cargo-chef
COPY . .
RUN cargo chef prepare --recipe-json recipe.json

# Stage 2: Cacher
FROM rust:1.85-slim AS cacher
WORKDIR /app
RUN cargo install cargo-chef
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-json recipe.json

# Stage 3: Builder
FROM rust:1.85-slim AS builder
WORKDIR /app
COPY . .
# Copy pre-built dependencies
COPY --from=cacher /app/target target
COPY --from=cacher /usr/local/cargo /usr/local/cargo
RUN cargo build --release -p converge-runtime

# Stage 4: Runtime
FROM gcr.io/distroless/cc-debian12 AS runtime
WORKDIR /app
COPY --from=builder /app/target/release/converge-runtime /app/converge-runtime
# Default configuration
ENV RUST_LOG=info
EXPOSE 8080
ENTRYPOINT ["/app/converge-runtime"]
