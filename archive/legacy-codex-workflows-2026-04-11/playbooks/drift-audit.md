# Drift Audit

## Goal
Detect and correct workflow drift from strict wave orchestration.

## Audit Cadence
- Run daily with a rolling window that includes yesterday and today.

## Required Checks
1. Plan packet exists before any implementation starts.
2. Every implementation packet includes dependencies, owned files, and verification command.
3. Spec review happens before quality review.
4. FAIL verdicts trigger fix loops, not bypasses.
5. Final handoff includes evidence and residual risk.

## Evidence To Capture
- Role used at each gate.
- PASS/FAIL verdict at each gate.
- Validation commands executed.
- Files changed outside packet scope.
- Retry counts per gate.

## Drift Signals
- Work starts without packet.
- Missing or vague validation output.
- Reviewer verdict skipped or merged.
- Repeated cross-scope edits.
- Repeated gate bypass under time pressure.

## Corrective Actions
- Re-enable strict packet requirement.
- Reduce parallel tracks.
- Tighten owned file lists.
- Add blockers for missing evidence.

## Escalate To Human
- Two consecutive cycles with unresolved critical findings.
- Conflicting requirements across teams.
- Missing environment required for verification.
