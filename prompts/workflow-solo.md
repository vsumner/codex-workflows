---
description: Run the personal RPIV workflow in a single-threaded topology for tiny or tightly coupled tasks.
argument-hint: "[task_or_scope]"
---
Run the personal `solo` workflow for: $ARGUMENTS

Execution contract:
- Keep the RPIV phases explicit even when single-threaded.
- Inherit RPIV prompt contracts for completeness, verification, and concise user updates.
- Use this for tiny, bounded, low-ambiguity work or tightly coupled edits where orchestration would add churn.
- Keep one thread, but still:
1. research briefly when needed
2. plan explicitly
3. execute
4. verify
- Write lightweight artifacts when context loss is likely.
- Run the smallest relevant verification before concluding.
- Return: current phase, evidence, residual risk, next step.
