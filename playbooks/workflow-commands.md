# Workflow Commands

Use these operator commands directly in chat.

- Native built-ins are preferred when available, except where Victor's workflow intentionally standardizes on stronger Plan or Review surfaces.
- `/plan <task>`: native Plan mode switch and optional inline planning request.
- `/collab`: native collaboration-mode picker.
- `/agent`: native agent/thread picker.
- `/fast [on|off|status]`: toggle Fast mode (requires `features.fast_mode = true`)
- `/review`: native Codex review flow
- `review-workflow` (or `review-workflow-codex`): run the parallel reviewer-agent workflow
- `/review-spacebot`: run the Spacebot maintainer + Rust review workflow (`review-spacebot-codex`)
- `/simplify`: run simplify triage/cleanup workflow (`simplify-codex`)
- `/verify-gates`: run standardized language-aware verification gates (`verify-gates-codex`)
- `/spec-sync`: generate/refresh spec-derived contract tests and detect drift (`spec-sync`)
- `/debate-review`: run bounded advocate-vs-auditor review debate (`adversarial-review`)
- `/security-review`: run focused exploitability-first security review (`security-review`)
- `/apps`: manage apps/connectors (requires `features.apps = true`)
- `/pr-comments`: fetch and format PR comments (`pr-comments`)
- `/review-pr`: review PR by number or current branch (`review-pr`)
- `fix-pr-feedback` (or `fix-pr-feedback-codex`): remediate clear PR findings locally with reuse-first workspace selection
- `bug-scanner-autopilot` (or `bug-scanner-autopilot-codex`): run UBS scan -> triage -> fix -> verify workflow
- `verification-specialist`: orchestrate verifier skills against a deterministic plan (`verification-specialist`)
- `init-verifiers`: scaffold verifier skills when none exist (`init-verifiers`)
- `/prompts:review-spacebot`: template alias for Spacebot review instructions
- `/prompts:verify-gates`: template alias for gate-verification instructions
- `/prompts:simplify`: template alias for simplify triage/cleanup
- `/prompts:workflow-rpiv`: canonical entrypoint for the personal RPIV workflow (`workflow-rpiv-codex`)
- `/prompts:workflow-research`: canonical entrypoint for the Research phase team (`workflow-research-codex`)
- `/prompts:workflow-plan`: canonical entrypoint for the Plan phase team (`workflow-plan-codex`)
- `/prompts:workflow-execute`: canonical entrypoint for the Execute phase team (`workflow-execute-codex`)
- `/prompts:workflow-verify`: canonical entrypoint for the Verify phase team (`workflow-verify-codex`)
- `/prompts:workflow-review`: canonical entrypoint for a dedicated review team (`workflow-review-codex`)
- `/prompts:workflow-status`: summarize RPIV artifact and phase state
- `/prompts:workflow-solo`: topology wrapper for running RPIV in a single thread
- `/prompts:workflow-team`: topology wrapper for running RPIV with teams
- `/prompts:workflow-deep-team`: topology wrapper for running RPIV with stronger validation
- `/prompts:workflow-learning-tests`: research subroutine for learning-test execution (`workflow-learning-tests-codex`)
- `/prompts:workflow-resume`: template alias for resuming paused team runs
- `/prompts:workflow-fix-loop`: template alias for focused remediation after failed verification
- `/prompts:workflow-authoring`: create or refactor Codex workflow surfaces (`workflow-authoring-codex`)
- `/prompts:fix-pr-feedback`: template alias for autonomous PR remediation with local verification
- `/prompts:bug-scanner-autopilot`: template alias for UBS scan + triage + minimal fixes + verification

## Canonical Surface
- Native built-ins remain the canonical surface for generic planning, review, agent/thread control, collaboration mode, fast mode, and apps.
- `/prompts:workflow-rpiv`, `/prompts:workflow-research`, `/prompts:workflow-plan`, `/prompts:workflow-execute`, `/prompts:workflow-verify`, and `/prompts:workflow-review` are the canonical public RPIV launch paths.
- `review-workflow`, `/simplify`, `/verify-gates`, `verification-specialist`, `bug-scanner-autopilot`, and `/review-spacebot` are RPIV-owned subroutines, not alternate top-level workflows.
- `fix-pr-feedback` is an Execute-owned PR remediation subroutine, not a separate workflow.
- `workflow-*-codex`, `review-workflow-codex`, and `bug-scanner-autopilot-codex` are backend skill names, not extra public workflows.
- `/prompts:*` wrappers listed here are the public compatibility layer; do not treat the matching skill file as a second user-facing surface.

## RPIV Integration
- `workflow-rpiv` should infer the minimal necessary phase span from the request and existing artifacts before running the workflow.
- RPIV should infer the lightest viable topology and artifact weight before defaulting to `deep-team` or graph artifacts.
- RPIV should infer Execute-mode state separately from topology:
1. `approval_gated` for risky or still-ambiguous execution that should wait for an explicit go/no-go or phase handoff
2. `autonomous` for low-risk unblocked execution where minor ambiguity can be resolved from context
3. `parallel_autonomous` for the same autonomy model when independent packets can run concurrently and verification capacity is available
- RPIV should use native `update_plan` automatically for non-trivial runs.
- `update_plan` should stay short, phase-first, and not mirror `features.json`.
- RPIV should use native `request_user_input` only for material decision forks.
- `request_user_input` should stay main-thread only and should not be used for routine ambiguity.
- `workflow-learning-tests` is the Research subroutine for proving uncertain behavior.
- `/simplify` is the optional Execute refinement pass after packet execution stabilizes.
  It stays a skill plus prompt wrapper, not a role or separate RPIV phase.
- `bug-scanner-autopilot` is the bug-focused Execute+Verify loop when UBS is the right tool.
- `fix-pr-feedback` is the PR remediation Execute+Verify loop for closing clear review findings locally.
- `/verify-gates` is the default Verify scrutiny engine.
- `verification-specialist` is the Verify behavior-proof engine.
- `review-workflow` is the heavy Review engine for parallel lanes, PR-aware dedupe, and post-verify review.
- `/review-spacebot` is the repo-specific Review specialization for `spacedriveapp/spacebot`.
- `/security-review` is the exploitability-first Review specialization for trust-boundary changes.
- `workflow-authoring-codex` is the doctrine-driven authoring utility for prompt/skill/role/playbook changes.

## Natural Language Equivalents
- "review my changes" -> `review-workflow` by default; use native `/review` only when the user explicitly wants the built-in reviewer or a narrow native pass
- "review this PR" / "review PR 123" / GitHub PR URL -> `review-workflow` as PR review
- "check existing PR comments" / "what feedback is still open" -> `/pr-comments`
- "verify this before merge" -> `/verify-gates`
- "check spec drift" -> `/spec-sync`
- "clean this up / simplify this" -> `/simplify`
- "run RPIV / do research plan execute verify" -> `/prompts:workflow-rpiv`
- "where is this wired?" / "find where X happens" / "quick codebase question" -> built-in `explorer`; escalate to `/prompts:workflow-research` only when planner-ready artifacts or deeper evidence are needed
- "research this / investigate this first" -> `/prompts:workflow-research`
- "plan this / create the plan" -> `/prompts:workflow-plan`
- "execute this plan / run the execute phase" -> `/prompts:workflow-execute`
- "verify this / run the validation team" -> `/prompts:workflow-verify`
- "review this after verify / run the review team" -> `/prompts:workflow-review`
- "use a team / split this into subagents / orchestrate workers" -> `/prompts:workflow-team`
- "keep this solo / do this yourself / no swarm for this one" -> `/prompts:workflow-solo`
- "this is risky / do the deep version / use the full team" -> `/prompts:workflow-deep-team`
- "prove this assumption first / learn before building / test the tool behavior" -> `/prompts:workflow-learning-tests`
- "resume the team / pick up the paused swarm" -> `/prompts:workflow-resume`
- "fork this approach / branch this workflow" -> native thread fork semantics via `/agent` plus RPIV artifacts
- "archive this run / close this old thread" -> native thread archive/close semantics via `/agent`
- "fix the failed verifier/reviewer findings" -> `/prompts:workflow-fix-loop`
- "fix this PR" / "address review findings on this PR" / "take this PR to green locally" -> `/prompts:fix-pr-feedback`
- "what phase is this in / what's the workflow status" -> `/prompts:workflow-status`
- "create a workflow / refactor this workflow / should this be a skill or role" -> `/prompts:workflow-authoring`
- "run bug scanner autopilot / scan and auto-fix UBS findings" -> `/prompts:bug-scanner-autopilot`
- "speed this up / fastest mode" -> `/fast on`

## Behavior
- Review is read-only by default.
- `/fast` is a service-tier toggle: `on` sets `service_tier = "fast"` and `off` clears it.
- Fast mode commands are available only when `features.fast_mode = true`.
- `/apps` is available only when `features.apps = true`, and connector mentions use `$`.
- `review-workflow` is the canonical review workflow when Victor asks for review in natural language.
- Use native `/review` when Victor explicitly asks for it or wants the built-in reviewer specifically.
- Use `review-workflow` when you want parallel reviewer agents, PR-comment dedupe, security/adversarial lanes, or review-to-verify/apply orchestration.
- `review-workflow` is context-aware and should auto-load applicable domain skills (`rust`, `react`, `typescript`, `node`, `nix`, `go`, `python`, `nextjs` when relevant).
- Do not let generic native-first routing override `review-workflow` for ordinary review requests.
- In Spacebot (`spacedriveapp/spacebot`), `review-workflow` and `/review-spacebot` should auto-apply `jamiepine-style` + `rust` and include `rust_correctness_reviewer`.
- `review-spacebot-codex` is a home-level compatibility alias in `~/.codex`; do not treat a missing repo-local `.agents/skills/review-spacebot-codex/` path as a review failure.
- `review-workflow` should infer the review path first:
1. explicit PR number, PR URL, review-thread text, or branch PR context -> PR review
2. otherwise staged diff, then `HEAD` diff -> local review
3. `my changes` or local RPIV outputs -> author stance
4. PR review or third-party change framing -> reviewer stance
- For explicit PR number/URL review, `review-workflow` should prefer PR-native context first and treat RPIV artifacts as optional only when already present and clearly relevant.
- `workflow-rpiv` should infer the earliest necessary phase:
1. missing or uncertain context -> Research
2. research present, plan missing/stale -> Plan
3. plan present, changes requested -> Execute
4. execution evidence present, proof requested -> Verify
- `workflow-rpiv` should maintain a native `update_plan` checklist:
1. create it for non-trivial runs
2. update it on phase transitions
3. use it as the live checklist, not the durable artifact store
- RPIV should prefer native thread lifecycle continuity:
1. continue the same thread when coherent
2. resume before recreating
3. fork for materially divergent approaches
4. archive or close superseded threads
- `workflow-rpiv` should use `request_user_input` only when:
1. the decision materially changes the plan or execution path
2. inference is unsafe
3. the question can be asked in a short structured form
- `workflow-research` should infer:
1. `quick` depth for bounded familiar repo-local work
2. `medium` depth for moderate ambiguity or cross-file work
3. `deep` depth for unfamiliar, external, or high-risk work
- `workflow-plan` should infer:
1. `lightweight` plan for small tightly coupled work
2. `standard` plan for medium or multi-step work
3. `graph` plan when resumability, dependency control, or risk justifies `features.json`
- `workflow-execute` should infer:
1. `solo` for one tight packet
2. `team` for cleanly split work
3. `deep-team` for risky or verifier-heavy execution
- `workflow-execute` should infer Execute-mode state separately from topology:
1. `approval_gated` when the next execution step is risky, expensive, destructive, or still awaiting an explicit go/no-go
2. `autonomous` when the next execution step is low-risk and unblocked
3. `parallel_autonomous` when multiple low-risk independent packets are ready together
- Low-risk execution ambiguity may continue in `autonomous` modes, but material forks still require `request_user_input`.
- `workflow-verify` should infer:
1. `gates` proof when command-level checks are enough
2. `behavior` proof when runtime/user-flow evidence matters
3. `full` proof when merge/PR confidence or dedicated review is needed
- `review-workflow` should infer an intent packet from workflow artifacts and the user request before blocking:
1. `goal`
2. `constraints`
3. `non_goals`
4. `acceptance_criteria`
- `review-workflow` should resolve intent-source precedence as:
1. explicit user request
2. RPIV artifacts when present and clearly applicable
3. PR title/body
4. commit history / branch history
- If intent fields are missing, infer from context first; if still unresolved, return `BLOCKED` with missing fields.
- Local author review should include simplify-style cleanup lanes by default.
- Remote PR review should stay read-only, auto-fetch PR comments when possible, deduplicate existing feedback, and suppress simplify noise unless requested.
- Explicit GitHub-link PR review should not spend material time searching for missing `plan.md` / `execution-summary.md` / `verification-report.md`, and their absence should not be treated as review friction.
- `fix-pr-feedback` should reuse the current repo root when it already matches the PR head safely; otherwise it should isolate in a worktree.
- `fix-pr-feedback` should resolve the exact PR head ref/SHA before creating or resetting a remediation workspace; do not rely on ambiguous `FETCH_HEAD` state after multi-ref fetches.
- `fix-pr-feedback` defaults to autonomous local remediation and local verification, but remains approval-gated for unclear ownership, explicit publish requests, or non-trivial rebase conflicts.
- When a review thread is followed by explicit PR-remediation language, route to `fix-pr-feedback` instead of generic review apply mode.
- `fix-pr-feedback` must stop with a prepared publish plan rather than committing, pushing, or editing PR metadata unless Victor explicitly asks.
- `review-workflow` may add conditional context lanes for PR history/rationale and instruction compliance when they provide distinct evidence.
- `review-workflow` should route TS review explicitly through `typescript`, compose `react` + `typescript` for TSX, and compose `nextjs` + `react` + `typescript` when appropriate.
- Plain JS review should route through `node` and `react` as appropriate; do not imply a `javascript` skill unless one exists.
- Review findings must be evidence-backed (`file:line` + concrete check command). Drop speculative/uncited findings.
- Review identifies risk/alignment gaps only; proof of behavior requires:
1. `verification-specialist` for functional verification
2. `/verify-gates` for command-level quality gates
- Screenshot/browser/user-flow evidence remains a Verify concern, not a default Review lane.
- Use `/debate-review` for high-stakes changes:
1. security/auth changes
2. public API/contract changes
3. concurrency/state-machine logic
4. large refactors
- `/debate-review` rules:
1. independent first pass for advocate and auditor
2. max 2 total rounds per side (initial + one rebuttal)
3. unresolved disagreements escalate to human decision
- Use `/spec-sync` when specs are present or drift is suspected to validate spec-implementation alignment.
- `/spec-sync` is report-and-remediate only; no auto-PR behavior.
- Critique/fix convergence limit: maximum 2 loops per finding set, then escalate to human decision.
- `/agent` and `/collab` are picker commands, not text-subcommand CLIs.
- Use native thread lifecycle controls through `/agent` and matching thread semantics, not custom prompt-level restart conventions.
- Use `/prompts:workflow-rpiv`, `/prompts:workflow-research`, `/prompts:workflow-plan`, `/prompts:workflow-execute`, `/prompts:workflow-verify`, and `/prompts:workflow-review` as the canonical personal workflow launch paths.
- Swarm coordination now lives inside RPIV, primarily in the Execute phase, rather than in separate legacy prompt wrappers.
- Treat `/simplify`, `/verify-gates`, `verification-specialist`, `review-workflow`, `bug-scanner-autopilot`, and `/review-spacebot` as RPIV subroutines, not alternate top-level workflows.
- `bug-scanner-autopilot-codex` defaults to `review-only`; use `mode=apply` to permit fixes.
- Bug scanner autopilot must report scanner evidence plus verification evidence before handoff.
- Infer required inputs first; return `BLOCKED` only when remaining ambiguity is material.
- Diff source priority: staged diff, then `HEAD` diff.
- Verification outputs explicit `PASS|FAIL|SKIP` per gate plus an overall verdict.
- Personal default for non-trivial work is RPIV.
- Choose `solo`, `team`, or `deep-team` as topology inside RPIV by inference first, explicit override second.
- Learning tests are part of Research and should update research artifacts.
- Artifact weight should be inferred:
1. `lightweight` for small low-risk work
2. `standard` for medium or multi-step work
3. `graph` for risky or dependency-heavy work
- Execution artifacts should include `plan.md`, `validation-contract.md`, and `validation-state.json` for medium and larger work, with `features.json` only when the graph earns its keep.
- Native todo/checklist visibility should come from `update_plan`, not from bloating artifact files.
- Native structured user questions should come from `request_user_input`, not long free-form clarifying chats.
- Swarm worker packet must include: `goal`, `constraints`, `non_goals`, `acceptance_criteria`, task ID, dependencies, file scope, and validation commands.
- Start swarm concurrency conservatively (2-6), increase only for independent tracks, and reduce on throttling/429.
- Review should likewise degrade reviewer-lane count gracefully when agent/thread capacity is tight instead of surfacing that as user-facing workflow friction.
- Plan inference order: explicit path > user-mentioned path > single `*-plan.md` > most recently modified `*-plan.md`.
