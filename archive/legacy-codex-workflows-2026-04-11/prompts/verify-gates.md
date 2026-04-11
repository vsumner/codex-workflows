---
description: Run standardized verification gates and return a strict gate matrix.
argument-hint: "scope=<files|paths|diff>"
---
Run the `verify-gates-codex` workflow.

Scope: $ARGUMENTS

Requirements:
- Build a language-aware gate matrix for this repository.
- Return explicit `PASS|FAIL|SKIP` per gate with command and concise evidence.
- Use fail-fast by default.
- For failures, include root cause, minimal remediation, and rerun plan for failed gates only.
- Return overall `PASS|FAIL` and residual risk.
