# Converge Design FAQ
## Defending a Pure Convergence-Based Agent System

This FAQ exists to answer recurring questions and objections
about Converge’s design choices.

It is intentionally direct.

---

## Q: Why not use a message bus (Kafka, NATS, Pub/Sub)?

Because message buses introduce:
- implicit buffering
- implicit retries
- implicit concurrency
- implicit ordering

This breaks:
- single semantic authority
- deterministic convergence
- causal traceability

Converge requires the engine to **always know why something happened**.

Queues make that impossible.

---

## Q: Why not use Temporal or a workflow engine?

Workflow engines assume:
- durable execution
- background progress
- retry-based correctness

Converge explicitly chooses:
- snapshot persistence
- restart over replay
- fact durability, not execution durability
- halt-on-wait HITL

Temporal would fight the model instead of supporting it.

---

## Q: Isn’t this just reinventing workflows?

No.

Workflows encode *how* work proceeds.
Converge encodes *what must be true*.

Execution is a byproduct of convergence,
not the primary abstraction.

---

## Q: Why only one semantic authority per root intent?

Because convergence requires:
- total ordering of facts
- known causality
- deterministic fixed-point detection

Distributed authority makes convergence ambiguous.

Converge scales by **replicating intents**, not sharding truth.

---

## Q: Why not let agents communicate directly?

Agent-to-agent communication creates:
- hidden dependencies
- emergent behavior
- non-deterministic ordering

Agents in Converge:
- read shared context
- emit effects
- never talk to each other

This keeps behavior explainable.

---

## Q: Are LLM agents second-class citizens?

Yes — intentionally.

LLMs:
- suggest
- summarize
- explore

They never:
- decide
- commit
- enforce invariants

This is what makes LLM use safe in business systems.

---

## Q: Why not CRDTs or eventual consistency?

Eventual consistency:
- delays truth
- obscures causality
- breaks fixed-point reasoning

Converge requires:
- explicit truth
- explicit authority
- explicit completion

CRDTs solve a different problem.

---

## Q: How does this scale to many agents?

Execution scales.
Authority does not.

- Agents can execute in parallel
- Effects are merged serially
- Convergence is centralized per intent

This preserves correctness while allowing throughput.

---

## Q: What about Human-in-the-Loop delays?

Waiting is a **state**, not a process.

- Execution halts
- Context is snapshotted
- Engine can shut down
- Resume happens via explicit facts

No queues. No background workers.

---

## Q: Is Converge an agent framework?

No.

Converge is a **semantic convergence engine**
that happens to use agents as contributors.

The difference matters.

---

## Q: Why is this better for SMB business software?

Because SMBs need:
- alignment
- clarity
- adaptability

Not:
- configuration screens
- consultants
- fragile workflows

Converge replaces configuration with intent
and automation with convergence.

---

## Final Answer to Most Objections

If a proposed feature introduces:
- hidden execution
- implicit authority
- background progress
- eventual truth

It is incompatible with Converge.

---

## One-Sentence Defense

> Converge trades implicit autonomy for explicit authority — and that is what makes it trustworthy.
