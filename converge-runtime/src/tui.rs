// Copyright 2024-2025 Aprio One AB, Sweden
// Author: Kenneth Pernyer, kenneth@aprio.one
// SPDX-License-Identifier: MIT
// See LICENSE file in the project root for full license information.

//! TUI (Terminal User Interface) implementation (prepared, not implemented yet).
//!
//! This module provides the structure for a future TUI using ratatui.
//! The actual implementation will be added when the TUI feature is enabled.

#[cfg(feature = "tui")]
use crate::config::TuiConfig;
use crate::error::RuntimeError;

/// TUI application for Converge Runtime (prepared, not implemented).
#[cfg(feature = "tui")]
pub struct TuiApp {
    config: TuiConfig,
}

#[cfg(feature = "tui")]
impl TuiApp {
    /// Create a new TUI application.
    pub fn new(config: TuiConfig) -> Self {
        Self { config }
    }

    /// Run the TUI application.
    ///
    /// # TODO
    ///
    /// - Implement terminal UI with ratatui
    /// - Add job submission interface
    /// - Add real-time job monitoring
    /// - Add context visualization
    /// - Add agent execution visualization
    pub async fn run(self) -> Result<(), RuntimeError> {
        if !self.config.enabled {
            return Ok(());
        }

        tracing::info!("TUI not yet implemented");

        // TODO: Implement TUI
        // use ratatui::prelude::*;
        // use crossterm::event::{self, Event, KeyCode};
        //
        // loop {
        //     // Render UI
        //     // Handle events
        //     // Update state
        // }

        Ok(())
    }
}

#[cfg(not(feature = "tui"))]
pub struct TuiApp;

#[cfg(not(feature = "tui"))]
impl TuiApp {
    pub fn new(_config: ()) -> Self {
        Self
    }

    pub async fn run(self) -> Result<(), RuntimeError> {
        Ok(())
    }
}
