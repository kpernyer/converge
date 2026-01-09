variable "project_id" {
  description = "The GCP project ID"
  type        = string
}

variable "region" {
  description = "The region to deploy resources"
  type        = string
  default     = "europe-west1"
}

variable "environment" {
  description = "Environment name (prod, staging, dev)"
  type        = string
  default     = "prod"
}

variable "service_name" {
  description = "The name of the Cloud Run service"
  type        = string
  default     = "converge-runtime"
}

variable "image_tag" {
  description = "The container image tag"
  type        = string
  default     = "latest"
}

# Cloud Run settings
variable "cpu" {
  description = "CPU allocation for Cloud Run"
  type        = string
  default     = "1"
}

variable "memory" {
  description = "Memory allocation for Cloud Run"
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

# Firestore settings
variable "firestore_location" {
  description = "Firestore location (multi-region recommended)"
  type        = string
  default     = "eur3" # Europe multi-region
}

variable "enable_firestore_pitr" {
  description = "Enable Firestore point-in-time recovery"
  type        = bool
  default     = true
}

variable "enable_firestore_delete_protection" {
  description = "Enable Firestore delete protection"
  type        = bool
  default     = true
}

# Service Directory settings
variable "service_directory_namespace" {
  description = "Service Directory namespace for service discovery"
  type        = string
  default     = "converge"
}

# Optional features
variable "enable_api_gateway" {
  description = "Enable API Gateway module"
  type        = bool
  default     = false
}
