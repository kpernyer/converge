variable "project_id" {
  description = "The GCP project ID"
  type        = string
}

variable "region" {
  description = "The region to deploy resources"
  type        = string
}

variable "service_name" {
  description = "The name of the Cloud Run service"
  type        = string
}

variable "image_url" {
  description = "The container image URL"
  type        = string
}

variable "snapshot_bucket" {
  description = "Name of the GCS bucket for snapshots"
  type        = string
}

variable "service_account_email" {
  description = "The service account email for Cloud Run"
  type        = string
}

variable "anthropic_secret_id" {
  description = "The ID of the Anthropic API key secret"
  type        = string
}

variable "openai_secret_id" {
  description = "The ID of the OpenAI API key secret"
  type        = string
}

variable "google_ai_secret_id" {
  description = "The ID of the Google AI API key secret"
  type        = string
}
