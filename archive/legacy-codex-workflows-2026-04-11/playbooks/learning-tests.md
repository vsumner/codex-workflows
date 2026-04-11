# Learning Tests

Use learning tests to prove or falsify assumptions before they poison planning or implementation.
Learning tests belong inside the Research phase and should update the current `research.md` / `research.json` outputs.

## When To Use
- External API or SDK behavior is unclear.
- Tool, hook, or command semantics are assumed rather than proven.
- Resume, fork, or continuity behavior matters.
- Shell, runtime, or environment edge behavior affects the design.
- Documentation exists but runtime behavior still matters.

## Rules
- Write the smallest possible proof.
- Prefer a runnable test, reproduction, or command over speculation.
- Capture observed behavior, not predicted behavior.
- Update the plan only after evidence is collected.
- If learning results are mixed, narrow the question and rerun.

## Output Contract
```md
Learning Verdict: PASS|FAIL|INCONCLUSIVE

Assumption:
- <what was believed>

Probe:
- <command or test>

Observed behavior:
- <what actually happened>

Impact on plan:
- <what changes now>

Next step:
- <proceed | rerun narrower probe | escalate>
```

## Common Targets
- SDK flags and default behavior
- Hook side effects and ordering
- Shell quoting and environment propagation
- Session continuity and resume semantics
- File watcher, sandbox, or build tool behavior
