output "namespace_id" {
  description = "The Service Directory namespace ID"
  value       = google_service_directory_namespace.converge.id
}

output "namespace_name" {
  description = "The Service Directory namespace name"
  value       = google_service_directory_namespace.converge.namespace_id
}

output "runtime_service_id" {
  description = "The runtime service ID in Service Directory"
  value       = google_service_directory_service.runtime.id
}

output "runtime_service_name" {
  description = "The runtime service name for gRPC resolution"
  value       = "dns:///${google_service_directory_service.runtime.service_id}.${var.namespace}.internal"
}

output "grpc_target" {
  description = "The gRPC target URI for service resolution"
  value       = "google-c2p:///${var.namespace}/${google_service_directory_service.runtime.service_id}"
}
