---
description: Resume a paused personal RPIV run from repo-local workflow artifacts and current thread state.
argument-hint: "[task_or_scope_or_slug]"
---
Resume the personal swarm workflow for: $ARGUMENTS

Execution contract:
- Reconstruct current state from `.codex-workflow/{slug}/` first, then use the active thread and handoffs to fill gaps.
- Inherit RPIV prompt contracts for completeness and concise user updates.
- Prefer native thread continuity over custom restart logic:
1. resume the existing paused orchestrator thread when state is coherent
2. fork only for speculative branches or materially different approaches
3. start fresh only when the existing thread is stale, contradictory, or untrustworthy
- Decide whether to:
1. continue the same orchestrator thread
2. fork a speculative branch
3. replace stale workers with fresh packets
- If the initiative is done or superseded, archive/close the old thread instead of resuming it.
- Do not blindly replay old packets if the facts changed.
- Return: current phase, stale packets/features, restart decisions, next dispatch set.
