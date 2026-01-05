# Reference & Comparisons

This directory explains Converge in the context of other systems and architectural patterns.

## Why Not Alternatives?

### [WHY_NOT_ACTORS.md](./WHY_NOT_ACTORS.md)
Why Converge doesn't use the Actor Model (Erlang/OTP, Akka, Orleans).

**Key difference:** Actors optimize for availability; Converge optimizes for correctness.

### [OTP_MODEL.md](./OTP_MODEL.md)
Detailed comparison with Erlang/OTP patterns.

**Key points:**
- No supervision trees (crash recovery is job-level)
- No mailboxes (context is the communication medium)
- No "let it crash" (correctness is preserved)

### [ACTORS.md](./ACTORS.md)
Additional actor model considerations.

## Integration Models

### [TEMPORAL_MODEL.md](./TEMPORAL_MODEL.md)
How Converge relates to Temporal (workflow engine).

**Key insight:** Use Temporal for long-lived workflows; use Converge for bounded decisions.

### [DISTRIBUTED_SYSTEMS.md](./DISTRIBUTED_SYSTEMS.md)
Distributed systems considerations for Converge.

### [SCALING_MODEL.md](./SCALING_MODEL.md)
How Converge scales (or doesn't) and when to use it.

### [FAILURE_MODES.md](./FAILURE_MODES.md)
What can go wrong and how Converge handles failures.

## Reference Architectures

### [REFERENCE_ARCHITECTURES.md](./REFERENCE_ARCHITECTURES.md)
Reference architectures and patterns that influenced Converge.

## Implementation Details

### [RUST_MEMORY_MODEL.md](./RUST_MEMORY_MODEL.md)
Rust-specific memory and concurrency considerations.

---

## Reading Order

**For understanding Converge's place:**
1. WHY_NOT_ACTORS.md
2. TEMPORAL_MODEL.md
3. WHEN_TO_USE_CONVERGE.md (in 01-core-philosophy)

**For system design:**
1. DISTRIBUTED_SYSTEMS.md
2. SCALING_MODEL.md
3. FAILURE_MODES.md

**For implementation:**
1. RUST_MEMORY_MODEL.md
2. REFERENCE_ARCHITECTURES.md

---

## For AI Agents

These documents help you:
- Understand when Converge is NOT the right tool
- Avoid proposing patterns that Converge explicitly rejects
- Understand the tradeoffs Converge makes
- Know when to recommend alternatives

**Key principle:** Converge is not trying to be everything. It's optimized for correctness-first, bounded decision-making.

