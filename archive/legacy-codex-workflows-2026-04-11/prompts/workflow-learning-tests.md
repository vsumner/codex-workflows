---
description: Run learning tests to prove or falsify assumptions before planning or implementation.
argument-hint: "[assumption_or_unknown]"
---
Run learning tests for: $ARGUMENTS

Execution contract:
- State the assumption being tested.
- Design the smallest probe that answers it.
- Prefer runnable tests, reproductions, or commands over speculation.
- Capture observed behavior and update the plan from evidence.
- Fold the result into the current `research.md` and `research.json`.
- If the probe is inconclusive, narrow the question and rerun once.
- Grounding rules:
1. report only observed behavior, not predicted behavior
2. cite the exact probe command or test
3. if environment-dependent, state the environment dependence explicitly
- Return the learning-test output contract from `playbooks/learning-tests.md`.
