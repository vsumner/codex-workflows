# Lessons From gstack

This note captures what `codex-workflows` should learn from `/Users/vsumner/src/github.com/garrytan/gstack` without turning into a gstack clone.

## Core Lesson

gstack's useful pattern is not "many slash commands." It is a layered system:

- fat skills encode judgment, phase order, and role behavior;
- deterministic tooling handles stateful or noisy execution;
- resolvers and generation keep skill docs aligned with source metadata;
- local learnings compound without bloating the always-loaded prompt.

That matches this repo's direction: Codex-native workflow skills plus small deterministic CLIs only where existing primitives are too noisy.

## What To Copy

- Host-specific adaptation. Codex skills should be generated or written for Codex's actual plugin, skill, memory, and CLI surfaces. Do not make Codex ingest Claude-specific skill internals by default.
- A bias toward committed, checkable skill artifacts. If plugin skills grow enough to drift, introduce a small template/check pipeline before the drift becomes manual review work.
- Command metadata as a source of truth. When this repo adds a real CLI, its command definitions should feed examples, docs, and validation checks.
- Cheap validation first. Parse examples, check paths and commands, and validate generated artifacts before adding expensive agent evals.
- Append-only learnings for non-obvious operational facts. Search them on demand, include confidence/source metadata, and refuse to log obvious facts.

## What Not To Copy

- Do not port gstack wholesale.
- Do not create a new CLI that only wraps `git`, `gh`, or Codex app/plugin primitives.
- Do not recreate `~/.codex/skills` pollution with broad global installs.
- Do not add template generation before there is enough plugin-skill drift to justify it.
- Do not let preambles become a new giant global prompt.

## Next Bet

The next useful primitive is probably a Codex-native learning loop, not an orchestration CLI.

A first version should be deliberately small:

1. Search local learnings only when a skill asks for prior context.
2. Record learnings as append-only local JSONL with `type`, `key`, `insight`, `confidence`, `source`, and relevant `files`.
3. Log only non-obvious findings that would save a future session time.
4. Keep memory-extension consolidation separate from local searchable learnings.

This would complement `codex-threads`: threads are evidence from past sessions; learnings are curated operational conclusions from that evidence.
