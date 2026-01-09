//! GCP configuration

use serde::Deserialize;

/// GCP configuration loaded from environment or config file
#[derive(Debug, Clone, Deserialize)]
pub struct GcpConfig {
    /// GCP project ID
    #[serde(default = "default_project")]
    pub project_id: String,

    /// Firestore database ID (usually "(default)")
    #[serde(default = "default_database")]
    pub firestore_database: String,

    /// Service Directory namespace
    #[serde(default = "default_namespace")]
    pub service_directory_namespace: String,

    /// Service Directory region
    #[serde(default = "default_region")]
    pub service_directory_region: String,

    /// Current service name for registration
    #[serde(default = "default_service_name")]
    pub service_name: String,

    /// Current service version
    #[serde(default = "default_version")]
    pub service_version: String,
}

impl Default for GcpConfig {
    fn default() -> Self {
        Self {
            project_id: default_project(),
            firestore_database: default_database(),
            service_directory_namespace: default_namespace(),
            service_directory_region: default_region(),
            service_name: default_service_name(),
            service_version: default_version(),
        }
    }
}

impl GcpConfig {
    /// Load configuration from environment variables
    pub fn from_env() -> Self {
        Self {
            project_id: std::env::var("GCP_PROJECT_ID")
                .or_else(|_| std::env::var("GOOGLE_CLOUD_PROJECT"))
                .unwrap_or_else(|_| default_project()),
            firestore_database: std::env::var("FIRESTORE_DATABASE")
                .unwrap_or_else(|_| default_database()),
            service_directory_namespace: std::env::var("SERVICE_DIRECTORY_NAMESPACE")
                .unwrap_or_else(|_| default_namespace()),
            service_directory_region: std::env::var("SERVICE_DIRECTORY_REGION")
                .or_else(|_| std::env::var("CLOUD_RUN_REGION"))
                .unwrap_or_else(|_| default_region()),
            service_name: std::env::var("K_SERVICE")
                .or_else(|_| std::env::var("SERVICE_NAME"))
                .unwrap_or_else(|_| default_service_name()),
            service_version: std::env::var("K_REVISION")
                .or_else(|_| std::env::var("SERVICE_VERSION"))
                .unwrap_or_else(|_| default_version()),
        }
    }
}

fn default_project() -> String {
    "hey-sh-production".to_string()
}

fn default_database() -> String {
    "(default)".to_string()
}

fn default_namespace() -> String {
    "converge".to_string()
}

fn default_region() -> String {
    "europe-west1".to_string()
}

fn default_service_name() -> String {
    "converge-runtime".to_string()
}

fn default_version() -> String {
    "v1".to_string()
}
