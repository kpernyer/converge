# Converge — LLM Integration Model

This document explains **how Large Language Models (LLMs) are integrated into Converge**
in a way that preserves correctness, convergence, and trust.

LLMs are treated as **fallible knowledge tools**, not authorities.

---

## Framing Principle

> **LLMs may suggest. The engine decides.**

No LLM output is ever considered correct merely because an LLM produced it.

---

## Q7.1: If LLMs Never Decide, Who Does?

### Question recap
If an LLM agent emits a suggested fact:
- who consumes it?
- how is it promoted?
- can humans intervene?

### Answer

### 1. LLM outputs are suggestions, not facts

LLM agents emit **ProposedFacts**, not Facts.

Conceptually:

```text
ProposedFact {
  content
  confidence_hint
  provenance: LLM(model, prompt_hash)
}
```

ProposedFacts are inert by default.

---

### 2. Deterministic validation agents

Promotion from ProposedFact → Fact requires validation.

Validation may include:
- schema validation
- constraint checks
- cross-checking against trusted data
- corroboration by multiple signals

These checks are performed by **deterministic governance agents**.

Only after validation does a ProposedFact become a Fact.

---

### 3. Human-in-the-loop (optional)

Root Intent may require human approval.

Example:
- high-risk decisions
- regulatory domains
- strategic commitments

In this case:
- LLM suggestions are surfaced
- humans approve or reject
- approval emits a Fact

Humans act as **explicit validators**, not silent overrides.

---

### 4. Automatic promotion (bounded)

Automatic promotion is allowed only when:
- validation rules pass
- confidence thresholds are met
- invariant checks succeed

Automatic does not mean uncontrolled.

---

## Q7.2: Hallucination Handling & Blast Radius Control

### Question recap
How do you:
- validate LLM outputs?
- limit damage from hallucinations?
- prevent pollution of context?

### Answer

### 1. LLM outputs are isolated by default

LLM outputs:
- are never merged directly into core context
- live in a quarantined suggestion space
- expire if not validated

This prevents permanent pollution.

---

### 2. Multi-layer validation

Validation layers include:

1. Structural validation
2. Constraint validation
3. Cross-signal corroboration
4. Invariant enforcement

A hallucinated output typically fails early.

---

### 3. Blast radius is explicitly bounded

Blast radius is bounded by:

- LLM outputs cannot create new domains
- They cannot override constraints
- They cannot trigger convergence
- They cannot block execution

Worst case:
- a suggestion is ignored

There is no cascading failure mode.

---

### 4. Provenance & traceability

Every LLM contribution includes:

- model identifier
- prompt hash
- timestamp
- validation outcome

This enables:
- audit
- debugging
- post-mortem analysis

---

### 5. Garbage in, bounded out

Even if an LLM produces confident nonsense:

- it remains a suggestion
- it fails validation
- it is dropped

The context remains correct.

---

## Comparison: Naive LLM Systems vs Converge

| Aspect | Naive Agent System | Converge |
|------|-------------------|----------|
| LLM authority | High | None |
| Validation | Ad hoc | Deterministic |
| Hallucination impact | High | Bounded |
| Traceability | Weak | Strong |
| Correctness | Best-effort | Enforced |

---

## Summary

- LLMs emit **suggestions**, not facts
- Deterministic agents validate and promote
- Humans may validate when required
- Hallucinations are quarantined and dropped
- Context correctness is preserved

---

## One-sentence takeaway

> In Converge, LLMs are creative interns — every output is checked before it becomes law.
