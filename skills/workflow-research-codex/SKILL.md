---
name: workflow-research-codex
description: This skill should be used when the user asks to "research this", "investigate this first", "analyze before coding", "gather findings", "do deep research", or wants the Research phase of the personal workflow.
---

# Workflow Research (Codex)

Run the Research phase and emit planner-ready artifacts.

## Inference Rules
### Depth
1. `quick` for bounded repo-local questions in familiar code with low ambiguity.
2. `medium` for cross-file work, moderate ambiguity, or unclear ownership.
3. `deep` for unfamiliar architecture, external/runtime uncertainty, public APIs, concurrency, or expensive mistakes.

### Topology
1. `solo` for most `quick` research.
2. `team` for `medium` research or when repo discovery and architecture analysis split cleanly.
3. `deep-team` for `deep` research or when learning tests and external context both matter.

## Team Shape
Default research team for non-trivial work:
1. `workflow_orchestrator`
2. `research_locator`
3. `architecture_analyst`
4. `researcher` and/or `learning_tester`

Start small.
- Default to 1-2 delegated workers.
- Add more only when the research cleanly splits into independent tracks.

## Required Outputs
Write:
1. `research.md`
2. `research.json`

Research outputs must include:
1. file inventory
2. control flow and architecture analysis
3. dependencies
4. external context
5. verified assumptions
6. unverified assumptions
7. AI-slop risks
8. directives for planner
9. suggested verification targets

## Rules
- Keep research read-only.
- For non-trivial research, start or update a native `update_plan` checklist and keep the in-progress step on Research.
- Prefer the built-in `explorer` for bounded repo questions, symbol lookups, and one-shot codebase discovery instead of opening a full research run.
- Escalate to full Research when planner-ready artifacts, cross-cutting analysis, or external/runtime evidence are actually needed.
- Use this skill as the canonical research path. Do not fall back to archived generic research prompts unless the user explicitly wants that older flow.
- Learning tests are a research subroutine.
- Separate facts, inferences, and unknowns.
- Cite exact file paths for local evidence and exact URLs for retrieved external evidence.
- Keep retrieving or probing until another pass is unlikely to change the conclusion.
- If the first search or probe is thin, broaden once before concluding the evidence is insufficient.
- Research should reduce ambiguity for planning, not drift into implementation.
- Create `.codex-workflow/{slug}/` early, then actually write `research.md` and `research.json` before declaring the phase complete.
- Prefer artifact-first output over long transcript synthesis.
- Keep the artifact shape stable, but scale the depth of each section to the inferred research depth.
- If uncertainty remains too high for planning, say so explicitly and keep the run in Research.
- Mark Research complete in `update_plan` before handing off to Plan.

## References
- `../../playbooks/workflow-artifacts.md`
- `../../playbooks/learning-tests.md`
