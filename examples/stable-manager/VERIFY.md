# VERIFY (Example: stable-manager)

## Verification Metadata
- initiative: codexify bootstrap adoption
- validator: validator
- date: 2026-02-19

### task_id: SM-001
- commands_executed: `npm run typecheck`; `npm run test:unit`
- outputs_status: typecheck=pass; unit=pass
- failures_and_fix_task_ids: none
- final_pass_evidence: checks passed in required order and artifacts created
- reviewer_signoff: pending

### task_id: SM-002
- commands_executed: `npm run typecheck`
- outputs_status: typecheck=fail (missing heading format)
- failures_and_fix_task_ids: FIX-001
- final_pass_evidence: pending
- reviewer_signoff: pending

### task_id: FIX-001
- commands_executed: `npm run typecheck`
- outputs_status: typecheck=pass
- failures_and_fix_task_ids: none
- final_pass_evidence: fixed heading format and unblocked SM-002
- reviewer_signoff: pending

### task_id: SM-003
- commands_executed: `npm run typecheck`; `npm run test:unit`
- outputs_status: typecheck=pass; unit=pass
- failures_and_fix_task_ids: none
- final_pass_evidence: verify record is complete and consistent
- reviewer_signoff: pending

### task_id: SM-004
- commands_executed: `npm run typecheck`
- outputs_status: typecheck=pass
- failures_and_fix_task_ids: none
- final_pass_evidence: reviewer checklist complete
- reviewer_signoff: approved_by_reviewer_2026-02-19
