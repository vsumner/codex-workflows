# PLAN (Example: stable-manager)

## Plan Metadata
- initiative: codexify bootstrap adoption
- source_design_discussion: DESIGN_DISCUSSION.md
- planner: task_decomposer
- date: 2026-02-19

### task_id: SM-001
- title: Create codex role and config scaffolding.
- owner_role: task_decomposer
- file_scope: .codex/**
- depends_on: []
- change_type: code
- required_checks: [typecheck, unit]
- done_criteria: Role config loads and all roles are registered.
- rollback_hint: Revert `.codex` directory to previous version.

### task_id: SM-002
- title: Add playbooks, templates, and schemas.
- owner_role: task_decomposer
- file_scope: playbooks/**, templates/**, schemas/**
- depends_on: [SM-001]
- change_type: docs
- required_checks: [typecheck]
- done_criteria: Required files exist with mandatory fields and policies.
- rollback_hint: Revert changed docs artifacts.

### task_id: SM-003
- title: Validate gate behavior and record verify evidence.
- owner_role: validator
- file_scope: examples/stable-manager/VERIFY.md
- depends_on: [SM-001, SM-002]
- change_type: fix
- required_checks: [typecheck, unit]
- done_criteria: Verify record includes commands, statuses, and final evidence.
- rollback_hint: Remove invalid verify entries and rerun validation.

### task_id: SM-004
- title: Perform final review signoff.
- owner_role: reviewer
- file_scope: examples/stable-manager/VERIFY.md, examples/stable-manager/RETRO.md
- depends_on: [SM-003]
- change_type: docs
- required_checks: [typecheck]
- done_criteria: Reviewer signoff present and retro completed.
- rollback_hint: Reopen review task and document unresolved findings.
