---
name: workflow-learning-tests-codex
description: This skill should be used when the user asks to "prove this assumption", "learn before building", "test the tool behavior", "check the runtime behavior", "verify the hook semantics", or when planning depends on uncertain external or runtime behavior.
---

# Workflow Learning Tests (Codex)

Run the learning-test subroutine during Research when assumptions need proof.

## Goal
- Replace assumptions with observed behavior.
- Keep probes tiny and fast.
- Feed results back into the plan before implementation.

## When To Trigger
- SDK or API behavior is unclear.
- Tool or hook semantics are inferred rather than tested.
- Resume, fork, or continuity behavior matters.
- Shell, environment, or runtime edge behavior affects the design.

## Process
1. State the assumption explicitly.
2. Write the smallest probe that answers it.
3. Run the probe and capture observed behavior.
4. Mark the result `PASS`, `FAIL`, or `INCONCLUSIVE`.
5. Update the plan or packet design from the evidence.

## Rules
- Prefer tests, reproductions, and commands over prose.
- Do not broaden the task into implementation.
- If the first probe is inconclusive, narrow the question and rerun once.
- If behavior varies by environment, state that precisely.
- Report only observed behavior, not predicted behavior.
- Fold the results back into `research.md` and `research.json`.

## Output
Use the learning-test output contract from `../../playbooks/learning-tests.md`.

## References
- `../../playbooks/learning-tests.md`
- `../../playbooks/workflow-artifacts.md`
