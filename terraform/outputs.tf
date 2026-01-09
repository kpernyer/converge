output "cloud_run_url" {
  description = "The URL of the Cloud Run service"
  value       = module.cloud_run.service_url
}

output "api_gateway_url" {
  description = "The URL of the API Gateway"
  value       = var.enable_api_gateway ? module.api_gateway[0].gateway_url : null
}

output "artifact_registry_url" {
  description = "The Artifact Registry repository URL"
  value       = module.artifact_registry.repository_url
}

output "snapshot_bucket_name" {
  description = "The name of the GCS bucket for snapshots"
  value       = module.storage.bucket_name
}

output "service_account_email" {
  description = "The service account email for the runtime"
  value       = module.secrets.service_account_email
}
