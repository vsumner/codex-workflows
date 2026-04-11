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
- `plugins/victor-workflows`: Codex plugin bundle with fat workflow skills and small deterministic CLI guidance.
  - `codex-threads`: use the local history CLI instead of reading raw transcripts.
  - `repo-orientation`: answer "what's next?" and branch/PR readiness questions using `git`, `gh`/GitHub, handoff state, and `codex-threads` only when needed.
  - `workflow-learning`: mine recent Codex sessions for repeated friction and recommend the smallest durable artifact.
  - `artifact-gate`: decide whether a proposed workflow artifact should be a skill, CLI, `AGENTS.md` rule, memory extension, handoff, docs, or nothing.
  - `debug-environment`: diagnose local dev, package-manager, workspace, and toolchain failures without blind reruns or broad rollback.
- `memories_extensions/victor-workflows`: memory consolidation guidance for retaining durable workflow-tooling lessons.

## Workflow Thesis

Codex is the harness. This repo should make Codex easier to steer, not replace its agent loop.

Default loop:

1. Use `repo-orientation` to decide the next action from current repo state.
2. Use `workflow-learning` to turn repeated session friction into evidence-backed recommendations.
3. Use `artifact-gate` before adding durable machinery.
4. Promote only the smallest useful artifact:
   - skill for repeated judgment, routing, and command order;
   - CLI for repeated deterministic access to noisy state;
   - `AGENTS.md` for stable behavior rules;
   - memory extension for future memory consolidation guidance;
   - handoff for temporary cross-session state.

## Design Notes

- [`docs/gstack-lessons.md`](docs/gstack-lessons.md): lessons from Garry Tan's gstack and the thin-harness, fat-skills model.

## Development

```bash
cargo build
cargo test
cargo run -p codex-threads -- --json doctor
```

Install the local CLI, plugin cache copy, and memory extension:

```bash
plugins/victor-workflows/scripts/install.sh
```
