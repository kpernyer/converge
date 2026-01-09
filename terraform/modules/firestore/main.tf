# Firestore Database
# Serverless NoSQL for user management and runtime state

# Enable Firestore API
resource "google_project_service" "firestore" {
  project = var.project_id
  service = "firestore.googleapis.com"

  disable_on_destroy = false
}

# Create Firestore database in Native mode
resource "google_firestore_database" "default" {
  project     = var.project_id
  name        = "(default)"
  location_id = var.location
  type        = "FIRESTORE_NATIVE"

  # Point-in-time recovery (optional, 7 days)
  point_in_time_recovery_enablement = var.enable_pitr ? "POINT_IN_TIME_RECOVERY_ENABLED" : "POINT_IN_TIME_RECOVERY_DISABLED"

  # Delete protection
  delete_protection_state = var.delete_protection ? "DELETE_PROTECTION_ENABLED" : "DELETE_PROTECTION_DISABLED"

  depends_on = [google_project_service.firestore]
}

# Grant Cloud Run service account access
resource "google_project_iam_member" "firestore_user" {
  project = var.project_id
  role    = "roles/datastore.user"
  member  = "serviceAccount:${var.service_account_email}"
}

# Security rules (deployed via firebase CLI or separate resource)
# For now, we'll use default rules and update via Firebase Console

# Indexes for common queries
resource "google_firestore_index" "users_by_email" {
  project    = var.project_id
  database   = google_firestore_database.default.name
  collection = "users"

  fields {
    field_path = "email"
    order      = "ASCENDING"
  }

  fields {
    field_path = "created_at"
    order      = "DESCENDING"
  }
}

resource "google_firestore_index" "jobs_by_user" {
  project    = var.project_id
  database   = google_firestore_database.default.name
  collection = "jobs"

  fields {
    field_path = "user_id"
    order      = "ASCENDING"
  }

  fields {
    field_path = "created_at"
    order      = "DESCENDING"
  }
}

resource "google_firestore_index" "jobs_by_status" {
  project    = var.project_id
  database   = google_firestore_database.default.name
  collection = "jobs"

  fields {
    field_path = "status"
    order      = "ASCENDING"
  }

  fields {
    field_path = "updated_at"
    order      = "DESCENDING"
  }
}
