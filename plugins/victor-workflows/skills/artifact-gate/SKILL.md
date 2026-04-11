---
name: artifact-gate
description: Use when Victor is considering a new CLI, plugin, skill, AGENTS.md rule, memory extension, handoff, automation, or workflow surface; checks whether the artifact is necessary or duplicates existing Codex/git/gh/tool primitives.
---

# Artifact Gate

Use this skill before creating durable workflow machinery.

The goal is to prevent overbuilding. Most workflow friction should become a better prompt, a skill instruction, or no artifact at all. Build a CLI only when there is a real deterministic/noisy-state gap.

## Gate Questions

Answer these before recommending a new artifact:

1. What repeated friction is this supposed to remove?
2. Is the friction visible in recent sessions, a current task, or only a speculative idea?
3. What is the ideal end state if this friction is solved well over repeated sessions?
4. What is the smallest artifact that moves toward that end state without closing off the better design?
5. Can existing primitives solve it cleanly: `git`, `gh`, shell, Codex apps/connectors, MCP tools, `codex-threads`, or existing skills?
6. Does the work require deterministic access to large/noisy state, stable IDs, pagination, structured JSON, or bounded output?
7. Would a skill be enough to teach command order, safety rules, and verification?
8. Would an `AGENTS.md` rule be better because it is a broad behavior preference?
9. Would a memory extension be better because it only affects future memory consolidation?
10. What is the validation contract for the proposed artifact?

## Artifact Decision Table

- **No artifact**: one-off issue or existing tools already handle it.
- **Skill**: repeated process, routing, safety boundary, review habit, verification habit, or command ordering.
- **CLI**: repeated access to noisy state where Codex needs search, resolve, read, bounded JSON, predictable errors, or file outputs.
- **`AGENTS.md`**: stable behavior Victor expects in every matching repo/session.
- **Memory extension**: guidance for what future memory consolidation should retain or ignore.
- **Handoff**: temporary cross-session state for a long-running branch or plan.
- **Docs**: human-facing explanation of repo direction, architecture, or decisions.
- **Plugin metadata**: packaging/discovery of stable skills and companion CLIs.

## CLI Bar

Only recommend a new CLI when at least two are true:

- Raw source output is too big, noisy, or awkward for repeated Codex use.
- The workflow needs stable IDs, resolve commands, pagination, bounded search, or file outputs.
- Existing connectors or apps provide access but not a composable command-shaped interface.
- Codex repeatedly guesses command flags, object IDs, or API paths.
- The safe default must be encoded: read-only, draft-only, dry-run, or explicit approval for writes.

Reject CLIs that just wrap:

- `git status`, `git diff`, `git log`, or branch state.
- `gh pr view`, `gh run view`, or simple GitHub queries.
- A single shell command with stable output.
- A workflow that is mostly judgment rather than deterministic state access.

## Validation Contract

Every approved artifact needs a concrete fresh-session test:

```text
Given <task/context>, a fresh Codex session should <observable behavior>
without <known failure mode>.
```

Examples:

- Given "what's next?", Codex checks `git status --short --branch`, recent commits, and handoff state before recommending a publish or implementation action.
- Given "learn from my last week of sessions", Codex uses `codex-threads` first and returns patterns with session ids instead of reading raw transcripts.
- Given a proposed CLI that wraps branch state, Codex rejects it and improves a skill over `git`/`gh` instead.

## Output Shape

```text
Decision: <build skill | build CLI | update AGENTS.md | update memory extension | write handoff | no artifact>.

Ideal end state: <target workflow shape this decision moves toward>.
Reason: <short rationale tied to the gate questions>.
Validation: <fresh-session behavior check>.
Risk: <only if meaningful>.
```
