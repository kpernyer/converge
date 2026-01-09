variable "project_id" {
  description = "The GCP project ID"
  type        = string
}

variable "region" {
  description = "The region for the registry"
  type        = string
}

variable "service_account_email" {
  description = "Service account email for pull access"
  type        = string
}
