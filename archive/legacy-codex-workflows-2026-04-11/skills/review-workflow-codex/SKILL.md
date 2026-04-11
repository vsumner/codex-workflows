---
name: review-workflow-codex
description: This skill should be used when the user asks for "review-workflow", "run a code review workflow", "review my diff with reviewer agents", "parallel review my changes", or "find high-confidence review issues".
metadata:
  invocation: explicit-only
---

# Review Workflow (Codex)

Run a deterministic, low-churn code review workflow over the current changes.

## Routing
This workflow is the canonical review path for Victor's workflow stack.

Use `review-workflow-codex` by default for ordinary review requests unless Victor explicitly asks for native `/review`.

This workflow is the right fit when the request needs:
1. parallel reviewer agents
2. higher-confidence or multi-lane review
3. PR-aware review with comment dedupe
4. a dedicated post-verify review pass
5. review plus apply/verification orchestration

Do not let generic native-first routing collapse ordinary review requests back to `/review`.

## Inference-First Inputs
Infer the review shape from context before asking for arguments.

- Review source inference:
1. Explicit PR number, GitHub PR URL, or review-thread/comments context -> `pr`
2. Current branch has an open PR and the user says "review the PR" or equivalent -> `pr`
3. Otherwise, use local changes:
   - `git diff --staged`
   - `git diff HEAD`
4. If both local diffs are empty and no PR target can be inferred, ask for target files or a compare base

- Review stance inference:
1. "review my changes", current RPIV Execute/Verify output, or local diff review -> `author`
2. PR review or language like "review this PR" / "review their changes" -> `reviewer`

- Write mode inference:
1. Default to `review-only`
2. Use `apply` only when the user explicitly asks to fix/apply, or issues a short imperative follow-up after findings

- Intent packet inference:
1. For local workflow-driven review, pull from `plan.md`, `execution-summary.md`, `validation-contract.md`, prior workflow artifacts, or the user request
2. For explicit GitHub PR number/URL review, treat RPIV artifacts as optional context and prefer PR-native sources first
3. Infer `goal`, `constraints`, `non_goals`, and `acceptance_criteria` before blocking
4. Return `BLOCKED` only when the remaining ambiguity would materially change the review

- Optional requirements/task packet:
1. Use provided requirements/spec context when present
2. Add `spec_reviewer` only when the task or repo artifacts justify it

## Mode Escalation Rule

When a finding report was already produced in the current thread, treat direct follow-ups like:

- "do it"
- "fix everything"
- "fix it all"
- "apply the fixes"

as `apply` mode requests unless the user explicitly says to remain read-only.

If the follow-up explicitly targets a GitHub PR, such as:

- "fix this PR"
- "address review findings on this PR"
- "take this PR to green"

handoff to `fix-pr-feedback-codex` instead of staying inside review `apply` mode.

## Context-Aware Skill Loading
Before running reviewers, load skills based on repository/scope signals.

- Rust project or `.rs` changes -> use `rust`
- TypeScript (`*.ts`, `*.tsx`, `tsconfig*.json`, `vitest.config.ts`, `eslint.config.ts`, type-heavy library/config diffs) -> use `typescript`
- React UI (`.tsx/.jsx`) -> use `react`
- Next.js signals (`next.config.*`, `app/`, `pages/`) -> add `nextjs`
- Node backend/tooling (`package.json`, server/CLI JS/TS) -> use `node`
- Nix project/files (`flake.nix`, `*.nix`) -> use `nix`
- Go project/files (`go.mod`, `*.go`) -> use `go`
- Python project/files (`pyproject.toml`, `*.py`) -> use `python`

Composition rules:
- `.tsx` or type-heavy React UI -> load `react` + `typescript`
- Next.js + TS -> load `nextjs` + `react` + `typescript`
- plain backend/tooling `.ts` -> load `node` + `typescript`
- plain `.js/.mjs/.cjs` -> route through `node`
- `.jsx` UI -> route through `react`, and add `node` only when server/tooling context also exists
- Do not refer to a `javascript` skill unless one is actually installed.

Spacebot auto-detection:
- If current repository is `spacedriveapp/spacebot` (by path or git remote), also load:
1. `jamiepine-style`
2. `rust`
3. Add `rust_correctness_reviewer` to reviewer set
- Prefer the installed home-level `review-spacebot-codex` compatibility alias when available.
- If the alias cannot be loaded, continue inside `review-workflow-codex` with the same Spacebot rules.
- Do not probe repo-local `.agents/skills/review-spacebot-codex/SKILL.md` paths or surface their absence as review friction.

## Review Routing Rules
- `source=local` + `stance=author`:
1. Run the full review workflow
2. Include simplify triage by default
3. Allow apply-mode follow-through if explicitly requested

- `source=pr` + `stance=reviewer`:
1. Stay read-only by default
2. For explicit GitHub PR number/URL review, start with PR metadata, diff, review comments, and branch history; use RPIV artifacts only when already present and clearly relevant
3. Auto-run `pr-comments` when GitHub context is available or the user supplied copied review feedback
4. Deduplicate existing comments before producing new findings
5. Run `history_reviewer` only when PR rationale/history context is materially useful
6. Suppress simplify/rewrite noise unless the user explicitly asks for cleanup suggestions

- `source=pr` + `stance=author`:
1. Treat as review of your own PR branch
2. For explicit GitHub PR number/URL review, start with PR metadata, diff, review comments, and branch history; use RPIV artifacts only when already present and clearly relevant
3. Auto-run `pr-comments` when available
4. Run `history_reviewer` only when PR rationale/history context is materially useful
5. Keep simplify off by default unless the user asks for cleanup/refinement

- `source=local` + `stance=reviewer`:
1. Use when the user points at specific files or a local compare target that is not framed as their own change
2. Keep simplify off unless requested

## Parallel Reviewers (Read-Only)
- `reuse_reviewer`: duplicate logic and missed helper reuse
- `quality_reviewer`: shallow bug scan baseline for defects, regressions, safety, maintainability, and test gaps
- `efficiency_reviewer`: performance and resource concerns
- `rust_correctness_reviewer`: add for Rust async/state/concurrency-sensitive changes

## Conditional Context Lanes
- `history_reviewer`: PR title/body, commit series, branch history, and rationale context when review intent or change history is unclear
- `instruction_compliance_reviewer`: project instruction compliance plus changed comment/doc alignment when behavior or instructions make that relevant

Run these only when they introduce distinct evidence.
Do not turn them into always-on checklist noise.

## External Feedback Deduplication (Required)

When the source is `pr` or the user provides copied feedback (CodeRabbit/GitHub comments), run a canonicalization pass before edits:

1. Parse each finding into `(file:line, normalized text, severity, source)`.
2. Create fingerprint: `file:line + normalized text`.
3. Collapse duplicates to one canonical finding.
4. Keep backlinks to duplicate sources for traceability.
5. Mark each canonical finding with initial status: `fix-needed`, `already-fixed`, or `not-applicable`.

Do not start code edits until this dedupe pass is complete.

## Adversarial Review Gate (High-Stakes)
Run `adversarial-review` when any high-stakes trigger is present:
1. Security/authentication/authorization changes
2. Public API/protocol/contract changes
3. Concurrency/state-machine/cancellation-sensitive logic
4. Large refactors with cross-module coupling

Debate rules:
1. Independent first pass (advocate vs auditor) on identical artifacts
2. Evidence required for every claim (`file:line` + check command)
3. Max 2 total rounds per side (initial + one rebuttal)
4. If unresolved disagreements remain, escalate to human decision

## Security Review Lane
Run `security-review` as a dedicated review lane when the diff changes trust boundaries or attack surface, including:
1. authentication, authorization, or session logic
2. request parsing, input validation, deserialization, or template rendering
3. secrets, crypto, token, or certificate handling
4. file access, path handling, shell execution, or database query construction
5. public API endpoints or externally reachable workflows

Security findings must be:
1. high-confidence
2. tied to a concrete exploit path
3. limited to newly introduced risk in the current diff

Drop theoretical or low-signal security findings.

## Process
1. Infer `source`, `stance`, write mode, diff target, and intent packet from context.
2. Gather diff/PR artifacts and changed files.
3. If the target is an explicit GitHub PR number/URL, treat PR-native context as primary and do not spend material time searching for absent RPIV artifacts.
4. Resolve intent-source precedence:
   1. explicit user request
   2. RPIV artifacts (`plan.md`, `validation-contract.md`, `execution-summary.md`) when present and clearly applicable
   3. PR title/body
   4. commit series / branch history
5. If reviewing a PR, fetch PR comments when available and canonicalize external feedback before new findings.
6. Detect language/repo context and load applicable skills.
7. Enable or suppress simplify lanes based on review routing rules.
8. Add `history_reviewer` only when PR/body/history context adds real evidence.
9. Add `instruction_compliance_reviewer` only when repo instructions or changed comments/docs are materially relevant.
10. Launch reviewers in parallel on identical artifacts.
   If agent/thread capacity is constrained, keep the strongest available reviewer set and continue; do not turn lane reduction into a user-facing blocker.
11. Aggregate and deduplicate findings.
12. Keep only concrete, actionable, high-confidence items.
13. Reject uncited/speculative findings.
14. For each finding, include: problem, why it matters, evidence command, and smallest fix.
15. Include intent alignment status (`ALIGNED|MISMATCH`) against goal/constraints/non-goals/acceptance criteria.
16. Attribute which sources established `goal`, `constraints`, `non_goals`, and `acceptance_criteria`.
17. Enforce convergence cap: maximum 2 critique/fix loops per finding set, then escalate to human decision.
18. If a requirements packet is provided, run `spec_reviewer` after triage.
19. If high-stakes triggers are present or user requests debate, run `adversarial-review` and merge its synthesis.
20. If security triggers are present, run `security-review` and keep only concrete, high-confidence exploitability findings.
21. If mode is `apply`, run minimal corrective edits, then run `verification-specialist` for functional proof and `/verify-gates` for gate proof.
22. For non-trivial apply runs, perform a final fresh-eyes pass focused on silent failures, cancellation/race branches, missing negative tests, and fixture determinism before concluding.
23. Return a consolidated report.

## Default Behavior
- Review-only by default. Do not edit code unless explicitly asked.
- If user asks to apply fixes, generate minimal corrective packets and run targeted verification.
- If user issues a short imperative follow-up after findings ("do it", "fix everything", "fix it all"), apply mode escalation.
- If the follow-up explicitly references PR remediation, hand off to `fix-pr-feedback-codex` rather than treating it as generic review apply work.
- Infer the correct review path before asking for arguments.
- Ordinary local author review should default to this workflow unless Victor explicitly asks for native `/review`.
- Remote PR review should feel read-only, deduplicated, and comment-aware by default.
- Explicit GitHub PR number/URL review should be PR-native first; absent RPIV artifacts are normal and should not be treated as a review problem.

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
  - goal: <user request|RPIV artifact|PR title/body|history|unknown>
  - constraints: <source list>
  - non_goals: <source list>
  - acceptance_criteria: <source list>

Recommended fixes:
- <smallest concrete fix list>

Debate outcome:
- Triggered: yes|no
- Advocate summary: <short summary or "n/a">
- Auditor summary: <short summary or "n/a">
- Unresolved disagreements: <none or list>

Security review:
- Triggered: yes|no
- Findings kept: <count or "n/a">
- Dropped as low-signal: <count or "n/a">

Context lanes:
- History review: on|off
- Instruction compliance: on|off

Verification:
- <targeted command(s)>

Residual risk:
- <remaining risk>
```
