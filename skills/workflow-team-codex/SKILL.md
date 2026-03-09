---
name: workflow-team-codex
description: This skill should be used when the user asks to "use a team", "run this with agents", "split this into subagents", "orchestrate workers", or wants the full RPIV workflow in team topology.
---

# Workflow Team (Codex)

Run Victor's personal RPIV workflow in team topology.

## Goal
- Run Research, Plan, Execute, and Verify with teams where they help.
- Preserve context through artifacts and bounded packets.
- Keep implementation on Spark when packet scope is bounded.

## Phase Rules
1. Research emits planner-ready artifacts.
2. Plan emits a canonical plan first and executable graph second only when the graph earns its keep.
3. Execute uses bounded Spark executors plus packet verifiers.
4. Verify updates assertion state and verification reports.

## Team Rules
- Use teams only where independence is real.
- Keep the orchestrator out of hands-on implementation.
- Use `team` as the default topology for medium and larger work.
- Escalate to `workflow-deep-team-codex` when risk or uncertainty rises.
- Inherit RPIV prompt contracts for completeness, verification, and brief user updates.

## References
- `../../playbooks/personal-swarm-workflow.md`
- `../../playbooks/workflow-artifacts.md`
