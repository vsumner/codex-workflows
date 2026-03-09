---
name: simplify-codex
description: This skill should be used when the user asks to "/simplify", "simplify this code", "cleanup recent changes", or "run simplify triage". It runs parallel reuse/quality/efficiency review and can optionally apply minimal fixes.
metadata:
  invocation: explicit-only
---

# Simplify Workflow (Codex)

Run a focused, low-churn simplification pass over current changes.

## Goal
- Review for reuse, quality, and efficiency in parallel.
- Keep only high-confidence findings.
- Optionally apply the smallest safe fixes.
- Treat this as a post-execute refinement pass inside RPIV, not a primary workflow.

## Design
- `simplify-codex` should remain an orchestration skill, not a role.
- It coordinates `reuse_reviewer`, `quality_reviewer`, and `efficiency_reviewer`.
- Keep `/prompts:simplify` as the user-facing wrapper for explicit invocation.
- Do not model Simplify as its own RPIV phase.

## Inputs
- Scope:
1. `git diff --staged`
2. `git diff HEAD`
3. If both empty, use user-provided files/paths
- Mode:
1. `review-only` (default)
2. `apply` (only if user asks to fix/apply)

## Context-Aware Skill Loading
Before review, load domain skills that match the repo/scope:
- Rust: `Cargo.toml` exists or `.rs` files changed -> use `rust`
- React/Next frontend: `.tsx/.jsx` or Next signals -> use `react` (and `nextjs` if Next is present)
- Node backend/tooling: `package.json` with server/cli changes -> use `node`
- Nix: `flake.nix`, `default.nix`, or `.nix` files -> use `nix`
- Go: `go.mod` or `.go` files -> use `go`
- Python: `pyproject.toml`/`.py` -> use `python`
- Spacebot repo (`spacedriveapp/spacebot`) -> additionally use `jamiepine-style` and `rust`

If no language-specific skill applies, continue with baseline reviewers.

## Parallel Reviewers (Read-Only)
- `reuse_reviewer`
- `quality_reviewer`
- `efficiency_reviewer`

Add:
- `rust_correctness_reviewer` for Rust async/state/concurrency-sensitive changes

## Process
1. Gather diff and changed file list.
2. Load applicable language/repo skills.
3. Run reviewers in parallel on the same artifacts.
4. Aggregate and dedupe findings.
5. Keep concrete, actionable, high-confidence findings only.
6. Require evidence commands for each kept finding.
7. If `review-only`, return report.
8. If `apply`, implement the smallest safe fixes and run targeted verification.

## Guardrails
- Preserve behavior; skip risky refactors.
- Do not force a fix for low-confidence findings.
- If a finding is not worth addressing, mark it as skipped with a short reason.

## Output Schema
```md
Verdict: PASS|FAIL

Findings:
- [critical|major|minor] <file:line> - <issue>
  Why it matters: <impact>
  Suggested fix: <smallest concrete fix>

Applied fixes:
- <fix or "none (review-only)">

Verification:
- <targeted command(s)>

Residual risk:
- <remaining risk>
```
