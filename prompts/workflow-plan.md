---
description: Run the Plan phase team to produce a canonical plan and emit a feature graph only when it earns its keep.
argument-hint: "[task_or_scope] [mode=auto|solo|team|deep-team] [weight=auto|lightweight|standard|graph]"
---
Use `workflow-plan-codex` as the canonical Plan phase for: $ARGUMENTS

Execution contract:
- Read `research.md` and `research.json` first; if they are missing or insufficient, step back to Research.
- Infer planning weight (`lightweight|standard|graph`) and topology unless the user overrides them.
- Keep planning read-only except for required RPIV artifacts.
- For non-trivial work, maintain a short native `update_plan` checklist with Plan in progress.
- Produce one canonical `plan.md`.
- Emit `validation-contract.md` for `standard` and `graph` work, and emit `features.json` only when decomposition or resumability clearly earns it.
- Make constraints, non-goals, acceptance criteria, verification strategy, stop conditions, and `Critical Files` explicit.
- Use native `request_user_input` only for material planning forks that cannot be resolved safely from context.
- If delegated planning lanes fail or stall, downgrade topology explicitly, continue locally, and still leave one coherent plan artifact set.
- Mark Plan complete in `update_plan` before handing off to Execute.
- Return: chosen weight, chosen topology, artifact paths, major constraints, and execution-ready packets or the blocking reason.
