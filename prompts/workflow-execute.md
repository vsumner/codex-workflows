---
description: Run the Execute phase team using bounded workers and per-packet verification.
argument-hint: "[task_or_scope] [mode=auto|solo|team|deep-team] [execute_mode=auto|approval_gated|autonomous|parallel_autonomous]"
---
Use `workflow-execute-codex` as the canonical Execute phase for: $ARGUMENTS

Execution contract:
- Read `plan.md`, `research.md`, `validation-contract.md`, and `features.json` when present before changing code.
- Infer topology and Execute-mode state unless the user overrides them.
- For non-trivial work, maintain a short native `update_plan` checklist with Execute in progress.
- Keep the orchestrator out of source edits; execute only bounded packets with explicit ownership.
- Prefer the built-in `explorer` for one-shot repo lookups that unblock a packet.
- Verify each packet before acceptance and keep `features.json` and `execution-summary.md` current.
- If a delegated worker fails because of role, model, config, or repeated stall issues, fix the lane or downgrade topology explicitly; do not keep retrying the same broken path.
- If execution falls back from team mode to solo/local execution, say so in chat and in `execution-summary.md`.
- If execution materially invalidates the plan, step back to Plan instead of improvising a new one silently.
- Mark Execute complete in `update_plan` before handing off to Verify.
- Return: chosen topology, chosen execute mode, accepted packets, failed or blocked packets, evidence, and next step.
