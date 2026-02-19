# FEEDBACK_HARNESS (Example: stable-manager)

## Command Matrix by Change Type
| change_type | typecheck | unit | e2e | compiler/build |
| --- | --- | --- | --- | --- |
| docs | required | optional | n/a | n/a |
| code | required | required | conditional | conditional |
| fix | required | required | conditional | conditional |

## Command Definitions
- typecheck: `npm run typecheck`
- unit: `npm run test:unit`
- e2e: `npm run test:e2e`
- compiler/build: `npm run build`

## Pass/Fail and Blocking Behavior
- Any failed required check creates a blocking fix-task.
- Dependents stay blocked until fix-task passes required checks.

## Rerun Policy
- Re-run only failed required checks after fix-task completion.
- Repeat fix-task loop until required checks pass.

## Completion Invariant
Cannot complete task without passed required checks.
