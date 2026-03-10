# Swarm Orchestration Playbook

Run multi-agent execution with explicit task state, bounded conflict risk, and model-aware routing.
This playbook now applies primarily to the Execute phase of the broader RPIV workflow.

## When To Use
- Complex plans with independent tracks that can run in parallel.
- Work that benefits from orchestrator + worker separation.
- Cases where single-thread execution is too slow.
- Use only after research and planning artifacts are good enough to support execution.

Do not use swarms for tiny, single-file edits where integration overhead exceeds gains.

## Inputs
- A plan file with task IDs (`T1`, `T2`, ...) and acceptance criteria.
- Dependency graph (`depends_on: []`) for `waves` mode.
- Intent packet:
1. `goal`
2. `constraints`
3. `non_goals`
4. `acceptance_criteria`

If required inputs are missing, return `BLOCKED` with missing fields.

Inference defaults before `BLOCKED`:
- Plan path: explicit path > user-mentioned path > single `*-plan.md` > most recently modified `*-plan.md`.
- Team lanes: infer from scope/repo when omitted (`frontend`, `backend`, `qa`, `docs`).
- Intent packet: infer from user request + plan/spec artifacts + AGENTS constraints.
- Return `BLOCKED` only when multiple plausible plans remain or intent is materially ambiguous.

## Modes
- `waves` (default): one worker per unblocked task; merge and verify between waves.
- `super` (opt-in): launch broad parallel workers for speed; expect higher integration/conflict cost.
- `team-lane` (opt-in): run explicit domain lanes with one owner worker per lane, synchronized by waves.

Prefer `waves` unless user explicitly requests `super`.

Natural-language triggers:
- "run this in waves", "dependency-aware swarm", "parallel but safe" -> `waves`
- "full parallel", "super swarm", "maximize speed" -> `super`
- "split by teams/lanes/frontend-backend-qa-docs" -> `team-lane`

## Role Split
- Planner: produce/update dependency-aware plan.
- Orchestrator: assign tasks, track state, integrate outputs, run checks, resolve conflicts.
- Workers: execute bounded tasks only.

Model routing:
1. Planner: `gpt-5.4` (`medium`)
2. Orchestrator: deep model (`high`)
3. Workers: `gpt-5.4` (`medium` for bounded tasks)
4. Learning tester / verifier / reviewer: deep model (`high`)

## Learning-Test Gate
- If planning depends on uncertain external, tool, hook, continuity, or runtime behavior, stop and run learning tests first.
- Do not dispatch execution workers until the learning contract has evidence or has been explicitly narrowed.
- In `deep-team`, learning tests are mandatory whenever assumptions materially affect design or verification.

## Community-Informed Patterns
Use these as optional tactics, not defaults:

- Team lanes:
1. Split lanes by domain (for example: frontend, backend, QA, docs).
2. Keep lane file scopes disjoint.
3. Assign one owner agent per lane.

- Forked alternatives:
1. Run 2-3 implementation variants for uncertain design choices.
2. Compare validation evidence and keep one path.
3. Avoid keeping all variants alive beyond decision point.

- Recursive delegation:
1. Allow nested subagents only for clearly decomposable tasks.
2. Keep `max_depth` conservative to avoid token blowups.
3. Escalate to orchestrator when nested workers disagree.

- Fan-out execution:
1. Use CSV fan-out or batched task packets for many independent tasks.
2. Prefer this for broad audits, checks, and repetitive transformations.
3. Keep integration centralized in orchestrator.

## Worker Packet Contract
Every worker packet must include:
1. Plan path
2. `task_id`
3. `task_name`
4. `problem_statement`
5. Dependencies and related tasks
6. Exact file scope / owned files
7. Relevant context and handoff inputs
8. Acceptance criteria
9. Invariants
10. Out-of-scope list
11. Verification plan / validation commands
12. Intent packet (`goal`, `constraints`, `non_goals`, `acceptance_criteria`)
13. Explicit "do not modify files outside scope" constraint

## Worker Handoff Contract
Every worker handoff must include:
1. `task_id`
2. `status`
3. `changes_made`
4. `verification_type`
5. `verification_command`
6. `verification_result`
7. `evidence_excerpt`
8. `reason_not_testable`
9. `open_issues`
10. `recommended_next_step`

## Orchestrator Loop
1. Read `plan.md`, `features.json`, and current validation constraints.
2. Select launch set:
- `waves`: all unblocked tasks, ordered by dependency satisfaction and then lowest task ID first
- `super`: bounded independent subsets
3. Dispatch workers with complete packets.
4. Wait for completion and collect evidence.
5. Integrate in dependency order.
6. Run targeted validation.
7. Update task states and repeat until done or blocked.

## Concurrency Controls
- Start with 2-6 workers.
- Respect `[agents].max_threads`.
- Increase only when file scopes are disjoint.
- On throttling/429, reduce concurrency and retry with backoff.
- If overlap, confusion, or repeated integration churn appears, reduce fanout before retrying.

## Backpressure
- Packet completion requires evidence, not confidence.
- `team` runs should finish with verifier evidence.
- `deep-team` runs require verifier plus reviewer gates before final completion.
- Failed verification should emit focused remediation packets, not a full reset.
- Limit critique/fix loops to 2 rounds before escalation.
- If plan quality is still moving materially, stop and return to the Plan phase instead of continuing to swarm.

## Continuity
- Resume the current orchestrator thread when state is coherent.
- Fork only for speculative alternatives.
- Replace stale workers with fresh packets rather than trying to rescue confused threads.
- Keep worker summaries terse, factual, and scoped to their assigned files and task IDs.

## Completion Criteria
- A task is complete only when acceptance criteria are satisfied with validation evidence.
- Do not mark completion from claims alone.
- Do not commit unless user requested commits.

## Output Contract
```md
Mode: waves|super
Overall: PASS|FAIL|BLOCKED

Plan state:
- Completed: <task IDs>
- In progress: <task IDs>
- Blocked: <task IDs + reason>

Dispatch summary:
- Workers launched: <count>
- Concurrency: <N>
- Conflicts detected: <none or list>

Validation evidence:
- <task ID>: <command> -> <result>

Next wave / next batch:
- <task IDs>

Residual risk:
- <remaining risk>
```
