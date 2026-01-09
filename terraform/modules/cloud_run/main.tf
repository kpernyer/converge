resource "google_cloud_run_v2_service" "default" {
  name     = var.service_name
  location = var.region
  ingress  = "INGRESS_TRAFFIC_ALL"

  template {
    service_account = var.service_account_email

    containers {
      image = var.image_url

      ports {
        container_port = 8080
      }

      resources {
        limits = {
          cpu    = var.cpu
          memory = var.memory
        }
      }

      # Environment variables
      env {
        name  = "SNAPSHOT_BUCKET"
        value = var.snapshot_bucket
      }

      env {
        name  = "RUST_LOG"
        value = "info"
      }

      # Secrets from Secret Manager
      dynamic "env" {
        for_each = var.secret_ids
        content {
          name = upper(env.value)
          value_source {
            secret_key_ref {
              secret  = env.value
              version = "latest"
            }
          }
        }
      }

      # Startup and liveness probes
      startup_probe {
        http_get {
          path = "/health"
        }
        initial_delay_seconds = 5
        timeout_seconds       = 3
        period_seconds        = 10
        failure_threshold     = 3
      }

      liveness_probe {
        http_get {
          path = "/health"
        }
        timeout_seconds   = 3
        period_seconds    = 30
        failure_threshold = 3
      }
    }

    scaling {
      min_instance_count = var.min_instances
      max_instance_count = var.max_instances
    }
  }

  traffic {
    type    = "TRAFFIC_TARGET_ALLOCATION_TYPE_LATEST"
    percent = 100
  }
}

# Allow unauthenticated access (auth handled by API Gateway)
resource "google_cloud_run_v2_service_iam_member" "public" {
  project  = var.project_id
  location = var.region
  name     = google_cloud_run_v2_service.default.name
  role     = "roles/run.invoker"
  member   = "allUsers"
}
