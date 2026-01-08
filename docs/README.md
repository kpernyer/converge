# Converge Documentation

Knowledge base for the Converge semantic convergence engine.

---

## 📚 Knowledge Base Structure

### [architecture/](./architecture/)
Core system architecture, execution model, and convergence guarantees.

**Key Documents:**
- `ARCHITECTURE.md` — High-level system architecture
- `ENGINE_EXECUTION_MODEL.md` — Execution and convergence detection
- `CONVERGENCE_SEMANTICS.md` — Convergence guarantees
- `ROOT_INTENT_SCHEMA.md` — Entry point schema
- `FAILURE_MODES.md` — Failure handling
- `SCALING_MODEL.md` — Scaling approach
- `CORE_CONCEPTS.md` — Essential concepts without implementation details
- `API_OVERVIEW.md` — Public API overview

### [agents/](./agents/)
Agent model, lifecycle, LLM integration, and human-in-the-loop patterns.

**Key Documents:**
- `AGENT_MODEL.md` — Agent trait and interface
- `AGENT_LIFECYCLE.md` — Agent lifecycle phases
- `LLM_INTEGRATION.md` — LLM agent integration
- `PROMPT_CONTRACT.md` — Prompt structuring for LLMs
- `HUMAN_IN_THE_LOOP.md` — Human approval patterns

### [governance/](./governance/)
Design tenets, terminology, and core principles.

**Key Documents:**
- `DESIGN_TENETS.md` — The 9 non-negotiable principles
- `GOVERNANCE.md` — Core manifesto and philosophy
- `TERMINOLOGY.md` — Precise definitions

### [testing/](./testing/)
Testing strategies, property testing, and invariant enforcement.

**Key Documents:**
- `INVARIANTS.md` — Gherkin invariant system

### [product/](./product/)
Product guide, FAQ, usage instructions, and strategic planning.

**Key Documents:**
- `PRODUCT_GUIDE.md` — When to use Converge
- `USAGE_GUIDE.md` — How to use the Converge Core library
- `FAQ.md` — Frequently asked questions
- `LONG_TERM_STRATEGIC_PLAN.md` — Strategic roadmap

### [deployment/](./deployment/)
Technology stack, deployment guides, and communication patterns.

**Key Documents:**
- `TECHNOLOGY_STACK.md` — Mandatory technology choices
- `DEPLOYMENT.md` — Deployment and operations
- `COMMUNICATION_MODEL.md` — Agent communication model

### [use-cases/](./use-cases/)
Concrete examples showing how Converge solves real problems.

**Key Documents:**
- `USE_CASE_TRACKER.md` — Implementation status
- `CONVERGE_GROWTH_STRATEGY_USECASE.md` — Growth strategy example
- `CONVERGE_MEETING_SCHEDULER_USECASE.md` — Meeting scheduler example
- `CONVERGE_RESOURCE_ROUTING_USECASE.md` — Resource routing example

### [reference/](./reference/)
Understanding Converge in context of other systems and patterns.

**Key Documents:**
- `WHY_NOT_ACTORS.md` — Why not actor systems
- `TEMPORAL_MODEL.md` — Temporal integration
- `DISTRIBUTED_SYSTEMS.md` — Distributed systems considerations
- `REFERENCE_ARCHITECTURES.md` — Reference architectures

### [development/](./development/)
Implementation status, plans, decisions, and repository guidelines.

**Key Documents:**
- `STATUS.md` — Current implementation status
- `DECISIONS.md` — Authoritative implementation decisions
- `TASKS.md` — Task lists
- `PROJECT_PLAN.md` — Project milestones
- `REPOSITORY_GUIDELINES.md` — Project structure and coding standards
- `SPECIFICATION_COMPLIANCE_ASSESSMENT.md` — Compliance assessment

### [assistant-guides/](./assistant-guides/)
Guidelines for AI assistants working on Converge.

**Key Documents:**
- `Rust-Best-Practices-v2.md` — Rust coding standards
- `cursor-use-case-owner-coder.md` — Use-case owner role
- `codex-assistant-coder.md` — Codex assistant guide
- `gemini-cloudops.md` — Gemini cloud ops guide

### [internal/](./internal/)
Internal documentation for core maintainers.

---

## 🚀 Quick Start Paths

### For New Contributors
1. Read `governance/DESIGN_TENETS.md`
2. Read `governance/TERMINOLOGY.md`
3. Read `architecture/ARCHITECTURE.md`
4. Review `development/STATUS.md` for current state

### For Domain Experts / Use-Case Authors
1. Read `product/PRODUCT_GUIDE.md`
2. Review `use-cases/` examples
3. Read `assistant-guides/cursor-use-case-owner-coder.md`
4. Study `architecture/ROOT_INTENT_SCHEMA.md`

### For AI Assistants
1. Read root `AGENTS.md` (comprehensive guide)
2. Read `assistant-guides/cursor-use-case-owner-coder.md` (use-case role)
3. Review `development/DECISIONS.md` (authoritative choices)
4. Study `architecture/` for system understanding

### For System Architects
1. Read `architecture/ARCHITECTURE.md`
2. Read `architecture/ENGINE_EXECUTION_MODEL.md`
3. Read `architecture/CONVERGENCE_SEMANTICS.md`
4. Review `reference/` for context

---

## 📖 Document Relationships

### Core Flow
```
governance/GOVERNANCE.md → architecture/ARCHITECTURE.md → architecture/ENGINE_EXECUTION_MODEL.md → use-cases/
```

### Implementation Flow
```
development/PROJECT_PLAN.md → development/STATUS.md → development/TASKS.md → development/DECISIONS.md
```

### Use-Case Flow
```
use-cases/ → architecture/ROOT_INTENT_SCHEMA.md → testing/INVARIANTS.md
```

---

## 🔍 Finding Information

### By Topic

**Convergence:**
- `architecture/CONVERGENCE_SEMANTICS.md` (how it works)
- `architecture/CONVERGENCE_PROOFS.md` (why it's guaranteed)
- `architecture/FAILURE_MODES.md` (what can go wrong)

**Agents:**
- `agents/AGENT_MODEL.md` (agent model)
- `agents/AGENT_LIFECYCLE.md` (agent lifecycle)
- `agents/LLM_INTEGRATION.md` (LLM agents)

**Context:**
- `architecture/ARCHITECTURE.md` (overview)
- `architecture/ROOT_INTENT_SCHEMA.md` (root intent)
- `use-cases/CONTEXT_SCHEMA_GROWTH.md` (example schema)

**Invariants:**
- `testing/INVARIANTS.md` (invariant system)
- `agents/HUMAN_IN_THE_LOOP.md` (human-in-the-loop)

**Implementation:**
- `development/DECISIONS.md` (authoritative choices)
- `development/STATUS.md` (what's built)
- `development/TASKS.md` (what's next)
- `deployment/DEPLOYMENT.md` (deployment and operations)

---

## 🔗 External References

- **Code:** `converge-core/` — Rust implementation
- **Rules:** `AGENTS.md` — AI assistant guide (root directory)
- **Build:** `Justfile` — Development commands

---

## 💡 Tips for AI Agents

When working with this documentation:

1. **Always check `development/DECISIONS.md`** for authoritative implementation choices
2. **Respect the architecture** — don't propose changes that violate core principles
3. **Use use-cases as templates** — they show the pattern for new domains
4. **Refer to terminology** — use `governance/TERMINOLOGY.md` for precise definitions
5. **Check status** — `development/STATUS.md` shows what's already implemented

---

## 📅 Last Updated

Documentation restructured as knowledge base: January 2025
