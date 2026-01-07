#!/bin/bash
# Run integration tests for prompt formatting
# Requires: ANTHROPIC_API_KEY environment variable

set -e

if [ -z "$ANTHROPIC_API_KEY" ]; then
    echo "Error: ANTHROPIC_API_KEY environment variable is not set"
    echo "Please set it before running integration tests:"
    echo "  export ANTHROPIC_API_KEY='your-api-key-here'"
    exit 1
fi

echo "Running integration tests for prompt formatting..."
echo "Using model: claude-3-5-haiku-20241022"
echo ""

# Run all integration tests
cargo test --test integration_prompt_formatting -- --ignored --nocapture

echo ""
echo "Integration tests completed!"

