---
description: Run simplify triage over current changes (reuse, quality, efficiency), optionally applying minimal fixes.
argument-hint: "scope=<diff|files> [mode=review-only|apply]"
---
Run the `simplify-codex` workflow.

Scope: $ARGUMENTS

Requirements:
- Treat this as an Execute-phase refinement subroutine inside RPIV, not as a replacement for Research or Plan.
- Treat this as a skill-orchestrated reviewer pass, not as its own role or RPIV phase.
- Use staged diff first, then `git diff HEAD`.
- Load applicable language/repo skills (Rust/React/Node/Nix/Go/Python; plus `jamiepine-style` for Spacebot).
- Run reuse, quality, and efficiency review in parallel.
- Keep high-confidence findings only.
- If mode is `apply`, make minimal behavior-preserving fixes and run targeted verification.
- Return `PASS|FAIL`, findings with `file:line`, minimal fix list, verification commands, and residual risk.
