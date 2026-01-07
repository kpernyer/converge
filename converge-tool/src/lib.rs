// Copyright 2024-2025 Aprio One AB, Sweden
// Author: Kenneth Pernyer, kenneth@aprio.one
// SPDX-License-Identifier: MIT
// See LICENSE file in the project root for full license information.

//! Development tools for Converge.
//!
//! This crate provides tooling for developing Converge applications:
//!
//! - [`gherkin`]: Gherkin spec validation (business sense, compilability, conventions)
//!
//! # Gherkin Validation
//!
//! The Gherkin validator uses LLMs to check specs for:
//!
//! 1. **Business Sense**: Does the spec describe a meaningful invariant?
//! 2. **Compilability**: Can this be translated to a Rust invariant?
//! 3. **Conventions**: Does it follow Converge's Gherkin patterns?
//!
//! # Example
//!
//! ```ignore
//! use converge_tool::gherkin::{GherkinValidator, ValidationConfig};
//! use converge_core::llm::MockProvider;
//! use std::sync::Arc;
//!
//! let provider = Arc::new(MockProvider::constant("Valid spec", 0.9));
//! let validator = GherkinValidator::new(provider, ValidationConfig::default());
//!
//! let result = validator.validate_file("specs/growth_strategy.feature")?;
//! println!("Valid: {}", result.is_valid);
//! ```

pub mod gherkin;

pub use gherkin::{GherkinValidator, SpecValidation, ValidationConfig, ValidationIssue};
