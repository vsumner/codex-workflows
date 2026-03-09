---
name: fix-pr-feedback-codex
description: This skill should be used when the user asks to "fix this PR", "address review findings on this PR", "take this PR to green locally", "checkout this PR and fix the issues", or wants a largely autonomous PR remediation loop with local verification.
metadata:
  invocation: explicit-only
---

# PR Feedback Fix Loop (Codex)

Run a focused Execute-owned remediation loop for a GitHub PR.

## Routing
Use this skill for plain-language PR remediation requests, especially after review findings already exist in the current thread.

Route here for requests like:
1. "fix this PR"
2. "address review findings on this PR"
3. "take this PR to green"
4. "check out this PR and fix it"

Do not collapse these requests into generic review `apply` mode or a plain `review-feedback-closure` loop when the target is a GitHub PR.

## Goal
- Close clear PR findings with minimal churn.
- Prefer autonomous local remediation and local proof.
- Keep Victor out of the loop unless fix direction is unclear or the repo is in a catastrophic state.
- Do not publish anything unless Victor explicitly asks.

## Design
- This is an Execute subroutine inside RPIV, not a new top-level workflow.
- It should reuse existing Git, worktree, review-feedback-closure, and verification patterns.
- It should treat PR-native context as primary:
1. PR metadata
2. PR diff
3. review comments / copied findings
4. branch history / mergeability state

## Inputs
- PR target:
1. explicit PR URL or number
2. current branch PR when that mapping is clear
- Findings source priority:
1. copied review findings or current-thread findings
2. GitHub PR review comments / reviews
3. PR mergeability or stale-metadata issues discovered during preflight
- Mode:
1. `apply` by default
2. `review-only` only when Victor explicitly asks not to edit
- Workspace mode:
1. `auto` (default)
2. `reuse`
3. `worktree`
- Rebase mode:
1. `auto` (default)
2. `off`
3. `on`

## Workspace Selection
Resolve the PR head branch/commit first, then choose the lightest safe workspace.

Reuse the current repo root only when all are true:
1. the current repository matches the PR repository
2. `HEAD` already matches the PR head branch or commit closely enough to remediate in place
3. the working tree is clean or otherwise safe to modify without colliding with unrelated local work
4. there is no better isolation reason such as risky dependency churn, branch divergence, or broad rebase work

Create an isolated worktree when any of these are true:
1. the current root does not already match the PR target
2. the current root contains unrelated local changes
3. the remediation likely needs rebase/merge conflict work
4. the repo/toolchain changes are broad enough that isolation reduces risk

If `workspace=reuse` is forced and the root is not safe, return `BLOCKED`.

## Autonomy Model
Default to `autonomous` for:
1. fetching PR metadata and comments
2. canonicalizing findings
3. creating or reusing a workspace
4. checking out the PR branch
5. applying clear local fixes
6. running lint, typecheck, test, build, and targeted verification
7. preparing a suggested PR body rewrite without publishing it

Switch to `approval_gated` when any of these are true:
1. branch ownership is unclear or the branch does not appear to be Victor's to rewrite
2. rebase/merge work becomes non-trivial or conflict-heavy
3. there are multiple legitimate fixes with materially different behavior
4. the repo is in a catastrophic state and remediation requires a bigger branch strategy decision
5. Victor explicitly asks for a publish action or a publish action becomes the only remaining next step:
   - commit
   - push
   - force-push
   - update PR body
   - post review comments

## Preflight
1. Resolve PR number, repo, base branch, head branch, and head SHA.
2. Fetch PR metadata, changed files, mergeability state, and review comments.
3. Build a canonical finding ledger:
   - `fix-needed`
   - `already-fixed`
   - `not-applicable`
4. Treat these as first-class remediation targets when present:
   - concrete code findings
   - mergeability / rebase / lockfile drift issues
   - stale PR title/body or misleading verification notes

If no actionable findings or PR issues exist, return `PASS` with no-op evidence.

## Rebase Rules
- `rebase=auto`:
1. allowed only for Victor-owned branches or branches clearly safe to rewrite locally
2. allowed only in the chosen remediation workspace
3. stop and escalate on non-trivial conflicts
- `rebase=off`:
1. do not attempt branch history changes
- `rebase=on`:
1. attempt the rebase locally
2. still stop and escalate on non-trivial conflicts

Do not push rebased history unless Victor explicitly asks.

## PR Head Sync Rules
- Resolve the PR head branch and head SHA before creating or resetting the remediation workspace.
- Create or reset the workspace from an exact PR head ref or SHA, not from ambiguous `FETCH_HEAD` state after a multi-ref fetch.
- If the local workspace does not match the resolved PR head after sync, stop and correct that before editing.

## Execution Loop
1. Choose workspace using the reuse-first policy above.
2. Sync the PR branch locally from the exact resolved PR head.
3. Verify each canonical finding against current code before editing.
4. Fix only `fix-needed` items.
5. Run a narrow-to-broad verification ladder:
   - targeted file/module tests first
   - lint/typecheck/build next
   - broader repo gates only when they are directly justified
6. If a command fails twice, stop rerunning blindly and isolate the root cause.
7. Keep PR metadata/body rewrites as prepared output unless Victor explicitly asks to publish them.
8. Stop after 2 remediation loops if the path is still ambiguous.

## Guardrails
- No commits, pushes, PR body edits, or review comments unless Victor explicitly asks.
- If local remediation succeeds and the next step is publish-only, stop with a prepared publish plan instead of crossing that boundary.
- Prefer the smallest safe fix set.
- Do not reopen a broad plan unless the failures prove the review findings were incomplete or wrong.
- Treat missing local prerequisites as a solvable environment issue first, not a reason to guess.
- If the current root is safe and already aligned to the PR, do not create a worktree just for ritual isolation.

## Output Schema
```md
Verdict: PASS|FAIL|BLOCKED

Workspace:
- Mode: reused-root|worktree
- Path: <absolute path>

Findings ledger:
- [F1] <fix-needed|already-fixed|not-applicable>
  Source: <review finding|PR comment|PR metadata|mergeability>
  Change: <smallest fix or "none">
  Verification: <exact command>
  Result: <pass|fail|skip>

Branch state:
- PR: <url or number>
- Mergeability: <state>
- Rebase attempted: yes|no
- Publish actions: none|pending explicit ask

Commands run:
- <exact command list>

Prepared follow-up:
- PR body rewrite: <present|none>
- Commit/push plan: <present|none>

Residual risk:
- <remaining risk>

Next step:
- <one recommended next step>
```
