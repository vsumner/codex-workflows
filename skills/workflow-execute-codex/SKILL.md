---
name: workflow-execute-codex
description: This skill should be used when the user asks to "execute this plan", "run the execute phase", "execute from the graph", "run the executor team", or wants the Execute phase of the personal workflow.
---

# Workflow Execute (Codex)

Run the Execute phase from existing artifacts.

## Inference Rules
### Topology
1. `solo` for one tight packet or low-risk tightly coupled work.
2. `team` for multiple mostly independent packets or when delegation preserves context.
3. `deep-team` for risky changes, heavy verification pressure, or dependency-heavy execution.

### Execute Mode
1. `approval_gated` for risky, destructive, expensive, or still-ambiguous execution that should wait for an explicit go/no-go or phase handoff.
2. `autonomous` for low-risk unblocked execution where minor ambiguity can be resolved from context.
3. `parallel_autonomous` for the same autonomy model when multiple independent packets are ready and verification capacity can keep up.

### Team Shape
Delegated execution team when activation is explicit:
1. `workflow_orchestrator`
2. `spark_implementer`
3. `spark_implementer_xhigh`
4. `packet_verifier`
5. `integrator`

Without explicit delegation activation, keep Execute local and record the recommended topology.

Use the smallest team that fits when delegation is active:
- `spark_implementer` is the default bounded executor.
- `spark_implementer_xhigh` is reserved for hard or failure-prone packets.
- `integrator` is only needed when outputs must merge cleanly.
- Stay solo when coordination cost is obviously higher than the benefit.

## Required Inputs
Read:
1. `research.md`
2. `plan.md`
3. `features.json` when the run uses graph artifacts
4. `validation-contract.md`

## Rules
- The orchestrator does not execute source changes directly.
- For non-trivial execution, start or update a native `update_plan` checklist and keep the in-progress step on Execute.
- If `features.json` is absent, decide whether the plan is intentionally lightweight or whether planning needs to emit a graph first.
- Infer Execute-mode state separately from topology.
- Executors work from bounded features or packets only.
- Use the built-in `explorer` for one-shot codebase lookups that unblock a packet; do not reopen broad Research unless uncertainty expands materially.
- Verify every packet before acceptance.
- Update `features.json` and `execution-summary.md` continuously.
- In graph runs, `features[].status = done` means execution-complete and ready for Verify for that packet, not globally verified.
- Do not stop after the first accepted packet if more unblocked packets remain.
- Low-risk execution ambiguity may continue in `autonomous` modes, but material forks still require `request_user_input`.
- If reality invalidates the plan materially, stop and return to planning.
- Mark Execute complete in `update_plan` before handing off to Verify.

## References
- `../../playbooks/personal-swarm-workflow.md`
- `../../playbooks/workflow-artifacts.md`
- `../../playbooks/swarm-orchestration.md`
