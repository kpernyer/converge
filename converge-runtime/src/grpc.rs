// Copyright 2024-2025 Aprio One AB, Sweden
// Author: Kenneth Pernyer, kenneth@aprio.one
// SPDX-License-Identifier: MIT
// See LICENSE file in the project root for full license information.

//! gRPC server implementation (prepared, not implemented yet).
//!
//! This module provides the structure for a future gRPC server using Tonic.
//! The actual implementation will be added when the gRPC feature is enabled.

#[cfg(feature = "grpc")]
use crate::config::GrpcConfig;
use crate::error::RuntimeError;

/// gRPC server for Converge Runtime (prepared, not implemented).
#[cfg(feature = "grpc")]
pub struct GrpcServer {
    config: GrpcConfig,
}

#[cfg(feature = "grpc")]
impl GrpcServer {
    /// Create a new gRPC server.
    pub fn new(config: GrpcConfig) -> Self {
        Self { config }
    }

    /// Start the gRPC server.
    ///
    /// # TODO
    ///
    /// - Define protobuf schema for Converge API
    /// - Implement service handlers
    /// - Add request/response conversion
    /// - Add authentication/authorization
    pub async fn start(self) -> Result<(), RuntimeError> {
        tracing::info!(addr = %self.config.bind, "gRPC server not yet implemented");
        
        // TODO: Implement gRPC server
        // use tonic::transport::Server;
        // 
        // Server::builder()
        //     .add_service(ConvergeServiceServer::new(ConvergeService::new()))
        //     .serve(self.config.bind)
        //     .await?;
        
        Ok(())
    }
}

#[cfg(not(feature = "grpc"))]
pub struct GrpcServer;

#[cfg(not(feature = "grpc"))]
impl GrpcServer {
    pub fn new(_config: ()) -> Self {
        Self
    }

    pub async fn start(self) -> Result<(), RuntimeError> {
        Ok(())
    }
}

