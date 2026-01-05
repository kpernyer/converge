# Converge — Scaling Model

This document describes how Converge scales in production.

---

## 1. Unit of Scale

The unit of scale is the **job**, not the agent.

Each job:
- has its own context
- runs in one runtime
- converges independently

---

## 2. Horizontal Scaling

Scaling is achieved by:
- running many runtimes
- processing many jobs in parallel
- using queues or APIs

No shared mutable state exists between jobs.

---

## 3. Multi-Tenant Scaling

Tenants may be isolated by:
- runtime instance
- namespace
- deployment

Context is never shared across tenants.

---

## 4. Regional Deployment

Runtimes may be deployed:
- per region
- per latency zone
- per regulatory boundary

Jobs are routed, not split.

---

## 5. What Converge Does Not Scale

Converge does not:
- shard a single context
- distribute agent execution
- replicate partial state

This preserves correctness.

---

## 6. Summary

Converge scales like:
- databases scale queries
- CI systems scale builds

Not like distributed actor systems.
