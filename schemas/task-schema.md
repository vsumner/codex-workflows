# Task Schema

## Contract
- schema_name: `task-schema`
- schema_version: `1.0.0`
- purpose: versioned task interface for codexify-compatible plans.

## Task Record
```yaml
task_id: string
title: string
owner_role: string
file_scope: string | string[]
depends_on: string[]
change_type: docs | code | infra | fix
required_checks: string[]
done_criteria: string | string[]
rollback_hint: string
```

## Field Rules
- `task_id`: unique within a plan.
- `title`: concise outcome statement.
- `owner_role`: required for executable tasks; must map to a registered role.
- `file_scope`: files or directories expected to change.
- `depends_on`: must reference existing `task_id` values.
- `change_type`: drives required check selection.
- `required_checks`: non-empty for executable tasks.
- `done_criteria`: objective and verifiable.
- `rollback_hint`: required rollback guidance.

## Invariants
1. Missing `owner_role` for executable tasks is invalid.
2. Missing `required_checks` for executable tasks is invalid.
3. A task cannot transition to done without verify evidence for all required checks.
4. Failed checks require a blocking fix-task.
