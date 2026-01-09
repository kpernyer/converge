output "database_name" {
  description = "The Firestore database name"
  value       = google_firestore_database.default.name
}

output "database_id" {
  description = "The Firestore database ID"
  value       = google_firestore_database.default.id
}

output "location" {
  description = "The Firestore location"
  value       = google_firestore_database.default.location_id
}
