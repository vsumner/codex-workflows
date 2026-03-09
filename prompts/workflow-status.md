---
description: Summarize RPIV artifact state and recommend the next phase or next action.
argument-hint: "[task_or_scope_or_slug]"
---
Summarize RPIV workflow status for: $ARGUMENTS

Execution contract:
- Inspect `.codex-workflow/{slug}/` artifacts first.
- Determine:
1. current phase
2. topology
3. artifact completeness
4. feature progress
5. validation status
- If execution appears complete but verification is stale or missing, recommend Verify next by default.
- Grounding rules:
1. base the status summary on artifacts actually present, not assumed workflow state
2. call out missing or stale artifacts explicitly
- Output contract:
1. recommend exactly one next action
2. distinguish current fact from inference when artifact state is incomplete
- Recommend exactly one next action.
- Return: phase, artifacts present/missing, progress summary, blockers, recommended next step.
