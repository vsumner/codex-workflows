# RPI Loop Playbook

## Purpose
Define the required end-to-end operating loop for repository process integration (RPI).

## Required Artifact Sequence
1. `LEARNING.md`
2. `DESIGN_DISCUSSION.md`
3. `PLAN.md`
4. `FEEDBACK_HARNESS.md`
5. execution (subagents only)
6. `VERIFY.md`
7. `RETRO.md`

## Protocol
1. Start in Plan mode and produce `LEARNING.md`.
2. Produce `DESIGN_DISCUSSION.md` from validated learnings.
3. Produce `PLAN.md` with dependency-aware tasks.
4. Produce `FEEDBACK_HARNESS.md` before any execution.
5. Confirm all executable tasks have `owner_role` and `required_checks`.
6. Execute only unblocked tasks through subagents.
7. After each task or batch, run required checks in canonical order.
8. On failure, create a blocking fix-task and halt dependents.
9. Record all evidence in `VERIFY.md`.
10. Run reviewer signoff, then complete `RETRO.md`.

## Canonical Gate Order
1. `typecheck`
2. `unit`
3. `e2e` (conditional)
4. `compiler/build` (conditional)

## Completion Criteria
- No open blocking tasks.
- All required checks passed for all completed tasks.
- `VERIFY.md` contains evidence and linked fix-task history.
- Reviewer signoff is present for production-impacting changes.
- `RETRO.md` is complete.
