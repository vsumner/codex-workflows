# Backpressure Policy

## Goal
Prevent invalid forward progress by enforcing dependency and quality gates.

## Blocking Rules
1. A task is `blocked` if any dependency is not `done`.
2. A task is `blocked` if any required check fails.
3. A failed check must create a fix-task with `change_type = fix`.
4. Dependents remain blocked until the fix-task passes all required checks.
5. A task with missing `owner_role` or `required_checks` is invalid and cannot start.
6. A task with missing verify evidence cannot close.

## Fix-Task Contract
- The fix-task must reference the failed task and failed check.
- The fix-task must define its own `required_checks`.
- The fix-task must include rollback guidance.
- The fix-task must be recorded in `VERIFY.md` with final outcome.

## Retry Policy
1. Re-run only failed checks after a fix-task completes.
2. If re-run passes, continue original gate sequence.
3. If re-run fails, emit a new fix-task and continue blocking.
4. Escalate to human review after repeated failure threshold (default: 2).

## Non-Bypass Rule
No task may be marked done without passed required checks and verify evidence.
