# Documentation Reorganization Summary

This document describes the reorganization of Converge documentation completed on 2024.

## New Structure

All documentation has been moved to the `docs/` directory and organized into logical categories:

```
docs/
├── README.md                          # Main index and navigation guide
├── 01-core-philosophy/                # What Converge is and why
│   ├── README.md
│   ├── MANIFESTO.md
│   ├── TERMINOLOGY.md
│   └── WHEN_TO_USE_CONVERGE.md
├── 02-architecture/                   # System design and core concepts
│   ├── README.md
│   ├── ARCHITECTURE.md
│   ├── ENGINE_EXECUTION_MODEL.md
│   ├── CONVERGENCE_SEMANTICS.md
│   ├── CONVERGENCE_PROOFS.md
│   ├── ROOT_INTENT_SCHEMA.md
│   ├── GHERKIN_MODEL.md
│   ├── LLM_INTEGRATION.md
│   ├── AGENT_LIFECYCLE.md
│   ├── AGENTS.md
│   └── COMMUNICATION_MODEL.md
├── 03-use-cases/                      # Concrete examples
│   ├── README.md
│   ├── CONVERGE_GROWTH_STRATEGY_USECASE.md
│   ├── CONVERGE_MEETING_SCHEDULER_USECASE.md
│   ├── CONVERGE_RESOURCE_ROUTING_USECASE.md
│   ├── CONTEXT_SCHEMA_GROWTH.md
│   └── SMB_PLATFORM_PLACEHOLDER.md
├── 04-reference-comparisons/          # Context and comparisons
│   ├── README.md
│   ├── WHY_NOT_ACTORS.md
│   ├── OTP_MODEL.md
│   ├── ACTORS.md
│   ├── TEMPORAL_MODEL.md
│   ├── REFERENCE_ARCHITECTURES.md
│   ├── DISTRIBUTED_SYSTEMS.md
│   ├── SCALING_MODEL.md
│   ├── FAILURE_MODES.md
│   └── RUST_MEMORY_MODEL.md
├── 05-development/                    # Implementation status and plans
│   ├── README.md
│   ├── PROJECT_PLAN.md
│   ├── STATUS.md
│   ├── TASKS.md
│   ├── DECISIONS.md
│   ├── HUMAN_IN_THE_LOOP.md
│   └── GHERKIN_HITL_EXAMPLES.md
└── 06-assistant-guides/               # AI assistant guidelines
    ├── README.md
    ├── cursor-use-case-owner-coder.md
    ├── codex-assistant-coder.md
    ├── gemini-cloudops.md
    └── Rust-Best-Practices-v2.md
```

## What Stayed in Root

The following files remain in the root directory:

- `.cursorrules` — Cursor AI rules (referenced by tools)
- `Justfile` — Development commands
- `converge-core/` — Rust implementation
- `DECISIONS.md` — **Note:** This should be moved to `docs/05-development/` if it exists in root

## Benefits

### For Humans
- **Clear navigation** — README files in each directory explain what's there
- **Logical grouping** — Related documents are together
- **Quick start paths** — Main README provides paths for different roles
- **Easy discovery** — Topic-based organization makes finding information easier

### For AI Agents
- **Structured access** — Clear hierarchy helps agents understand document relationships
- **Role-specific guides** — Assistant guides are clearly separated
- **Authoritative sources** — Decisions and status are easy to find
- **Cross-references** — README files explain how documents relate

## Navigation

Start with `docs/README.md` for:
- Overview of all documentation
- Quick start paths for different roles
- Topic-based finding guide
- Document relationship maps

Each subdirectory has its own README explaining:
- What documents are in that category
- Reading order recommendations
- How to use the documents
- Special notes for AI agents

## Migration Notes

- **No content was changed** — Only organization
- **All files preserved** — Nothing was deleted
- **Cross-references** — May need updating if they used relative paths
- **Links** — Internal document links should still work (relative paths preserved)

## Next Steps

1. **Update any external references** that point to moved documents
2. **Update code comments** that reference documentation paths
3. **Test navigation** — Verify the README structure works for your workflow
4. **Add to main README** — Consider adding a link to `docs/README.md` from root

## Verification

To verify the reorganization:

```bash
# Count documents
find docs -name "*.md" | wc -l

# List all documents
find docs -name "*.md" | sort

# Check for any remaining root docs (should be minimal)
ls -la *.md 2>/dev/null
```

---

**Status:** ✅ Complete
**All documents preserved and organized**
**Ready for use by humans and AI agents**

