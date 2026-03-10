---
description: Run the Verify phase team using explicit assertions, validation state, and a verification report.
argument-hint: "[task_or_scope] [mode=auto|solo|team|deep-team] [proof=auto|gates|behavior|full]"
---
Use `workflow-verify-codex` as the canonical Verify phase for: $ARGUMENTS

Execution contract:
- Read `validation-contract.md`, `plan.md`, `execution-summary.md`, and `features.json` when present.
- Infer proof weight (`gates|behavior|full`) and topology unless the user overrides them.
- For non-trivial work, maintain a short native `update_plan` checklist with Verify in progress.
- Reconstruct the assertion set, update `validation-state.json` early, and write `verification-report.md` with evidence and verdicts.
- Tie proof to exact commands, probes, or observed behavior; do not confuse implementation claims with proof.
- Use `/verify-gates` for command-level scrutiny by default and add broader behavior or review lanes only when the risk justifies them.
- If delegated validators fail, time out, or return partial evidence, continue locally, record the current evidence, and end with `passed`, `failed`, or `blocked` rather than stalling.
- If verification exposes a stale plan or graph, step back to the earliest invalid phase instead of forcing the fix through Verify.
- Mark Verify complete in `update_plan` only when required assertions are resolved.
- Return: chosen proof weight, chosen topology, overall verdict, failed or blocked assertions, and remediation recommendation.
