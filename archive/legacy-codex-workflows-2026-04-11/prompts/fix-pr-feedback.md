---
description: Autonomously remediate clear PR findings with local verification, reusing the current repo root when safe and isolating in a worktree when needed.
argument-hint: "[PR_URL_OR_NUMBER] [workspace=auto|reuse|worktree] [rebase=auto|off|on] [mode=apply|review-only]"
---
Run `fix-pr-feedback-codex` for: $ARGUMENTS

Execution contract:
- Treat this as an Execute-owned PR remediation loop, not a fresh broad review.
- Use this path for plain-language requests like "fix this PR" or "take this PR to green", especially after review findings already exist.
- Resolve PR target, findings input, mergeability, and branch ownership first.
- Reuse the current repo root only when it already matches the PR head safely; otherwise create an isolated worktree.
- Resolve the exact PR head ref/SHA before creating or resetting any workspace; do not rely on ambiguous `FETCH_HEAD` state after multi-ref fetches.
- Default to `autonomous` for clear local fixes and local verification.
- Switch to `approval_gated` for non-trivial rebase conflicts, unclear branch ownership, multiple legitimate fix paths, catastrophic repo issues, or when Victor explicitly asks for a publish action.
- Canonicalize findings before editing and fix only `fix-needed` items.
- Treat branch-state issues and stale PR metadata as first-class remediation targets.
- Run a narrow-to-broad local verification ladder after fixes.
- Do not commit, push, update the PR body, or post comments unless Victor explicitly asks. If the next step is publish-only, stop and return a prepared follow-up instead.
- Return: workspace used, findings ledger, branch state, commands run, prepared follow-up, residual risk, next step.
