---
name: verify-gates-codex
description: This skill should be used when the user asks to "/verify-gates", "run verification gates", "run lint/typecheck/test gates", or "prove this change passes quality gates".
metadata:
  invocation: explicit-only
---

# Verify Gates (Codex)

Run a standardized, language-aware command matrix and report explicit gate verdicts.

## Role Boundary
- Review identifies risk and intent/spec mismatches.
- `verification-specialist` verifies functional behavior.
- `verify-gates-codex` verifies command-level gate health (lint/type/build/test).
- Do not treat review output as correctness proof without verification evidence.

## Inputs
- Changed files (preferred) or repository root
- Optional user-specified gate commands
- Optional fail-fast override

## Gate Matrix Selection
Choose the smallest relevant matrix based on repo signals and changed files:

- Spec-contract gate (when specs exist or spec/code changed):
1. Run `spec-sync` to regenerate/refresh impacted contract tests.
2. Run generated contract tests with repo-appropriate command(s).
3. If no spec artifacts exist, mark this gate `SKIP`.

- Rust (`Cargo.toml`):
1. Build/check: `cargo check`
2. Tests (targeted first): `cargo test -p <changed-crate>` or focused test path

- Node/TypeScript (`package.json`):
1. Lint: project lint script if present
2. Typecheck: project typecheck script or `tsc --noEmit` if configured
3. Tests: project test script (prefer targeted)

- Python (`pyproject.toml`/`pytest.ini`):
1. Lint/static checks if configured
2. Tests: targeted `pytest` selection first

- Go (`go.mod`):
1. Build/check: `go test ./...` (or targeted package if possible)

If a gate command is unavailable, mark it `SKIP` with reason.

## Execution Policy
1. Execute gates in deterministic order.
2. Prefer fail-fast unless user asks full sweep.
3. On failure, capture root cause and exact command output summary.
4. If user asks for fixes, apply minimal fixes and rerun only failed gates.
5. Retry cap: 2 attempts per failed gate, then escalate.
6. No auto-PR behavior for spec drift; return manual remediation steps.

## Flaky-Test Triage Protocol

When a test gate fails and flakiness is suspected:

1. Isolate: rerun only the failing test command once with identical args.
2. Classify:
   - deterministic failure: treat as real regression and fix immediately.
   - pass on rerun: mark as `suspected flaky` and run one more confirmation rerun.
3. Confirm:
   - fails again: treat as deterministic.
   - passes twice: keep `suspected flaky` classification.
4. Stabilize with the smallest deterministic fix (fixture isolation, timeout bounds, seeded randomness, ordering guarantees).
5. Rerun failed gates only after the fix.
6. Report flaky evidence explicitly in gate output (`command`, run count, pass/fail pattern).

## Required Output Schema
```md
Overall: PASS|FAIL

Gate Results:
- [PASS|FAIL|SKIP] <gate-name>
  Command: <command>
  Evidence: <key output / reason>

Failed Gates:
- <gate-name>: <root cause>

Minimal remediation:
- <smallest fix list>

Rerun Plan:
- <failed gates only>

Residual risk:
- <remaining risk>
```
