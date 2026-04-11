---
name: repo-orientation
description: Use when Victor asks what is next, what branch/state the repo is in, whether there are uncommitted changes, whether to keep improving, commit, push, open a PR, or whether a branch/PR/review loop is ready to move forward.
---

# Repo Orientation

Use this skill to answer orientation questions with existing primitives. This is a fat skill over a thin harness: let deterministic tools collect state, then use judgment to recommend one next action.

## Principle

- Do not build or suggest a new CLI just to aggregate `git`, `gh`, or Codex plugin/app primitives.
- Use `codex-threads` only when cross-session history matters.
- Keep publish actions gated: do not commit, push, open a PR, or post comments unless Victor explicitly asks.
- Lead with one recommendation. Add evidence and residual risk only as needed.

## Deterministic Checks

Start with local state:

```bash
git status --short --branch
git log --oneline -5
```

If the repo has changes and the question is about readiness, inspect the diff narrowly:

```bash
git diff --stat
git diff --cached --stat
```

If the question is about publishing, review, CI, or merge readiness, use the GitHub app/plugin or `gh` to inspect PR and check state. Do this only when it matters to the question.

If the question depends on prior session intent, use the bounded history interface:

```bash
codex-threads --json messages search "query" --since 14d --limit 20
codex-threads --json threads read <session-id> --limit 80
```

Prefer a direct local handoff file when it exists and is relevant.

## Judgment

Classify the situation before answering:

- Dirty tracked changes: recommend review, test, stage, or commit based on risk and verification state.
- Untracked-only changes: say whether they look intentional, ephemeral, or should stay local.
- Ahead of upstream: recommend push if the commits are intentional and verified.
- Behind upstream: recommend pull or rebase only after checking local changes.
- Open PR work: recommend review-feedback, CI diagnosis, push, or merge-readiness checks based on current state.
- Ambiguous "what's next": combine local git state, recent commits, handoff context, and the user's last stated goal.

When there are multiple plausible next actions, pick the highest-leverage one and explain why.

## Response Shape

Use this shape for most answers:

```text
Next: <one recommendation>.

Evidence: <branch/state/commit/PR facts>.
Risk: <only if there is meaningful residual risk>.
```

For tiny cases, collapse this to one or two sentences.

## Learning Loop

If this skill gives a poor recommendation or the same orientation friction recurs, do not create a CLI by default. First use `codex-threads` to find examples, update this skill or the relevant workflow skill, and define a validation contract for the improved behavior.
