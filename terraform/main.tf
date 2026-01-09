# Converge Infrastructure
# Firebase Hosting + Cloud Run + API Gateway + Secret Manager

# Storage for context snapshots
module "storage" {
  source     = "./modules/storage"
  project_id = var.project_id
  region     = var.region
}

# Secrets for LLM API keys
module "secrets" {
  source     = "./modules/secrets"
  project_id = var.project_id
}

# Cloud Run service
module "cloud_run" {
  source                = "./modules/cloud_run"
  project_id            = var.project_id
  region                = var.region
  service_name          = var.service_name
  image_url             = "gcr.io/${var.project_id}/${var.service_name}:latest"
  snapshot_bucket       = module.storage.bucket_name
  service_account_email = module.secrets.service_account_email
  anthropic_secret_id   = module.secrets.anthropic_secret_id
  openai_secret_id      = module.secrets.openai_secret_id
  google_ai_secret_id   = module.secrets.google_ai_secret_id

  depends_on = [module.secrets]
}

# API Gateway for routing and auth
module "api_gateway" {
  source                = "./modules/api_gateway"
  project_id            = var.project_id
  region                = var.region
  service_name          = var.service_name
  cloud_run_url         = module.cloud_run.service_url
  service_account_email = module.secrets.service_account_email

  depends_on = [module.cloud_run]
}
