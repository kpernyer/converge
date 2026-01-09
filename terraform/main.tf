module "storage" {
  source     = "./modules/storage"
  project_id = var.project_id
  region     = var.region
}

module "cloud_run" {
  source          = "./modules/cloud_run"
  project_id      = var.project_id
  region          = var.region
  service_name    = var.service_name
  # This depends on the image being pushed to Artifact Registry
  image_url       = "gcr.io/${var.project_id}/${var.service_name}:latest"
  snapshot_bucket = module.storage.bucket_name
}
