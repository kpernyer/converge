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

# Firestore outputs
output "firestore_database" {
  description = "The Firestore database name"
  value       = module.firestore.database_name
}

output "firestore_location" {
  description = "The Firestore location"
  value       = module.firestore.location
}

# Service Directory outputs
output "service_directory_namespace" {
  description = "The Service Directory namespace"
  value       = module.service_directory.namespace_name
}

output "grpc_target" {
  description = "The gRPC target URI for service discovery"
  value       = module.service_directory.grpc_target
}
