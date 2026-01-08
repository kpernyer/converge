// Copyright 2024-2025 Aprio One AB, Sweden
// Author: Kenneth Pernyer, kenneth@aprio.one
// SPDX-License-Identifier: MIT
// See LICENSE file in the project root for full license information.

//! Converge Runtime Server
//!
//! Provides HTTP, gRPC, and TUI interfaces for the Converge engine.

mod api;
mod config;
mod error;
mod grpc;
mod handlers;
mod http;
mod tui;

use anyhow::Result;
use tracing::info;
use tracing_subscriber::EnvFilter;

use config::Config;
use http::HttpServer;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .with_target(false)
        .init();

    info!("Starting Converge Runtime");

    // Load configuration
    let config = Config::load()?;
    info!(?config, "Configuration loaded");

    // Start HTTP server (always enabled)
    let http_server = HttpServer::new(config.http.clone());
    let http_handle = tokio::spawn(async move {
        if let Err(e) = http_server.start().await {
            tracing::error!(error = %e, "HTTP server failed");
        }
    });

    // TODO: Start gRPC server when grpc feature is enabled
    #[cfg(feature = "grpc")]
    {
        let grpc_config = config.grpc();
        let grpc_server = grpc::GrpcServer::new(grpc_config);
        let grpc_handle = tokio::spawn(async move {
            if let Err(e) = grpc_server.start().await {
                tracing::error!(error = %e, "gRPC server failed");
            }
        });
        tokio::select! {
            _ = http_handle => {},
            _ = grpc_handle => {},
        }
        return Ok(());
    }

    // TODO: Start TUI when tui feature is enabled
    #[cfg(feature = "tui")]
    {
        let tui_config = config.tui();
        let tui_app = tui::TuiApp::new(tui_config);
        let tui_handle = tokio::spawn(async move {
            if let Err(e) = tui_app.run().await {
                tracing::error!(error = %e, "TUI failed");
            }
        });
        tokio::select! {
            _ = http_handle => {},
            _ = tui_handle => {},
        }
        return Ok(());
    }

    // Default: just wait for HTTP server
    http_handle
        .await
        .map_err(|e| anyhow::anyhow!("HTTP server task failed: {e}"))?;

    Ok(())
}
