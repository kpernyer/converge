# Internal Documentation

This directory contains internal implementation details, design specifications, and architecture decisions for the Converge Core library.

## ⚠️ Internal Use Only

These documents are **not** part of the public API documentation. They describe:
- Implementation details
- Design decisions and rationale
- Internal architecture
- Execution model specifics
- Convergence semantics

## For Core Maintainers

These documents are intended for those maintaining the `converge-core` library itself. They contain detailed specifications that drive the implementation.

## For Contributors

If you're contributing to other modules (domain, provider, runtime, tool), you typically don't need these internal specs. See:
- [Public Documentation](../public/) — For using the library
- [Development Documentation](../05-development/) — For contributing to other modules

## Structure

The internal architecture documentation is organized in the parent `docs/` directory:
- `02-architecture/` — Detailed architecture and execution model
- `05-development/` — Development guides and decisions

These are marked as internal because they expose implementation details that are not part of the public API contract.

