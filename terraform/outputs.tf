output "cloud_run_url" {
  description = "The URL of the Cloud Run service"
  value       = module.cloud_run.service_url
}

output "snapshot_bucket_name" {
  description = "The name of the GCS bucket for snapshots"
  value       = module.storage.bucket_name
}
