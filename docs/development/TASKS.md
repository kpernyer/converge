# Converge — Tasks (Next 2–3 Days)

This file lists **concrete, implementation-ready tasks**
for the immediate next work sessions.

Focus: **Phase 1 — Engine Skeleton**

---

## Day 1 — Project & Core Types

### Setup
- [ ] Create `converge-core` Rust crate
- [ ] Add basic CI (cargo test)
- [ ] Enforce `#![forbid(unsafe_code)]`

### Core data model
- [ ] Define `ContextKey` enum or interned symbol
- [ ] Define `Context` (read-only view + internal storage)
- [ ] Define `AgentEffect`
- [ ] Define `Fact` (minimal, typed)

### Decisions to lock
- [ ] How context equality is checked
- [ ] How facts are stored (Vec, Map, Arena handle)

---

## Day 2 — Engine Loop

### Engine core
- [ ] Implement agent registry
- [ ] Implement dependency index (`ContextKey -> Agents`)
- [ ] Implement eligibility check (`accepts(&Context)`)
- [ ] Implement effect buffering

### Convergence loop
- [ ] Implement `run_until_converged()`
- [ ] Detect context change vs no-op cycle
- [ ] Enforce cycle budget

---

## Day 3 — Minimal Proof

### Test agents
- [ ] `SeedAgent` — emits initial fact
- [ ] `ReactOnceAgent` — reacts once, then stops

### Tests
- [ ] Test deterministic convergence
- [ ] Test no infinite loop
- [ ] Test budget exhaustion

### Cleanup
- [ ] Remove unused abstractions
- [ ] Add module-level docs
- [ ] Commit engine skeleton

---

## Explicit Non-Goals (for now)

Do NOT implement yet:
- Gherkin
- LLMs
- MCP
- Persistence
- Async runtimes
- Optimization

These come later and will be simpler once the engine is proven.

---

## Success Criteria

By the end of these tasks:
- Converge has a working engine
- Convergence is observable
- The hardest claims are proven in code

Everything else is layering.
