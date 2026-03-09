---
name: workflow-deep-team-codex
description: This skill should be used when the user asks to "use the full team", "run the deep version", "treat this as high risk", "use deep-team", or wants the full RPIV workflow in deep-team topology.
---

# Workflow Deep Team (Codex)

Run Victor's RPIV workflow in deep-team topology for risky or uncertain work.

## Goal
- Slow down the parts that need proof.
- Add learning tests before assumptions harden.
- Require verifier and reviewer gates before completion.

## Entry Criteria
Use `deep-team` when any are true:
- public API, protocol, or schema changes
- concurrency, state, cancellation, or lifecycle-sensitive logic
- major refactor with hidden coupling
- external dependency or runtime behavior is uncertain
- the cost of a silent mistake is high

## Required Process
1. Research deeply and run learning tests where assumptions matter.
2. Stabilize the canonical plan before graph compilation.
3. Execute bounded features with Spark executors and packet verifiers.
4. Run separate scrutiny and user-flow validation.
5. Run dedicated review when needed.
6. Stop after 2 critique/fix loops and escalate if ambiguity remains.

## Backpressure Rules
- No execution packet should depend on unproven external behavior.
- No packet is complete without verification metadata.
- No final synthesis without verifier evidence.
- No risky completion without reviewer evidence.
- Inherit RPIV prompt contracts for completeness, verification, grounding, and brief user updates.

## References
- `../../playbooks/personal-swarm-workflow.md`
- `../../playbooks/workflow-artifacts.md`
