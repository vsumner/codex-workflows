---
description: Run Spacebot maintainer-style review with Rust correctness checks.
argument-hint: "scope=<diff|files> [focus]"
---
Run the `review-spacebot-codex` workflow for `spacedriveapp/spacebot`.

Scope: $ARGUMENTS

Requirements:
- Treat this as the Spacebot-specific Review specialization inside RPIV.
- This specialization lives in `~/.codex`, not a repo-local `.agents/skills/` path.
- Apply `jamiepine-style` and `rust` guidance.
- Include `rust_correctness_reviewer` in the review pass.
- Keep review read-only.
- Return `PASS|FAIL`.
- Provide severity-ranked findings with `file:line` evidence.
- Include smallest fix list, targeted verification commands, and residual risk.
