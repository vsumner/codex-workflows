---
description: Run UBS bug-scanner autopilot: scan, triage, minimal fixes, and verification evidence.
argument-hint: "scope=<diff|staged|files|paths> [mode=review-only|apply] [strict=true|false]"
---
Run the `bug-scanner-autopilot-codex` workflow.

Scope: $ARGUMENTS

Requirements:
- Treat this as a specialized bug-focused Execute+Verify loop inside RPIV, not as a general replacement for the full workflow.
- Use changed files first (staged, then `HEAD` diff) unless explicit scope is provided.
- Run UBS in machine-readable mode and dedupe findings with stable IDs.
- Keep only actionable/high-confidence findings.
- In `mode=apply`, fix `critical|major` with minimal, behavior-preserving changes.
- Run `/verify-gates` after fixes and report `PASS|FAIL|SKIP` evidence.
- If behavior risk remains, run `verification-specialist`.
- Return overall verdict, findings/actions, fix summary, and residual risk.
