# FEEDBACK_HARNESS

## Scope
Define the check command matrix and blocking behavior for this initiative.

## Command Matrix by Change Type
| change_type | typecheck | unit | e2e | compiler/build | notes |
| --- | --- | --- | --- | --- | --- |
| docs | required | optional | n/a | n/a | structural checks only |
| code | required | required | conditional | conditional | full verification path |
| infra | required | required | conditional | required | prioritize build validity |
| fix | required | required | conditional | conditional | must cover failed gate |

## Command Definitions
- typecheck:
- unit:
- e2e:
- compiler/build:

## Gate Order
1. typecheck
2. unit
3. e2e (if required)
4. compiler/build (if required)

## Pass/Fail and Blocking Behavior
- A failed required check is a hard stop.
- On failure, create a blocking fix-task and link it in `VERIFY.md`.
- Dependents remain blocked until fix-task passes required checks.

## Rerun Policy for Fix Tasks
- Re-run only the failed check(s) after a fix-task is complete.
- If checks pass, continue the remaining required gate order.
- If checks fail again, create another fix-task and continue blocking.

## Completion Invariant
A task cannot be completed without passed required checks.
