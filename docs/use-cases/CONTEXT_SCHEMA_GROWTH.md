# Converge — Context Schema: Growth Strategy

## Purpose

This document defines the **typed context schema** for the Growth Strategy runtime.

Context is:
- shared
- job-scoped
- monotonic

---

## High-Level Structure

```
Context
├─ RootIntent
├─ Market
├─ Product
├─ Signals
├─ Competitors
├─ Segments
├─ Channels
├─ Relationships
├─ Hypotheses
├─ Evaluations
└─ Trace
```

---

## Core Context Elements

### Signals
Raw observations with provenance.

Examples:
- Web mentions
- Social discussions
- News events

Signals are low-confidence inputs.

---

### Competitors
Structured competitor profiles.

Fields:
- Name
- Positioning
- Strength indicators

---

### Segments
Market segments under consideration.

Fields:
- Industry
- Company size
- Geography

---

### Channels
Potential go-to-market channels.

Fields:
- Saturation
- Cost level
- Reach

---

### Relationships
Graph of actors and influence.

Fields:
- Actor
- Relationship type
- Trust score
- Influence score

---

### Hypotheses
Candidate growth strategies.

Fields:
- Target segment
- Narrative
- Channel mix

Hypotheses are refined, not replaced.

---

### Evaluations
Scores and tradeoffs.

Fields:
- Confidence
- Risk
- Expected impact

---

## Monotonic Rules

- Signals may accumulate
- Hypotheses may be refined
- Scores may increase in confidence
- Nothing is silently removed

---

## Summary

This schema defines the **shared language** agents use to collaborate on strategy.
