# Converge Infrastructure
# Artifact Registry + Cloud Run + Secret Manager

# Secrets for LLM API keys
module "secrets" {
  source     = "./modules/secrets"
  project_id = var.project_id
}

# Artifact Registry for container images
module "artifact_registry" {
  source                = "./modules/artifact_registry"
  project_id            = var.project_id
  region                = var.region
  service_account_email = module.secrets.service_account_email

  depends_on = [module.secrets]
}

# Storage for context snapshots
module "storage" {
  source     = "./modules/storage"
  project_id = var.project_id
  region     = var.region
}

# Cloud Run service
module "cloud_run" {
  source                = "./modules/cloud_run"
  project_id            = var.project_id
  region                = var.region
  service_name          = var.service_name
  image_url             = "${module.artifact_registry.repository_url}/runtime:${var.image_tag}"
  snapshot_bucket       = module.storage.bucket_name
  service_account_email = module.secrets.service_account_email
  secret_ids            = module.secrets.secret_ids
  cpu                   = var.cpu
  memory                = var.memory
  min_instances         = var.min_instances
  max_instances         = var.max_instances

  depends_on = [module.secrets, module.artifact_registry]
}

# API Gateway for routing and auth (optional)
module "api_gateway" {
  count = var.enable_api_gateway ? 1 : 0

  source                = "./modules/api_gateway"
  project_id            = var.project_id
  region                = var.region
  service_name          = var.service_name
  cloud_run_url         = module.cloud_run.service_url
  service_account_email = module.secrets.service_account_email

  depends_on = [module.cloud_run]
}
