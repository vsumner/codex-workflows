---
name: workflow-review-codex
description: This skill should be used when the user asks to "run the review team", "do a post-verify review", "review after implementation", "review after verification", or wants a dedicated Review phase in the personal workflow.
---

# Workflow Review (Codex)

Run a dedicated review team after or alongside Verify on high-stakes work.

## Goal
- Catch correctness, regression, and risk issues that remain after verification.
- Keep findings concrete and evidence-backed.
- Use `review-workflow-codex` as the default review engine for this phase.
- If the repo is `spacedriveapp/spacebot`, escalate to `review-spacebot-codex`.
- If the change affects trust boundaries or attack surface, add `security-review` as a specialized review lane and keep only high-confidence exploitability findings.

## Inputs
- `plan.md`
- `execution-summary.md`
- `verification-report.md`
- relevant diff or changed files

## Rules
- Focus on real bugs, unsafe assumptions, silent failures, and maintainability risks.
- Use specialized reviewers only when the task warrants it.
- Keep review separate from verification evidence.
- Separate observed evidence from inference explicitly.
- Cite exact file paths, lines, commands, or retrieved PR/comment sources for concrete findings.
- Reuse the `review-workflow-codex` output schema instead of inventing a looser review summary format.
- Return findings ordered by severity with smallest fixes.

## References
- `../../playbooks/personal-swarm-workflow.md`
- `../../playbooks/reviewer-prompts.md`
