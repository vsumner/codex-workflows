# Simplify Playbook

Run a post-change cleanup pass with deterministic triage and minimal churn.

## Entry Conditions
- Code changes exist (`git diff --staged` or `git diff HEAD`), or user gives explicit files.
- User asked for simplify/cleanup or review hardening.

## Core Passes (Parallel)
- Reuse review (`reuse_reviewer`)
- Quality review (`quality_reviewer`)
- Efficiency review (`efficiency_reviewer`)
- Rust correctness (`rust_correctness_reviewer`) for Rust async/state/concurrency-sensitive changes

## Context Enrichment
- Load domain skills by repository/language signals:
1. `rust`
2. `react`
3. `nextjs` (when Next.js is present)
4. `node`
5. `nix`
6. `go`
7. `python`
- In Spacebot (`spacedriveapp/spacebot`), also load `jamiepine-style`.

## Execution Rules
1. Use staged diff first, then `HEAD`.
2. Keep only high-confidence, actionable findings.
3. Preserve behavior; skip risky or speculative refactors.
4. If apply mode is requested, implement the smallest safe fixes.
5. Run targeted verification after applied fixes.

## Output Contract
```md
Verdict: PASS|FAIL

Findings:
- [critical|major|minor] <file:line> - <issue>
  Why it matters: <impact>
  Suggested fix: <smallest concrete fix>

Applied fixes:
- <smallest concrete fix list or "none">

Verification:
- <targeted commands>

Residual risk:
- <remaining risk>
```
