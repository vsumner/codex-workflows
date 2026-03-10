# Model Routing Playbook

Use a two-tier model strategy for subagents.

## Tier 1: High-Criticality
Use deeper models when correctness and integration risk are high.

Roles:
- `workflow_orchestrator`: `gpt-5.4`
- `architecture_analyst`: `gpt-5.4`
- `learning_tester`: `gpt-5.4`
- `plan_reviewer`: `gpt-5.4`
- `workflow_verifier`: `gpt-5.4`
- `workflow_reviewer`: `gpt-5.4`
- `scrutiny_validator`: `gpt-5.4`
- `user_flow_validator`: `gpt-5.4`
- `planner`: `gpt-5.4`
- `implementer`: `gpt-5.4`
- `integrator`: `gpt-5.4`
- `reviewer`: `gpt-5.4`
- `quality_reviewer` (fast lane): `gpt-5.4`
- `spec_reviewer`: `gpt-5.4`
- `efficiency_reviewer`: `gpt-5.4`
- `rust_correctness_reviewer`: `gpt-5.4`

Reasoning guidance:
- Orchestration, architecture, verification, and final review synthesis: `high`
- Converted fast and review lanes now run on `gpt-5.4` with `medium` reasoning
- Efficiency triage: `medium` unless incident-level performance debugging

## Tier 2: Fast-Triage
Use fast-lane models for lower-risk, high-iteration work.

Roles:
- `worker` (default fast execution lane): `gpt-5.4`
- `spark_implementer`: `gpt-5.4`
- `spark_implementer_xhigh`: `gpt-5.4`
- `research_locator`: `gpt-5.4`
- `packet_verifier`: `gpt-5.4`
- `researcher`: `gpt-5.4`
- `reuse_reviewer`: `gpt-5.4`

Reasoning guidance:
- Tier-2 lanes use `medium` reasoning

## Global Defaults
- Default session model: `gpt-5.4`
- Default reasoning: `high`
- Plan mode reasoning: `xhigh`
- Review model pin (`/review`): `gpt-5.4`

## Swarm Routing
- Research phase:
1. orchestration and synthesis on `gpt-5.4`
2. file inventory on Spark
3. learning tests on `gpt-5.4`
- Plan phase:
1. canonical plan authoring on `gpt-5.4` with `medium` reasoning
2. plan review on `gpt-5.4`
3. graph compilation only after plan stabilizes
- Execute phase:
1. orchestration on `gpt-5.4`
2. worker execution on `gpt-5.4` with `medium` reasoning
3. packet verification on `gpt-5.4` with `medium` reasoning
- Verify phase:
1. verification orchestration on `gpt-5.4`
2. scrutiny and user-flow validation on `gpt-5.4`
- Keep reviewer lane split:
1. `quality_reviewer` on `gpt-5.4` with `medium` reasoning for fast triage.
2. Keep `reviewer` / `spec_reviewer` on `gpt-5.4` with `medium` reasoning.
- Use fast-lane workers only for clearly bounded tasks with explicit acceptance criteria and file scope.
- Default medium and larger work to RPIV with phase teams instead of one long execution thread.

## Escalation Rules
- Escalate former fast-tier tasks from `gpt-5.4` with `medium` reasoning to deeper `gpt-5.4` lanes if:
1. Findings are contradictory or low confidence
2. Changes involve concurrency/state machines/safety-critical behavior
3. Cross-cutting refactors span many modules with hidden coupling
4. Swarm integration reports repeated file conflicts across waves/batches
