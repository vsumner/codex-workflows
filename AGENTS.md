# Codex Workflows

This repo contains Victor's Codex-specific workflow tools.

- Prefer small, agent-friendly CLIs over large prompt-only workflows.
- Keep plugin skills concise: explain when to use the CLI, the command order, output expectations, and safety rules.
- CLIs must support `--help`, `--json`, predictable exit codes, bounded output, and a `doctor` command when they touch local state.
- Default new tooling to read-only behavior. Any command that writes outside its own cache/index must make that explicit in its name/help and require an intentional flag.
- Archive old workflow surfaces instead of deleting them when replacing the repo direction.
