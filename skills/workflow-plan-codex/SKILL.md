---
name: workflow-plan-codex
description: This skill should be used when the user asks to "plan this", "write the plan", "turn research into a plan", "create the task graph", or wants the Plan phase of the personal workflow.
---

# Workflow Plan (Codex)

Run the Plan phase after Research.

## Inference Rules
### Planning Weight
1. `lightweight` for small, tightly coupled, low-risk work.
2. `standard` for medium, multi-step, or moderately risky work.
3. `graph` for risky, dependency-heavy, resumable, or multi-track work.

### Topology
1. `solo` for most `lightweight` planning.
2. `team` for `standard` planning.
3. `deep-team` for `graph` planning or when critique and verification strategy are heavy.

## Team Shape
Use this delegated team only when activation is explicit:
1. `workflow_orchestrator`
2. `planner`
3. `plan_reviewer`

Without explicit delegation activation, keep Plan local and record the recommended topology.

## Required Outputs
Write according to the inferred planning weight:
1. `lightweight`: `plan.md`
2. `standard`: `plan.md` and `validation-contract.md`
3. `graph`: `plan.md`, `validation-contract.md`, and `features.json`

## Rules
- Read `research.md` and `research.json` first.
- For non-trivial planning, start or update a native `update_plan` checklist and keep the in-progress step on Plan.
- If planning hits a material fork that cannot be inferred safely, use `request_user_input` from the main thread.
- If research artifacts are missing and uncertainty is material, step back to Research.
- Produce one canonical plan before compiling executable work units.
- Critique and refine until plan changes are incremental.
- Embed verification in the plan and graph.
- Include a `Critical Files` section with the 3-5 files most important for execution.
- Keep planning strictly read-only; no file creation outside the required RPIV artifacts.
- Reject vague acceptance criteria, missing dependencies, and unstable plans disguised as ready.
- Emit `features.json` only when execution or resumability benefits from a graph.
- Do not mark planning complete until constraints, non-goals, acceptance criteria, and verification strategy are explicit.
- Mark Plan complete in `update_plan` before handing off to Execute.

## References
- `../../playbooks/personal-swarm-workflow.md`
- `../../playbooks/workflow-artifacts.md`
