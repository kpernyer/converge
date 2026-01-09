# converge-tool

Developer tools for the [Converge](https://crates.io/crates/converge-core) ecosystem.

## Gherkin Validation

LLM-powered validation of Gherkin feature specifications.

### Validation Dimensions

- **Convention Compliance** - Proper Given/When/Then structure, naming conventions
- **Compilability** - Can steps be automated without human intervention?
- **Business Sense** - Are scenarios testable and well-defined?

### Issue Detection

- Missing Then clauses
- Uncertain language ("should", "might", "probably")
- Vague preconditions
- Untestable assertions
- External API dependencies
- Human input requirements

## Installation

```toml
[dependencies]
converge-tool = "0.2"
```

## Example

```rust
use converge_tool::gherkin::{GherkinValidator, ValidationConfig};
use converge_provider::AnthropicProvider;

// Create validator with LLM provider
let provider = AnthropicProvider::from_env("claude-3-5-haiku-20241022")?;
let config = ValidationConfig::default();
let validator = GherkinValidator::new(provider, config);

// Parse and validate a feature file
let feature = gherkin::parse_feature(feature_text)?;
let report = validator.validate_feature(&feature)?;

// Check results
if report.is_valid() {
    println!("Feature is valid!");
} else {
    for issue in report.issues {
        println!("{}: {}", issue.severity, issue.message);
    }
}
```

### Validation Report

```rust
pub struct ValidationReport {
    pub feature_name: String,
    pub is_valid: bool,
    pub issues: Vec<ValidationIssue>,
    pub scenarios_checked: usize,
}

pub struct ValidationIssue {
    pub severity: Severity,  // Error, Warning, Info
    pub dimension: Dimension, // Convention, Compilability, BusinessSense
    pub message: String,
    pub location: Option<String>,
}
```

## License

MIT
