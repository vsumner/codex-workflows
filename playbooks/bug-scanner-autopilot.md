# Bug Scanner Autopilot Playbook

## Goal
Convert UBS findings into minimal verified fixes with explicit evidence and low churn.

## When To Use
- User asks for bug scanning + autofix workflow.
- Pre-merge hardening where scanner findings must be resolved or triaged.
- Regression triage where fast machine-readable findings are needed.

## Inputs
- Scope (`staged`, `diff`, explicit files/paths)
- Mode (`review-only` or `apply`)
- Strictness (`strict` for CI/pre-merge, default for local)

## Recommended Commands
- Staged scan:
  - `ubs --staged --format=json --report-json=.codex/tmp/ubs-findings.json`
- Strict repo scan:
  - `ubs . --profile=strict --fail-on-warning --ci --format=json --report-json=.codex/tmp/ubs-findings.json`
- File-targeted scan:
  - `ubs --files=<csv> . --format=json --report-json=.codex/tmp/ubs-findings.json`

## Execution Contract
1. Prefer smallest scope scan.
2. Parse findings and dedupe by stable key.
3. Keep actionable/high-confidence findings only.
4. In `apply` mode, fix `critical|major` with minimal patches.
5. Run `/verify-gates` and include explicit `PASS|FAIL|SKIP` evidence.
6. Use `verification-specialist` when behavioral correctness is in question.
7. Stop after 2 fix loops for same failing set; escalate.

## Output Contract
- Overall `PASS|FAIL`
- Scanner command + exit status
- Findings with `file:line`, severity, and action (`fixed|deferred|false-positive`)
- Applied fixes summary
- Verification matrix with command evidence
- Residual risk
