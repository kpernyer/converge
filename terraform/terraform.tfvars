# Converge Infrastructure Configuration

project_id   = "hey-sh-production"
region       = "europe-west1"
service_name = "converge-runtime"
image_tag    = "latest"

# Cloud Run resources
cpu           = "1"
memory        = "512Mi"
min_instances = 0
max_instances = 10

# API Gateway (disabled by default)
enable_api_gateway = false
