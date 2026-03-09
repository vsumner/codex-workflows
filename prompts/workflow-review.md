---
description: Run a dedicated post-verify review team for correctness, safety, and maintainability.
argument-hint: "[task_or_scope]"
---
Run the Review phase for: $ARGUMENTS

Execution contract:
- This is a dedicated review pass after or alongside Verify on high-stakes work.
- For local workflow-driven review, read `plan.md`, `execution-summary.md`, and `verification-report.md` first when present.
- For explicit GitHub PR number/URL review, treat those artifacts as optional context and prefer PR metadata, diff, comments, and history first.
- `review-workflow-codex` is the canonical review workflow in Victor's stack.
- Use native `/review` only when Victor explicitly asks for the built-in reviewer.
- Infer the review path from context first:
  - local diff or "my changes" -> local author review
  - PR number, PR URL, or review-thread context -> PR review
  - current branch PR context + PR language -> PR review
- Resolve intent-source precedence as:
1. explicit user request
2. RPIV artifacts (`plan.md`, `validation-contract.md`, `execution-summary.md`) when present and clearly applicable
3. PR title/body
4. commit series / branch history
- If the repository is `spacedriveapp/spacebot`, prefer the installed home-level `review-spacebot-codex` specialization when available.
- If that specialization alias is unavailable, stay in `review-workflow-codex` and apply Spacebot review rules inline instead of surfacing missing repo-local skill-path noise.
- For PR review, auto-fetch and deduplicate existing review comments before emitting new findings.
- For explicit GitHub PR number/URL review, do not spend material time searching for absent RPIV artifacts or treat their absence as a finding.
- Add a history/rationale lane only when PR/body/history context adds real evidence.
- Add an instruction-compliance lane only when repo instructions or changed comments/docs are materially relevant.
- Keep remote PR review read-only and low-noise by default.
- If a later follow-up explicitly asks to remediate a GitHub PR, hand off to `/prompts:fix-pr-feedback` instead of mutating Review into an ad hoc fixer.
- If the change affects trust boundaries or attack surface, run `security-review` as a specialized review lane and keep only high-confidence exploitability findings.
- Only include simplify-style cleanup lanes by default for local author review.
- Keep screenshot/browser evidence in Verify rather than treating it as a default Review lane.
- Focus on correctness, regressions, unsafe assumptions, silent failure paths, and reviewability.
- Use specialized reviewers when the change warrants it.
- Start with a small reviewer set and degrade gracefully when agent/thread capacity is tight.
- Keep findings concrete and evidence-backed.
- Grounding rules:
1. separate observed evidence from inference
2. cite exact file paths, lines, commands, or retrieved PR/comment sources
3. if the first evidence pass is thin, gather one more focused pass before concluding no finding exists
- User updates:
1. keep them brief and milestone-based
2. update when review scope changes, external comments are deduplicated, or the final finding set stabilizes
- Return findings ordered by severity with smallest fixes and residual risk.
