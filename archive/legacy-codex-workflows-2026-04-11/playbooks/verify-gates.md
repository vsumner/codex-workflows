# Verify Gates Playbook

Run deterministic verification gates and return explicit PASS/FAIL/SKIP evidence.

## Role Boundary
- Review establishes risk and intent/spec alignment.
- `verification-specialist` establishes functional behavior correctness.
- `verify-gates` establishes command-level gate health (lint/type/build/test).
- Do not treat review findings as proof of correctness without verification evidence.

## Entry Conditions
- A code change exists (staged, working tree, or named file set).
- User asked for validation or handoff readiness.

## Gate Matrix
Choose the smallest relevant matrix from repo signals and changed files.

- Spec-contract gate (when spec artifacts exist or spec/code changed):
1. Run `spec-sync` to regenerate/refresh impacted contract tests.
2. Execute generated contract tests with repo-appropriate command(s).
3. Mark as `SKIP` only when no spec artifacts exist.

- Rust (`Cargo.toml`):
1. `cargo check`
2. targeted `cargo test -p <crate>` (or focused test path)

- Node/TypeScript (`package.json`):
1. lint command (if present)
2. typecheck command (if present)
3. targeted test command (if present)

- Python (`pyproject.toml`/`pytest.ini`):
1. configured lint/static command (if present)
2. targeted `pytest` selection

- Go (`go.mod`):
1. targeted `go test` for touched packages

If a gate command is unavailable, mark `SKIP` with reason.

## Execution Rules
1. Deterministic gate order.
2. Fail-fast by default.
3. On failure: capture concise root cause.
4. If fixes are requested: apply minimal fixes and rerun only failed gates.
5. Retry cap: 2 attempts per failed gate, then escalate.
6. Evidence must be command-attributed for each gate result.
7. No auto-PR behavior for spec drift; return manual remediation steps.

## Output Contract
```md
Overall: PASS|FAIL

Gate Results:
- [PASS|FAIL|SKIP] <gate-name>
  Command: <command>
  Evidence: <summary>

Failed Gates:
- <gate-name>: <root cause>

Minimal remediation:
- <smallest fix list>

Rerun Plan:
- <failed gates only>

Residual risk:
- <remaining risk>
```
