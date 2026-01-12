// Copyright 2024-2025 Aprio One AB, Sweden
// Author: Kenneth Pernyer, kenneth@aprio.one
// SPDX-License-Identifier: MIT
// See LICENSE file in the project root for full license information.

//! Gherkin spec validation for Converge.
//!
//! This module provides LLM-powered validation of Gherkin specifications
//! to ensure they:
//!
//! 1. Make business sense (semantic validity)
//! 2. Can be compiled to Rust invariants (technical feasibility)
//! 3. Follow Converge conventions (style compliance)
//!
//! # Converge Truths
//!
//! Converge uses "Truth" as a branded alias for "Feature" in Gherkin specs.
//! Both keywords are valid:
//!
//! ```gherkin
//! Truth: Get paid for delivered work    # Converge branded syntax
//! Feature: Get paid for delivered work  # Standard Gherkin syntax
//! ```
//!
//! The preprocessor automatically converts `Truth:` to `Feature:` before parsing.
//!
//! # Architecture
//!
//! ```text
//! .feature file → Preprocessor → Parser → Scenarios → LLM Validator → Report
//!                 (Truth→Feature)              │
//!                                              ├── Business sense check
//!                                              ├── Compilability check
//!                                              └── Convention check
//! ```

use converge_core::llm::{LlmProvider, LlmRequest};
use regex::Regex;
use std::path::Path;
use std::sync::Arc;

/// Preprocesses Converge Truth syntax to standard Gherkin.
///
/// Converts `Truth:` keyword to `Feature:` for parser compatibility.
/// This allows Converge specs to use the branded "Truth" terminology
/// while maintaining compatibility with standard Gherkin parsers.
///
/// # Examples
///
/// ```
/// use converge_tool::gherkin::preprocess_truths;
///
/// let input = "Truth: Get paid for delivered work\n  Scenario: Invoice";
/// let output = preprocess_truths(input);
/// assert!(output.starts_with("Feature:"));
/// ```
pub fn preprocess_truths(content: &str) -> String {
    // Match "Truth:" at the start of a line (with optional leading whitespace)
    let re = Regex::new(r"(?m)^(\s*)Truth:").unwrap();
    re.replace_all(content, "${1}Feature:").to_string()
}

/// Configuration for Gherkin validation.
#[derive(Debug, Clone)]
pub struct ValidationConfig {
    /// Whether to check business sense.
    pub check_business_sense: bool,
    /// Whether to check compilability to Rust.
    pub check_compilability: bool,
    /// Whether to check convention compliance.
    pub check_conventions: bool,
    /// Minimum confidence threshold for LLM assessments.
    pub min_confidence: f64,
}

impl Default for ValidationConfig {
    fn default() -> Self {
        Self {
            check_business_sense: true,
            check_compilability: true,
            check_conventions: true,
            min_confidence: 0.7,
        }
    }
}

/// Issue found during validation.
#[derive(Debug, Clone)]
pub struct ValidationIssue {
    /// The scenario or step that has the issue.
    pub location: String,
    /// Category of the issue.
    pub category: IssueCategory,
    /// Severity level.
    pub severity: Severity,
    /// Human-readable description.
    pub message: String,
    /// Suggested fix (if available).
    pub suggestion: Option<String>,
}

/// Category of validation issue.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IssueCategory {
    /// The spec doesn't make business sense.
    BusinessSense,
    /// The spec cannot be compiled to a Rust invariant.
    Compilability,
    /// The spec doesn't follow conventions.
    Convention,
    /// Syntax error in Gherkin.
    Syntax,
    /// Error not related to Gherkin validation (e.g., LLM API errors, network issues).
    NotRelatedError,
}

/// Severity of a validation issue.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Severity {
    /// Informational suggestion.
    Info,
    /// Warning - might cause problems.
    Warning,
    /// Error - must be fixed.
    Error,
}

/// Result of validating a Gherkin specification.
#[derive(Debug, Clone)]
pub struct SpecValidation {
    /// Whether the spec is valid overall.
    pub is_valid: bool,
    /// Path to the validated file.
    pub file_path: String,
    /// Number of scenarios validated.
    pub scenario_count: usize,
    /// Issues found during validation.
    pub issues: Vec<ValidationIssue>,
    /// Overall confidence score (0.0 - 1.0).
    pub confidence: f64,
}

impl SpecValidation {
    /// Returns true if there are any errors.
    #[must_use]
    pub fn has_errors(&self) -> bool {
        self.issues.iter().any(|i| i.severity == Severity::Error)
    }

    /// Returns true if there are any warnings.
    #[must_use]
    pub fn has_warnings(&self) -> bool {
        self.issues.iter().any(|i| i.severity == Severity::Warning)
    }

    /// Returns a summary string.
    #[must_use]
    pub fn summary(&self) -> String {
        let errors = self
            .issues
            .iter()
            .filter(|i| i.severity == Severity::Error)
            .count();
        let warnings = self
            .issues
            .iter()
            .filter(|i| i.severity == Severity::Warning)
            .count();

        if self.is_valid {
            format!(
                "✓ {} validated ({} scenarios, {} warnings)",
                self.file_path, self.scenario_count, warnings
            )
        } else {
            format!(
                "✗ {} invalid ({} errors, {} warnings)",
                self.file_path, errors, warnings
            )
        }
    }
}

/// LLM-powered Gherkin specification validator.
pub struct GherkinValidator {
    provider: Arc<dyn LlmProvider>,
    config: ValidationConfig,
}

impl GherkinValidator {
    /// Creates a new validator with the given LLM provider.
    #[must_use]
    pub fn new(provider: Arc<dyn LlmProvider>, config: ValidationConfig) -> Self {
        Self { provider, config }
    }

    /// Validates a Gherkin specification from a string.
    ///
    /// Supports both standard Gherkin (`Feature:`) and Converge Truth (`Truth:`) syntax.
    ///
    /// # Errors
    ///
    /// Returns error if the specification cannot be parsed or validated.
    /// LLM API errors are wrapped as `ValidationError::LlmError` with "`NOT_RELATED_ERROR`:" prefix
    /// to distinguish them from Gherkin validation issues.
    pub fn validate(
        &self,
        content: &str,
        file_name: &str,
    ) -> Result<SpecValidation, ValidationError> {
        // Preprocess: convert Truth: to Feature: for parser compatibility
        let processed = preprocess_truths(content);

        // Parse the Gherkin content
        // Syntax errors are Gherkin validation issues
        let feature = gherkin::Feature::parse(&processed, gherkin::GherkinEnv::default())
            .map_err(|e| ValidationError::ParseError(format!("{e}")))?;

        let mut issues = Vec::new();
        let scenario_count = feature.scenarios.len();

        // Validate each scenario
        for scenario in &feature.scenarios {
            let scenario_issues = self.validate_scenario(&feature, scenario)?;
            issues.extend(scenario_issues);
        }

        // Check overall feature structure
        let feature_issues = self.validate_feature(&feature)?;
        issues.extend(feature_issues);

        let has_errors = issues.iter().any(|i| i.severity == Severity::Error);
        let confidence = if issues.is_empty() { 1.0 } else { 0.7 };

        Ok(SpecValidation {
            is_valid: !has_errors,
            file_path: file_name.to_string(),
            scenario_count,
            issues,
            confidence,
        })
    }

    /// Validates a Gherkin specification from a file.
    ///
    /// # Errors
    ///
    /// Returns error if the file cannot be read or validated.
    pub fn validate_file(&self, path: impl AsRef<Path>) -> Result<SpecValidation, ValidationError> {
        let path = path.as_ref();
        let content =
            std::fs::read_to_string(path).map_err(|e| ValidationError::IoError(format!("{e}")))?;

        let file_name = path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("unknown");

        self.validate(&content, file_name)
    }

    /// Validates a single scenario.
    ///
    /// # Errors
    ///
    /// Returns `ValidationError` if LLM API calls fail (wrapped as `NOT_RELATED_ERROR`).
    /// Gherkin validation issues are returned as `ValidationIssue` items, not errors.
    fn validate_scenario(
        &self,
        feature: &gherkin::Feature,
        scenario: &gherkin::Scenario,
    ) -> Result<Vec<ValidationIssue>, ValidationError> {
        let mut issues = Vec::new();

        // Check business sense if enabled
        if self.config.check_business_sense {
            match self.check_business_sense(feature, scenario) {
                Ok(Some(issue)) => issues.push(issue),
                Ok(None) => {} // No issue found
                Err(e) => {
                    // LLM errors are not Gherkin validation issues - propagate as error
                    return Err(e);
                }
            }
        }

        // Check compilability if enabled
        if self.config.check_compilability {
            match self.check_compilability(feature, scenario) {
                Ok(Some(issue)) => issues.push(issue),
                Ok(None) => {} // No issue found
                Err(e) => {
                    // LLM errors are not Gherkin validation issues - propagate as error
                    return Err(e);
                }
            }
        }

        // Check conventions if enabled (no LLM, so no errors possible)
        if self.config.check_conventions {
            issues.extend(self.check_conventions(scenario));
        }

        Ok(issues)
    }

    /// Validates the overall feature structure.
    fn validate_feature(
        &self,
        feature: &gherkin::Feature,
    ) -> Result<Vec<ValidationIssue>, ValidationError> {
        let mut issues = Vec::new();

        // Check that the feature has a description
        if feature.description.is_none() {
            issues.push(ValidationIssue {
                location: "Feature".to_string(),
                category: IssueCategory::Convention,
                severity: Severity::Warning,
                message: "Feature lacks a description".to_string(),
                suggestion: Some("Add a description explaining the business purpose".to_string()),
            });
        }

        // Check for empty feature
        if feature.scenarios.is_empty() {
            issues.push(ValidationIssue {
                location: "Feature".to_string(),
                category: IssueCategory::Convention,
                severity: Severity::Error,
                message: "Feature has no scenarios".to_string(),
                suggestion: Some("Add at least one scenario".to_string()),
            });
        }

        Ok(issues)
    }

    /// Uses LLM to check if a scenario makes business sense.
    fn check_business_sense(
        &self,
        feature: &gherkin::Feature,
        scenario: &gherkin::Scenario,
    ) -> Result<Option<ValidationIssue>, ValidationError> {
        let prompt = format!(
            r"You are a business analyst validating Gherkin specifications for a multi-agent AI system called Converge.

Feature: {}
Scenario: {}

Steps:
{}

Evaluate if this scenario makes business sense:
1. Is the precondition (Given) realistic and well-defined?
2. Is the action (When) meaningful and testable?
3. Is the expected outcome (Then) measurable and valuable?

Respond with ONLY one of:
- VALID: if the scenario makes business sense
- INVALID: <reason> if it doesn't make sense
- UNCLEAR: <question> if more context is needed",
            feature.name,
            scenario.name,
            format_steps(&scenario.steps)
        );

        let system_prompt = "You are a strict business requirements validator. Be concise.";
        let request = LlmRequest::new(prompt.clone())
            .with_system(system_prompt)
            .with_max_tokens(200)
            .with_temperature(0.3);

        eprintln!("\n📤 Business Sense Check - Sending to LLM:");
        eprintln!("   Scenario: {}", scenario.name);
        eprintln!("   System Prompt: {system_prompt}");
        eprintln!(
            "   User Prompt (first 200 chars): {}...",
            prompt.chars().take(200).collect::<String>()
        );
        eprintln!("   Request params: max_tokens=200, temperature=0.3");

        let response = self.provider.complete(&request).map_err(|e| {
            // LLM API errors are not Gherkin validation issues
            ValidationError::LlmError(format!("NOT_RELATED_ERROR: LLM API call failed: {e}"))
        })?;

        eprintln!("\n📥 Business Sense Check - Response from LLM:");
        eprintln!("   Raw response: {}", response.content);
        eprintln!("   Model: {}", response.model);
        eprintln!(
            "   Token usage: prompt={}, completion={}, total={}",
            response.usage.prompt_tokens,
            response.usage.completion_tokens,
            response.usage.total_tokens
        );
        eprintln!("   Finish reason: {:?}", response.finish_reason);

        let content = response.content.trim();
        eprintln!("\n🔍 Business Sense Check - Reasoning:");

        if content.starts_with("INVALID:") {
            let reason = content.strip_prefix("INVALID:").unwrap_or("").trim();
            eprintln!("   → Detected: INVALID");
            eprintln!("   → Reason: {reason}");
            eprintln!("   → Action: Creating Error-level ValidationIssue");
            Ok(Some(ValidationIssue {
                location: format!("Scenario: {}", scenario.name),
                category: IssueCategory::BusinessSense,
                severity: Severity::Error,
                message: reason.to_string(),
                suggestion: None,
            }))
        } else if content.starts_with("UNCLEAR:") {
            let question = content.strip_prefix("UNCLEAR:").unwrap_or("").trim();
            eprintln!("   → Detected: UNCLEAR");
            eprintln!("   → Question: {question}");
            eprintln!("   → Action: Creating Warning-level ValidationIssue with suggestion");
            Ok(Some(ValidationIssue {
                location: format!("Scenario: {}", scenario.name),
                category: IssueCategory::BusinessSense,
                severity: Severity::Warning,
                message: format!("Ambiguous: {question}"),
                suggestion: Some("Clarify the scenario requirements".to_string()),
            }))
        } else {
            eprintln!("   → Detected: VALID (or response doesn't match expected format)");
            eprintln!("   → Action: No issue created (scenario passes business sense check)");
            Ok(None) // VALID
        }
    }

    /// Uses LLM to check if a scenario can be compiled to a Rust invariant.
    fn check_compilability(
        &self,
        feature: &gherkin::Feature,
        scenario: &gherkin::Scenario,
    ) -> Result<Option<ValidationIssue>, ValidationError> {
        let prompt = format!(
            r"You are a Rust developer checking if a Gherkin scenario can be compiled to a runtime invariant.

In Converge, invariants are Rust structs implementing:
```rust
trait Invariant {{
    fn name(&self) -> &str;
    fn class(&self) -> InvariantClass; // Structural, Semantic, or Acceptance
    fn check(&self, ctx: &Context) -> InvariantResult;
}}
```

The Context has typed facts in categories: Seeds, Hypotheses, Strategies, Constraints, Signals, Competitors, Evaluations.

Feature: {}
Scenario: {}
Steps:
{}

Can this scenario be implemented as a Converge Invariant?

Respond with ONLY one of:
- COMPILABLE: <invariant_class> - brief description of implementation
- NOT_COMPILABLE: <reason why it cannot be a runtime check>
- NEEDS_REFACTOR: <suggestion to make it compilable>",
            feature.name,
            scenario.name,
            format_steps(&scenario.steps)
        );

        let system_prompt =
            "You are a Rust expert. Be precise about what can be checked at runtime.";
        let request = LlmRequest::new(prompt.clone())
            .with_system(system_prompt)
            .with_max_tokens(200)
            .with_temperature(0.3);

        eprintln!("\n📤 Compilability Check - Sending to LLM:");
        eprintln!("   Scenario: {}", scenario.name);
        eprintln!("   System Prompt: {system_prompt}");
        eprintln!(
            "   User Prompt (first 200 chars): {}...",
            prompt.chars().take(200).collect::<String>()
        );
        eprintln!("   Request params: max_tokens=200, temperature=0.3");

        let response = self.provider.complete(&request).map_err(|e| {
            // LLM API errors are not Gherkin validation issues
            ValidationError::LlmError(format!("NOT_RELATED_ERROR: LLM API call failed: {e}"))
        })?;

        eprintln!("\n📥 Compilability Check - Response from LLM:");
        eprintln!("   Raw response: {}", response.content);
        eprintln!("   Model: {}", response.model);
        eprintln!(
            "   Token usage: prompt={}, completion={}, total={}",
            response.usage.prompt_tokens,
            response.usage.completion_tokens,
            response.usage.total_tokens
        );
        eprintln!("   Finish reason: {:?}", response.finish_reason);

        let content = response.content.trim();
        eprintln!("\n🔍 Compilability Check - Reasoning:");

        if content.starts_with("NOT_COMPILABLE:") {
            let reason = content.strip_prefix("NOT_COMPILABLE:").unwrap_or("").trim();
            eprintln!("   → Detected: NOT_COMPILABLE");
            eprintln!("   → Reason: {reason}");
            eprintln!("   → Action: Creating Error-level ValidationIssue");
            Ok(Some(ValidationIssue {
                location: format!("Scenario: {}", scenario.name),
                category: IssueCategory::Compilability,
                severity: Severity::Error,
                message: format!("Cannot compile to invariant: {reason}"),
                suggestion: None,
            }))
        } else if content.starts_with("NEEDS_REFACTOR:") {
            let suggestion = content.strip_prefix("NEEDS_REFACTOR:").unwrap_or("").trim();
            eprintln!("   → Detected: NEEDS_REFACTOR");
            eprintln!("   → Suggestion: {suggestion}");
            eprintln!(
                "   → Action: Creating Warning-level ValidationIssue with refactoring suggestion"
            );
            Ok(Some(ValidationIssue {
                location: format!("Scenario: {}", scenario.name),
                category: IssueCategory::Compilability,
                severity: Severity::Warning,
                message: "Scenario needs refactoring to be compilable".to_string(),
                suggestion: Some(suggestion.to_string()),
            }))
        } else if content.starts_with("COMPILABLE:") {
            let details = content.strip_prefix("COMPILABLE:").unwrap_or("").trim();
            eprintln!("   → Detected: COMPILABLE");
            eprintln!("   → Details: {details}");
            eprintln!("   → Action: No issue created (scenario is compilable)");
            Ok(None) // COMPILABLE
        } else {
            eprintln!("   → Warning: Response doesn't match expected format");
            eprintln!("   → Raw response: {content}");
            eprintln!("   → Action: Treating as COMPILABLE (no issue created)");
            Ok(None) // Default to compilable if format doesn't match
        }
    }

    /// Checks scenario against Converge Gherkin conventions (no LLM needed).
    fn check_conventions(&self, scenario: &gherkin::Scenario) -> Vec<ValidationIssue> {
        let mut issues = Vec::new();

        // Check scenario naming convention
        if scenario.name.is_empty() {
            issues.push(ValidationIssue {
                location: "Scenario".to_string(),
                category: IssueCategory::Convention,
                severity: Severity::Error,
                message: "Scenario has no name".to_string(),
                suggestion: Some("Add a descriptive name".to_string()),
            });
        }

        // Check for Given/When/Then structure
        let has_given = scenario
            .steps
            .iter()
            .any(|s| matches!(s.ty, gherkin::StepType::Given));
        let has_when = scenario
            .steps
            .iter()
            .any(|s| matches!(s.ty, gherkin::StepType::When));
        let has_then = scenario
            .steps
            .iter()
            .any(|s| matches!(s.ty, gherkin::StepType::Then));

        if !has_given && !has_when {
            issues.push(ValidationIssue {
                location: format!("Scenario: {}", scenario.name),
                category: IssueCategory::Convention,
                severity: Severity::Warning,
                message: "Scenario lacks Given or When steps".to_string(),
                suggestion: Some("Add preconditions (Given) or actions (When)".to_string()),
            });
        }

        if !has_then {
            issues.push(ValidationIssue {
                location: format!("Scenario: {}", scenario.name),
                category: IssueCategory::Convention,
                severity: Severity::Error,
                message: "Scenario lacks Then steps (expected outcomes)".to_string(),
                suggestion: Some(
                    "Add at least one Then step defining the expected outcome".to_string(),
                ),
            });
        }

        // Check for Converge-specific patterns
        for step in &scenario.steps {
            if step.value.contains("should") && matches!(step.ty, gherkin::StepType::Then) {
                // Good pattern: "Then X should Y"
            } else if step.value.contains("must") || step.value.contains("always") {
                // Good pattern for invariants
            } else if step.value.contains("might") || step.value.contains("maybe") {
                issues.push(ValidationIssue {
                    location: format!("Step: {}", step.value),
                    category: IssueCategory::Convention,
                    severity: Severity::Warning,
                    message: "Uncertain language in step ('might', 'maybe')".to_string(),
                    suggestion: Some("Use definite language for testable assertions".to_string()),
                });
            }
        }

        issues
    }
}

/// Formats Gherkin steps for display.
fn format_steps(steps: &[gherkin::Step]) -> String {
    steps
        .iter()
        .map(|s| format!("{:?} {}", s.keyword, s.value))
        .collect::<Vec<_>>()
        .join("\n")
}

/// Error during Gherkin validation.
#[derive(Debug, Clone)]
pub enum ValidationError {
    /// Failed to parse the Gherkin file.
    ParseError(String),
    /// IO error reading file.
    IoError(String),
    /// LLM call failed.
    LlmError(String),
}

impl std::fmt::Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ParseError(msg) => write!(f, "Parse error: {msg}"),
            Self::IoError(msg) => write!(f, "IO error: {msg}"),
            Self::LlmError(msg) => write!(f, "LLM error: {msg}"),
        }
    }
}

impl std::error::Error for ValidationError {}

#[cfg(test)]
mod tests {
    use super::*;
    use converge_core::llm::{MockProvider, MockResponse};

    fn mock_valid_provider() -> Arc<dyn LlmProvider> {
        Arc::new(MockProvider::new(vec![
            MockResponse::success("VALID", 0.9),
            MockResponse::success("COMPILABLE: Acceptance - check strategy count", 0.9),
        ]))
    }

    #[test]
    fn preprocess_converts_truth_to_feature() {
        let input = "Truth: Get paid for delivered work\n  Scenario: Invoice";
        let output = preprocess_truths(input);
        assert!(output.starts_with("Feature:"));
        assert!(output.contains("Scenario: Invoice"));
    }

    #[test]
    fn preprocess_preserves_feature_keyword() {
        let input = "Feature: Standard Gherkin\n  Scenario: Test";
        let output = preprocess_truths(input);
        assert_eq!(input, output);
    }

    #[test]
    fn validates_truth_syntax() {
        let content = r"
Truth: Get paid for delivered work
  Scenario: Invoice and collect
    Given work is marked as delivered
    When the system converges
    Then invoice is issued
";

        let validator = GherkinValidator::new(mock_valid_provider(), ValidationConfig::default());

        let result = validator.validate(content, "money.feature").unwrap();

        assert_eq!(result.scenario_count, 1);
        // Should parse successfully with Truth: syntax
    }

    #[test]
    fn validates_simple_feature() {
        let content = r"
Feature: Growth Strategy Validation
  Scenario: Multiple strategies required
    When the system converges
    Then at least two distinct growth strategies exist
";

        let validator = GherkinValidator::new(mock_valid_provider(), ValidationConfig::default());

        let result = validator.validate(content, "test.feature").unwrap();

        assert_eq!(result.scenario_count, 1);
        // May have convention warnings but should be parseable
    }

    #[test]
    fn detects_missing_then() {
        let content = r"
Feature: Bad Spec
  Scenario: No assertions
    Given some precondition
    When something happens
";

        let validator = GherkinValidator::new(
            mock_valid_provider(),
            ValidationConfig {
                check_business_sense: false,
                check_compilability: false,
                check_conventions: true,
                min_confidence: 0.7,
            },
        );

        let result = validator.validate(content, "bad.feature").unwrap();

        assert!(result.has_errors());
        assert!(
            result
                .issues
                .iter()
                .any(|i| i.category == IssueCategory::Convention && i.message.contains("Then"))
        );
    }

    #[test]
    fn detects_uncertain_language() {
        let content = r"
Feature: Uncertain Spec
  Scenario: Maybe works
    When something happens
    Then it might succeed
";

        let validator = GherkinValidator::new(
            mock_valid_provider(),
            ValidationConfig {
                check_business_sense: false,
                check_compilability: false,
                check_conventions: true,
                min_confidence: 0.7,
            },
        );

        let result = validator.validate(content, "uncertain.feature").unwrap();

        assert!(result.has_warnings());
        assert!(result.issues.iter().any(|i| i.message.contains("might")));
    }

    #[test]
    fn handles_llm_invalid_response() {
        let provider = Arc::new(MockProvider::new(vec![
            MockResponse::success("INVALID: The scenario describes an untestable state", 0.8),
            MockResponse::success("COMPILABLE: Acceptance", 0.9),
        ]));

        let content = r"
Feature: Test
  Scenario: Bad business logic
    When magic happens
    Then everything is perfect forever
";

        let validator = GherkinValidator::new(provider, ValidationConfig::default());

        let result = validator.validate(content, "test.feature").unwrap();

        assert!(
            result.issues.iter().any(
                |i| i.category == IssueCategory::BusinessSense && i.severity == Severity::Error
            )
        );
    }
}
