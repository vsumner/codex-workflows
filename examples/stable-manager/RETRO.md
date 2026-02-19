# RETRO (Example: stable-manager)

## Summary
- initiative: codexify bootstrap adoption
- date: 2026-02-19
- owner: reviewer

## Outcomes
- what_went_well: Gate ordering and fix-task flow prevented invalid completion.
- what_went_wrong: Required command ownership was initially unclear for docs tasks.

## Gate and Backpressure Effectiveness
- prevented_failures: Prevented SM-002 from closing before FIX-001 passed.
- failure_patterns: Formatting/consistency checks failed first.
- fix_task_recurrence: 1

## Process Gaps
- gap: Command defaults vary across downstream repos.
  impact: Friction during first adoption.
  proposed_change: Add per-repo command override section in FEEDBACK_HARNESS.

## Actions
- action_id: A-001
  owner: validator
  due_date: 2026-02-24
  success_metric: zero unresolved check-command mapping questions at kickoff
