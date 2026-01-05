# Development Documentation

This directory contains implementation status, plans, tasks, and authoritative decisions.

## Current Status

### [STATUS.md](./STATUS.md)
**What's built, what's next, how to verify.**

Shows:
- Current implementation state
- Data model (Context, Fact, ProposedFact, AgentEffect)
- What's coming next (Engine loop)
- How to see it evolve

### [PROJECT_PLAN.md](./PROJECT_PLAN.md)
**Phased development plan.**

Phases:
- Phase 0: Architecture Lock (DONE)
- Phase 1: Engine Skeleton (CURRENT)
- Phase 2: Eligibility Index & Budgets
- Phase 3: Gherkin Runtime Enforcement
- Phase 4: Growth Strategy v1 (No LLMs)
- Phase 5: LLM Integration (Governed)
- Phase 6: Integration & Packaging
- Phase 7: Hardening & Expansion

### [TASKS.md](./TASKS.md)
**Concrete, implementation-ready tasks for next 2-3 days.**

Focus: Phase 1 — Engine Skeleton

## Authoritative Decisions

### [DECISIONS.md](./DECISIONS.md)
**Locked-in implementation decisions for v1.**

**Critical decisions:**
1. Effect merge ordering: Stable AgentId registration order
2. Dependency index: Incremental maintenance with dirty-key tracking
3. ProposedFact boundary: Separate types, compile-time enforced
4. Convergence check: Dirty-key tracking (not hashing, not deep compare)

**These are final for v1. Do not propose alternatives.**

## Human-in-the-Loop

### [HUMAN_IN_THE_LOOP.md](./HUMAN_IN_THE_LOOP.md)
Patterns for human validation and approval in Converge.

### [GHERKIN_HITL_EXAMPLES.md](./GHERKIN_HITL_EXAMPLES.md)
Gherkin examples showing human-in-the-loop patterns.

---

## Reading Order

**For current state:**
1. STATUS.md
2. PROJECT_PLAN.md
3. TASKS.md

**Before implementing:**
1. DECISIONS.md (authoritative choices)
2. STATUS.md (what exists)
3. TASKS.md (what to build)

**For human interaction:**
1. HUMAN_IN_THE_LOOP.md
2. GHERKIN_HITL_EXAMPLES.md

---

## For AI Agents

**Critical:** Always check `DECISIONS.md` before proposing implementation approaches. The decisions there are authoritative and locked in.

**When implementing:**
- Follow STATUS.md to see what's already built
- Use TASKS.md for immediate next steps
- Respect DECISIONS.md — don't propose alternatives to locked decisions
- Update STATUS.md as you build

**When proposing changes:**
- If it conflicts with DECISIONS.md, explain why the decision should be reconsidered
- If it's a new decision point, document it in DECISIONS.md
- If it changes architecture, flag it as potentially requiring Phase 0 review

