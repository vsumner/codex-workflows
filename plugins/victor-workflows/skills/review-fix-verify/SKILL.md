---
name: review-fix-verify
description: This skill should be used when Victor asks to "address review feedback", "fix PR comments", "verify each finding", "make this review-ready", "review-fix-verify", or handle reviewer findings without unnecessary churn.
---

# Review Fix Verify

Use this skill to handle review feedback with low churn and concrete proof.

The purpose is not to obey every comment. The purpose is to verify each finding against the current code, fix only what still matters, and leave behind enough evidence for Victor and reviewers to trust the result.

## Core Rule

Treat review findings as claims, not commands.

For every finding:

1. Locate the current code and confirm the finding still applies.
2. Classify the finding.
3. Apply the smallest fix that resolves a true issue.
4. Run targeted verification.
5. Report what changed, what was already safe, and what remains.

Do not broaden the patch just because a comment suggests a larger cleanup. Capture broader work as follow-up unless it is required to fix the current issue safely.

## Evidence Collection

Start with deterministic state:

```bash
git status --short --branch
git diff --stat
git diff
```

If the feedback comes from a PR, inspect only the needed PR context:

```bash
gh pr view --comments
gh pr diff
```

Use the GitHub app/plugin instead of `gh` when comments, review threads, or check state are easier to inspect there. Do not post review comments, push, merge, or resolve threads unless Victor explicitly asks.

If the feedback refers to previous session intent, use bounded history:

```bash
codex-threads --json messages search "review feedback" --since 14d --limit 20
codex-threads --json threads read <session-id> --limit 80
```

## Finding Classification

Classify each item before editing:

- **true issue**: reproduces or is clearly present in current code.
- **already fixed**: current code no longer has the issue.
- **not applicable**: reviewer assumption does not match the current code path.
- **scope expansion**: useful but not needed for this change.
- **needs clarification**: cannot be decided from available code, tests, or comments.

Fix only **true issue** items unless Victor explicitly asks to take broader cleanup.

For **already fixed** and **not applicable**, record the evidence instead of changing code. For **scope expansion**, recommend a follow-up. For **needs clarification**, gather one more local source if possible before asking Victor.

## Implementation

Keep the diff narrow:

- edit only files needed for verified findings;
- prefer existing helpers and local patterns;
- do not reformat unrelated code;
- do not rename or refactor as a side effect;
- do not touch generated or lock files unless the fix requires it.

When multiple comments point to the same root cause, fix the root once and mark the duplicates as covered by that fix.

## Verification

Match proof to the changed path:

- unit test for isolated logic;
- integration or CLI command for command behavior;
- typecheck/lint for type or style changes;
- UI/runtime proof for user-facing changes;
- direct inspection only for docs or metadata changes where execution adds no signal.

Prefer targeted checks first. Run broader checks only when the changed surface is shared or risky.

If a check fails:

1. Decide whether the failure is caused by the current change.
2. Fix caused failures.
3. Preserve unrelated pre-existing failures in the final report with command and failure summary.

Never claim the review is closed without evidence.

## Output Shape

Lead with the state of the findings:

```text
Findings handled:
- <finding>: fixed | already fixed | not applicable | deferred

Changed:
- <file>: <behavior-level change>

Verified:
- <command or inspection>: <result>

Residual:
- <pre-existing failure, deferred scope, or remaining risk>
```

For tiny fixes, collapse this into a short paragraph plus one verification line.

## Anti-Rationalization

Watch for these failure modes:

- treating reviewer text as authoritative without checking current code;
- fixing stale comments and adding churn;
- expanding scope because a reviewer mentioned a nice-to-have;
- claiming a finding is resolved because code was edited, not because behavior was verified;
- rerunning broad test suites repeatedly instead of targeting the changed path;
- hiding pre-existing failures in a vague "tests failed" note.

If pressure pushes toward "just make the reviewer happy," return to the core rule: verify first, then fix the smallest real issue.
