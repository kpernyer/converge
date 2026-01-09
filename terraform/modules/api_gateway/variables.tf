variable "project_id" {
  description = "The GCP project ID"
  type        = string
}

variable "region" {
  description = "The region to deploy resources"
  type        = string
}

variable "service_name" {
  description = "The name of the service"
  type        = string
}

variable "cloud_run_url" {
  description = "The URL of the Cloud Run service"
  type        = string
}

variable "service_account_email" {
  description = "Service account email for API Gateway"
  type        = string
}
