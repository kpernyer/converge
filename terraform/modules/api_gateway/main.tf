# API Gateway for Converge Runtime
# Routes traffic from converge.hey.sh/api/* to Cloud Run

resource "google_api_gateway_api" "default" {
  provider = google-beta
  api_id   = "${var.service_name}-api"
  project  = var.project_id
}

resource "google_api_gateway_api_config" "default" {
  provider      = google-beta
  api           = google_api_gateway_api.default.api_id
  api_config_id = "${var.service_name}-config-${formatdate("YYYYMMDDhhmmss", timestamp())}"
  project       = var.project_id

  openapi_documents {
    document {
      path     = "openapi.yaml"
      contents = base64encode(templatefile("${path.module}/openapi.yaml.tpl", {
        cloud_run_url = var.cloud_run_url
        project_id    = var.project_id
      }))
    }
  }

  gateway_config {
    backend_config {
      google_service_account = var.service_account_email
    }
  }

  lifecycle {
    create_before_destroy = true
  }
}

resource "google_api_gateway_gateway" "default" {
  provider   = google-beta
  api_config = google_api_gateway_api_config.default.id
  gateway_id = "${var.service_name}-gateway"
  project    = var.project_id
  region     = var.region
}

# Allow unauthenticated access to API Gateway (auth handled by Cloud Run)
resource "google_api_gateway_gateway_iam_member" "public" {
  provider = google-beta
  project  = var.project_id
  region   = var.region
  gateway  = google_api_gateway_gateway.default.gateway_id
  role     = "roles/apigateway.viewer"
  member   = "allUsers"
}
