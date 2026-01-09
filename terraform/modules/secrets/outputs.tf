output "service_account_email" {
  description = "The email of the runtime service account"
  value       = google_service_account.runtime.email
}

output "secret_ids" {
  description = "Map of secret IDs by provider name"
  value = {
    for k, v in google_secret_manager_secret.llm_keys : k => v.secret_id
  }
}

# Individual outputs for backwards compatibility
output "anthropic_secret_id" {
  description = "The ID of the Anthropic API key secret"
  value       = google_secret_manager_secret.llm_keys["anthropic"].secret_id
}

output "openai_secret_id" {
  description = "The ID of the OpenAI API key secret"
  value       = google_secret_manager_secret.llm_keys["openai"].secret_id
}

output "google_ai_secret_id" {
  description = "The ID of the Google AI API key secret"
  value       = google_secret_manager_secret.llm_keys["google_ai"].secret_id
}
