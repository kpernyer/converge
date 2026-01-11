// Copyright 2024-2025 Aprio One AB, Sweden
// Author: Kenneth Pernyer, kenneth@aprio.one
// SPDX-License-Identifier: MIT
// See LICENSE file in the project root for full license information.

//! Application state for Converge Runtime.

#[cfg(feature = "gcp")]
use crate::db::Database;

/// Application state shared across handlers.
#[derive(Clone)]
pub struct AppState {
    /// Database connection (when gcp feature is enabled).
    #[cfg(feature = "gcp")]
    pub db: Option<Arc<Database>>,
}

impl AppState {
    /// Create new application state without database.
    pub fn new() -> Self {
        Self {
            #[cfg(feature = "gcp")]
            db: None,
        }
    }

    /// Create application state with database connection.
    #[cfg(feature = "gcp")]
    pub fn with_database(db: Database) -> Self {
        Self {
            db: Some(Arc::new(db)),
        }
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}
