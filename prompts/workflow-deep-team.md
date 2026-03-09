---
description: Run the personal RPIV workflow in deep-team topology for risky or uncertain work.
argument-hint: "[task_or_scope]"
---
Run the personal `deep-team` RPIV workflow for: $ARGUMENTS

Execution contract:
- Use this for risky refactors, concurrency/state work, public API changes, or uncertain integrations.
- Inherit RPIV prompt contracts for completeness, verification, grounding, and concise user updates.
- Keep the RPIV phases explicit and stronger:
1. heavier research
2. stricter planning review
3. bounded execution packets
4. separate scrutiny and user-surface validation
5. dedicated review pass
- Learning tests should run inside Research whenever assumptions matter.
- Limit critique/fix loops to 2 rounds before escalation.
- Return: phase, artifacts, failed gates, remediation plan, residual risk.
