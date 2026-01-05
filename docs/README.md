# Converge Documentation Index

This directory contains all documentation for the Converge project, organized for both human readers and AI agents.

## 📚 Documentation Structure

### [01-core-philosophy/](./01-core-philosophy/)
**Start here** to understand what Converge is and why it exists.

- Core principles and manifesto
- Terminology and definitions
- When to use (and not use) Converge

### [02-architecture/](./02-architecture/)
**Deep dive** into system design, execution model, and core concepts.

- System architecture and layers
- Execution model and convergence semantics
- Root Intent schema
- Context model
- Agent model and lifecycle
- Gherkin invariant system
- LLM integration model

### [03-use-cases/](./03-use-cases/)
**Concrete examples** showing how Converge solves real problems.

- Growth Strategy Runtime
- Meeting Scheduler Runtime
- Resource Routing Runtime
- Domain-specific context schemas

### [04-reference-comparisons/](./04-reference-comparisons/)
**Understanding** Converge in context of other systems and patterns.

- Why not actors? (vs Erlang/OTP, Akka)
- Temporal integration model
- Distributed systems considerations
- Scaling model
- Failure modes
- Reference architectures

### [05-development/](./05-development/)
**Implementation** status, plans, and decisions.

- Project plan and milestones
- Current implementation status
- Task lists
- Authoritative implementation decisions
- Human-in-the-loop patterns

### [06-assistant-guides/](./06-assistant-guides/)
**Guidelines** for AI assistants working on Converge.

- Use-case owner role definition
- Rust best practices
- Cursor rules (also in root `.cursorrules`)
- Assistant-specific instructions

---

## 🚀 Quick Start Paths

### For New Contributors
1. Read `01-core-philosophy/MANIFESTO.md`
2. Read `01-core-philosophy/TERMINOLOGY.md`
3. Read `02-architecture/ARCHITECTURE.md`
4. Review `05-development/STATUS.md` for current state

### For Domain Experts / Use-Case Authors
1. Read `01-core-philosophy/WHEN_TO_USE_CONVERGE.md`
2. Review `03-use-cases/` examples
3. Read `06-assistant-guides/cursor-use-case-owner-coder.md`
4. Study `02-architecture/ROOT_INTENT_SCHEMA.md`

### For AI Assistants
1. Read root `.cursorrules` (comprehensive rules)
2. Read `06-assistant-guides/cursor-use-case-owner-coder.md` (use-case role)
3. Review `05-development/DECISIONS.md` (authoritative choices)
4. Study `02-architecture/` for system understanding

### For System Architects
1. Read `02-architecture/ARCHITECTURE.md`
2. Read `02-architecture/ENGINE_EXECUTION_MODEL.md`
3. Read `02-architecture/CONVERGENCE_SEMANTICS.md`
4. Review `04-reference-comparisons/` for context

---

## 📖 Document Relationships

### Core Flow
```
MANIFESTO.md → ARCHITECTURE.md → ENGINE_EXECUTION_MODEL.md → Use Cases
```

### Implementation Flow
```
PROJECT_PLAN.md → STATUS.md → TASKS.md → DECISIONS.md
```

### Use-Case Flow
```
Use Case Doc → Context Schema → Root Intent → Gherkin Invariants
```

---

## 🔍 Finding Information

### By Topic

**Convergence:**
- `02-architecture/CONVERGENCE_SEMANTICS.md` (how it works)
- `02-architecture/CONVERGENCE_PROOFS.md` (why it's guaranteed)
- `04-reference-comparisons/FAILURE_MODES.md` (what can go wrong)

**Agents:**
- `02-architecture/AGENTS.md` (agent model)
- `02-architecture/AGENT_LIFECYCLE.md` (agent lifecycle)
- `02-architecture/LLM_INTEGRATION.md` (LLM agents)

**Context:**
- `02-architecture/ARCHITECTURE.md` (overview)
- `03-use-cases/CONTEXT_SCHEMA_GROWTH.md` (example schema)
- `02-architecture/ROOT_INTENT_SCHEMA.md` (root intent)

**Gherkin:**
- `02-architecture/GHERKIN_MODEL.md` (invariant system)
- `05-development/GHERKIN_HITL_EXAMPLES.md` (human-in-the-loop)

**Implementation:**
- `05-development/DECISIONS.md` (authoritative choices)
- `05-development/STATUS.md` (what's built)
- `05-development/TASKS.md` (what's next)

---

## 📝 Document Status

All documents in this directory are:
- ✅ **Current** — Reflects agreed architecture
- ✅ **Authoritative** — No unresolved ambiguities
- ✅ **Complete** — Ready for implementation

---

## 🔗 External References

- **Code:** `converge-core/` — Rust implementation
- **Rules:** `.cursorrules` — Cursor AI rules (root directory)
- **Build:** `Justfile` — Development commands

---

## 💡 Tips for AI Agents

When working with this documentation:

1. **Always check `05-development/DECISIONS.md`** for authoritative implementation choices
2. **Respect the architecture** — don't propose changes that violate core principles
3. **Use use-cases as templates** — they show the pattern for new domains
4. **Refer to terminology** — use `TERMINOLOGY.md` for precise definitions
5. **Check status** — `STATUS.md` shows what's already implemented

---

## 📅 Last Updated

Documentation structure organized: 2024
All documents reflect Phase 0 (Architecture Lock) completion.

