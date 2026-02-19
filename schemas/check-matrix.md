# Check Matrix Schema

## Contract
- schema_name: `check-matrix`
- schema_version: `1.0.0`
- purpose: versioned mapping of change types to gate requirements.

## Canonical Gate Order
1. `typecheck`
2. `unit`
3. `e2e` (conditional)
4. `compiler/build` (conditional)

## Matrix Record
```yaml
change_type: docs | code | infra | fix
checks:
  typecheck: required | optional | n/a
  unit: required | optional | n/a
  e2e: required | conditional | n/a
  compiler/build: required | conditional | n/a
```

## Default Matrix
| change_type | typecheck | unit | e2e | compiler/build |
| --- | --- | --- | --- | --- |
| docs | required | optional | n/a | n/a |
| code | required | required | conditional | conditional |
| infra | required | required | conditional | required |
| fix | required | required | conditional | conditional |

## Invariants
1. Required checks must execute in canonical gate order.
2. Any failed required check blocks task completion.
3. A failed required check emits a blocking fix-task.
4. A task is complete only when verify evidence exists for all required checks.
