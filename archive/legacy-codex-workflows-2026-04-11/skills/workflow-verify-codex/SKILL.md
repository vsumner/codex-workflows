---
name: workflow-verify-codex
description: This skill should be used when the user asks to "verify this", "run the validation team", "prove the output works", "check assertions", or wants the Verify phase of the personal RPIV workflow.
---

# Workflow Verify (Codex)

Run the Verify phase for a personal RPIV run.

## Goal
- Judge the work against the validation contract and current assertion state.
- Produce evidence that the output actually works.
- Return the smallest next step when verification fails.
- Act as the counterweight to implementation bias. Assume bugs exist until evidence says otherwise.

## Inference Rules
### Proof Weight
1. `gates` for local, low-risk changes where command-level proof is enough.
2. `behavior` when user-visible flows, APIs, or runtime behavior need proof beyond gates.
3. `full` for risky work, merge/PR readiness, or when Review should follow immediately.

### Topology
1. `solo` for most `gates`.
2. `team` for `behavior`.
3. `deep-team` for `full`.
4. If `team` or `deep-team` is inferred without explicit delegation activation, keep Verify local and record the recommendation.

## Inputs
- `research.md`
- `plan.md`
- `validation-contract.md`
- `validation-state.json` when present
- `execution-summary.md`
- `features.json` when the run uses graph artifacts

## Process
1. Read the artifacts first.
2. For non-trivial verification, start or update a native `update_plan` checklist and keep the in-progress step on Verify.
3. If `validation-state.json` is missing for a `standard` or `graph` run, bootstrap it from `validation-contract.md` before running checks.
4. If `features.json` shows pending future work, verify the active executed slice first and distinguish that slice verdict from the whole-initiative verdict.
5. If verification exposes multiple valid remediation branches and inference is unsafe, use `request_user_input` from the main thread.
6. Run `/verify-gates` for command-level scrutiny by default.
7. Run separate scrutiny and user-flow validation where applicable.
8. Use `verification-specialist` when command gates alone do not prove behavior.
9. Require at least one adversarial probe beyond the happy path for non-trivial work.
10. Mark assertions `passed`, `failed`, or `blocked`.
11. Update `validation-state.json` early, then keep it current as evidence lands.
12. Write `verification-report.md`.
13. If delegated validators time out or return partial evidence, keep going locally and write the current blocked or failed state instead of ending without artifacts.
14. If verification fails, propose the smallest remediation features or packets.
15. Escalate to `workflow-review-codex` automatically when proof weight is `full` or the risk warrants dedicated review.
16. Mark the checklist complete in `update_plan` when verification closes successfully.

## Guardrails
- Do not confuse review comments with verification evidence.
- Do not drift into broad redesign.
- Keep the output decision-complete: pass, fail, or blocked with the reason.
- Every required assertion must end as `passed`, `failed`, or `blocked`.
- For graph runs, blocked assertions caused by pending future features should stay `blocked`, not be mislabeled as fresh regressions.
- Do not leave a `standard` or `graph` Verify run without `validation-state.json` and `verification-report.md`.
- If verification proves the plan or graph is stale, step back to the earliest invalid phase instead of forcing the fix through Verify.

## References
- `../../playbooks/personal-swarm-workflow.md`
- `../../playbooks/workflow-artifacts.md`
- `../../playbooks/verify-gates.md`
