---
description: Run the Research phase team and emit planner-ready research artifacts.
argument-hint: "[task_or_scope] [mode=solo|team|deep-team] [depth=quick|medium|deep]"
---
Run the Research phase for: $ARGUMENTS

Execution contract:
- For non-trivial research, start or update a native `update_plan` checklist.
- Keep the checklist phase-oriented, not file-oriented.
- Infer research depth unless the user overrides it:
1. `quick` for bounded repo-local questions in familiar code with low ambiguity
2. `medium` for moderate ambiguity, cross-file work, or unclear ownership
3. `deep` for unfamiliar architecture, external/runtime uncertainty, public APIs, concurrency, or expensive mistakes
- Infer topology from depth and split potential:
1. `solo` for most `quick` research
2. `team` for `medium` research or when repo discovery and architecture analysis can split cleanly
3. `deep-team` for `deep` research or when learning tests and external context both matter
- If the request is only a bounded repo question or file lookup, prefer the built-in `explorer` instead of opening a full research run.
- Escalate from `explorer` back to full Research only when planner-ready artifacts, cross-cutting architecture analysis, or external/runtime evidence are required.
- Record the chosen depth in `research.md` and `research.json`.
- Use `workflow-research` as the canonical research entrypoint. Do not pivot to archived generic research flows unless the user explicitly asks for them.
- If the user explicitly asked for delegation or selected `mode=team|deep-team`, use a research team:
1. `workflow_orchestrator`
2. `research_locator`
3. `architecture_analyst`
4. `researcher` and/or `learning_tester` when needed
- Otherwise keep research local, note the recommended topology in the artifacts, and only delegate if a later explicit instruction asks for it.
- When delegation is active, start with the smallest useful team. Default to 1-2 delegated research workers and only expand when the scope truly splits.
- Keep all research read-only.
- Use absolute file paths in artifact references and worker summaries.
- Write `research.md` and `research.json` under `.codex-workflow/{slug}/`.
- Create the slug directory early, but do not stop at directory creation. The phase is incomplete until both artifacts exist with planner-ready content.
- Keep the artifact shape stable, but scale depth:
1. `quick`: compact findings and planner directives only for relevant sections
2. `medium`: normal planner-ready coverage
3. `deep`: expanded evidence, assumptions, and verification targets
- Include:
1. file inventory
2. control flow and architecture analysis
3. dependencies
4. external context
5. verified assumptions
6. unverified assumptions
7. AI-slop risks
8. directives for planner
9. suggested verification targets
- Learning tests belong here; fold their results into the research artifacts.
- Keep findings compact and artifact-first. Prefer writing the research artifacts over producing long chat synthesis.
- Research mode:
1. separate facts, inferences, and unknowns explicitly
2. cite exact file paths for local evidence and exact URLs for retrieved external evidence
3. keep retrieving or probing until another pass is unlikely to change the conclusion
4. if the first search or probe is weak, broaden once before concluding the evidence is insufficient
- Completeness contract:
1. do not mark Research complete until all required sections are filled, explicitly not-applicable, or explicitly blocked by missing evidence
2. do not collapse unknowns into assumptions without labeling them
- User updates:
1. keep them brief and milestone-based
2. update on scope changes, key discoveries, and completion readiness
- If scope changes materially during research, update the artifacts instead of restarting the phase from scratch.
- Mark Research complete in `update_plan` before handing off to Plan.
- If uncertainty remains too high for responsible planning, say so explicitly instead of pretending research is complete.
- Return: chosen depth, chosen topology, research summary, open unknowns, planner directives, artifact paths.
