// Copyright 2024-2025 Aprio One AB, Sweden
// Author: Kenneth Pernyer, kenneth@aprio.one
// SPDX-License-Identifier: MIT
// See LICENSE file in the project root for full license information.

//! Configuration management for Converge Runtime.

use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

/// Runtime configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// HTTP server configuration.
    pub http: HttpConfig,
}

#[cfg(feature = "grpc")]
impl Config {
    /// Get gRPC configuration (only available when grpc feature is enabled).
    pub fn grpc(&self) -> GrpcConfig {
        GrpcConfig::default()
    }
}

#[cfg(feature = "tui")]
impl Config {
    /// Get TUI configuration (only available when tui feature is enabled).
    pub fn tui(&self) -> TuiConfig {
        TuiConfig::default()
    }
}

/// HTTP server configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HttpConfig {
    /// Bind address for HTTP server.
    pub bind: SocketAddr,
    /// Maximum request body size (bytes).
    pub max_body_size: usize,
}

impl Default for HttpConfig {
    fn default() -> Self {
        Self {
            bind: "0.0.0.0:8080".parse().expect("valid default address"),
            max_body_size: 10 * 1024 * 1024, // 10 MB
        }
    }
}

/// gRPC server configuration (prepared, not implemented).
#[cfg(feature = "grpc")]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GrpcConfig {
    /// Bind address for gRPC server.
    pub bind: SocketAddr,
}

#[cfg(feature = "grpc")]
impl Default for GrpcConfig {
    fn default() -> Self {
        Self {
            bind: "0.0.0.0:50051".parse().expect("valid default address"),
        }
    }
}

/// TUI configuration (prepared, not implemented).
#[cfg(feature = "tui")]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TuiConfig {
    /// Enable TUI mode.
    pub enabled: bool,
}

impl Config {
    /// Load configuration from environment and files.
    ///
    /// Environment variables:
    /// - `HTTP_PORT` or `PORT` - HTTP server port (default: 8080)
    /// - `HTTP_BIND` - Full bind address (default: 0.0.0.0:8080)
    pub fn load() -> anyhow::Result<Self> {
        let http_bind = if let Ok(bind) = std::env::var("HTTP_BIND") {
            bind.parse()?
        } else if let Ok(port) = std::env::var("HTTP_PORT").or_else(|_| std::env::var("PORT")) {
            format!("0.0.0.0:{port}").parse()?
        } else {
            "0.0.0.0:8080".parse()?
        };

        Ok(Self {
            http: HttpConfig {
                bind: http_bind,
                ..HttpConfig::default()
            },
        })
    }
}
