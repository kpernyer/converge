# Artifact Registry for container images

resource "google_artifact_registry_repository" "converge" {
  project       = var.project_id
  location      = var.region
  repository_id = "converge"
  description   = "Converge runtime container images"
  format        = "DOCKER"

  cleanup_policies {
    id     = "keep-recent"
    action = "KEEP"

    most_recent_versions {
      keep_count = 10
    }
  }
}

# Grant Cloud Run service account pull access
resource "google_artifact_registry_repository_iam_member" "runtime_reader" {
  project    = var.project_id
  location   = var.region
  repository = google_artifact_registry_repository.converge.name
  role       = "roles/artifactregistry.reader"
  member     = "serviceAccount:${var.service_account_email}"
}
