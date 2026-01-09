variable "project_id" {
  description = "The GCP project ID"
  type        = string
}

variable "region" {
  description = "The region to deploy resources"
  type        = string
  default     = "europe-west1"
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

variable "enable_api_gateway" {
  description = "Enable API Gateway module"
  type        = bool
  default     = false
}
