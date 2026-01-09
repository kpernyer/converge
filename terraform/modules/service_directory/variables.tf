variable "project_id" {
  description = "The GCP project ID"
  type        = string
}

variable "region" {
  description = "The region for the namespace"
  type        = string
}

variable "namespace" {
  description = "The Service Directory namespace"
  type        = string
  default     = "converge"
}

variable "service_account_email" {
  description = "Service account email for service discovery access"
  type        = string
}

variable "service_version" {
  description = "Version of the service (for metadata)"
  type        = string
  default     = "v1"
}

variable "environment" {
  description = "Environment name (prod, staging, dev)"
  type        = string
  default     = "prod"
}
