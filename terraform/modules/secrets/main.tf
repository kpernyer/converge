# Secret Manager for Converge Runtime
# Stores LLM provider API keys securely

# Enable Secret Manager API
resource "google_project_service" "secretmanager" {
  project = var.project_id
  service = "secretmanager.googleapis.com"

  disable_on_destroy = false
}

# Anthropic API Key
resource "google_secret_manager_secret" "anthropic_api_key" {
  project   = var.project_id
  secret_id = "anthropic-api-key"

  replication {
    auto {}
  }

  depends_on = [google_project_service.secretmanager]
}

# OpenAI API Key
resource "google_secret_manager_secret" "openai_api_key" {
  project   = var.project_id
  secret_id = "openai-api-key"

  replication {
    auto {}
  }

  depends_on = [google_project_service.secretmanager]
}

# Google AI API Key (for Gemini)
resource "google_secret_manager_secret" "google_ai_api_key" {
  project   = var.project_id
  secret_id = "google-ai-api-key"

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

# Grant secret accessor role
resource "google_secret_manager_secret_iam_member" "anthropic_accessor" {
  project   = var.project_id
  secret_id = google_secret_manager_secret.anthropic_api_key.secret_id
  role      = "roles/secretmanager.secretAccessor"
  member    = "serviceAccount:${google_service_account.runtime.email}"
}

resource "google_secret_manager_secret_iam_member" "openai_accessor" {
  project   = var.project_id
  secret_id = google_secret_manager_secret.openai_api_key.secret_id
  role      = "roles/secretmanager.secretAccessor"
  member    = "serviceAccount:${google_service_account.runtime.email}"
}

resource "google_secret_manager_secret_iam_member" "google_ai_accessor" {
  project   = var.project_id
  secret_id = google_secret_manager_secret.google_ai_api_key.secret_id
  role      = "roles/secretmanager.secretAccessor"
  member    = "serviceAccount:${google_service_account.runtime.email}"
}

# Grant Cloud Run invoker role to the service account
resource "google_project_iam_member" "run_invoker" {
  project = var.project_id
  role    = "roles/run.invoker"
  member  = "serviceAccount:${google_service_account.runtime.email}"
}
