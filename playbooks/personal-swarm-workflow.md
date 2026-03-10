# Personal RPIV Workflow

Run Codex as a phase-first system:

1. Research
2. Plan
3. Execute
4. Verify

`solo`, `team`, and `deep-team` are execution topologies layered on top of RPIV, not replacements for it.

## Core Rules
- One canonical orchestrator thread owns phase transitions and durable state.
- The orchestrator does not execute source changes directly.
- Research and execution should not share the same working context by default.
- Planning produces a canonical plan first and executable work units second.
- Execution packets are accepted only with evidence.
- Verification is a first-class phase, not a cleanup step.
- When Execute finishes a slice cleanly, the default path is to continue straight into Verify unless the user asked to pause or a material decision blocker appears.
- Prefer native thread compaction at noisy phase boundaries; if it is unavailable, continue the next phase from artifacts in a fresh thread/context.
- Infer the earliest necessary phase from the request and current artifacts instead of always starting from Research.
- Infer the lightest topology and artifact weight that still preserves correctness.
- For non-trivial runs, maintain a native Codex `update_plan` checklist as the live progress view.
- Treat `update_plan` as the working checklist and RPIV artifacts as the durable source of truth.
- Use native `request_user_input` only for material decision forks that cannot be resolved safely from context.

## Prompt Contract Doctrine
- Prefer explicit contracts over vague prose when behavior must be reliable.
- Use the smallest useful subset of:
1. `output_contract`
2. `default_follow_through_policy`
3. `tool_persistence_rules`
4. `dependency_checks`
5. `parallel_tool_calling`
6. `completeness_contract`
7. `verification_loop`
8. `missing_context_gating`
9. `research_mode`
10. `user_updates_spec`
- Tighten prompt contracts and grounding rules before raising reasoning effort.
- Treat heavier reasoning as a last-mile choice, not the default fix for weak instructions.

## Inference Model

### Phase Span
- Start at Research when artifacts are missing or uncertainty is material.
- Start at Plan when research is good enough but planning is missing, stale, or explicitly requested.
- Start at Execute when planning is ready and the user wants changes made or resumed.
- Start at Verify when execution evidence exists and the user wants proof, readiness, or merge confidence.
- Step backward when later artifacts contradict earlier ones.

### Artifact Weight
- `lightweight`: compact artifacts for small, low-risk, tightly coupled work
- `standard`: normal artifact set for medium or multi-step work
- `graph`: full resumable graph artifacts for risky, dependency-heavy, or multi-track work

### Topology
- `solo` for tiny, tightly coupled, low-risk work
- `team` for medium work or context-loss risk
- `deep-team` for risky refactors, public APIs, concurrency, or uncertain integrations

### Execute Mode
- `approval_gated` for risky, destructive, high-cost, or still-ambiguous execution where a human go/no-go or explicit phase handoff should happen before continuing
- `autonomous` for low-risk unblocked execution where small implementation ambiguities can be resolved from context without reopening planning
- `parallel_autonomous` for the same autonomy model when multiple independent packets are ready and verification capacity can keep up
- Execute mode is distinct from RPIV phase and distinct from `solo|team|deep-team` topology.
- Material forks still use native `request_user_input`.
- Low-risk execution ambiguity can continue autonomously; material planning ambiguity cannot.

### Phase-Boundary Continuation
- Execute -> Verify is automatic by default once the active slice is accepted and Verify is required.
- Verify -> fix loop is automatic when failed assertions have a bounded remediation path and the user did not ask to pause.
- Verify -> next Execute wave is automatic when the current slice passes and more unblocked packets remain.
- Before Execute -> Verify, Verify -> fix loop, or Verify -> next Execute, prefer native thread compaction; otherwise resume from artifacts in a fresh context.
- Only stop at a phase boundary when:
1. the user asked to pause
2. a material decision fork needs `request_user_input`
3. the plan or validation contract is stale
4. permissions or environment blockers prevent safe continuation
5. two remediation loops on the same failing set have already been exhausted

## Topologies

### `solo`
Use for tiny, tightly coupled, low-ambiguity work.

- One agent runs all RPIV phases.
- Still make the phase boundaries explicit.
- Verification remains mandatory.

### `team`
Use for medium work, multi-step work, or anything likely to lose context.

- One orchestrator owns RPIV.
- Each phase may use a dedicated team.
- Start with 2-6 workers where independence is real.

### `deep-team`
Use for risky refactors, public APIs, concurrency, uncertain integrations, or expensive mistakes.

- Same RPIV phases, stronger review and validation gates.
- Research and planning are heavier.
- Verification includes separate scrutiny and user-surface validation when applicable.

## Phase Teams

### Research Team
Purpose:
- understand the repo
- gather external/runtime evidence
- narrow uncertainty
- emit planner-ready outputs

Typical team:
1. `workflow_orchestrator`
2. `research_locator`
3. `architecture_analyst`
4. `researcher` or `learning_tester`

Outputs:
- `research.md`
- `research.json`

Depth is inferred:
1. `quick`
2. `medium`
3. `deep`

### Scoped Explorer Lane
Purpose:
- answer bounded codebase questions quickly
- gather file paths, symbol locations, and narrow control-flow facts without opening a full research run

Use the built-in `explorer` when:
1. the user asks a quick repo question
2. a phase needs one narrow codebase lookup
3. planner-ready artifacts are not yet justified

Escalate back to the full Research team when:
1. the question becomes cross-cutting or architecture-heavy
2. external/runtime evidence is needed
3. planner-ready artifacts are required

Do not create a home-level duplicate explorer role.

### Plan Team
Purpose:
- convert research into one canonical implementation plan
- critique and refine until changes flatten
- compile plan into executable work units

Typical team:
1. `workflow_orchestrator`
2. `planner`
3. `plan_reviewer`

Outputs:
- `plan.md`
- `features.json`
- `validation-contract.md`

Planning weight is inferred:
1. `lightweight`: `plan.md`
2. `standard`: `plan.md` and `validation-contract.md`
3. `graph`: `plan.md`, `validation-contract.md`, and `features.json`

### Execute Team
Purpose:
- execute the plan without re-architecting it mid-flight
- verify each packet before it is accepted

Typical team:
1. `workflow_orchestrator`
2. `spark_implementer`
3. `spark_implementer_xhigh`
4. `packet_verifier`
5. `integrator`

Outputs:
- `features.json` updates
- `execution-summary.md`
- packet handoffs

Execution topology is inferred from coupling, packet count, and risk.
Execute mode is inferred separately from topology:
1. `approval_gated`
2. `autonomous`
3. `parallel_autonomous`

### Verify Team
Purpose:
- prove implementation matches plan and validation contract
- keep fix loops narrow and resumable

Typical team:
1. `workflow_verifier`
2. `scrutiny_validator`
3. `user_flow_validator`
4. `workflow_reviewer`

Outputs:
- `validation-state.json`
- `verification-report.md`

Verification proof weight is inferred:
1. `gates`
2. `behavior`
3. `full`

## Orchestrator Contract
- Own phase transitions and artifact updates.
- Delegate hands-on work and deep granular analysis.
- Never execute source changes directly.
- Update artifacts when requirements or findings change.
- Emit remediation packets when validation fails.
- Prefer fresh worker context over rescuing stale threads.
- Prefer automatic phase continuation over manual handoff prompts when artifacts and evidence are sufficient.
- Do not stop early while requested phases or accepted next actions remain obvious.
- If the initiative changes mid-run, restate the new target, preserve compatible artifacts, and retire superseded work explicitly.

## Model Policy
- Orchestrator, architecture analysis, final verification, and final review: deep model with `high`.
- Planning and bounded implementation: `gpt-5.4` with `medium` reasoning.
- Use Spark only when packet scope is narrow and explicit.
- Concurrency is constrained by review capacity, not by eagerness to parallelize.

## Research Before Plan
Research is not optional support work.
It should produce:
- file inventory
- control-flow and architecture analysis
- dependencies
- external findings
- verified assumptions
- unverified assumptions
- planner directives

Learning tests are a research subroutine for uncertain runtime, tool, SDK, hook, or continuity behavior.

Research mode rules:
- Separate facts, inferences, and unknowns explicitly.
- Cite exact file paths or retrieved URLs for external facts.
- Keep searching or probing until another pass is unlikely to change the conclusion.
- If the first retrieval or probe comes up thin, broaden once before concluding evidence is insufficient.

## Plan Before Graph
- Keep one canonical plan.
- Critique and refine the plan before graph compilation.
- Convert plan to executable work units only when changes become incremental.
- Verification requirements belong in the plan and the graph.

## Verification Layers
Separate these when risk warrants it:

### Scrutiny
- lint
- typecheck
- build
- tests
- per-feature code review

### User-Surface Validation
- user-visible assertions
- API/flow assertions
- manual or browser evidence when relevant

The run is not done until all required assertions pass.

## User Updates
- Keep progress updates brief and outcome-based.
- Prefer updates only on:
1. phase changes
2. material blocker discovery
3. plan invalidation
4. final verification or review outcomes
- A good update is usually:
1. what changed
2. what happens next

## Integrated Utilities
The remaining active utilities are part of RPIV, not alternate workflows.

### Research
- `workflow-learning-tests` is the uncertainty-reduction subroutine inside Research.

### Execute
- `simplify-codex` is an optional post-execute refinement pass once packet execution is stable.
  It should remain an orchestration skill with a prompt wrapper, not a dedicated role or phase.
- `bug-scanner-autopilot-codex` is a specialized bug-focused Execute+Verify loop, not a replacement for RPIV.

### Verify
- `verify-gates-codex` is the default command-level scrutiny engine for lint, typecheck, build, and tests.
- `verification-specialist` is the behavior-proof engine when command gates are not enough.

### Review
- `review-workflow-codex` is the default dedicated review engine behind the Review phase.
- `review-spacebot-codex` is the Spacebot-specific Review specialization.
- `security-review` is the specialized exploitability-first review lane for trust-boundary changes.

## Continuity
- Keep durable state in repo-local artifacts, not hidden session memory.
- Resume from artifacts first, transcript second.
- Keep one slug-consistent directory for each RPIV run.
- Make fix loops visible and bounded.

## Native Thread Lifecycle
- Prefer native thread continuity over custom "restart the workflow" behavior.
- Continue the same orchestrator thread when:
1. the initiative is the same
2. the artifact state is coherent
3. the current approach still holds
- Resume an existing paused thread before starting a fresh thread for the same slug.
- Fork when:
1. you want a speculative branch
2. the approach diverges materially
3. you need to preserve the original thread as the canonical path
- Archive or close threads when:
1. the initiative is complete
2. the thread is superseded
3. the thread state is no longer trustworthy
- Do not create a new thread just because the phase changed.

## Native Todo Integration
- Use `update_plan` automatically for non-trivial RPIV runs.
- Keep the checklist short:
1. current phase
2. current substep when useful
3. next phase
4. remediation/review step only when needed
- Update `update_plan` on:
1. phase transitions
2. remediation loop start
3. major truth changes
4. completion
- Do not mirror `features.json` or every packet into the checklist.
- For `lightweight` runs, `update_plan` can carry most visible progress.
- For `standard` and `graph` runs, `update_plan` is the HUD and artifacts remain the durable record.

## Native User Input Integration
- Enable `request_user_input` in Default mode so RPIV can use it without forcing Plan mode.
- Use `request_user_input` only for decisions that materially change:
1. scope
2. topology
3. execution path
4. remediation choice
- Prefer it at:
1. planning forks
2. phase-boundary go/no-go decisions
3. verification failure branches
- Keep it main-thread only.
- Keep prompts short and structured.
- Do not use it for routine ambiguity or details that can be inferred safely.

## Native Permission Integration
- Use named permission profiles plus native `request_permissions` when the blocker is filesystem or network access, not product direction.
- Keep permission requests minimal and task-scoped; ask for the smallest profile that unblocks the next step.
- Prefer stable profile names in config over ad hoc prose about "more access".
- Keep workflow doctrine separate:
1. `request_user_input` for human decisions
2. `request_permissions` for structured capability escalation
- Do not build repo-local approval wrappers when the native permission surface already fits.

## Anti-Patterns
- Orchestrator doing execution work
- Research and execution in one bloated context
- Turning an unstable plan into a task graph too early
- Swarming before decomposition is good
- Accepting packet completion from prose alone
- Treating documentation as runtime proof
