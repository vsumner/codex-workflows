# Codex Workflows

This repo packages Victor's Codex-native workflow layer. It is the Codex counterpart to `vsumner/claudify`, adapted to Codex's actual plugin, skill, memory-extension, and CLI surfaces.

- Do not port Claudify wholesale. Reuse the methodology, not the implementation shape.
- Treat repeated workflow friction as evidence for what to build, not as permission to add machinery.
- Choose the smallest durable artifact:
  - skill for routing and operating guidance;
  - CLI for noisy local state that needs a bounded JSON interface;
  - memory extension for consolidation guidance;
  - handoff or validation artifact for cross-session state.
- Prefer small, agent-friendly CLIs over large prompt-only workflows.
- Keep plugin skills concise: explain when to use the CLI, the command order, output expectations, and safety rules.
- CLIs must support `--help`, `--json`, predictable exit codes, bounded output, and a `doctor` command when they touch local state.
- Do not add a CLI just to aggregate existing primitives like `git` or `gh`.
- Define the validation contract before designing a new workflow surface: what behavior should a fresh Codex session demonstrate after the change?
- Default new tooling to read-only behavior. Any command that writes outside its own cache/index must make that explicit in its name/help and require an intentional flag.
- Archive old workflow surfaces instead of deleting them when replacing the repo direction.
