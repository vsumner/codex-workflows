---
name: codex-threads
description: Use when Victor asks to search, summarize, mine, resolve, or learn from local Codex thread/session history, including questions like "what patterns are forming?", "find the thread where...", or "turn a past Codex workflow into a skill".
---

# Codex Threads

Use the `codex-threads` CLI instead of reading raw `~/.codex` history directly. It keeps transcript access bounded, JSON-shaped, and repeatable.

## Command Order

1. Check health when unsure:
   ```bash
   codex-threads --json doctor
   ```
2. Refresh the compact local index before searching:
   ```bash
   codex-threads --json sync
   ```
3. Prefer narrow search before reading a thread:
   ```bash
   codex-threads --json messages search "query" --since 14d --limit 20
   codex-threads --json threads resolve "fuzzy thread name"
   ```
4. Read only the smallest useful window:
   ```bash
   codex-threads --json threads read <session-id> --limit 80
   codex-threads --json events read <session-id> --limit 50
   ```

## Pattern Mining

For workflow improvement questions, start with:

```bash
codex-threads --json patterns recent --since 7d
codex-threads --json skill-candidates --since 14d
```

Then inspect only the session ids needed to support a concrete recommendation.

## Safety

- The CLI is read-only except for its own cache/index.
- Do not paste full raw transcripts unless Victor explicitly asks.
- Cite session ids, thread names, and short evidence snippets when recommending a new workflow or skill.
- If the CLI reports a stale/missing index, run `codex-threads --json sync` once and retry.
