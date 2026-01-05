# Product Guidelines: Converge

## Voice and Tone
- **Explicit, Authoritative, and Transparent:** Converge communicates like an operating system. It uses precise, technical language to describe its internal state. 
- **No Metaphor or Vagueness:** We avoid evocative or flowery language. We don't use "agent" as a metaphor for autonomy; we use it as a technical description of a semantic contributor.
- **System-Oriented:** Documentation and logs prioritize the transparency of the convergence process and the enforcement of invariants.

## Data Presentation and Authority
- **Authority is Explicit:** The system makes a hard distinction between a `ProposedFact` and a `Fact`.
- **Promotion is Explained:** Proposals are visible, but facts are validated. Every promotion from proposal to fact must be accompanied by an explanation of why the engine accepted it.
- **Zero-Trust Posture:** We treat all non-authoritative sources (especially LLMs) as proposers, never as deciders.

## Explainability and Provenance
- **Causal First:** Decision outputs must prioritize the "Why" through causal graphs showing fact dependencies and agent contributions.
- **Temporal and Logical Support:** History (the "When") and Logic (the "How" via invariant checks) provide supporting context but are secondary to the causal chain.

## Human-in-the-Loop (HITL)
- **Humans as Explicit Authorities:** Human interaction is not "feedback" or "chat." A human action in the loop is a discrete event that produces an authoritative `Fact`.
- **Auditability of Human Decisions:** When a human acts, the system records it as a decision, with the same provenance requirements as any other fact addition.
