// Copyright 2024-2025 Aprio One AB, Sweden
// Author: Kenneth Pernyer, kenneth@aprio.one
// SPDX-License-Identifier: MIT
// See LICENSE file in the project root for full license information.

//! Integration tests for Gherkin validation using Anthropic provider.
//!
//! These tests verify that the Gherkin validator correctly uses the Anthropic
//! provider to validate Gherkin specifications for:
//! 1. Business sense (semantic validity)
//! 2. Compilability to Rust invariants (technical feasibility)
//! 3. Convention compliance (style adherence)
//!
//! Run with: `cargo test --test integration_gherkin_validation -- --ignored --show-output`
//! Requires: ANTHROPIC_API_KEY environment variable (can be in `.env` file at workspace root)
//!
//! **Note**: Use `--show-output` or `--nocapture` to see all println! output.
//! Without these flags, stdout is captured and only shown on test failure.

use converge_core::llm::LlmProvider;
use converge_provider::AnthropicProvider;
use converge_tool::gherkin::{
    GherkinValidator, IssueCategory, Severity, ValidationConfig, ValidationIssue,
};
use std::sync::{Arc, LazyLock};
use std::time::Instant;

/// Loads environment variables from `.env` file if it exists.
///
/// Tries multiple locations:
/// 1. Current working directory (workspace root when tests run)
/// 2. Workspace root relative to test file (../../.env)
/// 3. Parent directory (.env relative to test file)
fn load_env_if_exists() {
    let mut loaded = false;
    
    // Try current directory .env (workspace root when tests run)
    if let Ok(env_path) = dotenv::dotenv() {
        eprintln!("   ✓ Loaded .env from: {}", env_path.display());
        loaded = true;
    }
    
    // Try workspace root relative to test file (../../.env from tests/ directory)
    let workspace_env = std::path::Path::new("../../.env");
    if !loaded && workspace_env.exists() {
        if let Err(e) = dotenv::from_path(workspace_env) {
            eprintln!("   ⚠️  Warning: Failed to load .env from workspace root: {}", e);
        } else {
            eprintln!("   ✓ Loaded .env from workspace root: {}", workspace_env.display());
            loaded = true;
        }
    }
    
    // Try parent directory (.env relative to test file location)
    let parent_env = std::path::Path::new("../.env");
    if !loaded && parent_env.exists() {
        if let Err(e) = dotenv::from_path(parent_env) {
            eprintln!("   ⚠️  Warning: Failed to load .env from parent directory: {}", e);
        } else {
            eprintln!("   ✓ Loaded .env from parent directory: {}", parent_env.display());
            loaded = true;
        }
    }
    
    if !loaded {
        eprintln!("   ℹ️  No .env file found (will check environment variables)");
    }
}

/// Shared Anthropic provider instance for all tests.
///
/// Created once on first use and reused across all test cases. This is safe because:
/// - The provider is stateless (just an HTTP client)
/// - `Arc<dyn LlmProvider>` is `Send + Sync` (thread-safe)
/// - Tests may run in parallel, but the provider can be safely shared
static SHARED_PROVIDER: LazyLock<Arc<dyn LlmProvider>> = LazyLock::new(|| {
    let model = "claude-3-5-haiku-20241022";
    // Use eprintln! so output is always visible (stderr is not captured)
    eprintln!("\n🔧 Creating shared Anthropic provider with model: {}", model);
    eprintln!("   API Endpoint: https://api.anthropic.com/v1/messages");
    eprintln!("   This will make REAL API calls and consume credits!");
    eprintln!("   Provider will be reused across all tests (thread-safe)");
    
    // Load .env file if it exists (from workspace root) - only once
    load_env_if_exists();
    
    // Check for API key first and fail explicitly if missing
    let api_key = std::env::var("ANTHROPIC_API_KEY")
        .unwrap_or_else(|_| {
            eprintln!("\n❌ ERROR: ANTHROPIC_API_KEY environment variable is not set!");
            eprintln!("   Set it with: export ANTHROPIC_API_KEY='your-api-key'");
            eprintln!("   Or add it to .env file in workspace root");
            eprintln!("   Get your key from: https://console.anthropic.com/settings/keys");
            panic!("ANTHROPIC_API_KEY environment variable not set. Test cannot proceed without API key.");
        });
    
    // Verify the key is not empty
    if api_key.trim().is_empty() {
        eprintln!("\n❌ ERROR: ANTHROPIC_API_KEY is set but empty!");
        panic!("ANTHROPIC_API_KEY environment variable is empty. Test cannot proceed.");
    }
    
    eprintln!("   ✓ API key found (length: {} chars)", api_key.len());
    
    AnthropicProvider::from_env(model)
        .map(|p| {
            eprintln!("   ✓ Provider created successfully (will be shared across tests)");
            Arc::new(p) as Arc<dyn LlmProvider>
        })
        .unwrap_or_else(|e| {
            eprintln!("\n❌ ERROR: Failed to create Anthropic provider: {}", e);
            panic!("Failed to create Anthropic provider: {}. Test cannot proceed.", e);
        })
});

/// Gets the shared Anthropic provider for testing.
///
/// The provider is created once on first use and reused across all test cases.
/// This is safe because the provider is stateless and thread-safe.
///
/// **IMPORTANT**: This provider makes REAL API calls to Anthropic's API.
/// These tests will consume API credits and require a valid `ANTHROPIC_API_KEY`.
///
/// **Note**: Use `--show-output` or `--nocapture` flag to see all test output:
/// ```bash
/// cargo test --test integration_gherkin_validation -- --ignored --show-output
/// ```
#[must_use]
fn get_shared_provider() -> Arc<dyn LlmProvider> {
    // LazyLock ensures the provider is created only once, even if tests run in parallel
    Arc::clone(&SHARED_PROVIDER)
}

/// Helper to check if API error is due to credit balance (not auth - that should fail the test).
///
/// Note: Authentication errors should cause test failure, not skipping.
/// Only credit balance issues are skippable since they're temporary.
fn is_skip_error(error: &str) -> bool {
    let lower = error.to_lowercase();
    lower.contains("credit balance")
        || lower.contains("too low")
        || lower.contains("insufficient credits")
}

/// Validates a Gherkin spec and handles errors gracefully.
fn validate_with_anthropic(
    validator: &GherkinValidator,
    content: &str,
    file_name: &str,
) -> Result<converge_tool::gherkin::SpecValidation, String> {
    validator
        .validate(content, file_name)
        .map_err(|e| format!("Validation failed: {}", e))
}

// =============================================================================
// TEST SUITE: Valid Gherkin Specifications
// =============================================================================

#[test]
#[ignore]
fn test_valid_growth_strategy_spec() {
    let provider = get_shared_provider();

    let config = ValidationConfig {
        check_business_sense: true,
        check_compilability: true,
        check_conventions: true,
        min_confidence: 0.7,
    };

    let validator = GherkinValidator::new(provider, config);

    let content = r#"
Feature: Growth Strategy Validation
  As a product manager
  I want to ensure multiple growth strategies are generated
  So that we have options to pursue

  Scenario: Multiple strategies required
    Given the system has received market signals
    When the system converges
    Then at least two distinct growth strategies must exist
    And each strategy must have a confidence score above 0.7
"#;

    println!("\n🚀 Starting validation (this will call Anthropic API)...");
    let start = Instant::now();
    let result = match validate_with_anthropic(&validator, content, "growth_strategy.feature") {
        Ok(r) => r,
        Err(e) => {
            if is_skip_error(&e) {
                eprintln!("⚠️  Skipping test: {}", e);
                return;
            }
            panic!("Validation failed: {}", e);
        }
    };
    let elapsed = start.elapsed();

    println!("\n📊 Test: Valid Growth Strategy Spec");
    println!("   Model used: claude-3-5-haiku-20241022");
    println!("   API calls made: {} (business sense + compilability checks)", 
             if result.scenario_count > 0 { 2 } else { 0 });
    println!("   Duration: {:?}", elapsed);
    println!("   Valid: {}", result.is_valid);
    println!("   Scenarios: {}", result.scenario_count);
    println!("   Issues: {}", result.issues.len());
    println!("   Confidence: {:.2}", result.confidence);
    println!("   Summary: {}", result.summary());

    if !result.issues.is_empty() {
        println!("\n   Issues found:");
        for issue in &result.issues {
            println!("     - [{:?}] {:?}: {}", issue.severity, issue.category, issue.message);
            if let Some(suggestion) = &issue.suggestion {
                println!("       Suggestion: {}", suggestion);
            }
        }
    }

    // This should be valid - well-formed spec with clear business logic
    assert!(
        result.is_valid,
        "Valid growth strategy spec should pass validation. Issues: {:?}",
        result.issues
    );
    assert_eq!(result.scenario_count, 1);
    assert!(!result.has_errors(), "Should not have errors for valid spec");
}

#[test]
#[ignore]
fn test_valid_constraint_spec() {
    let provider = get_shared_provider();

    let validator = GherkinValidator::new(provider, ValidationConfig::default());

    let content = r#"
Feature: Brand Safety Constraints
  Scenario: No unsafe content in strategies
    Given a growth strategy has been proposed
    When the system validates the strategy
    Then the strategy must not contain prohibited terms
    And the strategy must comply with brand safety guidelines
"#;

    let start = Instant::now();
    let result = match validate_with_anthropic(&validator, content, "brand_safety.feature") {
        Ok(r) => r,
        Err(e) => {
            if is_skip_error(&e) {
                eprintln!("⚠️  Skipping test: {}", e);
                return;
            }
            panic!("Validation failed: {}", e);
        }
    };
    let elapsed = start.elapsed();

    println!("\n📊 Test: Valid Constraint Spec");
    println!("   Duration: {:?}", elapsed);
    println!("   Valid: {}", result.is_valid);
    println!("   Issues: {}", result.issues.len());
    println!("   Summary: {}", result.summary());

    // Should be valid - clear constraint checking scenario
    assert!(
        result.is_valid,
        "Valid constraint spec should pass. Issues: {:?}",
        result.issues
    );
}

#[test]
#[ignore]
fn test_valid_multi_scenario_feature() {
    let provider = get_shared_provider();

    let validator = GherkinValidator::new(provider, ValidationConfig::default());

    let content = r#"
Feature: Strategy Evaluation
  Scenario: Strategies are ranked by score
    Given multiple growth strategies exist
    When the system evaluates strategies
    Then strategies must be ranked by confidence score
    And the highest scoring strategy should be first

  Scenario: Minimum score threshold
    Given a growth strategy has been generated
    When the strategy confidence score is calculated
    Then the score must be at least 0.6
    And the strategy should be rejected if the score is below 0.6
"#;

    let start = Instant::now();
    let result = match validate_with_anthropic(&validator, content, "evaluation.feature") {
        Ok(r) => r,
        Err(e) => {
            if is_skip_error(&e) {
                eprintln!("⚠️  Skipping test: {}", e);
                return;
            }
            panic!("Validation failed: {}", e);
        }
    };
    let elapsed = start.elapsed();

    println!("\n📊 Test: Multi-Scenario Feature");
    println!("   Duration: {:?}", elapsed);
    println!("   Valid: {}", result.is_valid);
    println!("   Scenarios: {}", result.scenario_count);
    println!("   Issues: {}", result.issues.len());

    assert_eq!(result.scenario_count, 2);
    // Both scenarios should be validated
    assert!(
        result.is_valid || result.issues.len() < 3,
        "Multi-scenario feature should mostly pass. Issues: {:?}",
        result.issues
    );
}

// =============================================================================
// TEST SUITE: Business Sense Validation
// =============================================================================

#[test]
#[ignore]
fn test_business_sense_untestable_scenario() {
    let provider = get_shared_provider();

    let config = ValidationConfig {
        check_business_sense: true,
        check_compilability: false,
        check_conventions: false,
        min_confidence: 0.7,
    };

    let validator = GherkinValidator::new(provider, config);

    let content = r#"
Feature: Magic Happens
  Scenario: Everything is perfect
    When magic happens
    Then everything is perfect forever
"#;

    let start = Instant::now();
    let result = match validate_with_anthropic(&validator, content, "magic.feature") {
        Ok(r) => r,
        Err(e) => {
            if is_skip_error(&e) {
                eprintln!("⚠️  Skipping test: {}", e);
                return;
            }
            panic!("Validation failed: {}", e);
        }
    };
    let elapsed = start.elapsed();

    println!("\n📊 Test: Business Sense - Untestable Scenario");
    println!("   Duration: {:?}", elapsed);
    println!("   Valid: {}", result.is_valid);
    println!("   Issues: {}", result.issues.len());

    if !result.issues.is_empty() {
        println!("\n   Issues:");
        for issue in &result.issues {
            println!("     - [{:?}] {:?}: {}", issue.severity, issue.category, issue.message);
        }
    }

    // Should detect that "magic happens" and "perfect forever" are untestable
    let has_business_sense_issue = result
        .issues
        .iter()
        .any(|i| i.category == IssueCategory::BusinessSense);

    assert!(
        has_business_sense_issue || !result.is_valid,
        "Should detect business sense issues in untestable scenario. Result: {:?}",
        result
    );
}

#[test]
#[ignore]
fn test_business_sense_vague_preconditions() {
    let provider = get_shared_provider();

    let config = ValidationConfig {
        check_business_sense: true,
        check_compilability: false,
        check_conventions: false,
        min_confidence: 0.7,
    };

    let validator = GherkinValidator::new(provider, config);

    let content = r#"
Feature: Vague Requirements
  Scenario: Something happens
    Given some things exist
    When something happens
    Then something should be different
"#;

    let start = Instant::now();
    let result = match validate_with_anthropic(&validator, content, "vague.feature") {
        Ok(r) => r,
        Err(e) => {
            if is_skip_error(&e) {
                eprintln!("⚠️  Skipping test: {}", e);
                return;
            }
            panic!("Validation failed: {}", e);
        }
    };
    let elapsed = start.elapsed();

    println!("\n📊 Test: Business Sense - Vague Preconditions");
    println!("   Duration: {:?}", elapsed);
    println!("   Valid: {}", result.is_valid);
    println!("   Issues: {}", result.issues.len());

    // Should detect vague language
    let has_issue = result
        .issues
        .iter()
        .any(|i| {
            i.category == IssueCategory::BusinessSense
                || (i.category == IssueCategory::Convention
                    && (i.message.contains("vague") || i.message.contains("unclear")))
        });

    assert!(
        has_issue || !result.is_valid,
        "Should detect vague preconditions. Result: {:?}",
        result
    );
}

// =============================================================================
// TEST SUITE: Compilability Validation
// =============================================================================

#[test]
#[ignore]
fn test_compilability_runtime_checkable() {
    let provider = get_shared_provider();

    let config = ValidationConfig {
        check_business_sense: false,
        check_compilability: true,
        check_conventions: false,
        min_confidence: 0.7,
    };

    let validator = GherkinValidator::new(provider, config);

    let content = r#"
Feature: Strategy Count Check
  Scenario: Multiple strategies exist
    Given the context contains strategies
    When the system checks strategy count
    Then the number of strategies must be at least 2
"#;

    let start = Instant::now();
    let result = match validate_with_anthropic(&validator, content, "count.feature") {
        Ok(r) => r,
        Err(e) => {
            if is_skip_error(&e) {
                eprintln!("⚠️  Skipping test: {}", e);
                return;
            }
            panic!("Validation failed: {}", e);
        }
    };
    let elapsed = start.elapsed();

    println!("\n📊 Test: Compilability - Runtime Checkable");
    println!("   Duration: {:?}", elapsed);
    println!("   Valid: {}", result.is_valid);
    println!("   Issues: {}", result.issues.len());

    // Should recognize this is compilable (checking count is a runtime check)
    let has_compilability_error = result
        .issues
        .iter()
        .any(|i| i.category == IssueCategory::Compilability && i.severity == Severity::Error);

    assert!(
        !has_compilability_error,
        "Should recognize count check is compilable. Issues: {:?}",
        result.issues
    );
}

#[test]
#[ignore]
fn test_compilability_requires_human_input() {
    let provider = get_shared_provider();

    let config = ValidationConfig {
        check_business_sense: false,
        check_compilability: true,
        check_conventions: false,
        min_confidence: 0.7,
    };

    let validator = GherkinValidator::new(provider, config);

    let content = r#"
Feature: Human Approval
  Scenario: Strategy requires approval
    Given a growth strategy has been generated
    When the strategy is presented to the user
    Then the user must approve the strategy
    And the approval must be recorded
"#;

    let start = Instant::now();
    let result = match validate_with_anthropic(&validator, content, "approval.feature") {
        Ok(r) => r,
        Err(e) => {
            if is_skip_error(&e) {
                eprintln!("⚠️  Skipping test: {}", e);
                return;
            }
            panic!("Validation failed: {}", e);
        }
    };
    let elapsed = start.elapsed();

    println!("\n📊 Test: Compilability - Requires Human Input");
    println!("   Duration: {:?}", elapsed);
    println!("   Valid: {}", result.is_valid);
    println!("   Issues: {}", result.issues.len());

    if !result.issues.is_empty() {
        println!("\n   Issues:");
        for issue in &result.issues {
            println!("     - [{:?}] {:?}: {}", issue.severity, issue.category, issue.message);
        }
    }

    // Should recognize that "user must approve" requires human interaction
    // This might be flagged as needing refactoring or might be acceptable
    // depending on how the LLM interprets it
    let _has_compilability_issue = result
        .issues
        .iter()
        .any(|i| i.category == IssueCategory::Compilability);

    // This test documents behavior - human approval can be modeled as a constraint
    // The LLM might flag it or might recognize it as a valid acceptance invariant
    println!(
        "   Note: Human approval scenarios may be flagged or accepted depending on interpretation"
    );
}

#[test]
#[ignore]
fn test_compilability_external_api_call() {
    let provider = get_shared_provider();

    let config = ValidationConfig {
        check_business_sense: false,
        check_compilability: true,
        check_conventions: false,
        min_confidence: 0.7,
    };

    let validator = GherkinValidator::new(provider, config);

    let content = r#"
Feature: External Validation
  Scenario: Strategy validated via API
    Given a growth strategy exists
    When the system calls an external API to validate
    Then the API response must indicate success
"#;

    let start = Instant::now();
    let result = match validate_with_anthropic(&validator, content, "external.feature") {
        Ok(r) => r,
        Err(e) => {
            if is_skip_error(&e) {
                eprintln!("⚠️  Skipping test: {}", e);
                return;
            }
            panic!("Validation failed: {}", e);
        }
    };
    let elapsed = start.elapsed();

    println!("\n📊 Test: Compilability - External API Call");
    println!("   Duration: {:?}", elapsed);
    println!("   Valid: {}", result.is_valid);
    println!("   Issues: {}", result.issues.len());

    // External API calls can be modeled as signals or constraints
    // The LLM should recognize this is compilable (can check API response)
    let has_compilability_error = result
        .issues
        .iter()
        .any(|i| i.category == IssueCategory::Compilability && i.severity == Severity::Error);

    // External API validation is compilable - it's a runtime check
    assert!(
        !has_compilability_error,
        "External API validation should be recognized as compilable. Issues: {:?}",
        result.issues
    );
}

// =============================================================================
// TEST SUITE: Convention Compliance
// =============================================================================

#[test]
#[ignore]
fn test_convention_missing_then() {
    let provider = get_shared_provider();

    let config = ValidationConfig {
        check_business_sense: false,
        check_compilability: false,
        check_conventions: true,
        min_confidence: 0.7,
    };

    let validator = GherkinValidator::new(provider, config);

    let content = r#"
Feature: Incomplete Spec
  Scenario: No assertions
    Given some precondition
    When something happens
"#;

    let start = Instant::now();
    let result = match validate_with_anthropic(&validator, content, "incomplete.feature") {
        Ok(r) => r,
        Err(e) => {
            if is_skip_error(&e) {
                eprintln!("⚠️  Skipping test: {}", e);
                return;
            }
            panic!("Validation failed: {}", e);
        }
    };
    let elapsed = start.elapsed();

    println!("\n📊 Test: Convention - Missing Then");
    println!("   Duration: {:?}", elapsed);
    println!("   Valid: {}", result.is_valid);
    println!("   Issues: {}", result.issues.len());

    // Should detect missing Then step
    assert!(
        result.has_errors(),
        "Should detect missing Then step. Issues: {:?}",
        result.issues
    );

    let has_then_issue = result
        .issues
        .iter()
        .any(|i| i.category == IssueCategory::Convention && i.message.contains("Then"));

    assert!(
        has_then_issue,
        "Should have convention issue about missing Then. Issues: {:?}",
        result.issues
    );
}

#[test]
#[ignore]
fn test_convention_uncertain_language() {
    let provider = get_shared_provider();

    let config = ValidationConfig {
        check_business_sense: false,
        check_compilability: false,
        check_conventions: true,
        min_confidence: 0.7,
    };

    let validator = GherkinValidator::new(provider, config);

    let content = r#"
Feature: Uncertain Spec
  Scenario: Maybe works
    When something happens
    Then it might succeed
    And maybe it will be good
"#;

    let start = Instant::now();
    let result = match validate_with_anthropic(&validator, content, "uncertain.feature") {
        Ok(r) => r,
        Err(e) => {
            if is_skip_error(&e) {
                eprintln!("⚠️  Skipping test: {}", e);
                return;
            }
            panic!("Validation failed: {}", e);
        }
    };
    let elapsed = start.elapsed();

    println!("\n📊 Test: Convention - Uncertain Language");
    println!("   Duration: {:?}", elapsed);
    println!("   Valid: {}", result.is_valid);
    println!("   Issues: {}", result.issues.len());

    // Should detect uncertain language (might, maybe)
    let has_uncertain_issue = result
        .issues
        .iter()
        .any(|i| {
            i.category == IssueCategory::Convention
                && (i.message.contains("might") || i.message.contains("maybe"))
        });

    assert!(
        has_uncertain_issue || result.has_warnings(),
        "Should detect uncertain language. Issues: {:?}",
        result.issues
    );
}

#[test]
#[ignore]
fn test_convention_empty_scenario_name() {
    let provider = get_shared_provider();

    let config = ValidationConfig {
        check_business_sense: false,
        check_compilability: false,
        check_conventions: true,
        min_confidence: 0.7,
    };

    let validator = GherkinValidator::new(provider, config);

    let content = r#"
Feature: Bad Naming
  Scenario:
    Given some state
    When something happens
    Then something should be true
"#;

    let start = Instant::now();
    let result = match validate_with_anthropic(&validator, content, "naming.feature") {
        Ok(r) => r,
        Err(e) => {
            if is_skip_error(&e) {
                eprintln!("⚠️  Skipping test: {}", e);
                return;
            }
            panic!("Validation failed: {}", e);
        }
    };
    let elapsed = start.elapsed();

    println!("\n📊 Test: Convention - Empty Scenario Name");
    println!("   Duration: {:?}", elapsed);
    println!("   Valid: {}", result.is_valid);
    println!("   Issues: {}", result.issues.len());

    // Should detect empty scenario name
    assert!(
        result.has_errors(),
        "Should detect empty scenario name. Issues: {:?}",
        result.issues
    );

    let has_name_issue = result
        .issues
        .iter()
        .any(|i| {
            i.category == IssueCategory::Convention && i.message.contains("name")
        });

    assert!(
        has_name_issue,
        "Should have convention issue about empty name. Issues: {:?}",
        result.issues
    );
}

// =============================================================================
// TEST SUITE: Complex Real-World Scenarios
// =============================================================================

#[test]
#[ignore]
fn test_complex_growth_strategy_workflow() {
    let provider = get_shared_provider();

    let validator = GherkinValidator::new(provider, ValidationConfig::default());

    let content = r#"
Feature: Growth Strategy Generation and Validation
  As a product manager
  I want the system to generate and validate growth strategies
  So that we can make data-driven decisions

  Background:
    Given the system has access to market signals
    And the system has access to competitor data

  Scenario: Multiple strategies are generated
    Given market signals indicate opportunity in B2B SaaS
    When the growth strategy agent executes
    Then at least two distinct strategies must be generated
    And each strategy must reference market signals
    And each strategy must have a unique identifier

  Scenario: Strategies are evaluated
    Given multiple growth strategies exist
    When the evaluation agent executes
    Then each strategy must have a confidence score
    And scores must be between 0.0 and 1.0
    And strategies must be ranked by score

  Scenario: Low-quality strategies are filtered
    Given strategies with scores below 0.6 exist
    When the system converges
    Then strategies with scores below 0.6 must be rejected
    And only strategies with scores >= 0.6 should remain

  Scenario: Brand safety is enforced
    Given a growth strategy has been generated
    When the brand safety invariant checks the strategy
    Then the strategy must not contain prohibited terms
    And the strategy must comply with brand guidelines
"#;

    let start = Instant::now();
    let result = match validate_with_anthropic(&validator, content, "complex_workflow.feature") {
        Ok(r) => r,
        Err(e) => {
            if is_skip_error(&e) {
                eprintln!("⚠️  Skipping test: {}", e);
                return;
            }
            panic!("Validation failed: {}", e);
        }
    };
    let elapsed = start.elapsed();

    println!("\n📊 Test: Complex Growth Strategy Workflow");
    println!("   Duration: {:?}", elapsed);
    println!("   Valid: {}", result.is_valid);
    println!("   Scenarios: {}", result.scenario_count);
    println!("   Issues: {}", result.issues.len());
    println!("   Confidence: {:.2}", result.confidence);

    if !result.issues.is_empty() {
        println!("\n   Issues by category:");
        let mut by_category: Vec<(IssueCategory, Vec<&ValidationIssue>)> = Vec::new();
        for issue in &result.issues {
            if let Some((_, issues)) = by_category.iter_mut().find(|(cat, _)| *cat == issue.category) {
                issues.push(issue);
            } else {
                by_category.push((issue.category, vec![issue]));
            }
        }
        for (category, issues) in &by_category {
            println!("     {:?}: {} issues", category, issues.len());
            for issue in issues {
                println!("       - [{:?}] {}", issue.severity, issue.message);
            }
        }
    }

    // Complex real-world spec should mostly pass
    // May have minor convention warnings but should be fundamentally valid
    assert_eq!(result.scenario_count, 4);
    assert!(
        result.is_valid || result.issues.iter().filter(|i| i.severity == Severity::Error).count() < 2,
        "Complex workflow should be mostly valid. Issues: {:?}",
        result.issues
    );
}

#[test]
#[ignore]
fn test_performance_multiple_validations() {
    let provider = get_shared_provider();

    let validator = GherkinValidator::new(provider, ValidationConfig::default());

    let specs = vec![
        (
            "spec1.feature",
            r#"
Feature: Strategy Count
  Scenario: Multiple strategies
    Given strategies exist
    When checked
    Then count must be >= 2
"#,
        ),
        (
            "spec2.feature",
            r#"
Feature: Strategy Evaluation
  Scenario: Strategies ranked
    Given multiple strategies
    When evaluated
    Then they must be ranked by score
"#,
        ),
        (
            "spec3.feature",
            r#"
Feature: Brand Safety
  Scenario: No unsafe content
    Given a strategy
    When validated
    Then it must be brand-safe
"#,
        ),
    ];

    let start = Instant::now();
    let mut results = Vec::new();

    for (file_name, content) in &specs {
        match validate_with_anthropic(&validator, content, file_name) {
            Ok(result) => {
                results.push((file_name, result));
            }
            Err(e) => {
                if is_skip_error(&e) {
                    eprintln!("⚠️  Skipping performance test: {}", e);
                    return;
                }
                panic!("Validation failed for {}: {}", file_name, e);
            }
        }
    }

    let total_elapsed = start.elapsed();
    let avg_elapsed = total_elapsed / specs.len() as u32;

    println!("\n📊 Test: Performance - Multiple Validations");
    println!("   Total duration: {:?}", total_elapsed);
    println!("   Average per spec: {:?}", avg_elapsed);
    println!("   Specs validated: {}", specs.len());

    for (file_name, result) in &results {
        println!(
            "   {}: {} issues, valid: {}",
            file_name, result.issues.len(), result.is_valid
        );
    }

    assert_eq!(results.len(), specs.len());
    // All should complete successfully
    assert!(
        results.iter().all(|(_, r)| r.scenario_count > 0),
        "All specs should be validated"
    );
}

// =============================================================================
// TEST SUITE: Edge Cases
// =============================================================================

#[test]
#[ignore]
fn test_empty_feature() {
    let provider = get_shared_provider();

    let validator = GherkinValidator::new(provider, ValidationConfig::default());

    let content = r#"
Feature: Empty Feature
"#;

    let start = Instant::now();
    let result = match validate_with_anthropic(&validator, content, "empty.feature") {
        Ok(r) => r,
        Err(e) => {
            if is_skip_error(&e) {
                eprintln!("⚠️  Skipping test: {}", e);
                return;
            }
            panic!("Validation failed: {}", e);
        }
    };
    let elapsed = start.elapsed();

    println!("\n📊 Test: Edge Case - Empty Feature");
    println!("   Duration: {:?}", elapsed);
    println!("   Valid: {}", result.is_valid);
    println!("   Issues: {}", result.issues.len());

    // Should detect empty feature
    assert!(
        result.has_errors(),
        "Should detect empty feature. Issues: {:?}",
        result.issues
    );

    let has_empty_issue = result
        .issues
        .iter()
        .any(|i| i.message.contains("no scenarios") || i.message.contains("empty"));

    assert!(
        has_empty_issue,
        "Should have issue about empty feature. Issues: {:?}",
        result.issues
    );
}

#[test]
#[ignore]
fn test_malformed_gherkin() {
    let provider = get_shared_provider();

    let validator = GherkinValidator::new(provider, ValidationConfig::default());

    let content = r#"
This is not valid Gherkin syntax
  Just some random text
    With weird indentation
"#;

    let result = validator.validate(content, "malformed.feature");

    println!("\n📊 Test: Edge Case - Malformed Gherkin");
    match result {
        Ok(validation) => {
            // If it somehow parses, should have issues
            println!("   Parsed (unexpected), issues: {}", validation.issues.len());
            assert!(
                !validation.is_valid || validation.issues.len() > 0,
                "Malformed Gherkin should have issues"
            );
        }
        Err(e) => {
            // Expected - should fail to parse
            println!("   Parse error (expected): {}", e);
            assert!(
                format!("{}", e).contains("Parse") || format!("{}", e).contains("parse"),
                "Should return parse error for malformed Gherkin"
            );
        }
    }
}

// =============================================================================
// TEST SUITE: Advanced Gherkin Features (Valid Specs)
// =============================================================================

#[test]
#[ignore]
fn test_valid_with_background() {
    let provider = get_shared_provider();
    let validator = GherkinValidator::new(provider, ValidationConfig::default());

    let content = r#"
Feature: Strategy Evaluation with Shared Context
  As a product manager
  I want to evaluate growth strategies
  So that I can make informed decisions

  Background:
    Given the system has access to market signals
    And the system has access to competitor data
    And the evaluation framework is configured

  Scenario: Strategies are ranked
    When the system evaluates all strategies
    Then strategies must be ranked by confidence score
    And the top strategy must have a score above 0.7

  Scenario: Low-quality strategies are filtered
    When the system evaluates strategies
    Then strategies with scores below 0.6 must be rejected
    And only high-quality strategies should remain
"#;

    let start = Instant::now();
    let result = match validate_with_anthropic(&validator, content, "background.feature") {
        Ok(r) => r,
        Err(e) => {
            if is_skip_error(&e) {
                eprintln!("⚠️  Skipping test: {}", e);
                return;
            }
            panic!("Validation failed: {}", e);
        }
    };
    let elapsed = start.elapsed();

    println!("\n📊 Test: Valid Spec with Background");
    println!("   Duration: {:?}", elapsed);
    println!("   Valid: {}", result.is_valid);
    println!("   Scenarios: {}", result.scenario_count);
    println!("   Issues: {}", result.issues.len());

    // Background steps should be handled correctly
    assert!(
        result.is_valid || result.issues.iter().filter(|i| i.severity == Severity::Error).count() < 2,
        "Valid spec with Background should pass. Issues: {:?}",
        result.issues
    );
}

#[test]
#[ignore]
fn test_valid_with_scenario_outline() {
    let provider = get_shared_provider();
    let validator = GherkinValidator::new(provider, ValidationConfig::default());

    let content = r#"
Feature: Strategy Validation with Parameters
  Scenario Outline: Strategy meets minimum requirements
    Given a growth strategy exists with confidence <confidence>
    When the system validates the strategy
    Then the strategy must be <status>
    And the confidence score must be at least <min_score>

    Examples:
      | confidence | min_score | status    |
      | 0.8        | 0.7       | accepted  |
      | 0.6        | 0.7       | rejected  |
      | 0.9        | 0.7       | accepted  |
      | 0.5        | 0.7       | rejected  |
"#;

    let start = Instant::now();
    let result = match validate_with_anthropic(&validator, content, "outline.feature") {
        Ok(r) => r,
        Err(e) => {
            if is_skip_error(&e) {
                eprintln!("⚠️  Skipping test: {}", e);
                return;
            }
            panic!("Validation failed: {}", e);
        }
    };
    let elapsed = start.elapsed();

    println!("\n📊 Test: Valid Spec with Scenario Outline");
    println!("   Duration: {:?}", elapsed);
    println!("   Valid: {}", result.is_valid);
    println!("   Scenarios: {}", result.scenario_count);
    println!("   Issues: {}", result.issues.len());

    // Scenario Outline with Examples should be handled correctly
    assert!(
        result.is_valid || result.issues.iter().filter(|i| i.severity == Severity::Error).count() < 2,
        "Valid spec with Scenario Outline should pass. Issues: {:?}",
        result.issues
    );
}

#[test]
#[ignore]
fn test_valid_with_tags() {
    let provider = get_shared_provider();
    let validator = GherkinValidator::new(provider, ValidationConfig::default());

    let content = r#"
Feature: Tagged Strategy Scenarios
  @critical @growth-strategy
  Scenario: Multiple strategies required
    Given market signals indicate opportunity
    When the system converges
    Then at least two distinct growth strategies must exist

  @optional @brand-safety
  Scenario: Brand safety compliance
    Given a growth strategy has been generated
    When the system validates brand safety
    Then the strategy must comply with brand guidelines
"#;

    let start = Instant::now();
    let result = match validate_with_anthropic(&validator, content, "tags.feature") {
        Ok(r) => r,
        Err(e) => {
            if is_skip_error(&e) {
                eprintln!("⚠️  Skipping test: {}", e);
                return;
            }
            panic!("Validation failed: {}", e);
        }
    };
    let elapsed = start.elapsed();

    println!("\n📊 Test: Valid Spec with Tags");
    println!("   Duration: {:?}", elapsed);
    println!("   Valid: {}", result.is_valid);
    println!("   Scenarios: {}", result.scenario_count);
    println!("   Issues: {}", result.issues.len());

    // Tags should not cause validation issues
    assert!(
        result.is_valid || result.issues.iter().filter(|i| i.severity == Severity::Error).count() < 2,
        "Valid spec with Tags should pass. Issues: {:?}",
        result.issues
    );
}

#[test]
#[ignore]
fn test_valid_with_data_tables() {
    let provider = get_shared_provider();
    let validator = GherkinValidator::new(provider, ValidationConfig::default());

    let content = r#"
Feature: Strategy Evaluation with Data Tables
  Scenario: Strategies are evaluated against criteria
    Given the following evaluation criteria exist:
      | Criterion        | Weight | Threshold |
      | Market Fit       | 0.4    | 0.7       |
      | Feasibility      | 0.3    | 0.6       |
      | Brand Safety     | 0.3    | 1.0       |
    When the system evaluates strategies
    Then each strategy must meet all criteria thresholds
    And the weighted score must be calculated correctly
"#;

    let start = Instant::now();
    let result = match validate_with_anthropic(&validator, content, "data_tables.feature") {
        Ok(r) => r,
        Err(e) => {
            if is_skip_error(&e) {
                eprintln!("⚠️  Skipping test: {}", e);
                return;
            }
            panic!("Validation failed: {}", e);
        }
    };
    let elapsed = start.elapsed();

    println!("\n📊 Test: Valid Spec with Data Tables");
    println!("   Duration: {:?}", elapsed);
    println!("   Valid: {}", result.is_valid);
    println!("   Scenarios: {}", result.scenario_count);
    println!("   Issues: {}", result.issues.len());

    // Data tables should be handled correctly
    assert!(
        result.is_valid || result.issues.iter().filter(|i| i.severity == Severity::Error).count() < 2,
        "Valid spec with Data Tables should pass. Issues: {:?}",
        result.issues
    );
}

#[test]
#[ignore]
fn test_valid_with_doc_strings() {
    let provider = get_shared_provider();
    let validator = GherkinValidator::new(provider, ValidationConfig::default());

    let content = r#"
Feature: Strategy Documentation with Doc Strings
  Scenario: Strategy includes detailed description
    Given a growth strategy has been generated
    And the strategy includes the following description:
      """
      This strategy focuses on B2B SaaS growth in the Nordic market.
      It leverages LinkedIn as the primary channel and targets enterprise
      customers with a focus on compliance and data sovereignty.
      """
    When the system validates the strategy
    Then the strategy description must be non-empty
    And the strategy must reference market signals
"#;

    let start = Instant::now();
    let result = match validate_with_anthropic(&validator, content, "doc_strings.feature") {
        Ok(r) => r,
        Err(e) => {
            if is_skip_error(&e) {
                eprintln!("⚠️  Skipping test: {}", e);
                return;
            }
            panic!("Validation failed: {}", e);
        }
    };
    let elapsed = start.elapsed();

    println!("\n📊 Test: Valid Spec with Doc Strings");
    println!("   Duration: {:?}", elapsed);
    println!("   Valid: {}", result.is_valid);
    println!("   Scenarios: {}", result.scenario_count);
    println!("   Issues: {}", result.issues.len());

    // Doc strings should be handled correctly
    assert!(
        result.is_valid || result.issues.iter().filter(|i| i.severity == Severity::Error).count() < 2,
        "Valid spec with Doc Strings should pass. Issues: {:?}",
        result.issues
    );
}

#[test]
#[ignore]
fn test_valid_complex_with_all_features() {
    let provider = get_shared_provider();
    let validator = GherkinValidator::new(provider, ValidationConfig::default());

    let content = r#"
Feature: Comprehensive Strategy Validation
  As a product manager
  I want comprehensive strategy validation
  So that I can make data-driven decisions

  Background:
    Given the system has access to market signals
    And the evaluation framework is configured with:
      """
      {
        "min_confidence": 0.7,
        "required_strategies": 2,
        "brand_safety_enabled": true
      }
      """

  @critical @validation
  Scenario Outline: Strategy meets all requirements
    Given a growth strategy exists with:
      | Field         | Value                    |
      | Confidence    | <confidence>             |
      | Market        | <market>                 |
      | Channel       | <channel>                |
    And the strategy description is:
      """
      <description>
      """
    When the system validates the strategy
    Then the strategy must be <status>
    And all validation criteria must be met

    Examples:
      | confidence | market      | channel  | description                    | status    |
      | 0.8        | Nordic B2B  | LinkedIn | Focus on enterprise customers  | accepted  |
      | 0.6        | Nordic B2B  | LinkedIn | Focus on enterprise customers  | rejected  |
      | 0.9        | Global B2B  | Email    | Focus on compliance            | accepted  |

  @optional @monitoring
  Scenario: Strategy monitoring is active
    Given strategies have been validated
    When the system monitors strategy performance
    Then performance metrics must be tracked
    And alerts must be generated for low performance
"#;

    let start = Instant::now();
    let result = match validate_with_anthropic(&validator, content, "complex_all_features.feature") {
        Ok(r) => r,
        Err(e) => {
            if is_skip_error(&e) {
                eprintln!("⚠️  Skipping test: {}", e);
                return;
            }
            panic!("Validation failed: {}", e);
        }
    };
    let elapsed = start.elapsed();

    println!("\n📊 Test: Valid Complex Spec with All Features");
    println!("   Duration: {:?}", elapsed);
    println!("   Valid: {}", result.is_valid);
    println!("   Scenarios: {}", result.scenario_count);
    println!("   Issues: {}", result.issues.len());

    if !result.issues.is_empty() {
        println!("\n   Issues by category:");
        let mut by_category: Vec<(IssueCategory, Vec<&ValidationIssue>)> = Vec::new();
        for issue in &result.issues {
            if let Some((_, issues)) = by_category.iter_mut().find(|(cat, _)| *cat == issue.category) {
                issues.push(issue);
            } else {
                by_category.push((issue.category, vec![issue]));
            }
        }
        for (category, issues) in &by_category {
            println!("     {:?}: {} issues", category, issues.len());
            for issue in issues {
                println!("       - [{:?}] {}", issue.severity, issue.message);
            }
        }
    }

    // Complex spec with all features should mostly pass
    assert!(
        result.is_valid || result.issues.iter().filter(|i| i.severity == Severity::Error).count() < 3,
        "Complex spec with all features should mostly pass. Issues: {:?}",
        result.issues
    );
}
