output "service_account_email" {
  description = "The email of the runtime service account"
  value       = google_service_account.runtime.email
}

output "anthropic_secret_id" {
  description = "The ID of the Anthropic API key secret"
  value       = google_secret_manager_secret.anthropic_api_key.secret_id
}

output "openai_secret_id" {
  description = "The ID of the OpenAI API key secret"
  value       = google_secret_manager_secret.openai_api_key.secret_id
}

output "google_ai_secret_id" {
  description = "The ID of the Google AI API key secret"
  value       = google_secret_manager_secret.google_ai_api_key.secret_id
}
