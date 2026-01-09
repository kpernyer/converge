output "gateway_url" {
  description = "The URL of the API Gateway"
  value       = "https://${google_api_gateway_gateway.default.default_hostname}"
}

output "gateway_id" {
  description = "The ID of the API Gateway"
  value       = google_api_gateway_gateway.default.gateway_id
}
