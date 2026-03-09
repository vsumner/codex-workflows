---
description: Run the personal RPIV workflow using team topology for non-trivial work.
argument-hint: "[task_or_scope]"
---
Run the personal `team` RPIV workflow for: $ARGUMENTS

Execution contract:
- Run `research -> plan -> execute -> verify` explicitly.
- Inherit RPIV prompt contracts for completeness, verification, and concise user updates.
- Use teams inside phases where independence is real.
- Keep the orchestrator out of source implementation.
- Research should emit planner-ready outputs.
- Plan should emit the lightest artifact set that fits the work:
1. `plan.md` always
2. `validation-contract.md` for standard and heavier work
3. `features.json` only when graph coordination or resumability is worth it
- Execute should use bounded Spark executors plus packet verification.
- Verify should update `validation-state.json` and `verification-report.md`.
- Return: phase, artifact paths, active topology, artifact weight, evidence, blockers, next step.
