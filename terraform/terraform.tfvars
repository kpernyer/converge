# Converge Infrastructure Configuration

project_id   = "hey-sh-production"
region       = "europe-west1"
environment  = "prod"
service_name = "converge-runtime"
image_tag    = "latest"

# Cloud Run resources
cpu           = "1"
memory        = "512Mi"
min_instances = 0
max_instances = 10

# Firestore (Europe multi-region)
firestore_location                 = "eur3"
enable_firestore_pitr              = true
enable_firestore_delete_protection = true

# Service Directory
service_directory_namespace = "converge"

# Optional features
enable_api_gateway = false
