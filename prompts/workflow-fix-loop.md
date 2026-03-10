---
description: Run a focused remediation loop after failed validation or review assertions in an RPIV workflow.
argument-hint: "[task_or_scope_or_slug]"
---
Use `workflow-fix-loop-codex` as the canonical remediation loop for: $ARGUMENTS

Execution contract:
- Read `validation-state.json`, `verification-report.md`, and any review findings first.
- Start or update a native `update_plan` checklist for the remediation loop.
- Emit the smallest remediation features or packets that address those failures.
- Keep ownership boundaries tight and avoid reopening already-proven areas.
- Prefer native thread compaction before the fix loop when the prior Verify transcript is long or noisy; otherwise resume from artifacts in a fresh thread/context.
- Rerun only the failed or impacted verification checks.
- Stop after 2 critique/fix loops and escalate if the failure remains ambiguous.
- Update whichever artifacts are active for the current run:
1. `features.json` when the run uses a graph
2. `validation-state.json`
3. `execution-summary.md`
- Reflect the remediation step and rerun status in `update_plan`.
- Output contract:
1. identify the exact failed assertions or findings being remediated
2. tie rerun evidence to exact commands or probes
3. recommend exactly one next step when blockers remain
- Completeness contract:
1. do not claim remediation is complete until affected checks are rerun or explicitly blocked
2. keep the remediation scope tight; do not reopen already-proven areas without evidence
- Return: remediation packets, rerun evidence, remaining blockers, residual risk, and exact next phase.
