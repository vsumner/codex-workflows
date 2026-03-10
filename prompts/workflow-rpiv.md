---
description: Run the personal RPIV workflow with inference-first phase routing, topology selection, execute-mode selection, and artifact weight.
argument-hint: "[task_or_scope] [phase=auto|research|plan|execute|verify] [mode=auto|solo|team|deep-team] [execute_mode=auto|approval_gated|autonomous|parallel_autonomous]"
---
Use `workflow-rpiv-codex` as the canonical RPIV workflow for: $ARGUMENTS

Execution contract:
- Use `research -> plan -> execute -> verify` as the canonical sequence and treat `solo|team|deep-team` as topology, not as separate workflows.
- Infer the earliest necessary phase span, topology, execute mode, and artifact weight unless the user overrides them.
- Create or reuse one slug-consistent `.codex-workflow/{slug}/` state directory.
- Keep one canonical orchestrator thread that does not execute source changes directly.
- For non-trivial work, maintain a short native `update_plan` checklist; use native `request_user_input` only for material workflow forks.
- When the blocker is filesystem or network access, prefer native `request_permissions` with the smallest fitting named permission profile.
- Keep human decision forks and capability escalation separate.
- Prefer the built-in `explorer` for bounded codebase lookups and full Research only when planner-ready artifacts or deeper evidence are required.
- Keep artifact weight proportional to the work: compact for `lightweight`, durable and resumable for `standard` or `graph`.
- If delegated phase work stalls or fails, write the current artifact state, downgrade topology or step back to the earliest invalid phase explicitly, and return one clear next step.
- Do not stop while later required phases or obvious next actions remain unless the user asked to pause.
- Return: inferred phase span, active topology, active execute mode when relevant, artifact weight, artifact paths, blockers, and next step.
