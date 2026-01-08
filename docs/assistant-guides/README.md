# Assistant Guides

This directory contains guidelines and instructions for AI assistants working on Converge.

## Role Definitions

### [cursor-use-case-owner-coder.md](./cursor-use-case-owner-coder.md)
**Use-case owner role definition.**

**Your responsibility:**
- Own and implement use-cases on top of the existing engine
- Ensure use-cases respect core principles
- Detect drift (flag if use-case forces engine changes)
- Express intent clearly using Context schemas, agents, and Gherkin invariants

**What you must NOT do:**
- Change engine code
- Add async, actors, message passing
- Weaken convergence semantics
- Bypass AgentEffect or mutate Context directly

### [codex-assistant-coder.md](./codex-assistant-coder.md)
Additional assistant coding guidelines.

### [gemini-cloudops.md](./gemini-cloudops.md)
Cloud operations and deployment guidelines.

## Coding Standards

### [Rust-Best-Practices-v2.md](./Rust-Best-Practices-v2.md)
**Comprehensive Rust coding standards for Converge.**

**Key requirements:**
- Rust Edition 2024
- No `unwrap()`, `expect()`, or `panic!` in production code
- Use `thiserror` for domain errors
- Always add `tracing` spans
- Follow Axum/Tokio stack
- Use SurrealDB, Qdrant, NATS, Temporal

**Also see:** Root `.cursorrules` for comprehensive AI assistant rules.

---

## Reading Order

**For use-case implementation:**
1. cursor-use-case-owner-coder.md (your role)
2. Rust-Best-Practices-v2.md (coding standards)
3. Root `.cursorrules` (comprehensive rules)

**For general development:**
1. Root `.cursorrules` (primary rules)
2. Rust-Best-Practices-v2.md (Rust specifics)
3. ../development/DECISIONS.md (authoritative choices)

---

## For AI Agents

**Primary rules file:** Root `.cursorrules` — this is the comprehensive guide.

**Use-case owners:** Read `cursor-use-case-owner-coder.md` to understand your specific role.

**Key principles:**
- Correctness over convenience
- Determinism first, sophistication later
- If something is dangerous, make it impossible to misuse
- You already know what changed — use that

**When in doubt:**
1. Check `.cursorrules` (root)
2. Check `../development/DECISIONS.md`
3. Check `../architecture/` for system understanding
4. Ask for clarification rather than guessing

