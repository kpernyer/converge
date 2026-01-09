variable "project_id" {
  type = string
}

variable "region" {
  type = string
}

variable "service_name" {
  type = string
}

variable "image_url" {
  type = string
}

variable "snapshot_bucket" {
  type        = string
  description = "Name of the GCS bucket for snapshots"
}
