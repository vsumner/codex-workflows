# Codex Workflows Operator Playbook

## Goal
Run Codex with predictable workflow routing, clear command priority, and explicit review/verification evidence.

Inference-first default:
- infer the earliest necessary phase
- infer the lightest topology that preserves correctness
- infer Execute-mode state separately from topology when running the Execute phase
- infer the lightest artifact weight that still keeps the run resumable and reviewable
- ask only when the ambiguity would materially change behavior
- use native `update_plan` automatically for non-trivial runs
- use native `request_user_input` only for material forks that cannot be inferred safely
- use native thread lifecycle semantics for continuity

## Command Matrix
- `/plan <task>`: enter Plan mode; with inline text, immediately submit the planning request.
- `/fast [on|off|status]`: toggle Fast mode (service tier).
- `/review`: run standard review flow on current changes.
- `/review <instructions>`: run targeted review with custom scope/criteria.
- `/simplify`: run post-change simplify triage (reuse, quality, efficiency), optionally apply minimal fixes.
- `/prompts:workflow-rpiv`: canonical launch path for the personal RPIV workflow.
- `/prompts:workflow-research`: canonical launch path for the Research phase team.
- `/prompts:workflow-plan`: canonical launch path for the Plan phase team.
- `/prompts:workflow-execute`: canonical launch path for the Execute phase team.
- `/prompts:workflow-verify`: canonical launch path for the Verify phase team.
- `/prompts:workflow-review`: canonical launch path for a dedicated review team.
- `/prompts:workflow-authoring`: canonical launch path for workflow-surface authoring and refactoring.
- `/agent`: open the agent/thread picker (switch/resume/close via picker UI).
- `/skills`: inspect available skills.
- `/apps`: manage available/connected apps and connectors.
- `/collab`: open collaboration mode picker (mode selection, not agent spawning).
- `/experimental`: toggle experimental features (including multi-agent).
- `/prompts:<name> [args]`: expand and run a saved custom prompt template.
- Recommended multi-agent templates:
1. `/prompts:workflow-rpiv` (full Research -> Plan -> Execute -> Verify)
2. `/prompts:workflow-research` (phase-first research team)
3. `/prompts:workflow-plan` (canonical plan and graph creation)
4. `/prompts:workflow-execute` (bounded executor team)
5. `/prompts:workflow-verify` (validation team)
6. `/prompts:workflow-review` (dedicated review team)
7. `/prompts:workflow-solo` (RPIV in a single thread)
8. `/prompts:workflow-team` (RPIV with teams)
9. `/prompts:workflow-deep-team` (RPIV with stronger validation)
10. `/prompts:workflow-learning-tests` (assumption-proof subroutine)
11. `/prompts:workflow-authoring` (workflow surface creation/refactoring)
12. `/prompts:bug-scanner-autopilot` (UBS scanner-to-verification autopilot)

## Verified Semantics
- `/agent` does not support text subcommands like `list|switch|inspect|stop|close`.
- `/collab` changes collaboration mode only; feature enablement is handled by `/experimental` or `config.toml`.
- `/fast` persists service tier selection (`on` => `fast`, `off` => unset) and requires `features.fast_mode = true`.
- `/apps` requires `features.apps = true` to appear and function.
- `/review` and `/plan` support inline arguments; `/agent` and `/collab` do not.
- `/prompts:*` is the right compatibility layer for Claude/Claudify-style reusable commands.
- Use `bug-scanner-autopilot-codex` for UBS scanner orchestration (`review-only` default, `mode=apply` for fixes).

## Review Adaptation Rules
- `/review` should auto-load domain skills by repository/language signals (Rust, React, TypeScript, Node, Nix, Go, Python, Next.js when relevant).
- In `spacedriveapp/spacebot`, `/review` should auto-load `jamiepine-style` + `rust` and include `rust_correctness_reviewer`.
- Keep `/review-spacebot` as optional compatibility alias only.
- Review should infer `source` first:
1. PR number, PR URL, copied review comments, or current branch PR context -> PR review
2. otherwise staged diff, then `HEAD` diff -> local review
- Review should infer `stance` next:
1. local change review or "my changes" -> author review
2. PR review or third-party framing -> reviewer review
- PR review should auto-run `pr-comments` when GitHub context is available and deduplicate existing feedback before new findings.
- For explicit GitHub PR number/URL review, prefer PR metadata, diff, comments, and branch history first; treat RPIV artifacts as optional context only when already present and clearly relevant.
- Review should resolve intent-source precedence as:
1. explicit user request
2. RPIV artifacts when present and clearly applicable
3. PR title/body
4. commit history / branch history
- Local author review should include simplify-style cleanup lanes by default.
- Remote PR review should stay read-only and suppress simplify noise unless cleanup is explicitly requested.
- Add history or instruction-compliance lanes only when they introduce distinct evidence.
- Keep screenshot/browser/user-flow evidence in Verify unless a later plan intentionally changes that boundary.

## Extension Matrix
- Native slash commands first for built-ins.
- Skills second for non-native workflows.
- `/prompts:*` third for reusable templates and compatibility aliases.

## RPIV Utility Mapping
- Execute refinement: `simplify-codex`
- Execute+Verify bug lane: `bug-scanner-autopilot-codex`
- Execute PR remediation: `fix-pr-feedback-codex`
- Verify scrutiny gates: `verify-gates-codex`
- Verify behavior proof: `verification-specialist`
- Review engine: `review-workflow-codex`
- Spacebot review specialization: `review-spacebot-codex`
- Workflow surface authoring: `workflow-authoring-codex`

## Natural Language First
- Do not require slash syntax when intent is clear.
- Infer workflow from plain-language intent and execute the matching command/skill.
- Common intent routing:
1. review intent -> `review-workflow-codex` with inferred local-vs-PR and author-vs-reviewer routing (native `/review` only when explicitly requested)
2. verification intent -> `verify-gates-codex`
3. simplify intent -> `simplify-codex`
4. full non-trivial workflow intent -> `/prompts:workflow-rpiv`
5. quick codebase-question intent -> built-in `explorer`
6. research intent -> `/prompts:workflow-research`
7. planning intent -> `/prompts:workflow-plan` (native `/plan` only when explicitly requested)
8. execution intent -> `/prompts:workflow-execute`
9. validation intent -> `/prompts:workflow-verify`
10. dedicated review intent -> `/prompts:workflow-review`
11. risky implementation topology -> `/prompts:workflow-deep-team`
12. assumption-proof intent -> `/prompts:workflow-learning-tests`
13. workflow surface authoring intent -> `/prompts:workflow-authoring`
14. scanner autopilot intent -> `/prompts:bug-scanner-autopilot`
15. PR remediation intent -> `/prompts:fix-pr-feedback`
- Infer missing inputs from context (diff, plan files, repo signals) before raising `BLOCKED`.
- For PR remediation, reuse the current repo root only when it is already aligned to the PR head and local state is safe; otherwise prefer an isolated worktree.
- Resolve the exact PR head ref/SHA before creating or resetting a remediation workspace; do not rely on ambiguous `FETCH_HEAD` state after multi-ref fetches.
- PR remediation should default to autonomous local closure of clear findings and escalate only for ambiguous fixes, catastrophic repo issues, ownership questions, or explicit publish requests.
- If review findings already exist and the user says "fix this PR" or equivalent, route to `/prompts:fix-pr-feedback` rather than generic review apply mode.
- PR remediation must stop with a prepared publish plan instead of committing, pushing, or editing PR metadata unless Victor explicitly asks.
- Prefer silent correct routing over asking the user to choose between workflow variants.
- For RPIV, infer:
1. earliest necessary phase from artifacts plus request
2. `quick|medium|deep` research depth from ambiguity and external risk
3. `lightweight|standard|graph` planning weight from coupling, risk, and resumability needs
4. `solo|team|deep-team` execution topology from packet independence and risk
5. `approval_gated|autonomous|parallel_autonomous` execute mode from execution risk and autonomy tolerance
6. `gates|behavior|full` verification proof weight from the type of evidence required
- For non-trivial RPIV runs, maintain a short native `update_plan` checklist:
1. one current phase step
2. one next phase step
3. optional remediation/review step when needed
4. never mirror the graph or every task packet
- When a user decision is genuinely required, prefer `request_user_input` over long free-form clarification:
1. main thread only
2. short structured questions
3. only when the answer materially changes the workflow
- For continuity, prefer native thread lifecycle:
1. continue same thread when coherent
2. resume before recreating
3. fork for speculative divergence
4. archive/close superseded threads

## Multi-Agent Operating Rules
- Use `explorer` for read-only repo discovery and scoped questions.
- Do not open a full Research run for a one-shot repo lookup unless the answer needs planner-ready artifacts or external/runtime evidence.
- Use `spark_implementer` or `spark_implementer_xhigh` for bounded execution tasks.
- Use `workflow_orchestrator` as the canonical owner for RPIV phase transitions.
- Keep orchestrators out of hands-on implementation.
- Use a research team before planning when uncertainty is material.
- Use a plan reviewer before implementation when work is non-trivial.
- Use packet verifiers during implementation, not only at the end.
- Require `workflow_verifier` for non-trivial runs and `workflow_reviewer` for risky or high-stakes runs.
- Prefer parallel subagents only for independent, non-overlapping scopes.
- Prefer wave execution by default; reserve full-parallel super swarms for explicit speed-first runs.
- Wait for all delegated subagents before final handoff.
- Close or switch threads through `/agent` when coordination is needed.
- Default to the smallest team that fits the inferred topology; do not spawn because a role exists.
- In review workflows, keep baseline lanes always-on and treat history/compliance lanes as conditional context lanes.

## Review Contract
- Verdict must be `PASS` or `FAIL`.
- Findings must be severity-ranked (`critical`, `major`, `minor`).
- Every finding must include `file:line` evidence and impact.
- Include smallest concrete fix list.
- Include targeted verification commands.
- Include residual risk.

## Verification Contract
- Overall verdict `PASS` or `FAIL`.
- Per-gate status `PASS|FAIL|SKIP` with command and evidence.
- Failed-gate root causes and minimal remediation list.
- Rerun plan for failed gates only.
- Residual risk.

## Failure Loop Limits
- Review/verify retry cap: 2 loops per finding set or gate.
- After 2 failed loops, escalate with blocker details and options.
