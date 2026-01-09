variable "project_id" {
  description = "The GCP project ID"
  type        = string
}

variable "location" {
  description = "Firestore location (multi-region recommended)"
  type        = string
  default     = "eur3" # Europe multi-region
}

variable "service_account_email" {
  description = "Service account email for database access"
  type        = string
}

variable "enable_pitr" {
  description = "Enable point-in-time recovery"
  type        = bool
  default     = true
}

variable "delete_protection" {
  description = "Enable delete protection"
  type        = bool
  default     = true
}
