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

variable "secret_ids" {
  description = "Map of secret IDs by provider name"
  type        = map(string)
}

variable "cpu" {
  description = "CPU allocation"
  type        = string
  default     = "1"
}

variable "memory" {
  description = "Memory allocation"
  type        = string
  default     = "512Mi"
}

variable "min_instances" {
  description = "Minimum number of instances"
  type        = number
  default     = 0
}

variable "max_instances" {
  description = "Maximum number of instances"
  type        = number
  default     = 10
}
