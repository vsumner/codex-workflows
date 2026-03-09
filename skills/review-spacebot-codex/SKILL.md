---
name: review-spacebot-codex
description: This skill should be used when the user asks to "/review-spacebot", "review this like Spacebot maintainers", "run spacebot review workflow", or "review Spacebot Rust changes with jamiepine style".
metadata:
  invocation: explicit-only
---

# Review Workflow: Spacebot (Compatibility Alias)

Use this for explicit Spacebot maintainer-style requests.

Location rule:
- This is a home-level compatibility alias under `~/.codex/skills/review-spacebot-codex/`.
- Do not expect or probe a repo-local `.agents/skills/review-spacebot-codex/SKILL.md`.
- If this alias cannot be loaded for some reason, fall back silently to `review-workflow-codex` plus the Spacebot-specific rules below.

Preferred path:
- Use `review-workflow` (or `review-workflow-codex`) and let repository detection auto-apply Spacebot rules.
- Within RPIV, treat this as the repo-specific Review specialization.

## Scope Gate
- Resolve repository owner/name first.
- If repo is not `spacedriveapp/spacebot`, fall back to `review-workflow-codex`.

## Mandatory Skills
- `jamiepine-style`
- `rust`

## Inputs
- Diff source priority:
1. `git diff --staged`
2. `git diff HEAD`
- Mode:
1. `review-only` (default)
2. `apply` (if user asks to fix/apply)
- Changed files
- Required intent packet:
1. `goal`
2. `constraints`
3. `non_goals`
4. `acceptance_criteria`
- Optional requirements/task packet

If required intent fields are missing, return `BLOCKED` with missing fields.

## Mode Escalation Rule

When a findings report already exists in-thread, treat direct follow-ups like:

- "do it"
- "fix everything"
- "fix it all"
- "apply the fixes"

as `apply` mode unless the user explicitly says read-only.

## Parallel Reviewers (Read-Only)
- `quality_reviewer`
- `efficiency_reviewer`
- `reuse_reviewer`
- `rust_correctness_reviewer` (for Rust/async/state/concurrency risk)

If agent/thread capacity is limited, keep `quality_reviewer` plus `rust_correctness_reviewer` first, then add the others only if slots remain.

## Spacebot-Specific Review Focus
1. Maintainer-style alignment (Jamiepine conventions)
2. Rust correctness and async/state safety
3. Deterministic behavior under retries/cancellation
4. Regression and test quality
5. Documentation/operator impact when behavior changes
6. Project-instruction compliance only when it adds distinct evidence beyond maintainer-style alignment

## Process
1. Gather diff and changed files.
2. Launch all reviewers in parallel on identical artifacts.
3. Aggregate and deduplicate findings.
4. Reject uncited/speculative findings.
5. If user provided external feedback (CodeRabbit/GitHub comment dumps), canonicalize first using fingerprint `file:line + normalized text`, collapse duplicates, and status each canonical finding as `fix-needed`, `already-fixed`, or `not-applicable`.
6. Add the instruction-compliance lane only when repo instructions or changed comments/docs add distinct evidence.
7. For each finding, include: problem, why it matters, evidence command, and smallest fix.
8. Include intent alignment status (`ALIGNED|MISMATCH`) against goal/constraints/non-goals/acceptance criteria, plus source attribution.
9. Enforce convergence cap: maximum 2 critique/fix loops per finding set, then escalate to human decision.
10. If mode is `apply`, apply minimal edits, then run `verification-specialist` and `/verify-gates`.
11. For non-trivial apply runs, add a fresh-eyes pass before final handoff.

## Default Behavior
- Review-only by default. Do not edit code unless explicitly asked.

## Required Output Schema
```md
Verdict: PASS|FAIL

Findings:
- [critical|major|minor] <file:line> - <issue>
  Why it matters: <impact>
  Evidence command: <exact command>
  Suggested fix: <smallest concrete fix>

Intent alignment:
- Status: ALIGNED|MISMATCH
- Mismatches: <none or list against goal|constraints|non_goals|acceptance_criteria>
- Sources:
  - goal: <source list>
  - constraints: <source list>
  - non_goals: <source list>
  - acceptance_criteria: <source list>

Required fixes:
- <smallest concrete fix list>

Spacebot style alignment:
- <aligned|not aligned> with short rationale

Verification:
- <targeted command(s)>

Residual risk:
- <remaining risk>
```
