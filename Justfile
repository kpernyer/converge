# Converge development and deployment commands
# Install just: cargo install just

# Configuration
project := "hey-sh-production"
region := "europe-west1"
service := "converge-runtime"
registry := region + "-docker.pkg.dev/" + project + "/converge"

# Default: show available commands
default:
    @just --list

# ============================================
# Local Development (Docker Compose)
# ============================================

# Start local development environment (Firestore emulator)
dev-up:
    docker compose up -d firebase
    @echo "✓ Firestore emulator running at http://localhost:8080"
    @echo "✓ Emulator UI at http://localhost:4000"

# Start full local environment (emulator + runtime)
dev-up-full:
    docker compose --profile full up -d
    @echo "✓ Full development environment running"
    @echo "✓ Firestore: http://localhost:8080"
    @echo "✓ Runtime: http://localhost:3000"
    @echo "✓ Emulator UI: http://localhost:4000"

# Stop local development environment
dev-down:
    docker compose --profile full down

# View development logs
dev-logs:
    docker compose logs -f

# Run runtime locally with emulator
dev-run:
    FIRESTORE_EMULATOR_HOST=localhost:8080 \
    LOCAL_DEV=true \
    GCP_PROJECT_ID=demo-converge \
    RUST_LOG=debug \
    cargo run -p converge-runtime --features gcp

# Run runtime with hot reload (requires cargo-watch)
dev-watch:
    FIRESTORE_EMULATOR_HOST=localhost:8080 \
    LOCAL_DEV=true \
    GCP_PROJECT_ID=demo-converge \
    RUST_LOG=debug \
    cargo watch -x "run -p converge-runtime --features gcp"

# Clean up development volumes
dev-clean:
    docker compose --profile full down -v
    @echo "✓ Development volumes removed"

# ============================================
# Development Checks
# ============================================

# Run all pre-push checks (fmt, clippy, test, doc)
check: fmt-check clippy test doc
    @echo "✓ All checks passed!"

# Check formatting (CI equivalent)
fmt-check:
    cargo fmt --check

# Apply formatting fixes
fmt:
    cargo fmt

# Run clippy lints
clippy:
    cargo clippy --all-targets --all-features -- -D warnings

# Run tests
test:
    cargo test

# Build documentation
doc:
    cargo doc --no-deps

# Run verbose axiom tests
axioms:
    cargo test --package converge-core -- axioms --nocapture

# Quick check before push (fmt + clippy + doc)
pre-push: fmt-check clippy doc
    @echo "✓ Ready to push!"

# ============================================
# Docker
# ============================================

# Build docker image for amd64
docker-build:
    docker buildx build --platform linux/amd64 \
        -t {{registry}}/runtime:latest \
        -f converge-runtime/Dockerfile .

# Build and push docker image
docker-push:
    docker buildx build --platform linux/amd64 \
        -t {{registry}}/runtime:latest \
        -t {{registry}}/runtime:$(git rev-parse --short HEAD) \
        -f converge-runtime/Dockerfile --push .

# ============================================
# Terraform
# ============================================

# Initialize terraform
tf-init:
    cd terraform && terraform init

# Plan terraform changes
tf-plan:
    cd terraform && terraform plan

# Apply terraform changes
tf-apply:
    cd terraform && terraform apply

# Apply terraform changes (auto-approve)
tf-apply-auto:
    cd terraform && terraform apply -auto-approve

# Destroy terraform resources
tf-destroy:
    cd terraform && terraform destroy

# Show terraform outputs
tf-output:
    cd terraform && terraform output

# ============================================
# Deployment
# ============================================

# Deploy everything (docker + terraform)
deploy: docker-push tf-apply-auto
    @echo "✓ Deployment complete!"

# Quick deploy (just update Cloud Run)
deploy-quick: docker-push
    gcloud run services update {{service}} \
        --region={{region}} \
        --image={{registry}}/runtime:latest

# Show service status
status:
    @echo "Cloud Run Service:"
    @gcloud run services describe {{service}} --region={{region}} --format="value(status.url)"
    @echo ""
    @echo "Health check:"
    @curl -s $(gcloud run services describe {{service}} --region={{region}} --format="value(status.url)")/health

# View logs
logs:
    gcloud run services logs read {{service}} --region={{region}} --limit=50

# Stream logs
logs-stream:
    gcloud run services logs tail {{service}} --region={{region}}

# ============================================
# Secrets
# ============================================

# Set a secret value
secret-set name value:
    echo -n "{{value}}" | gcloud secrets versions add {{name}} --data-file=-

# List all secrets
secret-list:
    gcloud secrets list --format="table(name,createTime)"

# ============================================
# Website (converge.zone)
# ============================================

# Deploy website to Firebase
www-deploy:
    cd ../converge.hey.sh-www && bun run deploy

# Preview website locally
www-dev:
    cd ../converge.hey.sh-www && bun run dev
