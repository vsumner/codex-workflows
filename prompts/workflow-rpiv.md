---
description: Run the personal RPIV workflow with inference-first phase routing, topology selection, execute-mode selection, and artifact weight.
argument-hint: "[task_or_scope] [phase=auto|research|plan|execute|verify] [mode=auto|solo|team|deep-team] [execute_mode=auto|approval_gated|autonomous|parallel_autonomous]"
---
Run the personal RPIV workflow for: $ARGUMENTS

Execution contract:
- Use `research -> plan -> execute -> verify` as the canonical sequence.
- Treat `solo`, `team`, or `deep-team` as topology, not as the workflow itself.
- Create or reuse `.codex-workflow/{slug}/`.
- Keep one canonical orchestrator thread that does not execute source changes directly.
- Use native thread lifecycle semantics for continuity:
1. continue the same thread when the initiative and state are still coherent
2. fork for speculative branches or materially divergent approaches
3. resume an existing paused thread before creating a fresh one for the same slug
4. archive/close threads that are complete, superseded, or no longer trustworthy
- For non-trivial work, maintain a native Codex `update_plan` checklist alongside repo artifacts.
- Use `update_plan` as the live operator HUD:
1. keep 3-5 short steps
2. exactly one `in_progress` step
3. update it on phase transitions, remediation loops, and major truth changes
4. do not mirror `features.json` line-for-line
- Use native `request_user_input` only for material decision forks that cannot be resolved safely from context.
- Keep `request_user_input` narrow:
1. main thread only
2. 1-3 short questions
3. 2-3 concrete options each
4. no use for routine ambiguity or things Codex can infer
- Infer the minimal necessary phase span from the request and current artifacts:
1. no usable artifacts or material uncertainty -> start at Research
2. research is present but planning is missing, stale, or explicitly requested -> start at Plan
3. plan is present and the user wants code changed, continued, or resumed -> start at Execute
4. execution evidence is present and the user wants proof, readiness, or validation -> start at Verify
5. if a later-phase artifact contradicts earlier artifacts, step back to the earliest invalid phase
- Infer topology unless the user overrides it:
1. `solo` for tiny, tightly coupled, low-risk work
2. `team` for medium work, context-loss risk, or cleanly split work
3. `deep-team` for risky refactors, public APIs, concurrency, or uncertain integrations
- Topology inference does not itself authorize subagent spawning.
- Only activate delegated workers when the user explicitly asks for delegation, parallel work, or `team|deep-team`, or when the existing run is already operating in that explicitly delegated mode.
- If topology is inferred as `team` or `deep-team` without explicit delegation activation, keep the run local, record the recommended topology, and continue.
- When Execute is in scope, infer Execute-mode state separately from topology:
1. `approval_gated` for risky or still-ambiguous execution awaiting an explicit go/no-go or phase handoff
2. `autonomous` for low-risk unblocked execution where minor ambiguity can be resolved from context
3. `parallel_autonomous` for the same autonomy model when multiple independent packets are ready
- Infer artifact weight unless the user forces a heavier path:
1. `lightweight` for small, low-risk work
2. `standard` for medium or multi-step work
3. `graph` for risky, resumable, or dependency-heavy work
- Only create artifacts for phases actually run, but keep slug-consistent state.
- Artifact policy by weight:
1. `lightweight`: compact `research.md`, `research.json`, `plan.md`, and phase-local verification notes; skip `features.json` unless decomposition helps
2. `standard`: add `validation-contract.md`, `execution-summary.md`, `validation-state.json`, and `verification-report.md`; create `features.json` when multiple packets or resumability justify it
3. `graph`: require the full artifact set, including `features.json`
- For `standard` and `graph` runs, do not treat Verify as complete or resumable-ready unless `validation-state.json` and `verification-report.md` both exist, even when the verdict is `failed` or `blocked`.
- Do not start execution until research and plan outputs are good enough for the inferred weight.
- For `lightweight` runs, let `update_plan` carry most of the visible checklist load.
- For `standard` and `graph` runs, use `update_plan` for live progress and artifacts for durable state.
- Prefer `request_user_input` at plan-time or phase boundaries, not deep inside execution.
- Use the built-in `explorer` for one-shot codebase questions; use full Research only when planner-ready artifacts or deeper evidence are needed.
- Run narrow remediation loops when verification fails.
- Completeness contract:
1. do not stop once one artifact exists if later required phases are still obviously needed
2. do not hand off with implicit "next obvious step" left undone unless the user asked to pause
3. if the request changed materially mid-run, restate the new target and explicitly mark superseded work
4. if delegated phase work times out or stalls, write the current artifact state and return one explicit next step instead of leaving the run half-finished
- User updates:
1. keep updates brief and phase-oriented
2. update on phase transitions, blockers, invalidated plans, and final outcomes
3. prefer one sentence on what changed and one sentence on what happens next
- Return: inferred phase span, active topology, active execute mode when Execute is in scope, artifact weight, artifact paths, blockers, next step.
