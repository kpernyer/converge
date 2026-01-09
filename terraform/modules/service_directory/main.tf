# Service Directory
# GCP's native service registry for gRPC discovery

# Enable Service Directory API
resource "google_project_service" "servicedirectory" {
  project = var.project_id
  service = "servicedirectory.googleapis.com"

  disable_on_destroy = false
}

# Create namespace for converge services
resource "google_service_directory_namespace" "converge" {
  provider     = google-beta
  project      = var.project_id
  namespace_id = var.namespace
  location     = var.region

  depends_on = [google_project_service.servicedirectory]
}

# Register the runtime service
resource "google_service_directory_service" "runtime" {
  provider   = google-beta
  namespace  = google_service_directory_namespace.converge.id
  service_id = "runtime"

  metadata = {
    version     = var.service_version
    environment = var.environment
    protocol    = "grpc"
  }
}

# Register an endpoint for the runtime service
resource "google_service_directory_endpoint" "runtime_endpoint" {
  provider    = google-beta
  service     = google_service_directory_service.runtime.id
  endpoint_id = "primary"

  metadata = {
    region = var.region
  }

  # For Cloud Run, we use the URL; for GKE/VMs, we'd use address/port
  # Cloud Run integration happens via annotations on the service
}

# Grant Cloud Run service account access to resolve services
resource "google_project_iam_member" "service_directory_viewer" {
  project = var.project_id
  role    = "roles/servicedirectory.viewer"
  member  = "serviceAccount:${var.service_account_email}"
}

# Grant ability to register/update services (for self-registration)
resource "google_project_iam_member" "service_directory_editor" {
  project = var.project_id
  role    = "roles/servicedirectory.editor"
  member  = "serviceAccount:${var.service_account_email}"
}
