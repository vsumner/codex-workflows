# Codex Workflows

Victor's Codex-native workflow layer: plugins, skills, memory extensions, and small CLIs that help Codex follow Victor's preferred research -> plan -> execute -> verify loops with less repeated instruction and less context noise.

The old prompt/role mirror has been moved to `archive/legacy-codex-workflows-2026-04-11/`.

This repo is the Codex counterpart to `vsumner/claudify`, but it should not port Claudify wholesale. Codex has different native primitives. Build only the pieces that fit Codex's actual plugin, skill, memory, and CLI surfaces.

Repeated workflow friction is the evidence source, not the mission. When Victor repeatedly has to explain the same workflow move to Codex, this repo should answer with the smallest durable artifact:

- a skill when Codex needs routing or operating guidance;
- a CLI when Codex needs a clean command-shaped interface to noisy local state;
- a memory extension when future consolidation should retain the lesson;
- a handoff or validation artifact when the next session needs state.

Do not add machinery just to aggregate existing primitives. Prefer `git`, `gh`, Codex plugins/apps, and existing CLIs unless there is a real missing primitive.

## Current Surface

- `codex-threads`: read-only CLI for searching and summarizing local Codex thread history.
- `plugins/victor-workflows`: Codex plugin bundle with skills that teach Codex how to use these CLIs.
- `memories_extensions/victor-workflows`: memory consolidation guidance for retaining durable workflow-tooling lessons.

## Development

```bash
cargo build
cargo test
cargo run -p codex-threads -- --json doctor
```

Install the first CLI locally:

```bash
cargo install --path crates/codex-threads
```
