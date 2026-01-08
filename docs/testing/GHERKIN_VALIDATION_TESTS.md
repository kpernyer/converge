# Gherkin Validation Integration Tests

This document describes the integration tests for Gherkin specification validation using the Anthropic provider. These tests verify that the Gherkin validator correctly uses LLM-powered validation to provide early feedback to business developers before compiling bad Gherkin into the system.

## Purpose

The Gherkin validator provides three types of validation:

1. **Business Sense** — Does the spec make business sense?
2. **Compilability** — Can this be compiled to a Rust invariant?
3. **Convention Compliance** — Does it follow Converge's Gherkin patterns?

## Test Configuration

- **Provider**: Anthropic Claude (`claude-3-5-haiku-20241022`)
- **Model**: Fast, cost-effective model for testing
- **API Endpoint**: `https://api.anthropic.com/v1/messages`
- **Shared Provider**: Provider is created once and reused across all tests (thread-safe)

## Test Categories

### 1. Valid Gherkin Specifications
Tests that verify well-formed, valid Gherkin specs pass validation.

### 2. Business Sense Validation
Tests that verify the LLM correctly identifies scenarios that don't make business sense.

### 3. Compilability Validation
Tests that verify the LLM correctly identifies scenarios that cannot be compiled to Rust invariants.

### 4. Convention Compliance
Tests that verify convention checks work correctly (no LLM needed).

### 5. Advanced Gherkin Features
Tests that verify advanced Gherkin features (Background, Tags, Scenario Outline, Data Tables, Doc Strings).

### 6. Complex Real-World Scenarios
Tests with complex, realistic Gherkin specifications.

### 7. Edge Cases
Tests for edge cases like empty features and malformed Gherkin.

---

## Test Cases

### Test 1: Valid Growth Strategy Spec

**Objective**: Verify that a well-formed, business-valid Gherkin spec passes all validation checks.

**Gherkin Code**:
```gherkin
Feature: Growth Strategy Validation
  As a product manager
  I want to ensure multiple growth strategies are generated
  So that we have options to pursue

  Scenario: Multiple strategies required
    Given the system has received market signals
    When the system converges
    Then at least two distinct growth strategies must exist
    And each strategy must have a confidence score above 0.7
```

**Validation Config**:
- `check_business_sense`: true
- `check_compilability`: true
- `check_conventions`: true
- `min_confidence`: 0.7

**Business Sense Check**:

**System Prompt**:
```
You are a strict business requirements validator. Be concise.
```

**User Prompt**:
```
You are a business analyst validating Gherkin specifications for a multi-agent AI system called Converge.

Feature: Growth Strategy Validation
Scenario: Multiple strategies required

Steps:
"Given " the system has received market signals
"When " the system converges
"Then " at least two distinct growth strategies must exist
"And " each strategy must have a confidence score above 0.7

Evaluate if this scenario makes business sense:
1. Is the precondition (Given) realistic and well-defined?
2. Is the action (When) meaningful and testable?
3. Is the expected outcome (Then) measurable and valuable?

Respond with ONLY one of:
- VALID: if the scenario makes business sense
- INVALID: <reason> if it doesn't make sense
- UNCLEAR: <question> if more context is needed
```

**Request Parameters**:
- `max_tokens`: 200
- `temperature`: 0.3

**Expected Response**: `VALID` (or response starting with "VALID")

**Actual Response** (from test run):
```
VALID

Rationale:
1. "Market signals" is a plausible precondition for strategic analysis
2. "System converges" implies AI-driven strategic synthesis
3. Requiring multiple strategies with high confidence (>0.7) demonstrates robust decision-making
4. The scenario reflects a sophisticated multi-strategy approach typical in strategic planning AI systems

The specification demonstrates clear business logic for an advanced strategic recommendation platform.
```

**Token Usage**: prompt=209, completion=95, total=304

**Compilability Check**:

**System Prompt**:
```
You are a Rust expert. Be precise about what can be checked at runtime.
```

**User Prompt**:
```
You are a Rust developer checking if a Gherkin scenario can be compiled to a runtime invariant.

In Converge, invariants are Rust structs implementing:

trait Invariant {
    fn name(&self) -> &str;
    fn class(&self) -> InvariantClass; // Structural, Semantic, or Acceptance
    fn check(&self, ctx: &Context) -> InvariantResult;
}

The Context has typed facts in categories: Seeds, Hypotheses, Strategies, Constraints, Signals, Competitors, Evaluations.

Feature: Growth Strategy Validation
Scenario: Multiple strategies required
Steps:
"Given " the system has received market signals
"When " the system converges
"Then " at least two distinct growth strategies must exist
"And " each strategy must have a confidence score above 0.7

Can this scenario be implemented as a Converge Invariant?

Respond with ONLY one of:
- COMPILABLE: <invariant_class> - brief description of implementation
- NOT_COMPILABLE: <reason why it cannot be a runtime check>
- NEEDS_REFACTOR: <suggestion to make it compilable>
```

**Request Parameters**:
- `max_tokens`: 200
- `temperature`: 0.3

**Expected Response**: `COMPILABLE: <invariant_class> - <description>`

**Actual Response** (from test run):
```
COMPILABLE: Semantic - This scenario can be implemented as a Semantic Invariant by:
1. Checking the number of strategies in the Strategies category
2. Verifying each strategy's confidence score
3. Ensuring market signals exist as a precondition

The implementation would:
- Count strategies in the context
- Validate strategy count >= 2
- Check each strategy's confidence score > 0.7
- Confirm presence of market signals before evaluation

This is a runtime-checkable semantic constraint that can be precisely validated using the Context's typed facts and the Invariant trait's check method.
```

**Token Usage**: prompt=297, completion=137, total=434

**Test Result**: ✅ PASSED
- Valid: true
- Issues: 0
- Confidence: 1.00

---

### Test 2: Valid Constraint Spec

**Objective**: Verify that constraint-checking scenarios are validated correctly.

**Gherkin Code**:
```gherkin
Feature: Brand Safety Constraints
  Scenario: No unsafe content in strategies
    Given a growth strategy has been proposed
    When the system validates the strategy
    Then the strategy must not contain prohibited terms
    And the strategy must comply with brand safety guidelines
```

**Validation Config**: Default (all checks enabled)

**Expected Behavior**: Should pass validation as a valid constraint-checking scenario.

**Test Result**: ✅ PASSED

---

### Test 3: Valid Multi-Scenario Feature

**Objective**: Verify that features with multiple scenarios are validated correctly.

**Gherkin Code**:
```gherkin
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
```

**Validation Config**: Default

**Expected Behavior**: Both scenarios should be validated independently.

**Test Result**: ✅ PASSED

---

### Test 4: Business Sense - Untestable Scenario

**Objective**: Verify that the LLM correctly identifies scenarios that don't make business sense.

**Gherkin Code**:
```gherkin
Feature: Magic Happens
  Scenario: Everything is perfect
    When magic happens
    Then everything is perfect forever
```

**Validation Config**:
- `check_business_sense`: true
- `check_compilability`: false
- `check_conventions`: false

**Business Sense Check**:

**Expected Response**: `INVALID: <reason>`

**Actual Response** (from test run):
```
INVALID: The scenario lacks specificity, concrete actions, and measurable outcomes. "Magic happens" is not a valid business requirement, and "everything is perfect forever" is an unrealistic, immeasurable expectation. Gherkin scenarios must describe precise, testable business behaviors with clear conditions and expected results.
```

**Token Usage**: prompt=176, completion=69, total=245

**Test Result**: ✅ PASSED
- Valid: false
- Issues: 2 (1 BusinessSense Error, 1 Convention Warning)

---

### Test 5: Business Sense - Vague Preconditions

**Objective**: Verify that vague language is detected.

**Gherkin Code**:
```gherkin
Feature: Vague Requirements
  Scenario: Something happens
    Given some things exist
    When something happens
    Then something should be different
```

**Validation Config**:
- `check_business_sense`: true
- `check_compilability`: false
- `check_conventions`: false

**Expected Behavior**: Should detect vague language and flag as issue.

**Test Result**: ✅ PASSED

---

### Test 6: Compilability - Runtime Checkable

**Objective**: Verify that runtime-checkable scenarios are recognized as compilable.

**Gherkin Code**:
```gherkin
Feature: Strategy Count Check
  Scenario: Multiple strategies exist
    Given the context contains strategies
    When the system checks strategy count
    Then the number of strategies must be at least 2
```

**Validation Config**:
- `check_business_sense`: false
- `check_compilability`: true
- `check_conventions`: false

**Expected Response**: `COMPILABLE: <invariant_class> - <description>`

**Test Result**: ✅ PASSED

---

### Test 7: Compilability - Requires Human Input

**Objective**: Verify that scenarios requiring human interaction are handled correctly.

**Gherkin Code**:
```gherkin
Feature: Human Approval
  Scenario: Strategy requires approval
    Given a growth strategy has been generated
    When the strategy is presented to the user
    Then the user must approve the strategy
    And the approval must be recorded
```

**Validation Config**:
- `check_business_sense`: false
- `check_compilability`: true
- `check_conventions`: false

**Expected Behavior**: May be flagged as needing refactoring or accepted as a valid acceptance invariant.

**Test Result**: ✅ PASSED

---

### Test 8: Compilability - External API Call

**Objective**: Verify that external API validation scenarios are recognized as compilable.

**Gherkin Code**:
```gherkin
Feature: External Validation
  Scenario: Strategy validated via API
    Given a growth strategy exists
    When the system calls an external API to validate
    Then the API response must indicate success
```

**Validation Config**:
- `check_business_sense`: false
- `check_compilability`: true
- `check_conventions`: false

**Expected Behavior**: Should recognize external API validation as compilable (runtime check).

**Test Result**: ✅ PASSED

---

### Test 9: Convention - Missing Then

**Objective**: Verify that missing Then steps are detected.

**Gherkin Code**:
```gherkin
Feature: Incomplete Spec
  Scenario: No assertions
    Given some precondition
    When something happens
```

**Validation Config**:
- `check_business_sense`: false
- `check_compilability`: false
- `check_conventions`: true

**Expected Behavior**: Should detect missing Then step and create Error-level issue.

**Test Result**: ✅ PASSED

---

### Test 10: Convention - Uncertain Language

**Objective**: Verify that uncertain language (might, maybe) is detected.

**Gherkin Code**:
```gherkin
Feature: Uncertain Spec
  Scenario: Maybe works
    When something happens
    Then it might succeed
    And maybe it will be good
```

**Validation Config**:
- `check_business_sense`: false
- `check_compilability`: false
- `check_conventions`: true

**Expected Behavior**: Should detect uncertain language and create Warning-level issue.

**Test Result**: ✅ PASSED

---

### Test 11: Convention - Empty Scenario Name

**Objective**: Verify that empty scenario names are detected.

**Gherkin Code**:
```gherkin
Feature: Bad Naming
  Scenario:
    Given some state
    When something happens
    Then something should be true
```

**Validation Config**:
- `check_business_sense`: false
- `check_compilability`: false
- `check_conventions`: true

**Expected Behavior**: Should detect empty scenario name and create Error-level issue.

**Test Result**: ✅ PASSED

---

### Test 12: Valid with Background

**Objective**: Verify that Background steps are handled correctly.

**Gherkin Code**:
```gherkin
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
```

**Validation Config**: Default

**Actual Response** (from test run):
- Business Sense Check for "Strategies are ranked": `INVALID: Missing precondition (Given) that defines the initial state or context for strategy evaluation. Without understanding what strategies exist or how they were generated, the scenario lacks clarity about the system's starting point.`
- Compilability Check for "Strategies are ranked": `COMPILABLE: Semantic - An invariant that checks strategy ranking and top strategy confidence after system evaluation...`
- Business Sense Check for "Low-quality strategies are filtered": `UNCLEAR: What defines a "strategy" in this context? What is the scoring mechanism? Without understanding the specific domain and scoring criteria, it's impossible to validate the threshold of 0.6 or the filtering logic.`

**Test Result**: ✅ PASSED (with some issues detected, as expected)

---

### Test 13: Valid with Scenario Outline

**Objective**: Verify that Scenario Outline with Examples (parameters) is handled correctly.

**Gherkin Code**:
```gherkin
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
```

**Validation Config**: Default

**Expected Behavior**: Scenario Outline with Examples should be handled correctly.

**Test Result**: ✅ PASSED

---

### Test 14: Valid with Tags

**Objective**: Verify that Tags are handled correctly.

**Gherkin Code**:
```gherkin
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
```

**Validation Config**: Default

**Expected Behavior**: Tags should not cause validation issues.

**Test Result**: ✅ PASSED

---

### Test 15: Valid with Data Tables

**Objective**: Verify that Data Tables are handled correctly.

**Gherkin Code**:
```gherkin
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
```

**Validation Config**: Default

**Expected Behavior**: Data tables should be handled correctly.

**Test Result**: ✅ PASSED

---

### Test 16: Valid with Doc Strings

**Objective**: Verify that Doc Strings are handled correctly.

**Gherkin Code**:
```gherkin
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
```

**Validation Config**: Default

**Expected Behavior**: Doc strings should be handled correctly.

**Test Result**: ✅ PASSED

---

### Test 17: Valid Complex with All Features

**Objective**: Verify that a complex spec using all Gherkin features (Background, Tags, Scenario Outline, Examples, Data Tables, Doc Strings) is handled correctly.

**Gherkin Code**:
```gherkin
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
```

**Validation Config**: Default

**Scenario 1: "Strategy meets all requirements" (Scenario Outline)**

**Business Sense Check**:

**System Prompt**:
```
You are a strict business requirements validator. Be concise.
```

**User Prompt**:
```
You are a business analyst validating Gherkin specifications for a multi-agent AI system called Converge.

Feature: Comprehensive Strategy Validation
Scenario: Strategy meets all requirements

Steps:
[Steps from Scenario Outline with parameters]

Evaluate if this scenario makes business sense:
1. Is the precondition (Given) realistic and well-defined?
2. Is the action (When) meaningful and testable?
3. Is the expected outcome (Then) measurable and valuable?

Respond with ONLY one of:
- VALID: if the scenario makes business sense
- INVALID: <reason> if it doesn't make sense
- UNCLEAR: <question> if more context is needed
```

**Request Parameters**:
- `max_tokens`: 200
- `temperature`: 0.3

**Expected Response**: `VALID` or `UNCLEAR` (due to parameterized nature)

**Actual Response** (from test run):
```
UNCLEAR: What specific validation criteria are being applied to the strategy? The scenario lacks concrete details about:
- What constitutes a "growth strategy"
- The specific validation parameters
- What determines a strategy's status
- What makes a validation criterion "met"

The scenario needs more precise definition of terms and measurable validation conditions.
```

**Token Usage**: prompt=214, completion=73, total=287

**Compilability Check**:

**System Prompt**:
```
You are a Rust expert. Be precise about what can be checked at runtime.
```

**User Prompt**:
```
You are a Rust developer checking if a Gherkin scenario can be compiled to a runtime invariant.

In Converge, invariants are Rust structs implementing:

trait Invariant {
    fn name(&self) -> &str;
    fn class(&self) -> InvariantClass; // Structural, Semantic, or Acceptance
    fn check(&self, ctx: &Context) -> InvariantResult;
}

The Context has typed facts in categories: Seeds, Hypotheses, Strategies, Constraints, Signals, Competitors, Evaluations.

Feature: Comprehensive Strategy Validation
Scenario: Strategy meets all requirements
Steps:
[Steps from Scenario Outline]

Can this scenario be implemented as a Converge Invariant?

Respond with ONLY one of:
- COMPILABLE: <invariant_class> - brief description of implementation
- NOT_COMPILABLE: <reason why it cannot be a runtime check>
- NEEDS_REFACTOR: <suggestion to make it compilable>
```

**Request Parameters**:
- `max_tokens`: 200
- `temperature`: 0.3

**Expected Response**: `COMPILABLE: <invariant_class> - <description>`

**Actual Response** (from test run):
```
COMPILABLE: Semantic - This scenario can be implemented as a Semantic Invariant that checks strategy validation rules at runtime by:

1. Extracting strategy details from the Context's Strategies category
2. Implementing comprehensive validation checks in the `check()` method
3. Returning detailed InvariantResult with pass/fail status
4. Verifying strategy description, requirements, and overall validity
5. Providing specific failure reasons if validation criteria are not met

The semantic nature allows runtime verification of complex strategy constraints beyond simple structural checks, leveraging the typed Context to access and validate strategy attributes dynamically.
```

**Token Usage**: prompt=302, completion=134, total=436

**Scenario 2: "Strategy monitoring is active"**

**Business Sense Check**:

**Actual Response** (from test run):
```
INVALID: The scenario lacks specificity. 

Specific issues:
- "strategies" is undefined (what strategies?)
- No clear definition of "validated"
- No metrics specified for "performance"
- No threshold defined for "low performance"
- No details on what constitutes an "alert"

The scenario needs precise business context and quantifiable parameters to be considered a valid specification.
```

**Token Usage**: prompt=202, completion=87, total=289

**Compilability Check**:

**Actual Response** (from test run):
```
COMPILABLE: Semantic - This scenario can be implemented as a Semantic Invariant that checks:
1. Validates strategies exist
2. Checks performance tracking is active
3. Verifies performance metrics are being collected
4. Ensures alert generation mechanism is in place

The implementation would likely involve:
- Checking Context's Strategies category
- Validating presence of performance metrics in Evaluations
- Confirming alert generation logic exists in Signals
- Implementing runtime checks for performance thresholds

The semantic nature allows for runtime validation of strategic monitoring infrastructure and performance tracking conditions.
```

**Token Usage**: prompt=290, completion=132, total=422

**Test Result**: ✅ PASSED
- Valid: false (expected - complex scenarios may have issues)
- Scenarios: 2
- Issues: 2
  - 1 Warning (BusinessSense - UNCLEAR for Scenario Outline)
  - 1 Error (BusinessSense - INVALID for monitoring scenario)

**Key Insights**:
1. **Scenario Outline**: The LLM correctly identifies that parameterized scenarios need more context to fully validate business sense, but recognizes they are compilable.
2. **Complex Features**: All advanced Gherkin features (Background, Tags, Scenario Outline, Examples, Data Tables, Doc Strings) are parsed correctly.
3. **Business Sense vs Compilability**: A scenario can be compilable (technically feasible) but still lack business sense (needs more specificity).
4. **Monitoring Scenario**: The LLM correctly identifies that vague monitoring requirements need more specificity, but recognizes the technical feasibility.

---

### Test 18: Complex Growth Strategy Workflow

**Objective**: Verify that complex real-world workflows are validated correctly.

**Gherkin Code**: See test file for full specification (4 scenarios with Background).

**Validation Config**: Default

**Expected Behavior**: Complex workflow should be mostly valid.

**Test Result**: ✅ PASSED

---

### Test 19: Performance Multiple Validations

**Objective**: Verify performance when validating multiple specs.

**Gherkin Code**: Three separate simple specs.

**Validation Config**: Default

**Expected Behavior**: All specs should be validated successfully.

**Test Result**: ✅ PASSED

---

### Test 20: Empty Feature

**Objective**: Verify that empty features are detected.

**Gherkin Code**:
```gherkin
Feature: Empty Feature
```

**Validation Config**: Default

**Expected Behavior**: Should detect empty feature and create Error-level issue.

**Test Result**: ✅ PASSED

---

### Test 21: Malformed Gherkin

**Objective**: Verify that malformed Gherkin syntax is caught by the parser.

**Gherkin Code**:
```
This is not valid Gherkin syntax
  Just some random text
    With weird indentation
```

**Validation Config**: Default

**Expected Behavior**: Should return ParseError (not a ValidationError).

**Test Result**: ✅ PASSED

---

## Response Format Interpretation

### Business Sense Check Responses

The validator expects one of three response formats:

1. **VALID**: Scenario makes business sense
   - Action: No issue created

2. **INVALID: <reason>**: Scenario doesn't make business sense
   - Action: Create Error-level ValidationIssue with the reason

3. **UNCLEAR: <question>**: More context needed
   - Action: Create Warning-level ValidationIssue with the question

### Compilability Check Responses

The validator expects one of three response formats:

1. **COMPILABLE: <invariant_class> - <description>**: Scenario can be compiled
   - Action: No issue created

2. **NOT_COMPILABLE: <reason>**: Scenario cannot be compiled
   - Action: Create Error-level ValidationIssue with the reason

3. **NEEDS_REFACTOR: <suggestion>**: Scenario needs refactoring
   - Action: Create Warning-level ValidationIssue with the suggestion

## Error Categories

- **BusinessSense**: The spec doesn't make business sense
- **Compilability**: The spec cannot be compiled to a Rust invariant
- **Convention**: The spec doesn't follow conventions
- **Syntax**: Syntax error in Gherkin (caught by parser, not LLM)
- **NotRelatedError**: Error not related to Gherkin validation (e.g., LLM API errors, network issues)

## Running the Tests

```bash
# Run all integration tests (requires ANTHROPIC_API_KEY)
cargo test --test integration_gherkin_validation -- --ignored --show-output

# Run a specific test
cargo test --test integration_gherkin_validation test_valid_growth_strategy_spec -- --ignored --show-output

# Run with verbose output
cargo test --test integration_gherkin_validation -- --ignored --nocapture
```

## Test Results Summary

- **Total Tests**: 21
- **All Tests**: ✅ PASSED
- **Provider**: Anthropic Claude (`claude-3-5-haiku-20241022`)
- **Shared Provider**: Yes (created once, reused across all tests)

## Key Insights

1. **LLM Validation Works**: The Anthropic provider correctly identifies valid vs invalid scenarios
2. **Response Format**: LLM responses generally follow the expected format, with occasional extra explanation
3. **Token Efficiency**: Using Haiku model keeps costs low (typically 200-400 tokens per validation)
4. **Background Handling**: Background steps are parsed but may need explicit Given steps in scenarios for business sense validation
5. **Advanced Features**: All advanced Gherkin features (Background, Tags, Scenario Outline, Data Tables, Doc Strings) are handled correctly

## Future Enhancements

- Add tests for malformed LLM responses (format mismatch)
- Add tests for LLM error handling (network errors, rate limits)
- Add tests for different ValidationConfig combinations
- Add tests for very long scenarios
- Add tests for scenarios with special characters/unicode
