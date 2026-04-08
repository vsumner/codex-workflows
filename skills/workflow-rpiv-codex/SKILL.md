---
name: workflow-rpiv-codex
description: This skill should be used when the user asks to "run RPIV", "do research plan execute verify", "use the full workflow", "take this through the whole workflow", or wants the full personal workflow rather than only one phase.
---

# Workflow RPIV (Codex)

Run Victor's full personal workflow:
1. Research
2. Plan
3. Execute
4. Verify

## Core Rules
- Treat RPIV as the workflow.
- Treat `solo`, `team`, and `deep-team` as execution topology.
- Treat `approval_gated`, `autonomous`, and `parallel_autonomous` as Execute-mode state, not as a phase or topology.
- Keep the orchestrator out of source implementation.
- Keep one slug-consistent artifact directory under `.codex-workflow/`.
- Infer the minimal necessary phase span from the request and current artifacts before doing work.
- Infer topology and artifact weight before defaulting to the heaviest path.
- Treat inferred topology as advisory until delegation is explicitly activated.
- Activate delegated workers only when the user explicitly asks for delegation, parallel work, or `team|deep-team`, or when the current run is already in that explicitly delegated mode.
- Use native thread lifecycle semantics for continuity.
- For non-trivial work, maintain a native `update_plan` checklist in parallel with repo artifacts.
- Use `update_plan` for live progress only; use RPIV artifacts for durable state.
- Use `request_user_input` only for material decision forks that cannot be resolved safely from context.
- Do not stop early while later required phases or obvious next actions remain.
- If the user's target changes mid-run, restate the new target, preserve compatible artifacts, and retire superseded assumptions explicitly.

## Artifact Set
Use the artifact contract from:
- `../../playbooks/personal-swarm-workflow.md`
- `../../playbooks/workflow-artifacts.md`

Default artifact weight model:
1. `lightweight`: compact `research.md`, `research.json`, `plan.md`, and phase-local verification notes
2. `standard`: add `validation-contract.md`, `execution-summary.md`, `validation-state.json`, and `verification-report.md`
3. `graph`: add `features.json` and require the full resumable artifact set

For `standard` and `graph` runs, Verify is not complete or resumable-ready unless
`validation-state.json` and `verification-report.md` both exist, even when the verdict is
`failed` or `blocked`.

Minimum artifacts for non-trivial work:
1. `research.md`
2. `research.json`
3. `plan.md`
4. `features.json`
5. `validation-contract.md`
6. `validation-state.json`
7. `execution-summary.md`
8. `verification-report.md`

## Inference Rules
### Phase Span
1. Start at Research when artifacts are missing or uncertainty is material.
2. Start at Plan when research is sufficient but planning is missing, stale, or explicitly requested.
3. Start at Execute when planning artifacts are ready and the user wants code changed or continued.
4. Start at Verify when execution evidence exists and the user wants proof or readiness.
5. If later-phase artifacts contradict earlier ones, step back to the earliest invalid phase.

### Topology
1. `solo` for tiny, tightly coupled, low-risk work.
2. `team` for medium work, context-loss risk, or work that splits cleanly.
3. `deep-team` for risky refactors, public APIs, concurrency, or uncertain integrations.
4. If `team` or `deep-team` is inferred without explicit delegation activation, keep the run local and record the recommendation rather than spawning workers automatically.

### Execute Mode
1. `approval_gated` for risky or still-ambiguous execution awaiting an explicit go/no-go or phase handoff.
2. `autonomous` for low-risk unblocked execution where minor ambiguity can be resolved from context.
3. `parallel_autonomous` for the same autonomy model when multiple independent packets are ready together.

### Artifact Weight
1. `lightweight` for small, low-risk work.
2. `standard` for medium or multi-step work.
3. `graph` for risky, dependency-heavy, resumable, or multi-track work.

## Phase Sequence
1. Run only the necessary RPIV phases in order, starting from the inferred earliest phase.
2. Use the built-in `explorer` for bounded codebase questions that do not justify planner-ready research artifacts.
3. Use `workflow-research-codex` when Research is required.
4. Use `workflow-plan-codex` when Plan is required.
5. Use `workflow-execute-codex` when Execute is required.
6. Use `workflow-verify-codex` when Verify is required.
7. Run `workflow-review-codex` when risk warrants it or when Verify escalates to Review.

## `update_plan` Rules
1. Create a short checklist for non-trivial runs.
2. Keep 3-5 steps maximum.
3. Use phase-first steps such as `Research`, `Plan`, `Execute`, `Verify`, and `Review/Fix loop` only when needed.
4. Keep exactly one `in_progress` step until the run is complete.
5. Update the checklist at every phase transition and major truth change.
6. Do not mirror packet graphs, `features.json`, or long artifact structure into the checklist.
7. Mark all steps `completed` when the run is done.

## `request_user_input` Rules
1. Use it only in the main thread.
2. Use it only when the decision would materially change the plan, topology, or remediation path.
3. Prefer it at plan time, phase transitions, or explicit branch points.
4. Keep it short: 1-3 questions, 2-3 options each.
5. Do not use it for routine ambiguity, missing low-risk details, or things Codex can infer.
6. Do not delegate a worker that depends on `request_user_input`.

## Prompt Contracts
- Use explicit output contracts for phase outputs and handoffs.
- Use explicit completeness contracts for multi-phase runs so RPIV does not stop at a plausible midpoint.
- Use explicit verification loops when a phase can otherwise end in a wrong-but-plausible state.
- Use explicit user-update rules so long-running workflows do not narrate every micro-step.

## Thread Lifecycle Rules
1. Continue the same orchestrator thread when the initiative and state are still coherent.
2. Resume an existing paused thread before starting a fresh thread for the same slug.
3. Fork when you need a speculative branch, competing approach, or a risky side path.
4. Archive or close threads that are complete, superseded, or untrustworthy.
5. Do not create a new thread just because the phase changed.
6. Prefer fresh worker threads over trying to rescue stale confused workers.

## Guardrails
- Do not skip Research when uncertainty is material.
- Do not skip Plan for medium or larger work.
- Do not start execution until the plan is stable enough for the inferred artifact weight.
- Do not conclude before required assertions pass.
- If delegated phase work times out or stalls, write the current artifact state and return one explicit next step instead of leaving the run half-finished.

## References
- `../../playbooks/personal-swarm-workflow.md`
- `../../playbooks/workflow-artifacts.md`
