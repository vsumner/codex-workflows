# PLAN

## Plan Metadata
- initiative:
- source_design_discussion:
- planner:
- date:

## Task Template
Use one section per task.

### task_id: <TASK-000>
- title:
- owner_role:
- file_scope:
- depends_on:
- change_type:
- required_checks:
- done_criteria:
- rollback_hint:

## Validation Rules
- `owner_role` is required for every executable task.
- `depends_on` must reference valid `task_id` values.
- `required_checks` must map to `FEEDBACK_HARNESS.md`.
- Missing `owner_role` makes the plan invalid.
- Missing `required_checks` makes the task invalid.
