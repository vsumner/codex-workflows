---
name: bug-scanner-autopilot-codex
description: This skill should be used when the user asks to "run bug scanner autopilot", "scan and auto-fix UBS findings", "run ultimate bug scanner workflow", "triage UBS findings", or "scanner -> fix -> verify".
metadata:
  invocation: explicit-only
---

# Bug Scanner Autopilot (Codex)

Run Ultimate Bug Scanner (UBS) end-to-end: scan, triage, minimal fixes, and verification evidence.

Use this as a specialized bug-focused Execute+Verify loop inside RPIV, not as a replacement for the full workflow.

## Inputs
- Scope:
1. staged diff (`git diff --name-only --cached`) when available
2. working diff (`git diff --name-only HEAD`) otherwise
3. explicit user paths/files if provided
- Mode:
1. `review-only` (default)
2. `apply` (fix high-confidence findings)
- Severity policy:
1. default fix threshold: `critical|major`
2. `minor` is optional and off by default
- Optional strict mode:
1. local/default: `ubs --format=json`
2. strict/CI: add `--profile=strict --fail-on-warning --ci`

## Preconditions
- Require `ubs` in PATH.
- If missing, return `BLOCKED` with install command:
1. `curl -sSL https://raw.githubusercontent.com/Dicklesworthstone/ultimate_bug_scanner/main/install.sh | bash`

## Process
1. Build smallest scan target from scope (changed files preferred over full repo).
2. Run UBS in machine-readable mode and capture artifacts:
   - Example staged scan:
     - `ubs --staged --format=json --report-json=.codex/tmp/ubs-findings.json`
   - Example explicit files:
     - `ubs --files=<csv> . --format=json --report-json=.codex/tmp/ubs-findings.json`
3. Normalize and dedupe findings by stable key:
   - `<language>:<file>:<line>:<column>:<category>:<message>`
4. Triage findings:
   - keep actionable, high-confidence findings
   - suppress obvious duplicates/noise with explicit reason
   - preserve all `critical`
5. If `mode=review-only`, return findings report and recommended smallest fixes.
6. If `mode=apply`:
   - create bounded fix packets with disjoint file ownership
   - apply smallest behavior-preserving fixes for `critical|major`
   - avoid broad refactors
7. Run verification after fixes:
1. targeted local checks for impacted areas
2. `/verify-gates` for gate proof
3. `verification-specialist` for behavior proof when requested/high-risk
8. If specs exist or drift is suspected, run `/spec-sync`.

## Guardrails
- Do not mark success from scanner output alone; require verification evidence.
- Never auto-fix low-confidence findings without explicit user approval.
- Stop after 2 fix loops on the same failing set and escalate.
- Keep an auditable mapping from finding IDs to code changes.

## Required Output Schema
```md
Overall: PASS|FAIL

Scanner:
- Command(s): <exact commands>
- Scope: <staged|diff|paths>
- Exit status: <code>

Findings:
- [critical|major|minor] <file:line> - <issue>
  Finding ID: <stable id>
  Evidence: <scanner snippet/category>
  Action: fixed|deferred|false-positive
  Reason: <short reason>

Applied fixes:
- <file list + smallest change summary>

Verification:
- [PASS|FAIL|SKIP] <gate>
  Command: <command>
  Evidence: <key output>

Residual risk:
- <remaining risk and deferred items>
```
