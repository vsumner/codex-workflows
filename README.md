# Codex Workflows

Personal Codex workflow tools built around agent-friendly CLIs and small companion skills.

The old prompt/role mirror has been moved to `archive/legacy-codex-workflows-2026-04-11/`.

## Current Surface

- `codex-threads`: read-only CLI for searching and summarizing local Codex thread history.
- `plugins/victor-workflows`: Codex plugin bundle with skills that teach Codex how to use these CLIs.

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
