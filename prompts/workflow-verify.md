---
description: Run the Verify phase team and update validation-state for a personal RPIV workflow.
argument-hint: "[task_or_scope] [mode=auto|solo|team|deep-team] [proof=auto|gates|behavior|full]"
---
Run the Verify phase for: $ARGUMENTS

Execution contract:
- Read `research.md`, `plan.md`, `validation-contract.md`, and `execution-summary.md` first.
- Read `validation-state.json` when present; if it is missing for a `standard` or `graph` run, bootstrap it from `validation-contract.md` before running checks.
- Read `features.json` when the run uses graph artifacts.
- When `features.json` shows pending future work, verify the active executed slice first.
  Distinguish the slice verdict from the whole-initiative verdict instead of silently treating planned future work as a fresh regression.
- For non-trivial verification, start or update a native `update_plan` checklist and move the in-progress step to Verify.
- If verification fails along multiple legitimate remediation paths, use native `request_user_input` from the main thread to choose the branch.
- Start from the assumption that bugs still exist. Verification is adversarial, not confirmatory.
- Infer proof weight unless the user overrides it:
1. `gates` for local, low-risk changes where command-level proof is enough
2. `behavior` when user-visible flows, APIs, or runtime behavior need proof beyond gates
3. `full` for risky work, merge/PR readiness, or when review should follow verification immediately
- Infer topology from proof weight and risk:
1. `solo` for most `gates`
2. `team` for `behavior`
3. `deep-team` for `full`
- Run a verify team for non-trivial work:
1. `workflow_verifier`
2. `scrutiny_validator`
3. `user_flow_validator`
4. `workflow_reviewer` when risk warrants it
- Separate:
1. scrutiny validation
2. user-surface or behavior validation
- Use `/verify-gates` as the default scrutiny engine for command-level proof.
- Use `verification-specialist` when behavior proof requires more than command gates.
- Require at least one adversarial probe beyond the happy path when the change is non-trivial.
- Update `validation-state.json` as soon as the assertion set is reconstructed, then keep it current as evidence lands.
- Write `verification-report.md` with completeness, correctness, and coherence.
- Auto-run dedicated Review when the inferred proof weight is `full`, the risk is high, or the work is heading toward PR/merge scrutiny.
- Do not mark the run complete until all required assertions are `passed`.
- If verification fails, propose narrow remediation features and a fix loop.
- If verification reveals the plan or graph is stale, step back to the earliest invalid phase instead of forcing the fix through Verify.
- If delegated validators time out or return partial evidence, continue locally, write the current `validation-state.json` and `verification-report.md`, and return a `failed` or `blocked` handoff instead of stalling.
- Output contract:
1. every required assertion must end as `passed`, `failed`, or `blocked`
2. every failed assertion must include remediation guidance or an escalation reason
3. verification evidence must be tied to exact commands, probes, or observed behavior
4. for graph runs, report both the current slice verdict and the initiative verdict when they differ
5. every blocked assertion must say whether it is blocked by pending implementation, missing proof harness, or timed-out evidence gathering
- Completeness contract:
1. do not mark verification complete while required assertions remain unstated or ambiguous
2. do not confuse review comments, intent alignment, or implementation claims with proof
3. for `standard` and `graph` runs, do not leave Verify without writing `validation-state.json` and `verification-report.md`
- User updates:
1. keep them brief and verdict-oriented
2. update on major failed assertions, remediation branch changes, and final verdict readiness
- When verification completes successfully, mark the checklist complete in `update_plan`.
- Return: chosen proof weight, chosen topology, overall verdict, assertion state, failed assertions, remediation recommendation.
