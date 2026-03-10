---
description: Run the Research phase team and emit planner-ready research artifacts.
argument-hint: "[task_or_scope] [mode=solo|team|deep-team] [depth=quick|medium|deep]"
---
Use `workflow-research-codex` as the canonical Research phase for: $ARGUMENTS

Execution contract:
- Infer research depth (`quick|medium|deep`) and topology (`solo|team|deep-team`) unless the user overrides them.
- Prefer the built-in `explorer` for bounded repo questions; escalate to full Research only when planner-ready artifacts, cross-cutting analysis, or external/runtime evidence are actually needed.
- Keep Research read-only.
- For non-trivial work, maintain a short native `update_plan` checklist with Research in progress.
- Write `research.md` and `research.json` under `.codex-workflow/{slug}/`.
- Artifacts must separate facts, inferences, and unknowns and cover file inventory, control flow/architecture, dependencies, assumptions, planner directives, and suggested verification targets.
- Use absolute file paths for local evidence and exact URLs for external evidence.
- If delegated research lanes fail or stall, downgrade topology explicitly, continue locally, and still finish the required artifacts.
- Mark Research complete in `update_plan` before handing off to Plan.
- Return: chosen depth, chosen topology, artifact paths, open unknowns, and planner directives.
