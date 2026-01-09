# Converge Infrastructure
# Artifact Registry + Cloud Run + Firestore + Service Directory + Secret Manager

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

# Firestore database for user management and runtime state
module "firestore" {
  source                = "./modules/firestore"
  project_id            = var.project_id
  location              = var.firestore_location
  service_account_email = module.secrets.service_account_email
  enable_pitr           = var.enable_firestore_pitr
  delete_protection     = var.enable_firestore_delete_protection

  depends_on = [module.secrets]
}

# Service Directory for gRPC service discovery
module "service_directory" {
  source                = "./modules/service_directory"
  project_id            = var.project_id
  region                = var.region
  namespace             = var.service_directory_namespace
  service_account_email = module.secrets.service_account_email
  service_version       = var.image_tag
  environment           = var.environment

  depends_on = [module.secrets]
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

  depends_on = [module.secrets, module.artifact_registry, module.firestore]
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
