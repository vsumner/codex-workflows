---
description: Run the Execute phase team using bounded Spark executors and per-packet verification.
argument-hint: "[task_or_scope] [mode=auto|solo|team|deep-team] [execute_mode=auto|approval_gated|autonomous|parallel_autonomous]"
---
Run the Execute phase for: $ARGUMENTS

Execution contract:
- Read `plan.md`, `research.md`, and `validation-contract.md` first.
- Read `features.json` when the run uses graph artifacts.
- For non-trivial execution, start or update a native `update_plan` checklist and move the in-progress step to Execute.
- If `features.json` is absent, infer whether the plan is intentionally lightweight:
1. if the plan is small and tightly coupled, execute directly from `plan.md`
2. if the work is actually multi-packet or resumable, step back and compile the graph first
- Do not re-architect the task inside execution unless reality invalidates the plan.
- Infer topology unless the user overrides it:
1. `solo` for one tight packet or low-risk tightly coupled work
2. `team` for multiple mostly independent packets or when bounded delegation will preserve context
3. `deep-team` for risky changes, heavy verification pressure, or dependency-heavy execution
- Infer Execute-mode state separately from topology unless the user overrides it:
1. `approval_gated` for risky, destructive, expensive, or still-ambiguous execution that should pause for an explicit go/no-go or phase handoff
2. `autonomous` for low-risk unblocked execution where minor ambiguity can be resolved from context
3. `parallel_autonomous` when multiple low-risk independent packets are ready and verification capacity can keep up
- Use an execution team for non-trivial work:
1. `workflow_orchestrator`
2. `spark_implementer`
3. `spark_implementer_xhigh`
4. `packet_verifier`
5. `integrator` when needed
- Team shape rules:
1. use `spark_implementer` by default for bounded packets
2. use `spark_implementer_xhigh` only for hard or failure-prone packets
3. add `integrator` only when packet outputs must merge cleanly
4. keep work solo when the coordination cost is obviously higher than the benefit
- Execute by dependency layers or waves.
- When multiple packets are ready, prefer dependency order and then lowest task ID first.
- Keep ownership boundaries explicit.
- If one packet needs a narrow repo lookup, prefer the built-in `explorer` over reopening broad Research.
- Verify each packet before accepting it into `features.json`.
- Update `execution-summary.md` continuously with evidence and deviations.
- In graph runs, `features[].status = done` means execution-complete and ready for Verify for that packet, not globally verified.
- When execution stabilizes and cleanup is warranted, use `/simplify` as a bounded refinement pass instead of reopening planning.
- For bug-focused work driven by UBS findings, prefer `bug-scanner-autopilot-codex` as a specialized Execute+Verify loop.
- When all planned packets for the current wave are accepted, hand off immediately to Verify unless the user explicitly defers verification.
- Completeness contract:
1. a packet is not complete until its acceptance criteria are met with verification evidence
2. do not stop after the first accepted packet if more unblocked packets remain
3. if execution invalidates the plan materially, stop and step back instead of improvising a new plan silently
4. low-risk execution ambiguity may continue in `autonomous` modes, but material forks still require native `request_user_input`
- User updates:
1. keep them brief and wave-oriented
2. update on accepted packets, blocked packets, and Verify handoff readiness
- Mark Execute complete in `update_plan` before handing off to Verify.
- Return: chosen topology, chosen execute mode, feature status, accepted packets, failed packets, evidence, next step.
