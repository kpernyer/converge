# Secret Manager for Converge Runtime
# Stores LLM provider API keys securely

# Enable Secret Manager API
resource "google_project_service" "secretmanager" {
  project = var.project_id
  service = "secretmanager.googleapis.com"

  disable_on_destroy = false
}

locals {
  secrets = {
    anthropic  = "ANTHROPIC_API_KEY"
    openai     = "OPENAI_API_KEY"
    google_ai  = "GOOGLE_API_KEY"
    mistral    = "MISTRAL_API_KEY"
    deepseek   = "DEEPSEEK_API_KEY"
    openrouter = "OPENROUTER_API_KEY"
  }
}

# Create all secrets
resource "google_secret_manager_secret" "llm_keys" {
  for_each  = local.secrets
  project   = var.project_id
  secret_id = each.value

  replication {
    auto {}
  }

  depends_on = [google_project_service.secretmanager]
}

# Service account for Cloud Run to access secrets
resource "google_service_account" "runtime" {
  project      = var.project_id
  account_id   = "converge-runtime"
  display_name = "Converge Runtime Service Account"
}

# Grant secret accessor role for all secrets
resource "google_secret_manager_secret_iam_member" "accessor" {
  for_each  = local.secrets
  project   = var.project_id
  secret_id = google_secret_manager_secret.llm_keys[each.key].secret_id
  role      = "roles/secretmanager.secretAccessor"
  member    = "serviceAccount:${google_service_account.runtime.email}"
}

# Grant Cloud Run invoker role to the service account
resource "google_project_iam_member" "run_invoker" {
  project = var.project_id
  role    = "roles/run.invoker"
  member  = "serviceAccount:${google_service_account.runtime.email}"
}
