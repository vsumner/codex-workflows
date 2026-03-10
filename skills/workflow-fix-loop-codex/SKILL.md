---
name: workflow-fix-loop-codex
description: This skill should be used when verification fails but a bounded remediation loop is still the right next move inside RPIV.
---

# Workflow Fix Loop (Codex)

Run a narrow remediation loop after failed verification.

## Goal
- Repair only the failed or blocked slice.
- Reuse current RPIV artifacts instead of reopening broad planning.
- Rerun only the impacted verification checks.
- Return to Verify or the next Execute wave as soon as the slice is decision-complete.

## Inputs
- `plan.md`
- `validation-contract.md`
- `validation-state.json`
- `verification-report.md`
- `execution-summary.md`
- `features.json` when the run uses graph artifacts

## Rules
1. Start or update a short native `update_plan` checklist for the remediation loop.
2. Read the failed assertions or findings first and turn them into the smallest remediation packet set.
3. Keep ownership boundaries tight; do not reopen already-proven areas without fresh evidence.
4. Prefer native thread compaction before the fix loop when the prior Verify transcript is long or noisy; otherwise resume from artifacts in a fresh thread/context.
5. Rerun only the failed or impacted verification checks.
6. Update `features.json`, `execution-summary.md`, `validation-state.json`, and `verification-report.md` as evidence changes.
7. If the remediation path stops being bounded, step back to Plan instead of stretching the fix loop into a redesign.
8. Stop after 2 loops on the same failing set and escalate if the failure remains ambiguous or unstable.
9. If rerun evidence passes, return to Verify verdicting immediately and continue to the next Execute wave only if safe continuation conditions hold.

## Return
- remediated assertions or packets
- rerun evidence
- remaining blockers or residual risk
- exact next phase: `verify`, `execute`, or `human-decision`

## References
- `../../playbooks/personal-swarm-workflow.md`
- `../../playbooks/workflow-artifacts.md`
- `../../prompts/workflow-fix-loop.md`
