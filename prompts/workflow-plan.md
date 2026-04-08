---
description: Run the Plan phase team to produce a canonical plan and emit a feature graph only when it earns its keep.
argument-hint: "[task_or_scope] [mode=auto|solo|team|deep-team] [weight=auto|lightweight|standard|graph]"
---
Run the Plan phase for: $ARGUMENTS

Execution contract:
- Read `research.md` and `research.json` first.
- For non-trivial planning, start or update a native `update_plan` checklist and move the in-progress step to Plan.
- If planning reaches a material fork that cannot be resolved safely from context, use native `request_user_input` from the main thread.
- If research artifacts are missing and uncertainty is material, step back to Research instead of pretending planning is ready.
- Keep planning read-only with respect to source files.
- Do not create temp files or use shell redirection that changes system state.
- Infer planning weight unless the user overrides it:
1. `lightweight` for small, tightly coupled, low-risk work
2. `standard` for medium, multi-step, or moderately risky work
3. `graph` for risky, dependency-heavy, resumable, or multi-track work
- Infer topology from planning weight:
1. `solo` for most `lightweight` planning
2. `team` for `standard` planning
3. `deep-team` for `graph` planning or when critique and verification strategy are heavy
- Produce one canonical `plan.md`.
- Artifact policy by planning weight:
1. `lightweight`: `plan.md` is required; keep validation strategy inline and skip `features.json` unless decomposition clearly helps
2. `standard`: write `plan.md` and `validation-contract.md`; create `features.json` when there are 2+ packets or resumeability benefits
3. `graph`: require `plan.md`, `validation-contract.md`, and `features.json`
- If the user explicitly asked for delegation or selected `mode=team|deep-team`, use a plan team:
1. `workflow_orchestrator`
2. `planner`
3. `plan_reviewer`
- Otherwise keep planning local, record the recommended topology, and only delegate if a later explicit instruction asks for it.
- Require:
1. explicit constraints and non-goals
2. verification strategy in the plan
3. stop conditions for planning refinement
4. no competing plan artifacts
5. `Critical Files` section listing the 3-5 files most important for execution
- Do not emit a graph just because the schema exists. Emit it because execution or resumeability benefits from it.
- Completeness contract:
1. do not mark Plan complete until constraints, non-goals, acceptance criteria, verification strategy, and critical files are explicit
2. do not emit `features.json` unless execution or resumability materially benefits from it
3. do not leave multiple competing plans active
- User updates:
1. keep them brief and decision-oriented
2. update when plan shape changes, when the graph is or is not justified, and when planning is ready for Execute
- Mark Plan complete in `update_plan` before handing off to Execute.
- Return: chosen weight, chosen topology, plan summary, graph summary, review findings, artifact paths, next step.
